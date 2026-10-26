// =========================================================================
// POLER-SciLang: Prebiotic DNA Tautomer under Interstellar Radiation
// Complete Multi-Domain Analytical & Numerical Solution (All Archetypes)
// Domains: Quantum Mechanics + Lindblad + Bateman Decay + Astrophysics + Bio ODE
// =========================================================================

// =========================================================================
// Archetype 1: Quantum Hamiltonian & Energy Spectrum
// =========================================================================
let eps = 2.0e-21;          // J (proton asymmetry energy)
let beta = 1.0e-23;         // J / (Gy/s) (radiation coupling)
let Delta = 5.0e-22;        // J (tunneling matrix element)
let hbar = 1.054571817e-34; // J*s (reduced Planck constant)

// Reference dose rate (D0 = 10 Gy/s)
let D0 = 10.0;              // Gy/s
let Omega0 = sqrt((eps + beta * D0)^2 + 4.0 * Delta^2);
let E_plus = Omega0 / 2.0;
let E_minus = -Omega0 / 2.0;

emit "1. Rabi Frequency Omega(D0) (J)": Omega0;
emit "1. Ground Energy Level E- (J)": E_minus;
emit "1. Excited Energy Level E+ (J)": E_plus;

// =========================================================================
// Archetype 2: Open Quantum System & Lindblad Bloch Dynamics
// =========================================================================
let gamma_rate = 1.0e6;     // 1/s (pure dephasing rate)
let Gamma_rate = 1.0e5;     // 1/s (spontaneous relaxation to |0>)
let kappa = 1.0e4;          // 1/(Gy*s) (radiation excitation rate to |1>)

let omega_z0 = (eps + beta * D0) / hbar;
let omega_x0 = 2.0 * Delta / hbar;
let Gamma_perp0 = 2.0 * gamma_rate + 0.5 * (Gamma_rate + kappa * D0);
let Gamma_par0 = Gamma_rate + kappa * D0;
let Delta_Gamma0 = kappa * D0 - Gamma_rate;

let denom0 = Gamma_par0 * (Gamma_perp0^2 + omega_z0^2) + omega_x0^2 * Gamma_perp0;
let z_ss0 = Delta_Gamma0 * (Gamma_perp0^2 + omega_z0^2) / denom0;
let p_ss0 = (1.0 + z_ss0) / 2.0;

// Bloch radius & von Neumann entropy
let r_ss0 = abs(z_ss0) * sqrt(1.0 + omega_x0^2 / (Gamma_perp0^2 + omega_z0^2));
let mu_plus0 = (1.0 + r_ss0) / 2.0;
let mu_minus0 = (1.0 - r_ss0) / 2.0;
let S0 = -mu_plus0 * ln(mu_plus0) - mu_minus0 * ln(mu_minus0);

emit "2. Stationary Inversion z_ss(D0)": z_ss0;
emit "2. Tautomer Population p_ss(D0)": p_ss0;
emit "2. Bloch Sphere Radius r_ss(D0)": r_ss0;
emit "2. von Neumann Entropy S(D0)": S0;

// =========================================================================
// Archetype 3: Nuclear Physics — Bateman Chain (U-238 daughter dynamics)
// =========================================================================
let lambda1 = 4.9e-18;      // 1/s (parent decay constant)
let lambda2 = 1.0e-13;      // 1/s (daughter decay constant)
let N10 = 1.0e24;           // initial parent nuclei
let a1 = 1.0e-12;           // Gy/decay parent
let a2 = 1.0e-11;           // Gy/decay daughter

// Peak time of daughter radioactivity
let t_peak = (1.0 / (lambda2 - lambda1)) * ln((a2 * lambda2^2) / (lambda1 * (a1 * (lambda2 - lambda1) + a2 * lambda2)));
let N1_peak = N10 * exp(-lambda1 * t_peak);
let N2_peak = (lambda1 * N10 / (lambda2 - lambda1)) * (exp(-lambda1 * t_peak) - exp(-lambda2 * t_peak));
let D_nuc_peak = a1 * lambda1 * N1_peak + a2 * lambda2 * N2_peak;

