// use std::env;
// use std::path::Path;

// use std::process::Command;

// fn main() {
//     println!("cargo:rerun-if-changed=build.rs");
//     let compiler = env::var("ZIG_COMPILER").expect("Failed to find ZIG_COMPILER");
//     let target = env::var("TB_TARGET").expect("TB_TARGET env var unspecified");

//     let dir = env::var("CARGO_MANIFEST_DIR").unwrap();
//     let path = Path::new(&dir);

//     env::set_current_dir(path.join("tigerbeetle")).unwrap();

//     let target_flag = format!("-Dtarget={}", target);

//     Command::new(compiler)
//         .args(&["build", "tb_client", "-Drelease-safe", &target_flag])
//         .output()
//         .expect("Failed to compile Tigerbeetle Zig lib");

//     env::set_current_dir(path).unwrap();

//     println!(
//         "cargo:rustc-link-search=native={}",
//         Path::new(&dir).join("zig/zig-cache/lib").display()
//     );

//     // On windows, link against ntdll?
//     // #[cfg(target_os = "windows")]
//     // {
//     //     println!("cargo:rustc-link-lib={}={}", "dylib", "ntdll");
//     // }
// }

extern crate bindgen;

use std::env;
use std::path::Path;
use std::path::PathBuf;
// use std::process::Command;

fn main() {
    let dir = env::var("CARGO_MANIFEST_DIR").unwrap();

    // Tell cargo to look for shared libraries in the specified directory
    println!(
        "cargo:rustc-link-search={}",
        Path::new(&dir).join("pkg/native/x86_64-macos").display()
    );

    // Tell cargo to tell rustc to link the shared library
    println!("cargo:rustc-link-lib=tb_client");

    // Tell cargo to invalidate the built crate whenever the wrapper changes
    println!("cargo:rerun-if-changed=wrapper.h");

    // The bindgen::Builder is the main entry point
    // to bindgen, and lets you build up options for
    // the resulting bindings.
    let bindings = bindgen::Builder::default()
        // The input header we would like to generate
        // bindings for.
        .header("wrapper.h")
        // Tell cargo to invalidate the built crate whenever any of the
        // included header files changed.
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        // Finish the builder and generate the bindings.
        .generate()
        // Unwrap the Result and panic on failure.
        .expect("Unable to generate bindings");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
