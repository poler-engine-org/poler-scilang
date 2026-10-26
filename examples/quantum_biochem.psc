// =========================================================================
// POLER-SciLang Example 2: Quantum Photobiology & Chemical Thermodynamics
// Photosynthetic photon absorption, ATP phosphorylation, and Arrhenius kinetics
// =========================================================================

// 1. Quantum Photon Absorption (Chlorophyll-a absorption peak at 680 nm)
let lambda_red = 680.0 nm;
let e_photon = photon_energy_lambda(lambda_red);
emit "Photon Energy (J)": e_photon;
emit "Photon Energy (eV)": e_photon to eV;

// 2. Photosynthesis Reaction: 6 CO2 + 6 H2O -> C6H12O6 + 6 O2
let m_glucose = chem_molar_mass("C6H12O6");
let m_water = chem_molar_mass("H2O");
let m_co2 = chem_molar_mass("CO2");
emit "Glucose Molar Mass": m_glucose to g/mol;
emit "Water Molar Mass": m_water to g/mol;
emit "Carbon Dioxide Molar Mass": m_co2 to g/mol;

// 3. Biochemical Reaction Kinetics (Arrhenius Equation at 310.15 K / 37 deg C body temp)
let T_cell = 310.15 K;
let A_enz = 1.0e11; // frequency factor (1/s)
let E_activation = 52.0 kJ/mol; // enzymatic barrier
let rate_k = arrhenius(A_enz, E_activation, T_cell);
emit "Enzyme Reaction Rate k (1/s)": rate_k;

// 4. Zig Runge-Kutta 4 Numerical Integration of Substrate Decay
// dy/dt = -k*y over 5 time steps of dt = 0.001 s
let substrate_0 = 100.0;
let dt = 0.0001;
let sub_t = substrate_0;
for step in 0..5 {
    sub_t = zig_rk4(sub_t, 0.0, dt, 120.0, 0.0);
}
emit "Substrate remaining after RK4": sub_t;

// 5. Quantum Gate Simulation (Hadamard Superposition on Biomolecule State)
let H = hadamard();
let state = [1.0, 0.0; 0.0, 0.0];
let superposition = matmul(H, state);
emit "Superposition State Matrix": superposition;
