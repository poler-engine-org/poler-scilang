//! Multi-Disciplinary Science Engine & Constant Registry for POLER-SciLang
//! Spans Astrophysics, Relativity, Quantum Mechanics, Thermodynamics, Chemistry,
//! Electromagnetism, Fluid Dynamics, and Native AVX2 ASM / Zig SIMD Hardware Acceleration.

use crate::native::*;
use crate::units::{Dimensions, Unit};
use crate::values::Value;
use std::collections::HashMap;

pub struct ScienceRegistry {
    constants: HashMap<String, Value>,
}

impl ScienceRegistry {
    pub fn new() -> Self {
        let mut constants = HashMap::new();

        // ---- Fundamental Universal Constants ----
        // Speed of light in vacuum c = 299,792,458 m/s
        constants.insert(
            "c".to_string(),
            Value::Scalar(299792458.0, Unit::new("m/s", Dimensions::VELOCITY, 1.0)),
        );
        // Gravitational constant G = 6.67430e-11 m^3 / (kg * s^2)
        constants.insert(
            "G".to_string(),
            Value::Scalar(
                6.67430e-11,
                Unit::new("m^3/(kg*s^2)", Dimensions { l: 3, m: -1, t: -2, i: 0, th: 0, n: 0, j: 0 }, 1.0),
            ),
        );
        // Planck constant h = 6.62607015e-34 J*s
        constants.insert(
            "h".to_string(),
            Value::Scalar(
                6.62607015e-34,
                Unit::new("J*s", Dimensions { l: 2, m: 1, t: -1, i: 0, th: 0, n: 0, j: 0 }, 1.0),
            ),
        );
        // Reduced Planck constant hbar = h / 2pi
        constants.insert(
            "hbar".to_string(),
            Value::Scalar(
                1.054571817e-34,
                Unit::new("J*s", Dimensions { l: 2, m: 1, t: -1, i: 0, th: 0, n: 0, j: 0 }, 1.0),
            ),
        );
        // Boltzmann constant k_B = 1.380649e-23 J/K
        constants.insert(
            "k_B".to_string(),
            Value::Scalar(
                1.380649e-23,
                Unit::new("J/K", Dimensions { l: 2, m: 1, t: -2, i: 0, th: -1, n: 0, j: 0 }, 1.0),
            ),
        );
        // Avogadro constant N_A = 6.02214076e23 / mol
        constants.insert(
            "N_A".to_string(),
            Value::Scalar(
                6.02214076e23,
                Unit::new("1/mol", Dimensions { l: 0, m: 0, t: 0, i: 0, th: 0, n: -1, j: 0 }, 1.0),
            ),
        );
        // Universal gas constant R = 8.314462618 J / (mol * K)
        constants.insert(
            "R".to_string(),
            Value::Scalar(
                8.314462618,
                Unit::new("J/(mol*K)", Dimensions { l: 2, m: 1, t: -2, i: 0, th: -1, n: -1, j: 0 }, 1.0),
            ),
        );
        // Elementary charge q_e = 1.602176634e-19 C
        constants.insert(
            "qe".to_string(),
            Value::Scalar(1.602176634e-19, Unit::new("C", Dimensions::CHARGE, 1.0)),
        );
        // Electron mass m_e = 9.1093837015e-31 kg
        constants.insert(
            "m_e".to_string(),
            Value::Scalar(9.1093837015e-31, Unit::new("kg", Dimensions::MASS, 1.0)),
        );
        // Proton mass m_p = 1.67262192369e-27 kg
        constants.insert(
            "m_p".to_string(),
            Value::Scalar(1.67262192369e-27, Unit::new("kg", Dimensions::MASS, 1.0)),
        );
        // Atomic mass unit u = 1.66053906660e-27 kg
        constants.insert(
            "u".to_string(),
            Value::Scalar(1.66053906660e-27, Unit::new("kg", Dimensions::MASS, 1.0)),
        );
        // Vacuum permittivity eps0 = 8.8541878128e-12 F/m
        constants.insert(
            "eps0".to_string(),
            Value::Scalar(
                8.8541878128e-12,
                Unit::new("F/m", Dimensions { l: -3, m: -1, t: 4, i: 2, th: 0, n: 0, j: 0 }, 1.0),
            ),
        );
        // Vacuum permeability mu0 = 1.25663706212e-6 N/A^2
        constants.insert(
            "mu0".to_string(),
            Value::Scalar(
                1.25663706212e-6,
                Unit::new("N/A^2", Dimensions { l: 1, m: 1, t: -2, i: -2, th: 0, n: 0, j: 0 }, 1.0),
            ),
        );
        // Stefan-Boltzmann constant sigma = 5.670374419e-8 W/(m^2 * K^4)
        constants.insert(
            "sigma_sb".to_string(),
            Value::Scalar(
                5.670374419e-8,
                Unit::new("W/(m^2*K^4)", Dimensions { l: 0, m: 1, t: -3, i: 0, th: -4, n: 0, j: 0 }, 1.0),
            ),
        );
        // Wien displacement law constant b = 2.897771955e-3 m * K
        constants.insert(
            "wien_b".to_string(),
            Value::Scalar(
                2.897771955e-3,
                Unit::new("m*K", Dimensions { l: 1, m: 0, t: 0, i: 0, th: 1, n: 0, j: 0 }, 1.0),
            ),
        );

        // ---- Astrophysics & Geophysics Constants ----
        constants.insert(
            "M_sun".to_string(),
            Value::Scalar(1.9885e30, Unit::new("kg", Dimensions::MASS, 1.0)),
        );
        constants.insert(
            "R_sun".to_string(),
            Value::Scalar(6.957e8, Unit::new("m", Dimensions::LENGTH, 1.0)),
        );
        constants.insert(
            "M_earth".to_string(),
            Value::Scalar(5.9722e24, Unit::new("kg", Dimensions::MASS, 1.0)),
        );
        constants.insert(
            "R_earth".to_string(),
            Value::Scalar(6.371e6, Unit::new("m", Dimensions::LENGTH, 1.0)),
        );
        constants.insert(
            "g0".to_string(),
            Value::Scalar(9.80665, Unit::new("m/s^2", Dimensions::ACCELERATION, 1.0)),
        );
        constants.insert(
            "AU".to_string(),
            Value::Scalar(1.495978707e11, Unit::new("m", Dimensions::LENGTH, 1.0)),
        );
        constants.insert(
            "ly".to_string(),
            Value::Scalar(9.4607304725808e15, Unit::new("m", Dimensions::LENGTH, 1.0)),
        );
        constants.insert(
            "pc".to_string(),
            Value::Scalar(3.085677581491367e16, Unit::new("m", Dimensions::LENGTH, 1.0)),
        );

        // ---- Mathematical Constants ----
        constants.insert("pi".to_string(), Value::scalar_raw(std::f64::consts::PI));
        constants.insert("e".to_string(), Value::scalar_raw(std::f64::consts::E));
        constants.insert("phi".to_string(), Value::scalar_raw(1.618033988749895));

        ScienceRegistry { constants }
    }

