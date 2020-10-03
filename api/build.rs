use cmake::Config;

fn main() {
    let build_type = if cfg!(debug_assertions) {
        "Debug"
    } else {
        "Release"
    };

    let dst = Config::new("h3")
        .define("BUILD_TESTING", "OFF")
        .define("BUILD_GENERATORS", "OFF")
        .define("BUILD_BENCHMARKS", "OFF")
        .define("BUILD_FILTERS", "OFF")
        .define("ENABLE_LINTING", "OFF")
        .define("ENABLE_DOCS", "OFF")
        .define("ENABLE_COVERAGE", "OFF")
        .define("BUILD_TYPE", build_type)
        .build();
        
    println!("cargo:rustc-link-search=native={}/lib", dst.display());
    println!("cargo:cargo:include={}/include", dst.display());
    println!("cargo:rustc-link-lib=static=h3");
}
