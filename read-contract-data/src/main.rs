#![forbid(unused_imports)]
use anyhow::Result;
use dotenv::dotenv;
use ethers::contract::abigen;
use ethers::providers::Provider;
use ethers::types::Address;
use std::sync::Arc;

abigen!(IERC20, "./src/abi/IERC20.json");

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok();

    let mainnet_rpc = std::env::var("RPC_URL_MAINNET")?;
    let provider = Arc::new(Provider::try_from(mainnet_rpc)?);

    let usdt_address: Address = "0xdAC17F958D2ee523a2206206994597C13D831ec7".parse()?;
    let usdt_contract = IERC20::new(usdt_address, provider);

    let decimals_raw = usdt_contract.decimals().call().await?;
    let name = usdt_contract.name().call().await?;
    let symbol = usdt_contract.symbol().call().await?;

    let decimals = decimals_raw
        .to_string()
        .chars()
        .filter(|c| c.is_numeric() && c > &'0')
        .collect::<String>();

    println!("Name: {:?}", name);
    println!("Symbol: {:?}", symbol);
    println!("Decimals: {:?}", decimals);

    Ok(())
}
