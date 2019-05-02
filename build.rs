extern crate cc;

use std::env;

fn main() {
    let target = env::var("TARGET").unwrap();
    let target_os = target.split('-').nth(2).unwrap();

    match target_os {
        "linux" => {
            //gcc::compile_library("libinfo.a", &["c/linux.c"])
            cc::Build::new().file("c/linux.c").compile("libinfo.a")
        }
        "darwin" => {
            //gcc::compile_library("libinfo.a", &["c/macos.c"])
            cc::Build::new().file("c/macos.c").compile("libinfo.a")
        }
        "windows" => {
            //gcc::compile_library("libinfo.a", &["c/windows.c"]);
            cc::Build::new().file("c/windows.c").compile("libinfo.a");
            println!("cargo:rustc-flags=-l psapi");
        }
        _ => panic!("Unsupported system"),
    };
}
