use dotenv::dotenv;
use ethers::prelude::Provider;

#[tokio::main]
async fn main() -> eyre::Result<()> {
    dotenv().ok();
    let rpc_url = std::env::var("RPC_URL").expect("RPC_URL must be set.");
    let provider = Provider::try_from(rpc_url)?;

    println!("Provider connected to {}", provider.url().as_str());

    Ok(())
}
