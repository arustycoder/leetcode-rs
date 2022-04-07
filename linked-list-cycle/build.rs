fn main() {
    cc::Build::new().file("src/solution.c").compile("solution");
    println!("cargo:rerun-if-changed=src/solution.c");
}