    pub fn get_constant(&self, name: &str) -> Option<Value> {
        self.constants.get(name).cloned()
    }

    pub fn call_function(&self, name: &str, args: &[Value]) -> Result<Value, String> {
        match name {
            // ---- Standard Math ----
            "sqrt" => {
                let x = Self::arg_f64(args, 0, "sqrt")?;
                if x < 0.0 {
                    return Err("sqrt of negative number".to_string());
                }
                let unit = Self::arg_unit(args, 0);
                let new_unit = if unit.is_dimensionless() {
                    unit
                } else {
                    Unit::from_dimensions(unit.dimensions.pow(1)) // fallback
                };
                Ok(Value::Scalar(x.sqrt(), new_unit))
            }
            "cbrt" => {
                let x = Self::arg_f64(args, 0, "cbrt")?;
                Ok(Value::scalar_raw(x.cbrt()))
            }
            "sin" => {
                let x = Self::arg_f64(args, 0, "sin")?;
                Ok(Value::scalar_raw(x.sin()))
            }
            "cos" => {
                let x = Self::arg_f64(args, 0, "cos")?;
                Ok(Value::scalar_raw(x.cos()))
            }
            "tan" => {
                let x = Self::arg_f64(args, 0, "tan")?;
                Ok(Value::scalar_raw(x.tan()))
            }
            "asin" => {
                let x = Self::arg_f64(args, 0, "asin")?;
                Ok(Value::scalar_raw(x.asin()))
            }
            "acos" => {
                let x = Self::arg_f64(args, 0, "acos")?;
                Ok(Value::scalar_raw(x.acos()))
            }
            "atan" => {
                let x = Self::arg_f64(args, 0, "atan")?;
                Ok(Value::scalar_raw(x.atan()))
            }
            "atan2" => {
                let y = Self::arg_f64(args, 0, "atan2")?;
                let x = Self::arg_f64(args, 1, "atan2")?;
                Ok(Value::scalar_raw(y.atan2(x)))
            }
            "exp" => {
                let x = Self::arg_f64(args, 0, "exp")?;
                Ok(Value::scalar_raw(x.exp()))
            }
            "ln" => {
                let x = Self::arg_f64(args, 0, "ln")?;
                if x <= 0.0 {
                    return Err("ln of non-positive number".to_string());
                }
                Ok(Value::scalar_raw(x.ln()))
            }
            "log10" => {
                let x = Self::arg_f64(args, 0, "log10")?;
                if x <= 0.0 {
                    return Err("log10 of non-positive number".to_string());
                }
                Ok(Value::scalar_raw(x.log10()))
            }
            "log2" => {
                let x = Self::arg_f64(args, 0, "log2")?;
                if x <= 0.0 {
                    return Err("log2 of non-positive number".to_string());
                }
                Ok(Value::scalar_raw(x.log2()))
            }
            "abs" => {
                match args.get(0) {
                    Some(Value::Scalar(v, u)) => Ok(Value::Scalar(v.abs(), u.clone())),
                    _ => Err("abs requires a scalar argument".to_string()),
                }
            }
            "round" => {
                let x = Self::arg_f64(args, 0, "round")?;
                Ok(Value::scalar_raw(x.round()))
            }
            "floor" => {
                let x = Self::arg_f64(args, 0, "floor")?;
                Ok(Value::scalar_raw(x.floor()))
            }
            "ceil" => {
                let x = Self::arg_f64(args, 0, "ceil")?;
                Ok(Value::scalar_raw(x.ceil()))
            }

            // ---- Native Assembly AVX2 / FMA Kernels ----
            "asm_dot" => {
                let (a, b) = Self::arg_two_vectors(args, "asm_dot")?;
                let res = asm_dot_product(&a, &b);
                Ok(Value::scalar_raw(res))
            }
            "asm_poly" => {
                let x = Self::arg_f64(args, 0, "asm_poly")?;
                let coeffs = Self::arg_vector(args, 1, "asm_poly")?;
                let res = asm_poly_eval(x, &coeffs);
                Ok(Value::scalar_raw(res))
            }
            "asm_invsqrt" => {
                let x = Self::arg_f64(args, 0, "asm_invsqrt")?;
                let res = asm_fast_invsqrt(x);
                Ok(Value::scalar_raw(res))
            }

            // ---- Native Zig SIMD & Matrix Kernels ----
            "zig_norm" | "norm" => {
                let v = Self::arg_vector(args, 0, "norm")?;
                let res = zig_vector_norm(&v);
                Ok(Value::scalar_raw(res))
            }
            "zig_fma" => {
                let a = Self::arg_vector(args, 0, "zig_fma")?;
                let b = Self::arg_vector(args, 1, "zig_fma")?;
                let c_init = Self::arg_vector(args, 2, "zig_fma")?;
                let mut c = c_init.clone();
                zig_vector_fma(&a, &b, &mut c);
                Ok(Value::Vector(c))
            }
            "zig_matmul" | "matmul" => {
                match (args.get(0), args.get(1)) {
                    (
                        Some(Value::Matrix { data: d1, rows: r1, cols: c1 }),
                        Some(Value::Matrix { data: d2, rows: r2, cols: c2 }),
                    ) => {
                        if c1 != r2 {
                            return Err(format!("Matrix size mismatch: {}x{} cannot multiply {}x{}", r1, c1, r2, c2));
                        }
                        let res = zig_matrix_mul(d1, d2, *r1, *c1, *c2);
                        Ok(Value::Matrix { data: res, rows: *r1, cols: *c2 })
                    }
                    _ => Err("matmul requires two matrix arguments".to_string()),
                }
            }
            "zig_rk4" | "rk4" => {
                // y_next = rk4(y, t, dt, rate_k, forcing)
                let y = Self::arg_f64(args, 0, "rk4")?;
                let t = Self::arg_f64(args, 1, "rk4")?;
                let dt = Self::arg_f64(args, 2, "rk4")?;
                let k = Self::arg_f64(args, 3, "rk4")?;
                let forcing = if args.len() > 4 { Self::arg_f64(args, 4, "rk4")? } else { 0.0 };
                let next_y = zig_integrate_rk4(y, t, dt, k, forcing);
                Ok(Value::scalar_raw(next_y))
            }

            // ---- Astrophysics & Space Mechanics ----
            "schwarzschild" => {
                // r_s = 2 * G * M / c^2
                let (m_si, _) = Self::arg_si(args, 0, "schwarzschild")?;
                let g = 6.67430e-11;
                let c = 299792458.0;
                let rs = 2.0 * g * m_si / (c * c);
                Ok(Value::Scalar(rs, Unit::new("m", Dimensions::LENGTH, 1.0)))
            }
            "escape_velocity" => {
                // v_esc = sqrt(2 * G * M / r)
                let (m_si, _) = Self::arg_si(args, 0, "escape_velocity")?;
                let (r_si, _) = Self::arg_si(args, 1, "escape_velocity")?;
                let g = 6.67430e-11;
                let v = (2.0 * g * m_si / r_si).sqrt();
                Ok(Value::Scalar(v, Unit::new("m/s", Dimensions::VELOCITY, 1.0)))
            }
            "orbital_velocity" => {
                // v_orb = sqrt(G * M / r)
                let (m_si, _) = Self::arg_si(args, 0, "orbital_velocity")?;
                let (r_si, _) = Self::arg_si(args, 1, "orbital_velocity")?;
                let g = 6.67430e-11;
                let v = (g * m_si / r_si).sqrt();
                Ok(Value::Scalar(v, Unit::new("m/s", Dimensions::VELOCITY, 1.0)))
            }
            "kepler3_period" => {
                // T = 2 * pi * sqrt(a^3 / (G * M))
                let (m_si, _) = Self::arg_si(args, 0, "kepler3_period")?;
                let (a_si, _) = Self::arg_si(args, 1, "kepler3_period")?;
                let g = 6.67430e-11;
                let period = 2.0 * std::f64::consts::PI * (a_si.powi(3) / (g * m_si)).sqrt();
                Ok(Value::Scalar(period, Unit::new("s", Dimensions::TIME, 1.0)))
            }
            "grav_force" => {
                // F = G * m1 * m2 / r^2
                let (m1, _) = Self::arg_si(args, 0, "grav_force")?;
                let (m2, _) = Self::arg_si(args, 1, "grav_force")?;
                let (r, _) = Self::arg_si(args, 2, "grav_force")?;
                let g = 6.67430e-11;
                let f = g * m1 * m2 / (r * r);
                Ok(Value::Scalar(f, Unit::new("N", Dimensions::FORCE, 1.0)))
            }
            "grav_time_dilation" => {
                // dt_proper = dt0 * sqrt(1 - 2*G*M / (r * c^2))
                let dt0 = Self::arg_f64(args, 0, "grav_time_dilation")?;
                let (m_si, _) = Self::arg_si(args, 1, "grav_time_dilation")?;
                let (r_si, _) = Self::arg_si(args, 2, "grav_time_dilation")?;
                let g = 6.67430e-11;
                let c = 299792458.0;
                let factor = (1.0 - 2.0 * g * m_si / (r_si * c * c)).sqrt();
                let u = Self::arg_unit(args, 0);
                Ok(Value::Scalar(dt0 * factor, u))
            }
            "roche_limit" => {
                // d = R_M * (2 * rho_M / rho_m)^(1/3)
                let (r_m, _) = Self::arg_si(args, 0, "roche_limit")?;
                let rho_m1 = Self::arg_f64(args, 1, "roche_limit")?;
                let rho_m2 = Self::arg_f64(args, 2, "roche_limit")?;
                let d = r_m * (2.0 * rho_m1 / rho_m2).cbrt();
                Ok(Value::Scalar(d, Unit::new("m", Dimensions::LENGTH, 1.0)))
            }
            "hubble_velocity" => {
                // v = H0 * d  (H0 in (km/s)/Mpc ~ 70 km/s/Mpc = 2.268e-18 1/s)
                let h0 = Self::arg_f64(args, 0, "hubble_velocity")?;
                let (d_si, _) = Self::arg_si(args, 1, "hubble_velocity")?;
                let h0_si = h0 * (1000.0 / 3.085677581491367e22);
                let v = h0_si * d_si;
                Ok(Value::Scalar(v, Unit::new("m/s", Dimensions::VELOCITY, 1.0)))
            }

            // ---- Relativity & High-Energy Physics ----
            "lorentz_gamma" | "gamma" => {
                // gamma = 1 / sqrt(1 - v^2 / c^2)
                let (v, _) = Self::arg_si(args, 0, "lorentz_gamma")?;
                let c = 299792458.0;
                let beta = v / c;
                if beta.abs() >= 1.0 {
                    return Err(format!("Velocity {} m/s exceeds speed of light", v));
                }
                let gamma = 1.0 / (1.0 - beta * beta).sqrt();
                Ok(Value::scalar_raw(gamma))
            }
            "emc2" => {
                // E = m * c^2
                let (m_si, _) = Self::arg_si(args, 0, "emc2")?;
                let c = 299792458.0;
                let e = m_si * c * c;
                Ok(Value::Scalar(e, Unit::new("J", Dimensions::ENERGY, 1.0)))
            }
            "rel_energy" => {
                // E = gamma * m * c^2
                let (m_si, _) = Self::arg_si(args, 0, "rel_energy")?;
                let (v_si, _) = Self::arg_si(args, 1, "rel_energy")?;
                let c = 299792458.0;
                let beta = v_si / c;
                if beta.abs() >= 1.0 {
                    return Err("Velocity exceeds speed of light".to_string());
                }
                let gamma = 1.0 / (1.0 - beta * beta).sqrt();
                let e = gamma * m_si * c * c;
                Ok(Value::Scalar(e, Unit::new("J", Dimensions::ENERGY, 1.0)))
            }
            "rel_momentum" => {
                // p = gamma * m * v
                let (m_si, _) = Self::arg_si(args, 0, "rel_momentum")?;
                let (v_si, _) = Self::arg_si(args, 1, "rel_momentum")?;
                let c = 299792458.0;
                let beta = v_si / c;
                let gamma = 1.0 / (1.0 - beta * beta).sqrt();
                let p = gamma * m_si * v_si;
                Ok(Value::Scalar(
                    p,
                    Unit::new("kg*m/s", Dimensions { l: 1, m: 1, t: -1, i: 0, th: 0, n: 0, j: 0 }, 1.0),
                ))
            }
            "doppler_freq" => {
                // f_observed = f0 * sqrt((1 - beta)/(1 + beta))
                let f0 = Self::arg_f64(args, 0, "doppler_freq")?;
                let (v_si, _) = Self::arg_si(args, 1, "doppler_freq")?;
                let c = 299792458.0;
                let beta = v_si / c;
                let factor = ((1.0 - beta) / (1.0 + beta)).sqrt();
                Ok(Value::Scalar(f0 * factor, Unit::new("Hz", Dimensions::FREQUENCY, 1.0)))
            }

            // ---- Quantum & Atomic Physics ----
            "photon_energy" => {
                // E = h * f
                let f = Self::arg_f64(args, 0, "photon_energy")?;
                let h = 6.62607015e-34;
                let e = h * f;
                Ok(Value::Scalar(e, Unit::new("J", Dimensions::ENERGY, 1.0)))
            }
            "photon_energy_lambda" => {
                // E = h * c / lambda
                let (lambda, _) = Self::arg_si(args, 0, "photon_energy_lambda")?;
                let h = 6.62607015e-34;
                let c = 299792458.0;
                let e = h * c / lambda;
                Ok(Value::Scalar(e, Unit::new("J", Dimensions::ENERGY, 1.0)))
            }
            "de_broglie" => {
                // lambda = h / p
                let (p, _) = Self::arg_si(args, 0, "de_broglie")?;
                let h = 6.62607015e-34;
                let lambda = h / p;
                Ok(Value::Scalar(lambda, Unit::new("m", Dimensions::LENGTH, 1.0)))
            }
            "stefan_boltzmann" => {
                // P = sigma * A * T^4
                let (area, _) = Self::arg_si(args, 0, "stefan_boltzmann")?;
                let (temp, _) = Self::arg_si(args, 1, "stefan_boltzmann")?;
                let sigma = 5.670374419e-8;
                let p = sigma * area * temp.powi(4);
                Ok(Value::Scalar(p, Unit::new("W", Dimensions::POWER, 1.0)))
            }
            "wien_displacement" => {
                // lambda_max = b / T
                let (temp, _) = Self::arg_si(args, 0, "wien_displacement")?;
                let b = 2.897771955e-3;
                let lambda = b / temp;
                Ok(Value::Scalar(lambda, Unit::new("m", Dimensions::LENGTH, 1.0)))
            }
            "rydberg_energy" => {
                // E_n = -13.605693 / n^2 eV
                let n = Self::arg_f64(args, 0, "rydberg_energy")?;
                let ev_to_j = 1.602176634e-19;
                let e_j = (-13.605693 / (n * n)) * ev_to_j;
                Ok(Value::Scalar(e_j, Unit::new("J", Dimensions::ENERGY, 1.0)))
            }
            "pauli_x" => Ok(Value::Matrix {
                data: vec![0.0, 1.0, 1.0, 0.0],
                rows: 2,
                cols: 2,
            }),
            "pauli_z" => Ok(Value::Matrix {
                data: vec![1.0, 0.0, 0.0, -1.0],
                rows: 2,
                cols: 2,
            }),
            "hadamard" => {
                let s = 1.0 / std::f64::consts::SQRT_2;
                Ok(Value::Matrix {
                    data: vec![s, s, s, -s],
                    rows: 2,
                    cols: 2,
                })
            }

            // ---- Thermodynamics & Physical Chemistry ----
            "ideal_gas_p" => {
                // P = n * R * T / V
                let (n, _) = Self::arg_si(args, 0, "ideal_gas_p")?;
                let (t, _) = Self::arg_si(args, 1, "ideal_gas_p")?;
                let (v, _) = Self::arg_si(args, 2, "ideal_gas_p")?;
                let r = 8.314462618;
                let p = n * r * t / v;
                Ok(Value::Scalar(p, Unit::new("Pa", Dimensions::PRESSURE, 1.0)))
            }
            "ideal_gas_v" => {
                // V = n * R * T / P
                let (n, _) = Self::arg_si(args, 0, "ideal_gas_v")?;
                let (t, _) = Self::arg_si(args, 1, "ideal_gas_v")?;
                let (p, _) = Self::arg_si(args, 2, "ideal_gas_v")?;
                let r = 8.314462618;
                let v = n * r * t / p;
                Ok(Value::Scalar(
                    v,
                    Unit::new("m^3", Dimensions { l: 3, m: 0, t: 0, i: 0, th: 0, n: 0, j: 0 }, 1.0),
                ))
            }
            "arrhenius" => {
                // k = A * exp(-Ea / (R * T))
                let a = Self::arg_f64(args, 0, "arrhenius")?;
                let (ea_si, _) = Self::arg_si(args, 1, "arrhenius")?;
                let (t_si, _) = Self::arg_si(args, 2, "arrhenius")?;
                let r = 8.314462618;
                let rate = a * (-ea_si / (r * t_si)).exp();
                Ok(Value::scalar_raw(rate))
            }
            "gibbs_free_energy" => {
                // dG = dH - T * dS
                let (dh, _) = Self::arg_si(args, 0, "gibbs_free_energy")?;
                let (t, _) = Self::arg_si(args, 1, "gibbs_free_energy")?;
                let (ds, _) = Self::arg_si(args, 2, "gibbs_free_energy")?;
                let dg = dh - t * ds;
                Ok(Value::Scalar(dg, Unit::new("J/mol", Dimensions { l: 2, m: 1, t: -2, i: 0, th: 0, n: -1, j: 0 }, 1.0)))
            }
            "chem_molar_mass" => {
                let formula = match args.get(0) {
                    Some(Value::Text(s)) => s.as_str(),
                    _ => return Err("chem_molar_mass requires a chemical formula string, e.g. \"H2O\"".to_string()),
                };
                let mass_g_per_mol = parse_chemical_molar_mass(formula)?;
                let mass_kg_per_mol = mass_g_per_mol * 1e-3;
                Ok(Value::Scalar(
                    mass_kg_per_mol,
                    Unit::new("kg/mol", Dimensions { l: 0, m: 1, t: 0, i: 0, th: 0, n: -1, j: 0 }, 1.0),
                ))
            }

            // ---- Electromagnetism & Circuits ----
            "coulomb_force" => {
                // F = (1 / 4*pi*eps0) * (q1 * q2 / r^2)
                let (q1, _) = Self::arg_si(args, 0, "coulomb_force")?;
                let (q2, _) = Self::arg_si(args, 1, "coulomb_force")?;
                let (r, _) = Self::arg_si(args, 2, "coulomb_force")?;
                let k_e = 8.9875517923e9; // 1 / (4*pi*eps0)
                let f = k_e * (q1 * q2).abs() / (r * r);
                Ok(Value::Scalar(f, Unit::new("N", Dimensions::FORCE, 1.0)))
            }
            "rc_tau" => {
                // tau = R * C
                let (r, _) = Self::arg_si(args, 0, "rc_tau")?;
                let (c, _) = Self::arg_si(args, 1, "rc_tau")?;
                let tau = r * c;
                Ok(Value::Scalar(tau, Unit::new("s", Dimensions::TIME, 1.0)))
            }
            "lc_freq" => {
                // f = 1 / (2 * pi * sqrt(L * C))
                let (l, _) = Self::arg_si(args, 0, "lc_freq")?;
                let (c, _) = Self::arg_si(args, 1, "lc_freq")?;
                let f = 1.0 / (2.0 * std::f64::consts::PI * (l * c).sqrt());
                Ok(Value::Scalar(f, Unit::new("Hz", Dimensions::FREQUENCY, 1.0)))
            }
            "magnetic_lorentz" => {
                // F = q * v * B
                let (q, _) = Self::arg_si(args, 0, "magnetic_lorentz")?;
                let (v, _) = Self::arg_si(args, 1, "magnetic_lorentz")?;
                let (b, _) = Self::arg_si(args, 2, "magnetic_lorentz")?;
                let f = q * v * b;
                Ok(Value::Scalar(f, Unit::new("N", Dimensions::FORCE, 1.0)))
            }

            // ---- Classical Mechanics & Rocketry ----
            "kinetic_energy" => {
                // E = 0.5 * m * v^2
                let (m, _) = Self::arg_si(args, 0, "kinetic_energy")?;
                let (v, _) = Self::arg_si(args, 1, "kinetic_energy")?;
                let e = 0.5 * m * v * v;
                Ok(Value::Scalar(e, Unit::new("J", Dimensions::ENERGY, 1.0)))
            }
            "reynolds_number" => {
                // Re = rho * v * L / mu
                let (rho, _) = Self::arg_si(args, 0, "reynolds_number")?;
                let (v, _) = Self::arg_si(args, 1, "reynolds_number")?;
                let (l, _) = Self::arg_si(args, 2, "reynolds_number")?;
                let (mu, _) = Self::arg_si(args, 3, "reynolds_number")?;
                let re = rho * v * l / mu;
                Ok(Value::scalar_raw(re))
            }
            "tsiolkovsky_dv" => {
                // dv = Isp * g0 * ln(m0 / mf)
                let isp = Self::arg_f64(args, 0, "tsiolkovsky_dv")?;
                let (m0, _) = Self::arg_si(args, 1, "tsiolkovsky_dv")?;
                let (mf, _) = Self::arg_si(args, 2, "tsiolkovsky_dv")?;
                let g0 = 9.80665;
                let dv = isp * g0 * (m0 / mf).ln();
                Ok(Value::Scalar(dv, Unit::new("m/s", Dimensions::VELOCITY, 1.0)))
            }
            "pendulum_period" => {
                // T = 2 * pi * sqrt(L / g)
                let (l, _) = Self::arg_si(args, 0, "pendulum_period")?;
                let g = if args.len() > 1 {
                    let (g_val, _) = Self::arg_si(args, 1, "pendulum_period")?;
                    g_val
                } else {
                    9.80665
                };
                let t = 2.0 * std::f64::consts::PI * (l / g).sqrt();
                Ok(Value::Scalar(t, Unit::new("s", Dimensions::TIME, 1.0)))
            }
            "sound_speed_air" => {
                // c_s = 331.3 * sqrt(T_K / 273.15)
                let (temp_k, _) = Self::arg_si(args, 0, "sound_speed_air")?;
                let cs = 331.3 * (temp_k / 273.15).sqrt();
                Ok(Value::Scalar(cs, Unit::new("m/s", Dimensions::VELOCITY, 1.0)))
            }

            _ => Err(format!("Unknown scientific function '{}'", name)),
        }
    }

