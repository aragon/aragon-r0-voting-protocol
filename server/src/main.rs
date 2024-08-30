use axum::{http::StatusCode, routing::post, Json, Router};
use dotenvy::dotenv;
use serde::{Deserialize, Serialize};
use std::process::Command;

#[tokio::main]
async fn main() {
    dotenv().ok();
    tracing_subscriber::fmt::init();

    let app = Router::new().route("/", post(root));

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

// basic handler that responds with a static string
async fn root(Json(payload): Json<VotingParams>) -> (StatusCode, Json<VotingResponse>) {
    let output = Command::new("./publisher")
        .current_dir("../target/release/")
        // .arg("--")
        .arg(format!(
            "--chain-id={}",
            std::env::var("CHAIN_ID").unwrap_or_else(|_| "11155111".to_string())
        ))
        .arg(format!(
            "--rpc-url={}",
            std::env::var("RPC_URL").unwrap_or_else(|_| "11155111".to_string())
        ))
        .arg(format!("--block-number={}", payload.block_number))
        .arg(format!("--voter-signature={}", payload.voter_signature))
        .arg(format!("--voter={}", payload.voter))
        .arg(format!("--dao-address={}", payload.dao_address))
        .arg(format!("--proposal-id={}", payload.proposal_id))
        .arg(format!("--direction={}", payload.direction))
        .arg(format!("--balance={}", payload.balance))
        .arg(format!("--config-contract={}", payload.config_contract))
        .arg(format!("--token={}", payload.token_address))
        .output()
        .expect("Failed to execute command");

    let success = output.status.success();
    let message = if success {
        String::from_utf8_lossy(&output.stdout).to_string()
    } else {
        String::from_utf8_lossy(&output.stderr).to_string()
    };

    let response = VotingResponse { success, message };

    (
        if success {
            StatusCode::OK
        } else {
            StatusCode::INTERNAL_SERVER_ERROR
        },
        Json(response),
    )
}

#[derive(Deserialize)]
struct VotingParams {
    block_number: String,
    voter_signature: String,
    voter: String,
    dao_address: String,
    proposal_id: String,
    direction: u8,
    balance: String,
    config_contract: String,
    token_address: String,
}

#[derive(Serialize)]
struct VotingResponse {
    success: bool,
    message: String,
}

