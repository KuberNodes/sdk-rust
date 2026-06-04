// AUTO-GENERATED — do not edit. Run `npm run generate:sdk` to rebuild.
#![allow(unused_variables)]
use serde_json::Value;
use crate::Result;

impl crate::KuberNodes {
    /// Returns the number of the most recent block.
    pub async fn eth_block_number(&self, chain_id: Option<u64>) -> Result<Value> {
        self.send("eth_blockNumber", vec![], chain_id).await
    }

    /// Returns the EIP-155 chain ID used for transaction signing.
    pub async fn eth_chain_id(&self, chain_id: Option<u64>) -> Result<Value> {
        self.send("eth_chainId", vec![], chain_id).await
    }

    /// Returns the current gas price in wei.
    pub async fn eth_gas_price(&self, chain_id: Option<u64>) -> Result<Value> {
        self.send("eth_gasPrice", vec![], chain_id).await
    }

    /// Returns the current maxPriorityFeePerGas (miner tip) in wei as a hex string.
    pub async fn eth_max_priority_fee_per_gas(&self, chain_id: Option<u64>) -> Result<Value> {
        self.send("eth_maxPriorityFeePerGas", vec![], chain_id).await
    }

    /// Returns an object with the node sync status, or false if the node is fully synced.
    pub async fn eth_syncing(&self, chain_id: Option<u64>) -> Result<Value> {
        self.send("eth_syncing", vec![], chain_id).await
    }

    /// Returns historical base fee, gas used ratio, and optional reward percentiles for a range of blocks.
    pub async fn eth_fee_history(&self, blockCount: serde_json::Value, newestBlock: String, rewardPercentiles: Vec<u64>, chain_id: Option<u64>) -> Result<Value> {
        self.send("eth_feeHistory", vec![serde_json::to_value(&blockCount)?, serde_json::to_value(&newestBlock)?, serde_json::to_value(&rewardPercentiles)?], chain_id).await
    }

    /// Returns the ether balance of an account at the given block.
    pub async fn eth_get_balance(&self, address: String, block: String, chain_id: Option<u64>) -> Result<Value> {
        self.send("eth_getBalance", vec![serde_json::to_value(&address)?, serde_json::to_value(&block)?], chain_id).await
    }

    /// Returns the number of transactions sent from an address (nonce) at the given block.
    pub async fn eth_get_transaction_count(&self, address: String, block: String, chain_id: Option<u64>) -> Result<Value> {
        self.send("eth_getTransactionCount", vec![serde_json::to_value(&address)?, serde_json::to_value(&block)?], chain_id).await
    }

    /// Returns the contract bytecode deployed at the given address.
    pub async fn eth_get_code(&self, address: String, block: String, chain_id: Option<u64>) -> Result<Value> {
        self.send("eth_getCode", vec![serde_json::to_value(&address)?, serde_json::to_value(&block)?], chain_id).await
    }

    /// Returns the value stored at a specific position in a contract's storage.
    pub async fn eth_get_storage_at(&self, address: String, position: String, block: String, chain_id: Option<u64>) -> Result<Value> {
        self.send("eth_getStorageAt", vec![serde_json::to_value(&address)?, serde_json::to_value(&position)?, serde_json::to_value(&block)?], chain_id).await
    }

    /// Executes a message call without creating a transaction on the blockchain. Useful for reading contract state.
    pub async fn eth_call(&self, transaction: serde_json::Value, block: String, chain_id: Option<u64>) -> Result<Value> {
        self.send("eth_call", vec![serde_json::to_value(&transaction)?, serde_json::to_value(&block)?], chain_id).await
    }

    /// Generates and returns an estimate of how much gas is necessary to execute the given transaction.
    pub async fn eth_estimate_gas(&self, transaction: serde_json::Value, block: String, chain_id: Option<u64>) -> Result<Value> {
        self.send("eth_estimateGas", vec![serde_json::to_value(&transaction)?, serde_json::to_value(&block)?], chain_id).await
    }

    /// Creates an EIP-2930 access list for the given transaction, reducing gas cost for subsequent calls.
    pub async fn eth_create_access_list(&self, transaction: serde_json::Value, block: String, chain_id: Option<u64>) -> Result<Value> {
        self.send("eth_createAccessList", vec![serde_json::to_value(&transaction)?, serde_json::to_value(&block)?], chain_id).await
    }

