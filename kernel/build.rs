fn main() {
    println!("cargo:rerun-if-changed=linker.lds");
    println!("cargo:rustc-link-arg=--script=linker.lds");
}