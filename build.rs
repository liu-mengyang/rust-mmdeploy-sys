extern crate bindgen;

use std::io;
use std::env;
use std::path::PathBuf;
use std::fs;


struct Library {
    name: &'static str,
}


static LIBRARIES: &[Library] = &[
    Library {
        name: "mmdeploy"
    }
];


fn output() -> PathBuf {
    PathBuf::from(env::var("OUT_DIR").unwrap())
}


fn build() -> io::Result<()> {


    Ok(())
}


fn link_to_libraries() {
    // let mmdeploy_ty = if statik { "static" } else { "dylib" };
    let mmdeploy_ty = "dylib";
    for lib in LIBRARIES {
        println!("cargo:rustc-link-lib={}={}", mmdeploy_ty, lib.name);
    }
}


fn search_include(include_paths: &[PathBuf], header: &str) -> String {
    for dir in include_paths {
        let include = dir.join(header);
        if fs::metadata(&include).is_ok() {
            return include.as_path().to_str().unwrap().to_string();
        }
    }
    format!("/usr/include/{}", header)
}


fn main() {
    // let statik = env::var("CARGO_FEATURE_STATIC").is_ok();
    let include_path: Vec<PathBuf> = if env::var("CARGO_FEATURE_BUILD").is_ok() {
        // TODO: Support static build mode.
        println!("Static build mode");

        vec![]
    } else if let Ok(mmdeploy_dir) = env::var("MMDEPLOY_DIR") {
        let mmdeploy_dir = PathBuf::from(mmdeploy_dir);
        println!(
            "cargo:rustc-link-search=native={}",
            mmdeploy_dir.join("sdk").join("lib").to_string_lossy()
        );
        link_to_libraries();
        vec![mmdeploy_dir.join("sdk").join("include").join("mmdeploy")]
    } else {
        // TODO: Support pkg-config
        println!("Fallback to pkg-config");

        vec![]
    };

    let clang_includes = include_path
        .iter()
        .map(|include| format!("-I{}", include.to_string_lossy()));

    // create builder
    let mut builder = bindgen::Builder::default()
        .clang_args(clang_includes);

    builder = builder
        .header(search_include(&include_path, "common.h"))
        .header(search_include(&include_path, "model.h"))
        .header(search_include(&include_path, "executor.h"))
        .header(search_include(&include_path, "pipeline.h"))
        .header(search_include(&include_path, "classifier.h"))
        .header(search_include(&include_path, "detector.h"))
        .header(search_include(&include_path, "segmentor.h"))
        .header(search_include(&include_path, "pose_detector.h"))
        .header(search_include(&include_path, "rotated_detector.h"))
        .header(search_include(&include_path, "text_recognizer.h"))
        .header(search_include(&include_path, "text_detector.h"))
        .header(search_include(&include_path, "restorer.h"));

    // generate builder
    let bindings = builder
        .generate()
        .expect("Unable to generate bindings");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    bindings
        .write_to_file(output().join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
