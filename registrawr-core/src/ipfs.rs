use ipfs_api::{response::Error, IpfsClient};
use std::fs::File;
use std::path::Path;

pub async fn publish_artifact(file_path: &Path) -> Result<String, Error> {
    let client = IpfsClient::default();

    let file = File::open(file_path)?;

    let res = client.add(file).await?;

    Ok(res.hash)
}
