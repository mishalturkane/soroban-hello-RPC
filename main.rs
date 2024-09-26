use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::error::Error;

#[derive(Debug, Serialize, Deserialize)]
struct SorobanRequest {
    method: String,
    params: Vec<String>,
    jsonrpc: String,
    id: u32,
}

#[derive(Debug, Serialize, Deserialize)]
struct SorobanResponse {
    jsonrpc: String,
    result: Option<serde_json::Value>,
    error: Option<serde_json::Value>,
    id: u32,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let client = Client::new();

    // Example Soroban RPC request to get contract info
    let request = SorobanRequest {
        method: "getContract".to_string(),
        params: vec!["<contract_id>".to_string()],
        jsonrpc: "2.0".to_string(),
        id: 1,
    };

    let response: SorobanResponse = client
        .post("https://horizon-testnet.stellar.org/soroban/rpc")
        .json(&request)
        .send()
        .await?
        .json()
        .await?;

    if let Some(result) = response.result {
        println!("Result: {:?}", result);
    } else if let Some(error) = response.error {
        println!("Error: {:?}", error);
    }

    Ok(())
}
