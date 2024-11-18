extern crate bindgen;

fn main() {
    // Use pkg-config to find libmpv
    let libmpv = pkg_config::Config::new()
        .probe("mpv")
        .expect("Could not find libmpv using pkg-config");

    // Link library
    for lib in libmpv.libs {
        println!("cargo:rustc-link-lib={}", lib);
    }
    for path in libmpv.link_paths {
        println!("cargo:rustc-link-search=native={}", path.to_str().unwrap());
    }
}
