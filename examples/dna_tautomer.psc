// Tautomer transition in DNA-like pair under interstellar radiation
let eps = 2.0e-21;
let beta = 1.0e-23;
let Delta = 5.0e-22;
let D = 10.0;

let Omega = sqrt((eps + beta * D)^2 + 4.0 * Delta^2);
let E_plus = Omega / 2.0;
let E_minus = -Omega / 2.0;

let Gamma_rate = 1.0e5;
let kappa = 1.0e4;
let gamma_rate = 1.0e6;
let hbar = 1.054571817e-34;

let omega_z = (eps + beta * D) / hbar;
let omega_x = 2.0 * Delta / hbar;
let Gamma_perp = 2.0 * gamma_rate + 0.5 * (Gamma_rate + kappa * D);
let Gamma_par = Gamma_rate + kappa * D;
let Delta_Gamma = kappa * D - Gamma_rate;

let denom = Gamma_par * (Gamma_perp^2 + omega_z^2) + omega_x^2 * Gamma_perp;
let z_ss = Delta_Gamma * (Gamma_perp^2 + omega_z^2) / denom;
let p_ss = (1.0 + z_ss) / 2.0;

emit "Rabi Frequency Omega": Omega;
emit "Eigenvalue E+": E_plus;
emit "Eigenvalue E-": E_minus;
emit "Steady population p_ss": p_ss;