    fn arg_f64(args: &[Value], idx: usize, func: &str) -> Result<f64, String> {
        match args.get(idx) {
            Some(Value::Scalar(v, _)) => Ok(*v),
            Some(Value::Bool(b)) => Ok(if *b { 1.0 } else { 0.0 }),
            _ => Err(format!("Function '{}' requires numeric argument at position {}", func, idx + 1)),
        }
    }

    fn arg_si(args: &[Value], idx: usize, func: &str) -> Result<(f64, Dimensions), String> {
        match args.get(idx) {
            Some(v @ Value::Scalar(..)) => v.to_si_scalar(),
            _ => Err(format!("Function '{}' requires scalar quantity at position {}", func, idx + 1)),
        }
    }

    fn arg_unit(args: &[Value], idx: usize) -> Unit {
        match args.get(idx) {
            Some(Value::Scalar(_, u)) => u.clone(),
            _ => Unit::dimensionless(),
        }
    }

    fn arg_vector(args: &[Value], idx: usize, func: &str) -> Result<Vec<f64>, String> {
        match args.get(idx) {
            Some(Value::Vector(v)) => Ok(v.clone()),
            _ => Err(format!("Function '{}' requires vector at position {}", func, idx + 1)),
        }
    }

    fn arg_two_vectors(args: &[Value], func: &str) -> Result<(Vec<f64>, Vec<f64>), String> {
        let v1 = Self::arg_vector(args, 0, func)?;
        let v2 = Self::arg_vector(args, 1, func)?;
        Ok((v1, v2))
    }
}

