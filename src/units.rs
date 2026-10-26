//! Dimensional Analysis & SI Unit Registry for POLER-SciLang

use std::fmt;

/// Dimensions in standard SI base: [Length (m), Mass (kg), Time (s), Current (A), Temp (K), Amount (mol), Luminous (cd)]
#[derive(Clone, Copy, PartialEq, Eq, Hash, Default, Debug)]
pub struct Dimensions {
    pub l: i8, // Length: m
    pub m: i8, // Mass: kg
    pub t: i8, // Time: s
    pub i: i8, // Electric current: A
    pub th: i8, // Temperature: K
    pub n: i8, // Amount: mol
    pub j: i8, // Luminous: cd
}

impl fmt::Display for Dimensions {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut parts = Vec::new();
        if self.m != 0 { parts.push(format_dim("kg", self.m)); }
        if self.l != 0 { parts.push(format_dim("m", self.l)); }
        if self.t != 0 { parts.push(format_dim("s", self.t)); }
        if self.i != 0 { parts.push(format_dim("A", self.i)); }
        if self.th != 0 { parts.push(format_dim("K", self.th)); }
        if self.n != 0 { parts.push(format_dim("mol", self.n)); }
        if parts.is_empty() {
            write!(f, "1")
        } else {
            write!(f, "{}", parts.join("*"))
        }
    }
}

impl Dimensions {
    pub const DIMENSIONLESS: Self = Dimensions { l: 0, m: 0, t: 0, i: 0, th: 0, n: 0, j: 0 };
    pub const LENGTH: Self = Dimensions { l: 1, m: 0, t: 0, i: 0, th: 0, n: 0, j: 0 };
    pub const MASS: Self = Dimensions { l: 0, m: 1, t: 0, i: 0, th: 0, n: 0, j: 0 };
    pub const TIME: Self = Dimensions { l: 0, m: 0, t: 1, i: 0, th: 0, n: 0, j: 0 };
    pub const CURRENT: Self = Dimensions { l: 0, m: 0, t: 0, i: 1, th: 0, n: 0, j: 0 };
    pub const TEMPERATURE: Self = Dimensions { l: 0, m: 0, t: 0, i: 0, th: 1, n: 0, j: 0 };
    pub const AMOUNT: Self = Dimensions { l: 0, m: 0, t: 0, i: 0, th: 0, n: 1, j: 0 };

    // Derived physical dimensions
    pub const VELOCITY: Self = Dimensions { l: 1, m: 0, t: -1, i: 0, th: 0, n: 0, j: 0 };
    pub const ACCELERATION: Self = Dimensions { l: 1, m: 0, t: -2, i: 0, th: 0, n: 0, j: 0 };
    pub const FORCE: Self = Dimensions { l: 1, m: 1, t: -2, i: 0, th: 0, n: 0, j: 0 }; // N = kg*m/s^2
    pub const ENERGY: Self = Dimensions { l: 2, m: 1, t: -2, i: 0, th: 0, n: 0, j: 0 }; // J = N*m
    pub const POWER: Self = Dimensions { l: 2, m: 1, t: -3, i: 0, th: 0, n: 0, j: 0 };  // W = J/s
    pub const PRESSURE: Self = Dimensions { l: -1, m: 1, t: -2, i: 0, th: 0, n: 0, j: 0 }; // Pa = N/m^2
    pub const CHARGE: Self = Dimensions { l: 0, m: 0, t: 1, i: 1, th: 0, n: 0, j: 0 }; // C = A*s
    pub const VOLTAGE: Self = Dimensions { l: 2, m: 1, t: -3, i: -1, th: 0, n: 0, j: 0 }; // V = W/A
    pub const RESISTANCE: Self = Dimensions { l: 2, m: 1, t: -3, i: -2, th: 0, n: 0, j: 0 }; // ohm = V/A
    pub const CAPACITANCE: Self = Dimensions { l: -2, m: -1, t: 4, i: 2, th: 0, n: 0, j: 0 }; // F = C/V
    pub const FREQUENCY: Self = Dimensions { l: 0, m: 0, t: -1, i: 0, th: 0, n: 0, j: 0 }; // Hz = 1/s

