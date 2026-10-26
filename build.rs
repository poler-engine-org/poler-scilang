use std::process::Command;

fn main() {
    println!("cargo:rerun-if-changed=asm/avx2_kernels.s");
    println!("cargo:rerun-if-changed=zig/matrix_simd.zig");

    // 1. Compile x86_64 AVX2 / FMA assembly microkernels
    cc::Build::new()
        .file("asm/avx2_kernels.s")
        .compile("avx2_kernels");

    // 2. Compile Zig SIMD matrix & numerical library to static archive
    let out_dir = std::env::var("OUT_DIR").expect("OUT_DIR must be set by Cargo");
    let zig_out_path = format!("{}/libmatrix_simd.a", out_dir);

    let status = Command::new("zig")
        .args(&[
            "build-lib",
            "-O", "ReleaseFast",
            "-fPIC",
            "--name", "matrix_simd",
            "zig/matrix_simd.zig",
            &format!("-femit-bin={}", zig_out_path),
        ])
        .status()
        .expect("Failed to execute zig build-lib. Ensure 'zig' is installed.");

    if !status.success() {
        panic!("Zig SIMD compilation failed with exit status {:?}", status);
    }

    println!("cargo:rustc-link-search=native={}", out_dir);
    println!("cargo:rustc-link-lib=static=matrix_simd");
}
