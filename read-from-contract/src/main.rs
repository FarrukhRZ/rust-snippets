// #![forbid(unused_imports)]
use ethers::prelude::*;
use dotenv::dotenv;

fn main() {
    dotenv().ok();
    let mainnet_rpc = std::env::var("MAINNET_RPC_URL").expect("MAINNET_RPC_URL is not set in .env");
    println!("Loaded MAINNET_RPC_URL from .env: {}", mainnet_rpc);
}
