use super::PackageData;
use futures::TryStreamExt;
use ipfs_api::{response::Error, IpfsClient};
use std::fs::File;
use std::io::{Cursor, Write};
use std::path::Path;

pub(crate) async fn publish_artifact_from_path(file_path: &Path) -> Result<String, Error> {
    let client = IpfsClient::default();
    let file = File::open(file_path)?;
    let res = client.add(file).await?;

    Ok(res.hash)
}

pub(crate) async fn publish_artifact_from_tarball(tarball: Vec<u8>) -> Result<String, Error> {
    let client = IpfsClient::default();
    let cursor = Cursor::new(tarball);
    let res = client.tar_add(cursor).await?;

    Ok(res.hash)
}

pub(crate) async fn publish_json(data: PackageData) -> Result<String, Error> {
    let client = IpfsClient::default();
    let cursor = Cursor::new(serde_json::to_string(&data)?);
    let res = client.add(cursor).await?;

    Ok(res.hash)
}

pub(crate) async fn download_json(cid: String) -> Result<PackageData, Error> {
    let client = IpfsClient::default();

    let res = client
        .cat(&cid)
        .map_ok(|chunk| chunk.to_vec())
        .try_concat()
        .await?;
    let data: PackageData = serde_json::from_str(&String::from_utf8(res)?)?;

    Ok(data)
}

pub(crate) async fn download_tarball(cid: &str) -> Result<Vec<u8>, Error> {
    let client = IpfsClient::default();

    let res = client
        .tar_cat(&format!("/ipfs/{}", cid))
        .map_ok(|chunk| chunk.to_vec())
        .try_concat()
        .await?;

    Ok(res)
}