    /// Submits a pre-signed raw transaction for execution. Returns the transaction hash.
    pub async fn eth_send_raw_transaction(&self, signedTxData: String, chain_id: Option<u64>) -> Result<Value> {
        self.send("eth_sendRawTransaction", vec![serde_json::to_value(&signedTxData)?], chain_id).await
    }

    /// Returns information about a block identified by its number.
    pub async fn eth_get_block_by_number(&self, blockNumber: String, fullTransactions: bool, chain_id: Option<u64>) -> Result<Value> {
        self.send("eth_getBlockByNumber", vec![serde_json::to_value(&blockNumber)?, serde_json::to_value(&fullTransactions)?], chain_id).await
    }

    /// Returns information about a block identified by its hash.
    pub async fn eth_get_block_by_hash(&self, blockHash: String, fullTransactions: bool, chain_id: Option<u64>) -> Result<Value> {
        self.send("eth_getBlockByHash", vec![serde_json::to_value(&blockHash)?, serde_json::to_value(&fullTransactions)?], chain_id).await
    }

    /// Returns all transaction receipts for a given block (Alchemy/Erigon extension).
    pub async fn eth_get_block_receipts(&self, blockNumber: String, chain_id: Option<u64>) -> Result<Value> {
        self.send("eth_getBlockReceipts", vec![serde_json::to_value(&blockNumber)?], chain_id).await
    }

    /// Returns the number of transactions in a block matching the given block number.
    pub async fn eth_get_block_transaction_count_by_number(&self, blockNumber: String, chain_id: Option<u64>) -> Result<Value> {
        self.send("eth_getBlockTransactionCountByNumber", vec![serde_json::to_value(&blockNumber)?], chain_id).await
    }

    /// Returns the number of transactions in a block matching the given block hash.
    pub async fn eth_get_block_transaction_count_by_hash(&self, blockHash: String, chain_id: Option<u64>) -> Result<Value> {
        self.send("eth_getBlockTransactionCountByHash", vec![serde_json::to_value(&blockHash)?], chain_id).await
    }

    /// Returns the information about a transaction requested by its hash.
    pub async fn eth_get_transaction_by_hash(&self, txHash: String, chain_id: Option<u64>) -> Result<Value> {
        self.send("eth_getTransactionByHash", vec![serde_json::to_value(&txHash)?], chain_id).await
    }

    /// Returns the receipt of a transaction by its hash. Available only after the transaction is mined.
    pub async fn eth_get_transaction_receipt(&self, txHash: String, chain_id: Option<u64>) -> Result<Value> {
        self.send("eth_getTransactionReceipt", vec![serde_json::to_value(&txHash)?], chain_id).await
    }

    /// Returns information about a transaction by block number and transaction index position.
    pub async fn eth_get_transaction_by_block_number_and_index(&self, blockNumber: String, index: String, chain_id: Option<u64>) -> Result<Value> {
        self.send("eth_getTransactionByBlockNumberAndIndex", vec![serde_json::to_value(&blockNumber)?, serde_json::to_value(&index)?], chain_id).await
    }

    /// Returns information about a transaction by block hash and transaction index position.
    pub async fn eth_get_transaction_by_block_hash_and_index(&self, blockHash: String, index: String, chain_id: Option<u64>) -> Result<Value> {
        self.send("eth_getTransactionByBlockHashAndIndex", vec![serde_json::to_value(&blockHash)?, serde_json::to_value(&index)?], chain_id).await
    }

    /// Returns an array of logs matching the given filter object.
    pub async fn eth_get_logs(&self, filter: serde_json::Value, chain_id: Option<u64>) -> Result<Value> {
        self.send("eth_getLogs", vec![serde_json::to_value(&filter)?], chain_id).await
    }

    /// Creates a filter object to notify when the state changes (logs). Returns a filter ID.
    pub async fn eth_new_filter(&self, filter: serde_json::Value, chain_id: Option<u64>) -> Result<Value> {
        self.send("eth_newFilter", vec![serde_json::to_value(&filter)?], chain_id).await
    }

    /// Creates a filter that fires each time a new block is added to the chain. Returns a filter ID.
    pub async fn eth_new_block_filter(&self, chain_id: Option<u64>) -> Result<Value> {
        self.send("eth_newBlockFilter", vec![], chain_id).await
    }

