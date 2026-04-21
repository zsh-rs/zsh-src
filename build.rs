
fn main() {
    println!("cargo::rustc-check-cfg=cfg(ci)");
    
    if std::env::var("CI").is_ok() {
        println!("cargo:rustc-cfg=ci");
    }
}