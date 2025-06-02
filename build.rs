use std::env;
use std::path::PathBuf;
use std::process::Command;

fn main() {
    let target_dir =
        PathBuf::from(env::var("CARGO_TARGET_DIR").unwrap_or_else(|_| "target".to_string()));
    let profile = env::var("PROFILE").unwrap();
    let wasm_file = target_dir
        .join("wasm32-unknown-unknown")
        .join(profile)
        .join(format!("{}.wasm", env::var("CARGO_PKG_NAME").unwrap()));
    let optimized_wasm_file = wasm_file.with_extension("opt.wasm");

    println!("cargo:rerun-if-changed={}", wasm_file.display());

    // Check if wasm-opt is in PATH
    if which::which("wasm-opt").is_err() {
        println!("cargo:warning=wasm-opt not found in PATH. Skipping optimization.");
        return;
    }

    println!("Running wasm-opt on {}", wasm_file.display());

    let result = Command::new("wasm-opt")
        .arg(wasm_file.as_os_str())
        .arg("-Oz") // You can change the optimization level here (-O0, -O1, -O2, -O3, -Os, -Oz)
        .arg("-o")
        .arg(optimized_wasm_file.as_os_str())
        .status();

    match result {
        Ok(status) if status.success() => {
            println!(
                "wasm-opt finished successfully. Optimized file: {}",
                optimized_wasm_file.display()
            );
            // Optionally, you can move/rename the optimized file to replace the original
            // std::fs::rename(&optimized_wasm_file, &wasm_file).unwrap();
            println!(
                "cargo:rustc-env=WASM_OPT_OUTPUT={}",
                optimized_wasm_file.display()
            );
        }
        Ok(status) => {
            println!(
                "cargo:warning=wasm-opt failed with exit code: {:?}",
                status.code()
            );
        }
        Err(e) => {
            println!("cargo:warning=Failed to run wasm-opt: {}", e);
        }
    }
}