    /// Creates a filter that returns new pending transactions. Returns a filter ID.
    pub async fn eth_new_pending_transaction_filter(&self, chain_id: Option<u64>) -> Result<Value> {
        self.send("eth_newPendingTransactionFilter", vec![], chain_id).await
    }

    /// Polling method for a filter. Returns an array of new logs, block hashes, or tx hashes since last poll.
    pub async fn eth_get_filter_changes(&self, filterId: String, chain_id: Option<u64>) -> Result<Value> {
        self.send("eth_getFilterChanges", vec![serde_json::to_value(&filterId)?], chain_id).await
    }

    /// Returns an array of all logs matching the filter with the given ID.
    pub async fn eth_get_filter_logs(&self, filterId: String, chain_id: Option<u64>) -> Result<Value> {
        self.send("eth_getFilterLogs", vec![serde_json::to_value(&filterId)?], chain_id).await
    }

    /// Uninstalls a filter with the given ID. Should always be called when the filter is no longer needed.
    pub async fn eth_uninstall_filter(&self, filterId: String, chain_id: Option<u64>) -> Result<Value> {
        self.send("eth_uninstallFilter", vec![serde_json::to_value(&filterId)?], chain_id).await
    }

    /// Returns the current network ID as a string (e.g. "1" for Ethereum mainnet).
    pub async fn net_version(&self, chain_id: Option<u64>) -> Result<Value> {
        self.send("net_version", vec![], chain_id).await
    }

    /// Returns true if the client is actively listening for network connections.
    pub async fn net_listening(&self, chain_id: Option<u64>) -> Result<Value> {
        self.send("net_listening", vec![], chain_id).await
    }

    /// Returns the current client version string (e.g. "Geth/v1.13.0/linux-amd64").
    pub async fn web3_client_version(&self, chain_id: Option<u64>) -> Result<Value> {
        self.send("web3_clientVersion", vec![], chain_id).await
    }

    /// Returns the Keccak-256 (not the standardised SHA3-256) of the given data.
    pub async fn web3_sha3(&self, data: String, chain_id: Option<u64>) -> Result<Value> {
        self.send("web3_sha3", vec![serde_json::to_value(&data)?], chain_id).await
    }

    /// Replays a transaction and returns a full EVM execution trace with opcode-level detail.
    pub async fn debug_trace_transaction(&self, txHash: String, options: serde_json::Value, chain_id: Option<u64>) -> Result<Value> {
        self.send("debug_traceTransaction", vec![serde_json::to_value(&txHash)?, serde_json::to_value(&options)?], chain_id).await
    }

    /// Traces an eth_call against a given block without creating a transaction. Useful for pre-deploying contracts.
    pub async fn debug_trace_call(&self, transaction: serde_json::Value, block: String, options: serde_json::Value, chain_id: Option<u64>) -> Result<Value> {
        self.send("debug_traceCall", vec![serde_json::to_value(&transaction)?, serde_json::to_value(&block)?, serde_json::to_value(&options)?], chain_id).await
    }

    /// Replays a block and returns execution traces for all transactions in the block, identified by number.
    pub async fn debug_trace_block_by_number(&self, blockNumber: String, options: serde_json::Value, chain_id: Option<u64>) -> Result<Value> {
        self.send("debug_traceBlockByNumber", vec![serde_json::to_value(&blockNumber)?, serde_json::to_value(&options)?], chain_id).await
    }

    /// Replays a block and returns execution traces for all transactions in the block, identified by hash.
    pub async fn debug_trace_block_by_hash(&self, blockHash: String, options: serde_json::Value, chain_id: Option<u64>) -> Result<Value> {
        self.send("debug_traceBlockByHash", vec![serde_json::to_value(&blockHash)?, serde_json::to_value(&options)?], chain_id).await
    }

    /// Returns all traces generated by a transaction (Parity/OpenEthereum-compatible trace API).
    pub async fn trace_transaction(&self, txHash: String, chain_id: Option<u64>) -> Result<Value> {
        self.send("trace_transaction", vec![serde_json::to_value(&txHash)?], chain_id).await
    }

    /// Returns traces created at the given block.
    pub async fn trace_block(&self, blockNumber: String, chain_id: Option<u64>) -> Result<Value> {
        self.send("trace_block", vec![serde_json::to_value(&blockNumber)?], chain_id).await
    }

