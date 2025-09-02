fn main() {
    cxx_build::bridge("src/event2/quest1.rs")
        .file("src/event2/quest1.cc")
        .std("c++23")
        .compile("ec");
    println!("cargo:rerun-if-changed=src/event2/quest1.cc");
    println!("cargo:rerun-if-changed=include/event2/quest1.h");
}
