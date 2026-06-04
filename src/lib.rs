//! KuberNodes Rust SDK — multi-chain RPC infrastructure.
//!
//! # Quick Start
//!
//! ```no_run
//! use kubernodes::KuberNodes;
//!
//! #[tokio::main]
//! async fn main() {
//!     let kn = KuberNodes::new("pk_live_your_key")
//!         .with_chain_id(1);
//!     let block = kn.get_block_number(None).await.unwrap();
//!     println!("Block: {block}");
//! }
//! ```

use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;
use std::time::Duration;

use reqwest::Client as HttpClient;
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use serde_json::Value;
use thiserror::Error;

// ── Errors ───────────────────────────────────────────────────────────────────

#[derive(Debug, Error)]
pub enum Error {
    #[error("HTTP error {status}: {body}")]
    Http { status: u16, body: String },

    #[error("RPC error {code}: {message}")]
    Rpc { code: i64, message: String },

    #[error("Request error: {0}")]
    Request(#[from] reqwest::Error),

    #[error("Serialization error: {0}")]
    Serde(#[from] serde_json::Error),

    #[error("{0}")]
    Other(String),
}

pub type Result<T> = std::result::Result<T, Error>;

// ── Chain registry ───────────────────────────────────────────────────────────

#[derive(Debug, Clone)]
pub struct ChainInfo {
    pub chain_id: u64,
    pub name: &'static str,
    pub slug: &'static str,
    pub network: &'static str,
    pub native_currency: &'static str,
}

pub const CHAINS: &[ChainInfo] = &[
    ChainInfo { chain_id: 1,        name: "Ethereum",          slug: "eth-mainnet",  network: "mainnet", native_currency: "ETH"  },
    ChainInfo { chain_id: 42161,    name: "Arbitrum One",      slug: "arb-one",      network: "mainnet", native_currency: "ETH"  },
    ChainInfo { chain_id: 8453,     name: "Base",              slug: "base",         network: "mainnet", native_currency: "ETH"  },
    ChainInfo { chain_id: 10,       name: "Optimism",          slug: "optimism",     network: "mainnet", native_currency: "ETH"  },
    ChainInfo { chain_id: 137,      name: "Polygon",           slug: "polygon",      network: "mainnet", native_currency: "MATIC"},
    ChainInfo { chain_id: 56,       name: "BNB Smart Chain",   slug: "bsc",          network: "mainnet", native_currency: "BNB"  },
    ChainInfo { chain_id: 43114,    name: "Avalanche C-Chain", slug: "avax-c",       network: "mainnet", native_currency: "AVAX" },
    ChainInfo { chain_id: 534352,   name: "Scroll",            slug: "scroll",       network: "mainnet", native_currency: "ETH"  },
    ChainInfo { chain_id: 324,      name: "zkSync Era",        slug: "zksync-era",   network: "mainnet", native_currency: "ETH"  },
    ChainInfo { chain_id: 59144,    name: "Linea",             slug: "linea",        network: "mainnet", native_currency: "ETH"  },
    ChainInfo { chain_id: 11155111, name: "Ethereum Sepolia",  slug: "sepolia",      network: "testnet", native_currency: "ETH"  },
    ChainInfo { chain_id: 421614,   name: "Arbitrum Sepolia",  slug: "arb-sepolia",  network: "testnet", native_currency: "ETH"  },
    ChainInfo { chain_id: 84532,    name: "Base Sepolia",      slug: "base-sepolia", network: "testnet", native_currency: "ETH"  },
    ChainInfo { chain_id: 11155420, name: "Optimism Sepolia",  slug: "op-sepolia",   network: "testnet", native_currency: "ETH"  },
    ChainInfo { chain_id: 80002,    name: "Polygon Amoy",      slug: "polygon-amoy", network: "testnet", native_currency: "MATIC"},
];

pub fn chain_id_to_slug(chain_id: u64) -> String {
    CHAINS
        .iter()
        .find(|c| c.chain_id == chain_id)
        .map(|c| c.slug.to_owned())
        .unwrap_or_else(|| chain_id.to_string())
}

// ── JSON-RPC types ────────────────────────────────────────────────────────────

#[derive(Serialize)]
struct RpcRequest<'a> {
    jsonrpc: &'static str,
    id: u64,
    method: &'a str,
    params: Value,
}

#[derive(Deserialize)]
struct RpcResponse {
    result: Option<Value>,
    error: Option<RpcErrorBody>,
}

#[derive(Deserialize)]
struct RpcErrorBody {
    code: i64,
    message: String,
}

// ── Response types ────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Block {
    pub hash: String,
    pub parent_hash: String,
    pub number: String,
    pub timestamp: String,
    pub gas_limit: String,
    pub gas_used: String,
    pub miner: String,
    pub extra_data: String,
    #[serde(default)]
    pub transactions: Vec<Value>,
    #[serde(default)]
    pub uncles: Vec<String>,
    pub base_fee_per_gas: Option<String>,
}

