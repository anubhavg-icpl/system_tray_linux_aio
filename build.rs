fn main() {
    // Set up Linux-specific build configurations
    if cfg!(target_os = "linux") {
        println!("cargo:rustc-cfg=linux");
        
        // Link against X11 libraries
        println!("cargo:rustc-link-lib=X11");
        println!("cargo:rustc-link-lib=Xext");
    }
}