    pub fn is_dimensionless(&self) -> bool {
        *self == Self::DIMENSIONLESS
    }

    pub fn mul(&self, rhs: &Self) -> Self {
        Dimensions {
            l: self.l + rhs.l,
            m: self.m + rhs.m,
            t: self.t + rhs.t,
            i: self.i + rhs.i,
            th: self.th + rhs.th,
            n: self.n + rhs.n,
            j: self.j + rhs.j,
        }
    }

    pub fn div(&self, rhs: &Self) -> Self {
        Dimensions {
            l: self.l - rhs.l,
            m: self.m - rhs.m,
            t: self.t - rhs.t,
            i: self.i - rhs.i,
            th: self.th - rhs.th,
            n: self.n - rhs.n,
            j: self.j - rhs.j,
        }
    }

    pub fn pow(&self, p: i8) -> Self {
        Dimensions {
            l: self.l * p,
            m: self.m * p,
            t: self.t * p,
            i: self.i * p,
            th: self.th * p,
            n: self.n * p,
            j: self.j * p,
        }
    }
}

/// Unit definition with scaling factor relative to base SI
#[derive(Clone, PartialEq, Debug)]
pub struct Unit {
    pub name: String,
    pub dimensions: Dimensions,
    pub factor: f64, // multiply by factor to get base SI value
}

impl Unit {
    pub fn dimensionless() -> Self {
        Unit {
            name: String::new(),
            dimensions: Dimensions::DIMENSIONLESS,
            factor: 1.0,
        }
    }

    pub fn new(name: impl Into<String>, dimensions: Dimensions, factor: f64) -> Self {
        Unit {
            name: name.into(),
            dimensions,
            factor,
        }
    }

    pub fn is_dimensionless(&self) -> bool {
        self.dimensions.is_dimensionless()
    }

    pub fn from_dimensions(dims: Dimensions) -> Self {
        if dims.is_dimensionless() {
            Self::dimensionless()
        } else if dims == Dimensions::LENGTH {
            Self::new("m", dims, 1.0)
        } else if dims == Dimensions::MASS {
            Self::new("kg", dims, 1.0)
        } else if dims == Dimensions::TIME {
            Self::new("s", dims, 1.0)
        } else if dims == Dimensions::CURRENT {
            Self::new("A", dims, 1.0)
        } else if dims == Dimensions::TEMPERATURE {
            Self::new("K", dims, 1.0)
        } else if dims == Dimensions::AMOUNT {
            Self::new("mol", dims, 1.0)
        } else if dims == Dimensions::VELOCITY {
            Self::new("m/s", dims, 1.0)
        } else if dims == Dimensions::ACCELERATION {
            Self::new("m/s^2", dims, 1.0)
        } else if dims == Dimensions::FORCE {
            Self::new("N", dims, 1.0)
        } else if dims == Dimensions::ENERGY {
            Self::new("J", dims, 1.0)
        } else if dims == Dimensions::POWER {
            Self::new("W", dims, 1.0)
        } else if dims == Dimensions::PRESSURE {
            Self::new("Pa", dims, 1.0)
        } else if dims == Dimensions::VOLTAGE {
            Self::new("V", dims, 1.0)
        } else if dims == Dimensions::RESISTANCE {
            Self::new("ohm", dims, 1.0)
        } else if dims == Dimensions::CHARGE {
            Self::new("C", dims, 1.0)
        } else if dims == Dimensions::CAPACITANCE {
            Self::new("F", dims, 1.0)
        } else if dims == Dimensions::FREQUENCY {
            Self::new("Hz", dims, 1.0)
        } else {
            Self::new("", dims, 1.0)
        }
    }
}