emit "3. Bateman Peak Time t_peak (s)": t_peak;
emit "3. Daughter Nuclei N2(t_peak)": N2_peak;
emit "3. Nuclear Dose Rate at Peak (Gy/s)": D_nuc_peak;

// =========================================================================
// Archetype 4: Astrophysics — Interstellar Cosmic Ray Modulation
// =========================================================================
let D_CR_0 = 5.0e-4;        // Gy/s (unattenuated cosmic ray dose)
let Sigma_cloud = 100.0;    // g/cm^2 (dust cloud column density)
let Lambda_att = 80.0;      // g/cm^2 (attenuation length)
let delta_var = 0.25;       // cosmic ray flux periodic modulation amplitude

let D_CR_max = D_CR_0 * exp(-Sigma_cloud / Lambda_att) * (1.0 + delta_var);
let D_max = D_nuc_peak + D_CR_max;

emit "4. Cosmic Ray Peak Rate (Gy/s)": D_CR_max;
emit "4. Total Max Dose Rate D_max (Gy/s)": D_max;

// =========================================================================
// Archetype 5: Quantum Equilibrium at Environmental Maximum D_max
// =========================================================================
let omega_z_max = (eps + beta * D_max) / hbar;
let Gamma_perp_max = 2.0 * gamma_rate + 0.5 * (Gamma_rate + kappa * D_max);
let Gamma_par_max = Gamma_rate + kappa * D_max;
let Delta_Gamma_max = kappa * D_max - Gamma_rate;

let denom_max = Gamma_par_max * (Gamma_perp_max^2 + omega_z_max^2) + omega_x0^2 * Gamma_perp_max;
let z_ss_max = Delta_Gamma_max * (Gamma_perp_max^2 + omega_z_max^2) / denom_max;
let p_ss_max = (1.0 + z_ss_max) / 2.0;

let r_ss_max = abs(z_ss_max) * sqrt(1.0 + omega_x0^2 / (Gamma_perp_max^2 + omega_z_max^2));
let mu_plus_max = (1.0 + r_ss_max) / 2.0;
let mu_minus_max = (1.0 - r_ss_max) / 2.0;
let S_max = -mu_plus_max * ln(mu_plus_max) - mu_minus_max * ln(mu_minus_max);

emit "5. Peak Tautomer Fraction p_ss(D_max)": p_ss_max;
emit "5. Peak Bloch Radius r_ss(D_max)": r_ss_max;
emit "5. Peak von Neumann Entropy S(D_max)": S_max;

// =========================================================================
// Archetype 6: Molecular Biology & Non-Linear Repair Dynamics
// =========================================================================
let k0 = 1.0e-8;            // 1/s (spontaneous damage rate)
let k1 = 5.0e-4;            // 1/s (tautomeric mispairing rate)
let M_star = 0.05;          // 5% maximum permissible damage threshold
let m0 = 0.01;              // 1% initial damage fraction

// Analytical minimum repair rate condition: k_r >= [k0 + k1 * p_ss(D_max)] * (1 - M*) / M*
let k_r_min = (k0 + k1 * p_ss_max) * (1.0 - M_star) / M_star;

emit "6. Damage Threshold M*": M_star;
emit "6. Required Repair Rate k_r_min (1/s)": k_r_min;

// Direct Dynamic ODE Simulation: dm/dt = A*(1 - m) - k_r*m
let kr_active = 0.0040;     // active enzymatic repair (above k_r_min)
let m = m0;
let dt = 10.0;              // seconds per time step

for step in 0..100 {
    let dm = ((k0 + k1 * p_ss_max) * (1.0 - m) - kr_active * m) * dt;
    m = m + dm;
}

emit "6. Dynamic Simulated m(1000s)": m;
emit "6. Threshold Preserved (m <= M*)": m <= M_star;
