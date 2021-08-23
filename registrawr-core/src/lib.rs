mod build;
mod error;
mod package;
mod publish;

use build::build;
use ethers::{
    abi::Abi,
    contract::Contract,
    providers::{Http, Provider},
    types::{Address, H256},
};
use ethers_middleware::SignerMiddleware;
use ethers_signers::LocalWallet;
use package::{extract_artifcats, package_artifacts};
use publish::{download_json, download_tarball, publish_artifact_from_tarball, publish_json};
use rpassword;
use serde::{Deserialize, Serialize};
use std::{
    convert::TryFrom,
    path::{Path, PathBuf},
};
#[derive(Deserialize, Debug)]
struct HardhatArtifact {
    pub abi: Abi,
}

#[derive(Deserialize, Debug)]
struct ContractAddresses {
    pub registrawr: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct PackageData {
    name: String,
    version: String,
    asset_cid: String,
}

const CONTRACT_ARTIFACT: &str =
    include_str!("../../contracts/artifacts/contracts/Registrawr.sol/Registrawr.json");

const CONTRACT_ADDRESSES: &str = include_str!("../../contracts/addresses.json");

//TODO:
// - Add publish option right from github
// - Extract git infromation when packaging (commit hash)
// - Add SqlLite support for tracking installed dapps
// - Add config file support

pub async fn list_dapps() -> Result<Vec<String>, anyhow::Error> {
    let provider = Provider::<Http>::try_from("http://localhost:8545")?;

    let artifact: HardhatArtifact = serde_json::from_str(CONTRACT_ARTIFACT)?;

    let addresses: ContractAddresses = serde_json::from_str(CONTRACT_ADDRESSES)?;
    let address = addresses.registrawr.parse::<Address>()?;

    let contract = Contract::new(address, artifact.abi, provider);

    let dapp_names: Vec<String> = contract.method::<_, _>("listDapps", ())?.call().await?;
    let dapp_names = dapp_names
        .into_iter()
        .filter(|elem| elem != &String::from(""))
        .collect::<Vec<String>>();
    Ok(dapp_names)
}

pub fn build_dapp(source_path: &Path) -> PathBuf {
    build(source_path)
}

pub async fn register_dapp(dapp_name: &str, asset_path: &Path) -> Result<(), anyhow::Error> {
    let tarball = package_artifacts(asset_path);
    let tarball_cid = match publish_artifact_from_tarball(tarball).await {
        Ok(cid) => cid,
        Err(err) => {
            return Err(anyhow::anyhow!(
                "Error publishing artifact as tarball! {}",
                err
            ))
        }
    };

    let json_data = PackageData {
        name: dapp_name.to_owned(),
        version: "0.0.1".to_owned(),
        asset_cid: tarball_cid,
    };
    let json_cid = match publish_json(json_data).await {
        Ok(cid) => cid,
        Err(_) => return Err(anyhow::anyhow!("Error publishing JSON")),
    };

    let wallet = unlock_wallet()?;
    let provider = Provider::<Http>::try_from("http://localhost:8545")?;
    let provider = SignerMiddleware::new(provider, wallet);

    let artifact: HardhatArtifact = serde_json::from_str(CONTRACT_ARTIFACT)?;

    let addresses: ContractAddresses = serde_json::from_str(CONTRACT_ADDRESSES)?;
    let address = addresses.registrawr.parse::<Address>()?;

    let contract = Contract::new(address, artifact.abi, provider);

    let call = contract.method::<_, H256>("register", (dapp_name.to_owned(), json_cid))?;
    let pending_tx = call.send().await?;

    pending_tx.confirmations(1).await?;

    Ok(())
}

pub async fn get_dapp(dapp_name: &str) -> Result<String, anyhow::Error> {
    let provider = Provider::<Http>::try_from("http://localhost:8545")?;

    let artifact: HardhatArtifact = serde_json::from_str(CONTRACT_ARTIFACT)?;

    let addresses: ContractAddresses = serde_json::from_str(CONTRACT_ADDRESSES)?;
    let address = addresses.registrawr.parse::<Address>()?;

    let contract = Contract::new(address, artifact.abi, provider);

    let message: String = contract
        .method::<_, _>("getDapp", dapp_name.to_owned())?
        .call()
        .await?;

    let package_data = download_json(message.clone()).await.unwrap();
    let tarball = download_tarball(&package_data.asset_cid).await.unwrap();

    extract_artifcats(&tarball);

    Ok(message)
}

fn unlock_wallet() -> Result<LocalWallet, anyhow::Error> {
    let password = rpassword::prompt_password_stdout("Enter password to confirm transaction: ")?;

    Ok(LocalWallet::decrypt_keystore(
        "../contracts/wallet.json",
        password,
    )?)
}
