
#![forbid(unused_imports)]
use std::sync::Arc;
use dotenv::dotenv;
use ethers::contract::abigen;
use ethers::providers::{Provider, ProviderError};
use ethers::types::Address;
use thiserror::Error;
use url::ParseError;

abigen!(IERC20, "./src/constants/IERC20.json");

#[derive(Error, Debug)]
enum Error {
    #[error("provider error: {0}")]
    ProviderError(#[from] ProviderError),

    #[error("parse error: {0}")]
    ParseError(#[from] ParseError),

    #[error("environment variable error: {0}")]
    VarError(#[from] std::env::VarError),
}
#[tokio::main]
async fn main() -> Result<(), Error> {
    dotenv().ok();
    let mainnet_rpc = std::env::var("MAINNET_RPC_URL")?;
    let provider = Arc::new(Provider::try_from(mainnet_rpc)?);
    let usdt_address: Address = "0xdAC17F958D2ee523a2206206994597C13D831ec7"
        .parse()
        .unwrap();
    let usdt_contract = IERC20::new(usdt_address, provider);
    let decimals_raw = usdt_contract.decimals().call().await.unwrap();
    let name = usdt_contract.name().call().await.unwrap();
    let symbol = usdt_contract.symbol().call().await.unwrap();
    let decimals = decimals_raw.to_string().chars().filter(|c| c.is_numeric() && c > &'0').collect::<String>();
    
    println!("Name: {:?}", name);
    println!("Symbol: {:?}", symbol);
    println!("Decimals: {:?}", decimals);
    

    Ok(())
}
