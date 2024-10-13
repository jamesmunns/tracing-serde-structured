fn main() {
    println!("cargo::rustc-check-cfg=cfg(tracing_unstable)");
}