impl fmt::Display for Unit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if !self.name.is_empty() {
            write!(f, "{}", self.name)
        } else if self.dimensions.is_dimensionless() {
            Ok(())
        } else {
            // Pretty print standard dimensions
            let mut parts = Vec::new();
            if self.dimensions == Dimensions::VELOCITY { return write!(f, "m/s"); }
            if self.dimensions == Dimensions::ACCELERATION { return write!(f, "m/s^2"); }
            if self.dimensions == Dimensions::FORCE { return write!(f, "N"); }
            if self.dimensions == Dimensions::ENERGY { return write!(f, "J"); }
            if self.dimensions == Dimensions::POWER { return write!(f, "W"); }
            if self.dimensions == Dimensions::PRESSURE { return write!(f, "Pa"); }
            if self.dimensions == Dimensions::VOLTAGE { return write!(f, "V"); }
            if self.dimensions == Dimensions::RESISTANCE { return write!(f, "ohm"); }
            if self.dimensions == Dimensions::CHARGE { return write!(f, "C"); }
            if self.dimensions == Dimensions::FREQUENCY { return write!(f, "Hz"); }

            if self.dimensions.m != 0 { parts.push(format_dim("kg", self.dimensions.m)); }
            if self.dimensions.l != 0 { parts.push(format_dim("m", self.dimensions.l)); }
            if self.dimensions.t != 0 { parts.push(format_dim("s", self.dimensions.t)); }
            if self.dimensions.i != 0 { parts.push(format_dim("A", self.dimensions.i)); }
            if self.dimensions.th != 0 { parts.push(format_dim("K", self.dimensions.th)); }
            if self.dimensions.n != 0 { parts.push(format_dim("mol", self.dimensions.n)); }
            write!(f, "{}", parts.join("*"))
        }
    }
}

fn format_dim(sym: &str, exp: i8) -> String {
    if exp == 1 {
        sym.to_string()
    } else {
        format!("{}^{}", sym, exp)
    }
}

