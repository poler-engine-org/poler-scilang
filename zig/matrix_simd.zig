const std = @import("std");
const math = std.math;

// ============================================================================
// POLER-SciLang: Zig SIMD & Linear Algebra Engine
// C-ABI export interface for Rust & Assembler interop
// ============================================================================

/// Fused Multiply-Add vector: c[i] = a[i] * b[i] + c[i]
export fn zig_vec_fma(a: [*]const f64, b: [*]const f64, c: [*]f64, len: usize) callconv(.c) void {
    const Vector4 = @Vector(4, f64);
    var i: usize = 0;

    // Process 4 elements at a time via SIMD registers
    while (i + 4 <= len) : (i += 4) {
        const va: Vector4 = a[i..][0..4].*;
        const vb: Vector4 = b[i..][0..4].*;
        const vc: Vector4 = c[i..][0..4].*;
        const vres = va * vb + vc;
        @memcpy(c[i..][0..4], &@as([4]f64, vres));
    }

    // Remainder scalar loop
    while (i < len) : (i += 1) {
        c[i] += a[i] * b[i];
    }
}

/// Euclidean L2 Norm: sqrt(sum(x_i^2))
export fn zig_vec_norm(a: [*]const f64, len: usize) callconv(.c) f64 {
    var sum: f64 = 0.0;
    for (0..len) |i| {
        sum += a[i] * a[i];
    }
    return math.sqrt(sum);
}

/// High-performance matrix multiplication: C = A * B
/// A: rows_a x cols_a, B: cols_a x cols_b, Out C: rows_a x cols_b
export fn zig_mat_mul(
    a: [*]const f64,
    b: [*]const f64,
    out: [*]f64,
    rows_a: usize,
    cols_a: usize,
    cols_b: usize,
) callconv(.c) void {
    // Zero initialize out matrix
    const total = rows_a * cols_b;
    @memset(out[0..total], 0.0);

    // Cache-friendly i-k-j loop with SIMD vectorization across j
    const Vector4 = @Vector(4, f64);
    for (0..rows_a) |i| {
        for (0..cols_a) |k| {
            const aik = a[i * cols_a + k];
            if (aik == 0.0) continue;
            const vaik: Vector4 = @splat(aik);

            var j: usize = 0;
            while (j + 4 <= cols_b) : (j += 4) {
                const b_ptr = b + (k * cols_b + j);
                const out_ptr = out + (i * cols_b + j);

                const vb: Vector4 = b_ptr[0..4].*;
                const vc: Vector4 = out_ptr[0..4].*;
                const vres = vaik * vb + vc;
                @memcpy(out_ptr[0..4], &@as([4]f64, vres));
            }

            while (j < cols_b) : (j += 1) {
                out[i * cols_b + j] += aik * b[k * cols_b + j];
            }
        }
    }
}

/// 4th-order Runge-Kutta step for scientific differential equations: dy/dt = f(t, y)
/// Solves orbital mechanics, chemical kinetics, cooling models: dy/dt = -k * y + forcing
export fn zig_rk4_step(
    y: f64,
    t: f64,
    dt: f64,
    rate_k: f64,
    forcing: f64,
) callconv(.c) f64 {
    const f = struct {
        fn eval(time: f64, val: f64, k: f64, force: f64) f64 {
            _ = time;
            return -k * val + force;
        }
    }.eval;

    const k1 = f(t, y, rate_k, forcing);
    const k2 = f(t + 0.5 * dt, y + 0.5 * dt * k1, rate_k, forcing);
    const k3 = f(t + 0.5 * dt, y + 0.5 * dt * k2, rate_k, forcing);
    const k4 = f(t + dt, y + dt * k3, rate_k, forcing);

    return y + (dt / 6.0) * (k1 + 2.0 * k2 + 2.0 * k3 + k4);
}
