use std::{env, fs, path::{Path, PathBuf}};

const OUT_FILE: &str = "bindings.rs";

fn main() {
    let out_dir = PathBuf::from(env::var("OUT_DIR").expect("OUT_DIR not set"));
    let include_dir = PathBuf::from(env::var("HEADERS_PATH").expect("HEADERS_PATH not set"));
    let sdk_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR not set")).join("sdk");

    let headers = collect_headers(include_dir.as_path());
    let mut root = String::new();       // Buffer for the generated root "bindings" file

    for header in &headers {
        let header_str = header.to_string_lossy().into_owned();
        let stem = header.file_stem().and_then(|s| s.to_str()).expect("header has no valid stem");
        let out_rs = format!("{stem}_bindings.rs");

        println!("cargo:rerun-if-changed={header_str}");

        let builder = bindgen::Builder::default()
            .header(&header_str)
            .clang_arg(format!("-I{}", include_dir.display()))  // user defined headers
            .clang_arg(format!("-I{}", sdk_dir.display())).clang_arg("-include").clang_arg("types.h")  // bundled SDK/injection headers
            .use_core().ctypes_prefix("core::ffi")  // use core instead of std & use canonical C types
            .layout_tests(false)    // dont generate test functions
            .allowlist_file(&format!(r".*/{stem}\.h$")).allowlist_recursively(false);  // only generate bindings for current file (non recurisvely)

        let bindings = builder.generate().expect("Unable to generate bindings");
        bindings.write_to_file(out_dir.join(&out_rs)).expect("Couldn't write bindings");

        root.push_str(&format!(
            "pub mod {stem} {{\n\
                \tuse crate::types::*;\n\
                \tinclude!(concat!(env!(\"OUT_DIR\"), \"/{out_rs}\"));\n\
            }}\n\n"
        ));
    }
    // write the root bindings file (defines all per-header modules)
    fs::write(out_dir.join(OUT_FILE), root).expect("Couldn't write mod.rs");
}

/// Collect all .h files from the given directory
fn collect_headers(include_path: &Path) -> Vec<PathBuf> {
    fs::read_dir(include_path)
        .expect("failed to read header dir")
        .filter_map(|e| {
            let p = e.ok()?.path();
            if p.extension()?.to_str()? == "h" {
                Some(p)
            } else {
                None
            }
        })
        .collect()
}
