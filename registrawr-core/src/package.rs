use flate2::read::GzDecoder;
use flate2::write::GzEncoder;
use flate2::Compression;
use std::io::{Cursor, Read, Write};
use std::path::Path;
use tar::{Archive, Builder};

pub fn package_artifacts(asset_path: &Path) -> Vec<u8> {
    println!("packaging {}", asset_path.display());
    let mut tarball = Vec::new();
    {
        let mut builder = Builder::new(&mut tarball);

        if asset_path.is_dir() {
            builder
                .append_dir_all(".", asset_path)
                .expect("Error adding directory to archive");
        } else if asset_path.is_file() {
            builder
                .append_path(asset_path)
                .expect("Error adding file to archive");
        } else {
            panic!("Error: Asset path is neither a directory or a file");
        }

        builder.finish().expect("Failed to create tar file");
    }

    let mut buf = GzEncoder::new(Vec::new(), Compression::default());
    buf.write_all(&tarball)
        .expect("Error writing tarball to compression buffer");

    buf.finish().expect("Error compressing the tarball")
}

pub fn extract_artifcats(compressed_tarball: &Vec<u8>, install_location: &Path) {
    let mut d = GzDecoder::new(Cursor::new(compressed_tarball));
    let mut tarball = vec![];
    d.read_to_end(&mut tarball)
        .expect("Error decompressing to tarball");

    let mut ar = Archive::new(Cursor::new(tarball));
    ar.unpack(install_location).unwrap();
}
