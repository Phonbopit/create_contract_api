use dotenv::dotenv;
use ethers::prelude::{Chain, ContractFactory, LocalWallet, Provider, Signer, SignerMiddleware};
use eyre::Result;
use std::sync::Arc;

mod abis {
    ethers::contract::abigen!(
        AhoyToken,
        "./abi/AhoyToken.json",
        event_derives (serde::Deserialize, serde::Serialize);
    );
}

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok();
    let rpc_url = std::env::var("RPC_URL").expect("RPC_URL must be set.");

    // Do not include '0x' at the start of the private key
    let private_key = std::env::var("PRIVATE_KEY").expect("PRIVATE_KEY must be set.");

    let provider = Provider::try_from(rpc_url)?;

    println!("Provider connected to {}", provider.url().as_str());

    // compile and deploy a contract
    // 1. load wallet from private key
    let pk = private_key.parse::<LocalWallet>()?;
    let wallet: LocalWallet = LocalWallet::from(pk).with_chain_id(Chain::Sepolia);

    // 2. wrap signer and wallet together
    let client = SignerMiddleware::new(provider.clone(), wallet.clone());

    // 3. create a contract factory from JSON ABI and bytecode
    let abi = abis::AHOYTOKEN_ABI.clone();
    let bytecode = abis::AHOYTOKEN_BYTECODE.clone();
    let factory = ContractFactory::new(abi, bytecode, Arc::new(client.clone()));

    // 4. Deploy, with no args
    let contract = factory.deploy(())?.send().await?;

    // 5. Print out the address
    let contract_address = contract.address();
    println!("AhoyToken.sol has been deployed to {:?}", contract_address);

    Ok(())
}
