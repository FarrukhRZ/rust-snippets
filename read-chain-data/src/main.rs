// #![forbid(unused_imports)]
use ethers::providers::{Http, Middleware, Provider, ProviderError};
use ethers::types::{U64, U256};
use dotenv::dotenv;
use url::{ParseError};

#[tokio::main]
async fn main() -> Result<(),ParseError>{
    dotenv().ok();
    let mainnet_rpc = std::env::var("MAINNET_RPC_URL").expect("MAINNET_RPC_URL is not set in .env");
    let provider = Provider::try_from(mainnet_rpc)?;
    let block_number = read_block_number(&provider).await;
    println!("Block number: {:?}", block_number.unwrap());
    let chain_id = read_chain_id(&provider).await;
    println!("Chain id: {:?}", chain_id.unwrap());
    Ok(())
}

async fn read_block_number(provider: &Provider<Http>) -> Result<U64, ProviderError> {
    provider.get_block_number().await
}

async fn read_chain_id(provider: &Provider<Http>) -> Result<U256, ProviderError> {
    provider.get_chainid().await
}