/// Chemical element weights (g/mol)
fn element_weight(elem: &str) -> Option<f64> {
    match elem {
        "H" => Some(1.008),
        "He" => Some(4.0026),
        "Li" => Some(6.94),
        "Be" => Some(9.0122),
        "B" => Some(10.81),
        "C" => Some(12.011),
        "N" => Some(14.007),
        "O" => Some(15.999),
        "F" => Some(18.998),
        "Ne" => Some(20.180),
        "Na" => Some(22.990),
        "Mg" => Some(24.305),
        "Al" => Some(26.982),
        "Si" => Some(28.085),
        "P" => Some(30.974),
        "S" => Some(32.06),
        "Cl" => Some(35.45),
        "Ar" => Some(39.948),
        "K" => Some(39.098),
        "Ca" => Some(40.078),
        "Ti" => Some(47.867),
        "Cr" => Some(51.996),
        "Mn" => Some(54.938),
        "Fe" => Some(55.845),
        "Ni" => Some(58.693),
        "Cu" => Some(63.546),
        "Zn" => Some(65.38),
        "Br" => Some(79.904),
        "Ag" => Some(107.8682),
        "I" => Some(126.904),
        "Au" => Some(196.96657),
        "Hg" => Some(200.592),
        "Pb" => Some(207.2),
        "U" => Some(238.02891),
        _ => None,
    }
}

/// Parse molecular formula and return molar mass in g/mol
pub fn parse_chemical_molar_mass(formula: &str) -> Result<f64, String> {
    let chars: Vec<char> = formula.chars().collect();
    let mut i = 0;
    let mut total_mass = 0.0;

    while i < chars.len() {
        if !chars[i].is_ascii_uppercase() {
            return Err(format!("Invalid chemical formula syntax at '{}'", &formula[i..]));
        }
        let mut sym = String::new();
        sym.push(chars[i]);
        i += 1;

        if i < chars.len() && chars[i].is_ascii_lowercase() {
            sym.push(chars[i]);
            i += 1;
        }

        let mut count_str = String::new();
        while i < chars.len() && chars[i].is_ascii_digit() {
            count_str.push(chars[i]);
            i += 1;
        }

        let count: f64 = if count_str.is_empty() {
            1.0
        } else {
            count_str.parse().map_err(|e| format!("Invalid element count: {}", e))?
        };

        let mass = element_weight(&sym)
            .ok_or_else(|| format!("Unknown chemical element '{}'", sym))?;
        total_mass += mass * count;
    }

    Ok(total_mass)
}
