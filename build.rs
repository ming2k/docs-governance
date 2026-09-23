use flate2::write::GzEncoder;
use flate2::Compression;
use std::env;
use std::fs::File;
use std::path::Path;

fn main() {
    println!("cargo:rerun-if-changed=spec");
    let out_dir = env::var("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("docgov-assets.tar.gz");
    let tar_gz = File::create(dest_path).expect("Failed to create docgov-assets.tar.gz");
    let enc = GzEncoder::new(tar_gz, Compression::default());
    let mut tar = tar::Builder::new(enc);
    if Path::new("spec").exists() {
        tar.append_dir_all("spec", "spec")
            .expect("Failed to package spec directory");
    }
    tar.finish().expect("Failed to finish tar archive");
}
