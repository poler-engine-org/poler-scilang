//! Native x86_64 Assembly (AVX2/FMA) & Zig SIMD bindings

extern "C" {
    // ---- x86_64 AVX2 Assembly Kernels ----
    fn poler_asm_dot_product(a: *const f64, b: *const f64, n: usize) -> f64;
    fn poler_asm_poly_eval(x: f64, coeffs: *const f64, count: usize) -> f64;
    fn poler_asm_fast_invsqrt(x: f64) -> f64;

    // ---- Zig SIMD & Matrix Kernels ----
    fn zig_vec_fma(a: *const f64, b: *const f64, c: *mut f64, len: usize);
    fn zig_vec_norm(a: *const f64, len: usize) -> f64;
    fn zig_mat_mul(
        a: *const f64,
        b: *const f64,
        out: *mut f64,
        rows_a: usize,
        cols_a: usize,
        cols_b: usize,
    );
    fn zig_rk4_step(y: f64, t: f64, dt: f64, rate_k: f64, forcing: f64) -> f64;
}

/// Compute vector dot product using hand-written AVX2 FMA assembly
#[inline]
pub fn asm_dot_product(a: &[f64], b: &[f64]) -> f64 {
    let n = a.len().min(b.len());
    if n == 0 {
        return 0.0;
    }
    unsafe { poler_asm_dot_product(a.as_ptr(), b.as_ptr(), n) }
}

/// Evaluate polynomial P(x) = c[0]*x^n + c[1]*x^(n-1) + ... + c[n] using FMA3 Horner in ASM
#[inline]
pub fn asm_poly_eval(x: f64, coeffs: &[f64]) -> f64 {
    if coeffs.is_empty() {
        return 0.0;
    }
    unsafe { poler_asm_poly_eval(x, coeffs.as_ptr(), coeffs.len()) }
}

/// Compute fast 1/sqrt(x) using hardware rsqrtss with two double-precision Newton-Raphson iterations
#[inline]
pub fn asm_fast_invsqrt(x: f64) -> f64 {
    if x <= 0.0 {
        return f64::NAN;
    }
    unsafe { poler_asm_fast_invsqrt(x) }
}

/// Fused Multiply-Add vector: c[i] += a[i] * b[i] using Zig SIMD
#[inline]
pub fn zig_vector_fma(a: &[f64], b: &[f64], c: &mut [f64]) {
    let n = a.len().min(b.len()).min(c.len());
    if n > 0 {
        unsafe { zig_vec_fma(a.as_ptr(), b.as_ptr(), c.as_mut_ptr(), n) }
    }
}

/// Euclidean L2 Norm using Zig SIMD
#[inline]
pub fn zig_vector_norm(a: &[f64]) -> f64 {
    if a.is_empty() {
        return 0.0;
    }
    unsafe { zig_vec_norm(a.as_ptr(), a.len()) }
}

/// Matrix multiplication C = A * B using Zig SIMD cache-friendly kernel
pub fn zig_matrix_mul(
    a: &[f64],
    b: &[f64],
    rows_a: usize,
    cols_a: usize,
    cols_b: usize,
) -> Vec<f64> {
    assert_eq!(a.len(), rows_a * cols_a, "Matrix A dimensions mismatch");
    assert_eq!(b.len(), cols_a * cols_b, "Matrix B dimensions mismatch");
    let mut out = vec![0.0; rows_a * cols_b];
    unsafe {
        zig_mat_mul(
            a.as_ptr(),
            b.as_ptr(),
            out.as_mut_ptr(),
            rows_a,
            cols_a,
            cols_b,
        );
    }
    out
}

/// Single 4th order Runge-Kutta step dy/dt = -k*y + forcing computed in Zig
#[inline]
pub fn zig_integrate_rk4(y: f64, t: f64, dt: f64, rate_k: f64, forcing: f64) -> f64 {
    unsafe { zig_rk4_step(y, t, dt, rate_k, forcing) }
}
