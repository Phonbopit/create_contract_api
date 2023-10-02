use actix_web::{get, middleware, post, web, App, Error, HttpResponse, HttpServer};
use dotenv::dotenv;
use ethers::prelude::{
    k256, Chain, ContractFactory, Http, LocalWallet, Provider, Signer, SignerMiddleware, Wallet,
    H160,
};

use ethers::utils::format_units;
use serde::{Deserialize, Serialize};

use eyre::Result;
use std::sync::Arc;

type Client = SignerMiddleware<Provider<Http>, Wallet<k256::ecdsa::SigningKey>>;

#[derive(Debug, Serialize, Deserialize)]
pub struct TotalSupplyResponse {
    pub message: String,
    pub total_supply: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreatedContractResponse {
    pub message: String,
    pub contract_address: String,
}

mod abis {
    ethers::contract::abigen!(
        AhoyToken,
        "./abi/AhoyToken.json",
        event_derives (serde::Deserialize, serde::Serialize);
    );
}

#[get("/api/{contract_address}/total_supply")]
async fn total_supply(contract_address: web::Path<H160>) -> Result<HttpResponse, Error> {
    let contract_address = contract_address.into_inner();

    let client = connect().await.unwrap();
    let contract = abis::AhoyToken::new(contract_address.clone(), Arc::new(client.clone()));

    let total_supply = contract.total_supply().call().await.unwrap();

    println!(
        "Total supply of {:?} is {:?}",
        contract_address, total_supply
    );

    let total_supply_in_eth = format_units(total_supply, "ether").unwrap();

    let result = TotalSupplyResponse {
        message: String::from("OK"),
        total_supply: total_supply_in_eth,
    };

    Ok(HttpResponse::Ok().json(result))
}

#[post("/api/contracts")]
async fn create_contract() -> Result<HttpResponse, Error> {
    let contract_address = deploy_contract().await.unwrap();

    let result = CreatedContractResponse {
        message: String::from("Contract created"),
        contract_address: contract_address.to_string(),
    };
    Ok(HttpResponse::Ok().json(result))
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    println!("Starting server...");

    HttpServer::new(move || {
        App::new()
            .wrap(middleware::Logger::default())
            .route("/", web::get().to(|| async { "Actix REST API" }))
            .service(total_supply)
            .service(create_contract)
    })
    .bind(("127.0.0.1", 7777))?
    .run()
    .await
}

async fn connect() -> Result<Client> {
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

    Ok(client)
}

async fn deploy_contract() -> Result<H160> {
    let client = connect().await?;

    // 3. create a contract factory from JSON ABI and bytecode
    let abi = abis::AHOYTOKEN_ABI.clone();
    let bytecode = abis::AHOYTOKEN_BYTECODE.clone();
    let factory = ContractFactory::new(abi, bytecode, Arc::new(client.clone()));

    // 4. Deploy, with no args
    let contract = factory.deploy(())?.send().await?;

    // 5. Print out the address
    let contract_address = contract.address();
    println!("AhoyToken.sol has been deployed to {:?}", contract_address);

    Ok(contract_address)
}
