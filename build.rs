extern crate rustc_version;

use rustc_version::{version, Version};
use std::process::exit;

fn main() {
    let minimal_ver = "1.47.0";
    let ver = version().unwrap();
    if ver < Version::parse(minimal_ver).unwrap() {
        eprintln!(
            "This crate requires a version of rustc >= {}\nVersion found is {}",
            minimal_ver, ver
        );
        exit(1);
    }
}
