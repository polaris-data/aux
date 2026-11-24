use std::{env, path::PathBuf};

fn main() {
    let target = env::var("TARGET").expect("Cargo sets TARGET");
    let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("prebuilt")
        .join(&target);
    let archive = dir.join("libaux_zig_core.a");
    assert!(archive.is_file(), "no packaged aux Zig core for {target}");
    println!("cargo:rerun-if-changed={}", archive.display());
    println!("cargo:rustc-link-search=native={}", dir.display());
    println!("cargo:rustc-link-lib=static=aux_zig_core");
}