/// Registry of standard physical and astronomical units
pub fn lookup_unit(name: &str) -> Option<Unit> {
    match name {
        // Dimensionless
        "" | "1" => Some(Unit::dimensionless()),
        "rad" => Some(Unit::new("rad", Dimensions::DIMENSIONLESS, 1.0)),
        "deg" => Some(Unit::new("deg", Dimensions::DIMENSIONLESS, std::f64::consts::PI / 180.0)),

        // Length
        "m" => Some(Unit::new("m", Dimensions::LENGTH, 1.0)),
        "km" => Some(Unit::new("km", Dimensions::LENGTH, 1e3)),
        "cm" => Some(Unit::new("cm", Dimensions::LENGTH, 1e-2)),
        "mm" => Some(Unit::new("mm", Dimensions::LENGTH, 1e-3)),
        "um" => Some(Unit::new("um", Dimensions::LENGTH, 1e-6)),
        "nm" => Some(Unit::new("nm", Dimensions::LENGTH, 1e-9)),
        "pm" => Some(Unit::new("pm", Dimensions::LENGTH, 1e-12)),
        "au" => Some(Unit::new("au", Dimensions::LENGTH, 1.495978707e11)),
        "ly" => Some(Unit::new("ly", Dimensions::LENGTH, 9.4607304725808e15)),
        "pc" => Some(Unit::new("pc", Dimensions::LENGTH, 3.085677581491367e16)),
        "mi" => Some(Unit::new("mi", Dimensions::LENGTH, 1609.344)),
        "ft" => Some(Unit::new("ft", Dimensions::LENGTH, 0.3048)),
        "in" => Some(Unit::new("in", Dimensions::LENGTH, 0.0254)),

        // Mass
        "kg" => Some(Unit::new("kg", Dimensions::MASS, 1.0)),
        "g" => Some(Unit::new("g", Dimensions::MASS, 1e-3)),
        "mg" => Some(Unit::new("mg", Dimensions::MASS, 1e-6)),
        "t" => Some(Unit::new("t", Dimensions::MASS, 1e3)),
        "u" | "Da" => Some(Unit::new("u", Dimensions::MASS, 1.66053906892e-27)),
        "lb" => Some(Unit::new("lb", Dimensions::MASS, 0.45359237)),
        "oz" => Some(Unit::new("oz", Dimensions::MASS, 0.028349523125)),
        "M_sun" => Some(Unit::new("M_sun", Dimensions::MASS, 1.9885e30)),
        "M_earth" => Some(Unit::new("M_earth", Dimensions::MASS, 5.9722e24)),

        // Time
        "s" => Some(Unit::new("s", Dimensions::TIME, 1.0)),
        "ms" => Some(Unit::new("ms", Dimensions::TIME, 1e-3)),
        "us" => Some(Unit::new("us", Dimensions::TIME, 1e-6)),
        "ns" => Some(Unit::new("ns", Dimensions::TIME, 1e-9)),
        "min" => Some(Unit::new("min", Dimensions::TIME, 60.0)),
        "h" => Some(Unit::new("h", Dimensions::TIME, 3600.0)),
        "day" => Some(Unit::new("day", Dimensions::TIME, 86400.0)),
        "yr" => Some(Unit::new("yr", Dimensions::TIME, 31557600.0)), // Julian year

        // Temperature
        "K" => Some(Unit::new("K", Dimensions::TEMPERATURE, 1.0)),

        // Current
        "A" => Some(Unit::new("A", Dimensions::CURRENT, 1.0)),
        "mA" => Some(Unit::new("mA", Dimensions::CURRENT, 1e-3)),
        "uA" => Some(Unit::new("uA", Dimensions::CURRENT, 1e-6)),

        // Amount of substance
        "mol" => Some(Unit::new("mol", Dimensions::AMOUNT, 1.0)),
        "mmol" => Some(Unit::new("mmol", Dimensions::AMOUNT, 1e-3)),

        // Frequency
        "Hz" => Some(Unit::new("Hz", Dimensions::FREQUENCY, 1.0)),
        "kHz" => Some(Unit::new("kHz", Dimensions::FREQUENCY, 1e3)),
        "MHz" => Some(Unit::new("MHz", Dimensions::FREQUENCY, 1e6)),
        "GHz" => Some(Unit::new("GHz", Dimensions::FREQUENCY, 1e9)),
        "THz" => Some(Unit::new("THz", Dimensions::FREQUENCY, 1e12)),

        // Force
        "N" => Some(Unit::new("N", Dimensions::FORCE, 1.0)),
        "kN" => Some(Unit::new("kN", Dimensions::FORCE, 1e3)),
        "MN" => Some(Unit::new("MN", Dimensions::FORCE, 1e6)),

        // Energy
        "J" => Some(Unit::new("J", Dimensions::ENERGY, 1.0)),
        "kJ" => Some(Unit::new("kJ", Dimensions::ENERGY, 1e3)),
        "MJ" => Some(Unit::new("MJ", Dimensions::ENERGY, 1e6)),
        "GJ" => Some(Unit::new("GJ", Dimensions::ENERGY, 1e9)),
        "eV" => Some(Unit::new("eV", Dimensions::ENERGY, 1.602176634e-19)),
        "keV" => Some(Unit::new("keV", Dimensions::ENERGY, 1.602176634e-16)),
        "MeV" => Some(Unit::new("MeV", Dimensions::ENERGY, 1.602176634e-13)),
        "GeV" => Some(Unit::new("GeV", Dimensions::ENERGY, 1.602176634e-10)),
        "cal" => Some(Unit::new("cal", Dimensions::ENERGY, 4.184)),
        "kcal" => Some(Unit::new("kcal", Dimensions::ENERGY, 4184.0)),

        // Power
        "W" => Some(Unit::new("W", Dimensions::POWER, 1.0)),
        "kW" => Some(Unit::new("kW", Dimensions::POWER, 1e3)),
        "MW" => Some(Unit::new("MW", Dimensions::POWER, 1e6)),
        "GW" => Some(Unit::new("GW", Dimensions::POWER, 1e9)),
        "hp" => Some(Unit::new("hp", Dimensions::POWER, 745.69987)),

        // Pressure
        "Pa" => Some(Unit::new("Pa", Dimensions::PRESSURE, 1.0)),
        "kPa" => Some(Unit::new("kPa", Dimensions::PRESSURE, 1e3)),
        "MPa" => Some(Unit::new("MPa", Dimensions::PRESSURE, 1e6)),
        "GPa" => Some(Unit::new("GPa", Dimensions::PRESSURE, 1e9)),
        "bar" => Some(Unit::new("bar", Dimensions::PRESSURE, 1e5)),
        "atm" => Some(Unit::new("atm", Dimensions::PRESSURE, 101325.0)),
        "psi" => Some(Unit::new("psi", Dimensions::PRESSURE, 6894.75729)),

        // Electromagnetism
        "C" => Some(Unit::new("C", Dimensions::CHARGE, 1.0)),
        "V" => Some(Unit::new("V", Dimensions::VOLTAGE, 1.0)),
        "mV" => Some(Unit::new("mV", Dimensions::VOLTAGE, 1e-3)),
        "kV" => Some(Unit::new("kV", Dimensions::VOLTAGE, 1e3)),
        "ohm" => Some(Unit::new("ohm", Dimensions::RESISTANCE, 1.0)),
        "kohm" => Some(Unit::new("kohm", Dimensions::RESISTANCE, 1e3)),
        "Mohm" => Some(Unit::new("Mohm", Dimensions::RESISTANCE, 1e6)),
        "F" => Some(Unit::new("F", Dimensions::CAPACITANCE, 1.0)),
        "uF" => Some(Unit::new("uF", Dimensions::CAPACITANCE, 1e-6)),
        "nF" => Some(Unit::new("nF", Dimensions::CAPACITANCE, 1e-9)),
        "pF" => Some(Unit::new("pF", Dimensions::CAPACITANCE, 1e-12)),

        // Velocity & Acceleration
        "m/s" => Some(Unit::new("m/s", Dimensions::VELOCITY, 1.0)),
        "km/h" => Some(Unit::new("km/h", Dimensions::VELOCITY, 1000.0 / 3600.0)),
        "mph" => Some(Unit::new("mph", Dimensions::VELOCITY, 0.44704)),
        "m/s^2" => Some(Unit::new("m/s^2", Dimensions::ACCELERATION, 1.0)),

        // Density
        "kg/m^3" => Some(Unit::new("kg/m^3", Dimensions { l: -3, m: 1, t: 0, i: 0, th: 0, n: 0, j: 0 }, 1.0)),
        "g/cm^3" => Some(Unit::new("g/cm^3", Dimensions { l: -3, m: 1, t: 0, i: 0, th: 0, n: 0, j: 0 }, 1000.0)),

        // Chemical & Thermal
        "J/mol" => Some(Unit::new("J/mol", Dimensions { l: 2, m: 1, t: -2, i: 0, th: 0, n: -1, j: 0 }, 1.0)),
        "kJ/mol" => Some(Unit::new("kJ/mol", Dimensions { l: 2, m: 1, t: -2, i: 0, th: 0, n: -1, j: 0 }, 1e3)),
        "J/K" => Some(Unit::new("J/K", Dimensions { l: 2, m: 1, t: -2, i: 0, th: -1, n: 0, j: 0 }, 1.0)),
        "J/(mol*K)" | "J/mol/K" => Some(Unit::new("J/(mol*K)", Dimensions { l: 2, m: 1, t: -2, i: 0, th: -1, n: -1, j: 0 }, 1.0)),

        _ => {
            // Check if slash composite like "A/B"
            if let Some(pos) = name.find('/') {
                let left_str = &name[..pos];
                let right_str = &name[pos + 1..];
                if let (Some(u1), Some(u2)) = (lookup_unit(left_str), lookup_unit(right_str)) {
                    let dims = u1.dimensions.div(&u2.dimensions);
                    let factor = u1.factor / u2.factor;
                    return Some(Unit::new(name, dims, factor));
                }
            }
            None
        }
    }
}
