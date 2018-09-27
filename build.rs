use std::env;
use std::fs;
use std::path::PathBuf;
use std::process::Command;

fn main() {
    println!("cargo:rerun-if-changed=build.rs");

    let outdir = PathBuf::from(env::var_os("OUT_DIR").unwrap());
    let build_dir = outdir.join("build");
    let _ = fs::remove_dir_all(&build_dir);
    fs::create_dir(&build_dir).unwrap();

    for file in &["install_deps.sh", "liberasurecode.patch"] {
        fs::copy(file, build_dir.join(file)).unwrap();
    }

    match Command::new("./install_deps.sh")
        .current_dir(&build_dir)
        .output()
    {
        Err(e) => {
            panic!("{}: {}", build_dir.display(), e);
        }
        Ok(output) => {
            if !output.status.success() {
                panic!(
                    "./install_deps.sh failed: exit-code={:?}\n{}",
                    output.status.code(),
                    String::from_utf8_lossy(&output.stderr)
                );
            }
        }
    }

    println!("cargo:rustc-link-search={}/lib", build_dir.display());
}
