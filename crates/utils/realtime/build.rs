fn main() {
    println!("cargo:rerun-if-changed=c_src/realtime.c");
    println!("cargo:rerun-if-changed=c_src/realtime.h");

    cc::Build::new()
        .file("c_src/realtime.c")
        .include("c_src")
        .define("SQLITE_CORE", "1")
        .define("SQLITE_ENABLE_REALTIME", "1")
        .compile("sqlite_realtime_c");
}
