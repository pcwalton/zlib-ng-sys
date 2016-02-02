// zlib-ng-sys/build.rs

extern crate gcc;

use gcc::Config;
use std::env;
use std::ffi::OsString;
use std::process::Command;

fn main() {
    let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let out_dir = env::var("OUT_DIR").unwrap();

    let compiler = Config::new().get_compiler();
    let mut cflags = OsString::new();
    for arg in compiler.args() {
        cflags.push(arg);
        cflags.push(" ");
    }

    Command::new(&format!("{}/zlib-ng/configure", manifest_dir)).current_dir(&out_dir)
                                                                .env("CC", compiler.path())
                                                                .env("CFLAGS", &cflags)
                                                                .status()
                                                                .unwrap();
    Command::new("make").current_dir(&out_dir).env("CC", compiler.path())
                                              .env("CFLAGS", &cflags)
                                              .status()
                                              .unwrap();
    println!("cargo:rustc-link-search=native={}", out_dir);
    println!("cargo:rustc-link-lib=static=z");
}

