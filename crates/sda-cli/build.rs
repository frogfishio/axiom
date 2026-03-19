use std::env;
use std::fs;
use std::path::PathBuf;

fn main() {
    let manifest_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").expect("missing CARGO_MANIFEST_DIR"));
    let repo_root = manifest_dir.join("../..");
    let version_path = repo_root.join("VERSION");
    let build_path = repo_root.join("BUILD");

    println!("cargo:rerun-if-changed={}", version_path.display());
    println!("cargo:rerun-if-changed={}", build_path.display());

    let version = fs::read_to_string(&version_path)
        .unwrap_or_else(|error| panic!("failed to read {}: {error}", version_path.display()))
        .trim()
        .to_string();
    let build = fs::read_to_string(&build_path)
        .unwrap_or_else(|error| panic!("failed to read {}: {error}", build_path.display()))
        .trim()
        .to_string();

    println!("cargo:rustc-env=AXIOM_VERSION={version}");
    println!("cargo:rustc-env=AXIOM_BUILD={build}");
}