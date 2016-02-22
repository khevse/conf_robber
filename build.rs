
use std::env::{current_dir, split_paths};

fn main() {

    let paths = match current_dir() {
        Ok(exe_path) => exe_path,
        Err(_) => panic!("Failed to get current directory"),
    };

    let path_to_current_dir = split_paths(&paths).nth(0).unwrap();

    let path_to_cpp = match path_to_current_dir.as_path().join("cpp_src").as_path().to_str() {
        None => panic!("path to string"),
        Some(path_str) => path_str.to_string(),
    };

    println!("path_to_cpp==={}", path_to_cpp);
    println!("cargo:rustc-link-search=native={}", path_to_cpp);
    println!("cargo:rustc-link-lib=static=zlibstatic");
    println!("cargo:rustc-link-lib=static=zlibwrapper");
    println!("cargo:rustc-link-lib=static=gcc");
    println!("cargo:rustc-link-lib=static=stdc++");
}
