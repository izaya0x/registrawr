use std::path::Path;
use tar::Builder;

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
