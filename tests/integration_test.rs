use kubernodes::{Error, KuberNodes};
use wiremock::matchers::{method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

fn mock_response(result: serde_json::Value) -> ResponseTemplate {
    ResponseTemplate::new(200).set_body_json(serde_json::json!({
        "jsonrpc": "2.0",
        "id": 1,
        "result": result,
    }))
}

#[tokio::test]
async fn test_get_block_number() {
    let server = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path("/api/rpc/11155111"))
        .respond_with(mock_response(serde_json::json!("0x12d687")))
        .mount(&server)
        .await;

    let kn = KuberNodes::new("pk_test")
        .with_base_url(server.uri())
        .with_retries(0);

    let n = kn.get_block_number(None).await.unwrap();
    assert_eq!(n, 0x12d687);
}

#[tokio::test]
async fn test_get_balance() {
    let server = MockServer::start().await;
    Mock::given(method("POST"))
        .respond_with(mock_response(serde_json::json!("0xde0b6b3a7640000")))
        .mount(&server)
        .await;

    let kn = KuberNodes::new("pk_test")
        .with_base_url(server.uri())
        .with_retries(0);

    let bal = kn.get_balance("0xdeadbeef", None, None).await.unwrap();
    assert_eq!(bal, 10u128.pow(18));
}

#[tokio::test]
async fn test_rpc_error() {
    let server = MockServer::start().await;
    Mock::given(method("POST"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "jsonrpc": "2.0",
            "id": 1,
            "error": { "code": -32601, "message": "Method not found" }
        })))
        .mount(&server)
        .await;

    let kn = KuberNodes::new("pk_test")
        .with_base_url(server.uri())
        .with_retries(0);

    let result = kn.get_block_number(None).await;
    assert!(matches!(result, Err(Error::Rpc { code: -32601, .. })));
}

#[test]
fn test_api_key_required() {
    let result = std::panic::catch_unwind(|| KuberNodes::new(""));
    assert!(result.is_err());
}

#[test]
fn test_chain_scoping() {
    let kn = KuberNodes::new("pk_test");
    let eth = kn.chain(1);
    // Verify the scoped client sends to chain 1
    assert_eq!(eth.chain_id_for_test(), 1);
}
