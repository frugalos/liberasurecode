use std::env;
use std::fs;
use std::path::PathBuf;
use std::process::{Command, Stdio};

fn main() {
    println!("cargo:rerun-if-changed=build.rs");

    let outdir = PathBuf::from(env::var_os("OUT_DIR").unwrap());
    let build_dir = outdir.join("build");
    let _ = fs::remove_dir_all(&build_dir);
    fs::create_dir(&build_dir).unwrap();

    for file in &["install_deps.sh"] {
        fs::copy(file, build_dir.join(file)).unwrap();
    }

    // the path to target/(profile)/
    let manifest = PathBuf::from(env::var_os("CARGO_MANIFEST_DIR").unwrap());
    let target = manifest.join("target").join(env::var_os("PROFILE").unwrap());
    match Command::new("./install_deps.sh")
        .args(&[target.to_str().unwrap()])
        .current_dir(&build_dir)
        .stderr(Stdio::inherit())
        .output()
    {
        Err(e) => {
            panic!("{}: {}", build_dir.display(), e);
        }
        Ok(output) => {
            if !output.status.success() {
                panic!(
                    "./install_deps.sh failed: exit-code={:?}",
                    output.status.code()
                );
            }
        }
    }

    println!("cargo:rustc-link-search={}/lib", build_dir.display());
}
