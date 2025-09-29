use alloy_signer_local::PrivateKeySigner;
use dotenvy::dotenv;
use reqwest::Client;
use solana_client::nonblocking::rpc_client::RpcClient;
use solana_keypair::Keypair;
use std::env;
use std::sync::Arc;
use awesome402_reqwest::{ReqwestWithPayments, ReqwestWithPaymentsBuild, Awesome402Client};
use awesome402::scheme::v1_eip155_exact::client::V1Eip155ExactClient;
use awesome402::scheme::v1_solana_exact::client::V1SolanaExactClient;
use awesome402::scheme::v2_eip155_exact::client::V2Eip155ExactClient;
use awesome402::scheme::v2_solana_exact::client::V2SolanaExactClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    let mut awesome402_client = Awesome402Client::new();
    // Register eip155 "exact" scheme
    {
        let signer: Option<PrivateKeySigner> = env::var("EVM_PRIVATE_KEY")
            .ok()
            .and_then(|key| key.parse().ok());
        if let Some(signer) = signer {
            println!("Using EVM signer address: {:?}", signer.address());
            let signer = Arc::new(signer);
            awesome402_client = awesome402_client
                .register(V1Eip155ExactClient::new(signer.clone()))
                .register(V2Eip155ExactClient::new(signer));
            println!("Enabled eip155 exact scheme")
        }
    };

    // Register solana "exact" scheme
    {
        let keypair = env::var("SOLANA_PRIVATE_KEY")
            .ok()
            .map(|v| Keypair::from_base58_string(&v));
        let rpc_client = env::var("SOLANA_RPC_URL").ok().map(|v| RpcClient::new(v));
        if let Some((keypair, rpc_client)) = keypair.zip(rpc_client) {
            let keypair = Arc::new(keypair);
            let rpc_client = Arc::new(rpc_client);
            awesome402_client = awesome402_client
                .register(V1SolanaExactClient::new(
                    keypair.clone(),
                    rpc_client.clone(),
                ))
                .register(V2SolanaExactClient::new(keypair, rpc_client));
            println!("Enabled solana exact scheme")
        }
    }

    let http_client = Client::new().with_payments(awesome402_client).build();

    let response = http_client
        .get("http://localhost:3000/protected-route")
        .send()
        .await?;

    println!("Response: {:?}", response.text().await?);

    Ok(())
}
