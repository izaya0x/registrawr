use ethers::{
    abi::Abi,
    contract::Contract,
    providers::{Http, Provider},
    types::{Address, H256},
};
use ethers_middleware::SignerMiddleware;
use ethers_signers::LocalWallet;
use rpassword;
use serde::Deserialize;
use std::convert::TryFrom;

#[derive(Deserialize, Debug)]
struct HardhatArtifact {
    pub abi: Abi,
}

#[derive(Deserialize, Debug)]
struct ContractAddresses {
    pub registrawr: String,
}

const CONTRACT_ARTIFACT: &str =
    include_str!("../../contracts/artifacts/contracts/Registrawr.sol/Registrawr.json");

const CONTRACT_ADDRESSES: &str = include_str!("../../contracts/addresses.json");

pub async fn list_dapps() -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let provider = Provider::<Http>::try_from("http://localhost:8545")?;

    let artifact: HardhatArtifact = serde_json::from_str(CONTRACT_ARTIFACT)?;

    let addresses: ContractAddresses = serde_json::from_str(CONTRACT_ADDRESSES)?;
    let address = addresses.registrawr.parse::<Address>()?;

    let contract = Contract::new(address, artifact.abi, provider);

    let dapp_names: Vec<String> = contract.method::<_, _>("listDapps", ())?.call().await?;
    Ok(dapp_names)
}

pub async fn register_dapp(dapp_name: &str) -> Result<(), Box<dyn std::error::Error>> {
    let wallet = unlock_wallet()?;
    let provider = Provider::<Http>::try_from("http://localhost:8545")?;
    let provider = SignerMiddleware::new(provider, wallet);

    let artifact: HardhatArtifact = serde_json::from_str(CONTRACT_ARTIFACT)?;

    let addresses: ContractAddresses = serde_json::from_str(CONTRACT_ADDRESSES)?;
    let address = addresses.registrawr.parse::<Address>()?;

    let contract = Contract::new(address, artifact.abi, provider);

    let call = contract.method::<_, H256>("register", dapp_name.to_owned())?;
    let pending_tx = call.send().await?;

    let receipt = pending_tx.confirmations(1).await?;
    println!("tx receipt: {:#?}", receipt);
    Ok(())
}

pub async fn get_dapp_data(_dapp_name: &str) -> Result<String, Box<dyn std::error::Error>> {
    let provider = Provider::<Http>::try_from("http://localhost:8545")?;

    let artifact: HardhatArtifact = serde_json::from_str(CONTRACT_ARTIFACT)?;

    let addresses: ContractAddresses = serde_json::from_str(CONTRACT_ADDRESSES)?;
    let address = addresses.registrawr.parse::<Address>()?;

    let contract = Contract::new(address, artifact.abi, provider);

    let message: String = contract.method::<_, _>("helloWorld", ())?.call().await?;
    Ok(message)
}

fn unlock_wallet() -> Result<LocalWallet, Box<dyn std::error::Error>> {
    let password = rpassword::prompt_password_stdout("Enter password to confirm transaction: ")?;

    Ok(LocalWallet::decrypt_keystore(
        "../contracts/wallet.json",
        password,
    )?)
}
