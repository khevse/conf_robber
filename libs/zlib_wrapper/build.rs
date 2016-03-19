
use std::process::Command;
use std::env::{current_dir, split_paths};
use std::path::Path;

fn main() {

    let paths = current_dir().ok().expect("Failed to get current directory");
    let path_to_current_dir = split_paths(&paths).nth(0).unwrap();
    let path_to_cpp = match path_to_current_dir.as_path()
                                               .join("cpp_src")
                                               .as_path()
                                               .to_str() {
        None => panic!("path to string"),
        Some(path_str) => path_str.to_string(),
    };

    if !Path::new(&path_to_cpp).join("libzlibwrapper.a").exists() ||
       !Path::new(&path_to_cpp).join("libzlibstatic.a").exists() {

        // Compile C++ libs
        let path_to_build_bat = match path_to_current_dir.as_path()
                                                         .join("build.bat")
                                                         .to_str() {
            None => panic!("path to string"),
            Some(path_str) => path_str.to_string(),
        };

        let output = Command::new(&*path_to_build_bat)
                         .output()
                         .unwrap_or_else(|e| panic!("failed to execute process: {}", e));

        println!("status: {}", output.status);
        println!("stdout: {}", String::from_utf8_lossy(&output.stdout));
        println!("stderr: {}", String::from_utf8_lossy(&output.stderr));
    }


    // Linking
    println!("path_to_cpp==={}", path_to_cpp);
    println!("cargo:rustc-link-search=native={}", path_to_cpp);
    println!("cargo:rustc-link-lib=static=zlibstatic");
    println!("cargo:rustc-link-lib=static=zlibwrapper");
    println!("cargo:rustc-link-lib=static=gcc");
    println!("cargo:rustc-link-lib=static=stdc++");
}
