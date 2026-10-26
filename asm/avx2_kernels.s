/*
 * POLER-SciLang: x86_64 Hand-Tuned AVX 256-bit SIMD Math Microkernels
 * SysV ABI: %rdi, %rsi, %rdx, %rcx, %r8, %r9 -> return %xmm0
 * Compatible with all x86_64 AVX processors
 */

.text
.globl poler_asm_dot_product
.type poler_asm_dot_product, @function

/*
 * double poler_asm_dot_product(const double *a, const double *b, size_t n);
 * %rdi = a, %rsi = b, %rdx = n
 */
poler_asm_dot_product:
    vxorpd %ymm0, %ymm0, %ymm0      /* acc0 = 0 */
    vxorpd %ymm1, %ymm1, %ymm1      /* acc1 = 0 */

    movq %rdx, %rcx
    shrq $3, %rcx                   /* count of 8-element chunks */
    jz .Lscalar_check

.Lloop8:
    vmovupd (%rdi), %ymm2
    vmovupd 32(%rdi), %ymm3
    vmovupd (%rsi), %ymm4
    vmovupd 32(%rsi), %ymm5

    vmulpd %ymm4, %ymm2, %ymm2      /* a[0..3] * b[0..3] */
    vmulpd %ymm5, %ymm3, %ymm3      /* a[4..7] * b[4..7] */

    vaddpd %ymm2, %ymm0, %ymm0      /* acc0 += prod0 */
    vaddpd %ymm3, %ymm1, %ymm1      /* acc1 += prod1 */

    addq $64, %rdi
    addq $64, %rsi
    decq %rcx
    jnz .Lloop8

.Lscalar_check:
    vaddpd %ymm1, %ymm0, %ymm0      /* merge acc1 into acc0 */

    /* Horizontal add ymm0 (4 x f64) */
    vextractf128 $1, %ymm0, %xmm2   /* high 128 (2 x f64) */
    vaddpd %xmm2, %xmm0, %xmm0      /* sum into low 128 (2 x f64) */
    vmovhlps %xmm0, %xmm2, %xmm2
    vaddsd %xmm2, %xmm0, %xmm0      /* low f64 has vector sum */

    /* Handle remainder elements (n & 7) */
    movq %rdx, %rcx
    andq $7, %rcx
    jz .Ldone

.Lrem_loop:
    vmovsd (%rdi), %xmm2
    vmovsd (%rsi), %xmm3
    vmulsd %xmm3, %xmm2, %xmm2
    vaddsd %xmm2, %xmm0, %xmm0
    addq $8, %rdi
    addq $8, %rsi
    decq %rcx
    jnz .Lrem_loop

.Ldone:
    vzeroupper
    ret

.globl poler_asm_poly_eval
.type poler_asm_poly_eval, @function

/*
 * double poler_asm_poly_eval(double x, const double *coeffs, size_t count);
 * %xmm0 = x, %rdi = coeffs (from highest power down to c0), %rsi = count
 * Horner's scheme: result = result * x + coeffs[i]
 */
poler_asm_poly_eval:
    testq %rsi, %rsi
    jz .Lpoly_zero

    vmovsd (%rdi), %xmm1            /* result = coeffs[0] */
    decq %rsi
    addq $8, %rdi
    jz .Lpoly_done

.Lpoly_loop:
    vmulsd %xmm0, %xmm1, %xmm1      /* result * x */
    vaddsd (%rdi), %xmm1, %xmm1      /* + coeffs[i] */
    addq $8, %rdi
    decq %rsi
    jnz .Lpoly_loop

.Lpoly_done:
    vmovapd %xmm1, %xmm0
    ret

.Lpoly_zero:
    vxorpd %xmm0, %xmm0, %xmm0
    ret

.globl poler_asm_fast_invsqrt
.type poler_asm_fast_invsqrt, @function

/*
 * double poler_asm_fast_invsqrt(double x);
 * %xmm0 = x
 * Uses rsqrtss + 2 Newton-Raphson steps in double precision
 */
poler_asm_fast_invsqrt:
    vcvtsd2ss %xmm0, %xmm1, %xmm1   /* float x */
    vrsqrtss %xmm1, %xmm1, %xmm1    /* approx 1/sqrt(x) */
    vcvtss2sd %xmm1, %xmm1, %xmm1   /* double y0 */

    /* Constants 1.5 and 0.5 */
    movabsq $0x3ff8000000000000, %rax /* 1.5 */
    vmovq %rax, %xmm2
    movabsq $0x3fe0000000000000, %rax /* 0.5 */
    vmovq %rax, %xmm3

    /* NR step 1: y1 = y0 * (1.5 - 0.5 * x * y0 * y0) */
    vmulsd %xmm1, %xmm0, %xmm4      /* x * y0 */
    vmulsd %xmm1, %xmm4, %xmm4      /* x * y0 * y0 */
    vmulsd %xmm3, %xmm4, %xmm4      /* 0.5 * x * y0^2 */
    vsubsd %xmm4, %xmm2, %xmm4      /* 1.5 - 0.5 * x * y0^2 */
    vmulsd %xmm4, %xmm1, %xmm1      /* y1 */

    /* NR step 2 for full 53-bit IEEE754 precision */
    vmulsd %xmm1, %xmm0, %xmm4
    vmulsd %xmm1, %xmm4, %xmm4
    vmulsd %xmm3, %xmm4, %xmm4
    vsubsd %xmm4, %xmm2, %xmm4
    vmulsd %xmm4, %xmm1, %xmm0      /* y2 returned in %xmm0 */
    ret
