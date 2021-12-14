use std::env;

fn main() {
    let spdk_dir = env::var("DEP_SPDK_BUNDLE_ROOT").unwrap();
    println!("cargo:rustc-link-search=native={}", spdk_dir);
    println!("cargo:rustc-link-arg=-Wl,-rpath={}", spdk_dir);
}