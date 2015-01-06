use std::io::{fs, Command};
use std::io::fs::PathExtensions;
use std::os;
use std::io::process::InheritFd;

fn main() {
    let manifest_dir = Path::new(os::getenv("CARGO_MANIFEST_DIR").unwrap());
    println!("manifest_dir: {}", manifest_dir.display());
    let out_dir = Path::new(os::getenv("OUT_DIR").unwrap());

    let src_dir = manifest_dir.join("src");
    if !src_dir.exists() {
        panic!("Source directory {} does not exist", src_dir.display());
    }

    let wren_dir = src_dir.join("wren");
    if !wren_dir.exists() {
        panic!("Wren directory {} does not exist", wren_dir.display());
    }

    let wren_lib = wren_dir.join("libwren.a");

    let mut make = Command::new("make");

    assert!(make.cwd(&wren_dir)
                .arg("release")
                .stdout(InheritFd(1))
                .stderr(InheritFd(2))
                .status()
                .unwrap()
                .success());

    if let Err(_) = fs::copy(&wren_lib, &out_dir.join("libwren.a")) {
        println!("ERROR COPYING libwren.a");
    }

    println!("cargo:rustc-flags=-L {} -l wren:static", out_dir.display());
}
