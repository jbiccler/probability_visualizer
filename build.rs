use std::env;
use std::path::PathBuf;
use std::process::Command;

fn main() {
    let target_triple = env::var("TARGET").unwrap_or_else(|_| "unknown".to_string());

    // If the target is wasm32-unknown-unknown, proceed with wasm-opt
    if target_triple == "wasm32-unknown-unknown" {
        let target_dir =
            PathBuf::from(env::var("CARGO_TARGET_DIR").unwrap_or_else(|_| "target".to_string()));
        let profile = env::var("PROFILE").unwrap();
        let crate_name = env::var("CARGO_PKG_NAME").unwrap().replace('-', "_"); // Hyphens to underscores
        let wasm_file = target_dir
            .join("wasm32-unknown-unknown")
            .join(profile)
            .join(format!("{}.wasm", crate_name));
        let optimized_wasm_file = wasm_file.with_extension("opt.wasm");

        println!("cargo:rerun-if-changed={}", wasm_file.display());

        // Check if wasm-opt is in PATH
        if which::which("wasm-opt").is_err() {
            println!("cargo:warning=wasm-opt not found in PATH. Skipping WASM optimization.");
            return;
        }

        println!("Running wasm-opt on {}", wasm_file.display());

        let result = Command::new("wasm-opt")
            .arg(wasm_file.as_os_str())
            .arg("-Oz") // Optimization level for WASM
            .arg("-o")
            .arg(optimized_wasm_file.as_os_str())
            .status();

        match result {
            Ok(status) if status.success() => {
                println!(
                    "wasm-opt finished successfully. Optimized file: {}",
                    optimized_wasm_file.display()
                );
                println!(
                    "rustc-env=WASM_OPT_OUTPUT={}",
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
    } else {
        // For other targets, just print a message (optimization level is controlled by Cargo.toml)
        println!(
            "cargo:warning=Target is not wasm32-unknown-unknown (it's {}). Skipping wasm-opt.",
            target_triple
        );
    }
}