    /// Executes a new message call and returns a number of traces.
    pub async fn trace_call(&self, transaction: serde_json::Value, traceTypes: Vec<String>, block: String, chain_id: Option<u64>) -> Result<Value> {
        self.send("trace_call", vec![serde_json::to_value(&transaction)?, serde_json::to_value(&traceTypes)?, serde_json::to_value(&block)?], chain_id).await
    }

    /// Returns traces matching a given filter object.
    pub async fn trace_filter(&self, filter: serde_json::Value, chain_id: Option<u64>) -> Result<Value> {
        self.send("trace_filter", vec![serde_json::to_value(&filter)?], chain_id).await
    }

    /// Replays a transaction and returns the requested trace types.
    pub async fn trace_replay_transaction(&self, txHash: String, traceTypes: Vec<String>, chain_id: Option<u64>) -> Result<Value> {
        self.send("trace_replayTransaction", vec![serde_json::to_value(&txHash)?, serde_json::to_value(&traceTypes)?], chain_id).await
    }

    /// Replays all transactions in a block and returns the requested trace types for each.
    pub async fn trace_replay_block_transactions(&self, blockNumber: String, traceTypes: Vec<String>, chain_id: Option<u64>) -> Result<Value> {
        self.send("trace_replayBlockTransactions", vec![serde_json::to_value(&blockNumber)?, serde_json::to_value(&traceTypes)?], chain_id).await
    }

    /// Executes a list of call traces on top of a block.
    pub async fn trace_call_many(&self, calls: serde_json::Value, block: String, chain_id: Option<u64>) -> Result<Value> {
        self.send("trace_callMany", vec![serde_json::to_value(&calls)?, serde_json::to_value(&block)?], chain_id).await
    }

    /// Submits a bundle of transactions to an MEV relay. Bundles are included atomically.
    pub async fn eth_send_bundle(&self, bundle: serde_json::Value, chain_id: Option<u64>) -> Result<Value> {
        self.send("eth_sendBundle", vec![serde_json::to_value(&bundle)?], chain_id).await
    }

    /// Simulates a bundle of transactions without submitting them, returning execution results.
    pub async fn eth_call_bundle(&self, bundle: serde_json::Value, chain_id: Option<u64>) -> Result<Value> {
        self.send("eth_callBundle", vec![serde_json::to_value(&bundle)?], chain_id).await
    }

    /// Cancels a previously submitted bundle by its replacement UUID.
    pub async fn eth_cancel_bundle(&self, replacementUuid: String, chain_id: Option<u64>) -> Result<Value> {
        self.send("eth_cancelBundle", vec![serde_json::to_value(&replacementUuid)?], chain_id).await
    }

    /// Returns stats for a submitted bundle including simulation success and relay timing.
    pub async fn mev_get_bundle_stats(&self, bundleHash: String, blockNumber: String, chain_id: Option<u64>) -> Result<Value> {
        self.send("mev_getBundleStats", vec![serde_json::to_value(&bundleHash)?, serde_json::to_value(&blockNumber)?], chain_id).await
    }

    /// Returns the current block number (Erigon-specific alias for eth_blockNumber).
    pub async fn erigon_block_number(&self, chain_id: Option<u64>) -> Result<Value> {
        self.send("erigon_blockNumber", vec![], chain_id).await
    }

    /// Returns the block closest to the given Unix timestamp (Erigon extension).
    pub async fn erigon_get_block_by_timestamp(&self, timestamp: String, fullTransactions: bool, chain_id: Option<u64>) -> Result<Value> {
        self.send("erigon_getBlockByTimestamp", vec![serde_json::to_value(&timestamp)?, serde_json::to_value(&fullTransactions)?], chain_id).await
    }

    /// Returns the block header for a given block number without the full body (Erigon extension).
    pub async fn erigon_get_header_by_number(&self, blockNumber: String, chain_id: Option<u64>) -> Result<Value> {
        self.send("erigon_getHeaderByNumber", vec![serde_json::to_value(&blockNumber)?], chain_id).await
    }

    /// Returns a list of known network forks and the block numbers at which they occurred (Erigon extension).
    pub async fn erigon_forks(&self, chain_id: Option<u64>) -> Result<Value> {
        self.send("erigon_forks", vec![], chain_id).await
    }

    /// Returns information about the Erigon node (version, ports, protocols).
    pub async fn erigon_node_info(&self, chain_id: Option<u64>) -> Result<Value> {
        self.send("erigon_nodeInfo", vec![], chain_id).await
    }
}
