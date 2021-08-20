use std::io::Cursor;
use std::path::Path;
use tar::{Archive, Builder};

pub fn package_artifacts(asset_path: &Path) -> Vec<u8> {
    let mut buf = Vec::new();
    {
        let mut builder = Builder::new(&mut buf);

        if asset_path.is_dir() {
            builder
                .append_dir_all(".", asset_path)
                .expect("Error adding directory to archive");
        } else if asset_path.is_file() {
            builder
                .append_path(asset_path)
                .expect("Error adding file to archive");
        }

        builder.finish().expect("Failed to create tar file");
    }

    buf
}

pub fn extract_artifcats(tarball: &Vec<u8>) {
    let mut ar = Archive::new(Cursor::new(tarball));
    ar.unpack("testInstalledArtifacts").unwrap();
}
