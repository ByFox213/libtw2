use std::env;

fn non_windows_msvc() {
    println!("cargo:rustc-cdylib-link-arg=-Wl,-undefined,dynamic_lookup");
}

fn main() {
    let msvc = env::var_os("CARGO_CFG_TARGET_VENDOR")
        .map_or(false, |v| v == "pc")
        && env::var_os("CARGO_CFG_TARGET_FAMILY")
            .map_or(false, |v| v == "windows")
        && env::var_os("CARGO_CFG_TARGET_ENV")
            .map_or(false, |v| v == "msvc");
    if !msvc {
        non_windows_msvc();
    }
}
