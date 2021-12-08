#[cfg(target_os = "linux")]
fn main() {
    println!("cargo:rustc-link-search=native=/home/linuxbrew/.linuxbrew/lib");
    println!("cargo:rustc-link-arg=-Wl,-rpath,/home/linuxbrew/.linuxbrew/lib");
}