impl Block {
    pub fn number_int(&self) -> u64 {
        u64::from_str_radix(self.number.trim_start_matches("0x"), 16).unwrap_or(0)
    }
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Transaction {
    pub hash: String,
    pub block_hash: Option<String>,
    pub block_number: Option<String>,
    pub from: String,
    pub to: Option<String>,
    pub value: String,
    pub gas: String,
    pub gas_price: String,
    pub input: String,
    pub nonce: String,
    pub transaction_index: Option<String>,
    #[serde(rename = "type")]
    pub tx_type: String,
    pub chain_id: Option<String>,
    pub max_fee_per_gas: Option<String>,
    pub max_priority_fee_per_gas: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Log {
    pub address: String,
    pub topics: Vec<String>,
    pub data: String,
    pub block_number: String,
    pub block_hash: String,
    pub transaction_hash: String,
    pub transaction_index: String,
    pub log_index: String,
    pub removed: bool,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TransactionReceipt {
    pub transaction_hash: String,
    pub block_number: String,
    pub block_hash: String,
    pub from: String,
    pub to: Option<String>,
    pub contract_address: Option<String>,
    pub cumulative_gas_used: String,
    pub effective_gas_price: String,
    pub gas_used: String,
    pub logs: Vec<Log>,
    pub status: String,
    #[serde(rename = "type")]
    pub tx_type: String,
}

impl TransactionReceipt {
    pub fn success(&self) -> bool {
        self.status == "0x1"
    }
}

// ── Client ────────────────────────────────────────────────────────────────────

#[derive(Clone)]
pub struct KuberNodes {
    api_key: String,
    access_token: Option<String>,
    base_url: String,
    chain_id: u64,
    retries: u32,
    http: HttpClient,
    req_id: Arc<AtomicU64>,
}

impl KuberNodes {
    /// Create a new client. Panics if `api_key` is empty.
    pub fn new(api_key: impl Into<String>) -> Self {
        let api_key = api_key.into();
        assert!(!api_key.is_empty(), "kubernodes: api_key is required");
        Self {
            api_key,
            access_token: None,
            base_url: "https://rpc.kubernodes.com".to_owned(),
            chain_id: 11155111,
            retries: 3,
            http: HttpClient::builder()
                .timeout(Duration::from_secs(30))
                .build()
                .expect("failed to build HTTP client"),
            req_id: Arc::new(AtomicU64::new(0)),
        }
    }

    pub fn with_base_url(mut self, url: impl Into<String>) -> Self {
        let u: String = url.into().trim_end_matches('/').to_owned();
        if !u.starts_with("https://") && !u.starts_with("http://localhost") && !u.starts_with("http://127.0.0.1") {
            panic!("base_url must use https:// (or http://localhost for dev)");
        }
        self.base_url = u;
        self
    }

    pub fn with_chain_id(mut self, id: u64) -> Self {
        self.chain_id = id;
        self
    }

    pub fn with_retries(mut self, n: u32) -> Self {
        self.retries = n;
        self
    }

    pub fn with_access_token(mut self, tok: impl Into<String>) -> Self {
        self.access_token = Some(tok.into());
        self
    }

    /// Returns a cloned client scoped to `chain_id`.
    pub fn chain(&self, chain_id: u64) -> Self {
        let mut c = self.clone();
        c.chain_id = chain_id;
        c
    }

    /// Exposes chain_id for testing.
    #[doc(hidden)]
    pub fn chain_id_for_test(&self) -> u64 {
        self.chain_id
    }

    fn effective_chain(&self, override_id: Option<u64>) -> u64 {
        override_id.unwrap_or(self.chain_id)
    }

    fn rpc_url(&self, chain_id: u64) -> String {
        format!("{}/api/rpc/{}", self.base_url, chain_id)
    }

    fn next_id(&self) -> u64 {
        self.req_id.fetch_add(1, Ordering::Relaxed) + 1
    }

    /// Send a raw JSON-RPC request with retries.
    pub async fn send<T: DeserializeOwned>(
        &self,
        method: &str,
        params: Value,
        chain_id: Option<u64>,
    ) -> Result<T> {
        let cid = self.effective_chain(chain_id);
        let url = self.rpc_url(cid);
        let mut last_err: Option<Error> = None;

        for attempt in 0..=self.retries {
            let payload = RpcRequest {
                jsonrpc: "2.0",
                id: self.next_id(),
                method,
                params: params.clone(),
            };
            let res = self
                .http
                .post(&url)
                .header("X-API-Key", &self.api_key)
                .json(&payload)
                .send()
                .await;

            match res {
                Err(e) => {
                    last_err = Some(Error::Request(e));
                    if attempt < self.retries {
                        tokio::time::sleep(backoff(attempt)).await;
                    }
                }
                Ok(resp) => {
                    let status = resp.status().as_u16();
                    let body = resp.text().await.unwrap_or_default();
                    if status >= 400 {
                        last_err = Some(Error::Http { status, body });
                        if attempt < self.retries {
                            tokio::time::sleep(backoff(attempt)).await;
                        }
                        continue;
                    }
                    let rr: RpcResponse = serde_json::from_str(&body)?;
                    if let Some(e) = rr.error {
                        return Err(Error::Rpc { code: e.code, message: e.message });
                    }
                    let result = rr.result.unwrap_or(Value::Null);
                    return Ok(serde_json::from_value(result)?);
                }
            }
        }
        Err(last_err.unwrap_or_else(|| Error::Other("request failed".to_owned())))
    }

    // ── Block methods ─────────────────────────────────────────────────────────

    pub async fn get_block_number(&self, chain_id: Option<u64>) -> Result<u64> {
        let h: String = self.send("eth_blockNumber", Value::Array(vec![]), chain_id).await?;
        Ok(u64::from_str_radix(h.trim_start_matches("0x"), 16)
            .map_err(|e| Error::Other(e.to_string()))?)
    }

    pub async fn get_chain_id(&self, chain_id: Option<u64>) -> Result<u64> {
        let h: String = self.send("eth_chainId", Value::Array(vec![]), chain_id).await?;
        Ok(u64::from_str_radix(h.trim_start_matches("0x"), 16)
            .map_err(|e| Error::Other(e.to_string()))?)
    }

    pub async fn get_block(
        &self,
        block: &str,
        full_txs: bool,
        chain_id: Option<u64>,
    ) -> Result<Block> {
        self.send(
            "eth_getBlockByNumber",
            serde_json::json!([block, full_txs]),
            chain_id,
        )
        .await
    }

    // ── Account methods ───────────────────────────────────────────────────────

    pub async fn get_balance(
        &self,
        address: &str,
        block: Option<&str>,
        chain_id: Option<u64>,
    ) -> Result<u128> {
        let h: String = self
            .send(
                "eth_getBalance",
                serde_json::json!([address, block.unwrap_or("latest")]),
                chain_id,
            )
            .await?;
        u128::from_str_radix(h.trim_start_matches("0x"), 16)
            .map_err(|e| Error::Other(e.to_string()))
    }

    pub async fn get_transaction_count(
        &self,
        address: &str,
        block: Option<&str>,
        chain_id: Option<u64>,
    ) -> Result<u64> {
        let h: String = self
            .send(
                "eth_getTransactionCount",
                serde_json::json!([address, block.unwrap_or("latest")]),
                chain_id,
            )
            .await?;
        u64::from_str_radix(h.trim_start_matches("0x"), 16)
            .map_err(|e| Error::Other(e.to_string()))
    }

    // ── Gas methods ───────────────────────────────────────────────────────────

    pub async fn get_gas_price(&self, chain_id: Option<u64>) -> Result<u128> {
        let h: String = self
            .send("eth_gasPrice", Value::Array(vec![]), chain_id)
            .await?;
        u128::from_str_radix(h.trim_start_matches("0x"), 16)
            .map_err(|e| Error::Other(e.to_string()))
    }

    pub async fn estimate_gas(
        &self,
        tx: Value,
        chain_id: Option<u64>,
    ) -> Result<u64> {
        let h: String = self
            .send("eth_estimateGas", serde_json::json!([tx]), chain_id)
            .await?;
        u64::from_str_radix(h.trim_start_matches("0x"), 16)
            .map_err(|e| Error::Other(e.to_string()))
    }

    // ── Transaction methods ───────────────────────────────────────────────────

    pub async fn get_transaction(
        &self,
        tx_hash: &str,
        chain_id: Option<u64>,
    ) -> Result<Option<Transaction>> {
        self.send("eth_getTransactionByHash", serde_json::json!([tx_hash]), chain_id)
            .await
    }

    pub async fn get_transaction_receipt(
        &self,
        tx_hash: &str,
        chain_id: Option<u64>,
    ) -> Result<Option<TransactionReceipt>> {
        self.send("eth_getTransactionReceipt", serde_json::json!([tx_hash]), chain_id)
            .await
    }

    pub async fn send_raw_transaction(
        &self,
        signed_tx: &str,
        chain_id: Option<u64>,
    ) -> Result<String> {
        self.send("eth_sendRawTransaction", serde_json::json!([signed_tx]), chain_id)
            .await
    }

    // ── Contract/log methods ──────────────────────────────────────────────────

    pub async fn call(
        &self,
        tx: Value,
        block: Option<&str>,
        chain_id: Option<u64>,
    ) -> Result<String> {
        self.send(
            "eth_call",
            serde_json::json!([tx, block.unwrap_or("latest")]),
            chain_id,
        )
        .await
    }

    pub async fn get_logs(&self, filter: Value, chain_id: Option<u64>) -> Result<Vec<Log>> {
        self.send("eth_getLogs", serde_json::json!([filter]), chain_id)
            .await
    }
}

fn backoff(attempt: u32) -> Duration {
    let secs = 1u64.checked_shl(attempt).unwrap_or(u64::MAX).min(10);
    Duration::from_secs(secs)
}
