//! POLER-SciLang Value Types (Scalars with Units, Vectors, Matrices, Strings, Booleans)

use crate::units::{Dimensions, Unit};
use std::fmt;

#[derive(Clone, PartialEq, Debug)]
pub enum Value {
    Scalar(f64, Unit),
    Vector(Vec<f64>),
    Matrix {
        data: Vec<f64>,
        rows: usize,
        cols: usize,
    },
    Text(String),
    Bool(bool),
    Void,
}

impl Value {
    pub fn scalar_raw(val: f64) -> Self {
        Value::Scalar(val, Unit::dimensionless())
    }

    pub fn scalar(val: f64, unit: Unit) -> Self {
        Value::Scalar(val, unit)
    }

    pub fn as_f64(&self) -> Result<f64, String> {
        match self {
            Value::Scalar(v, _) => Ok(*v),
            Value::Bool(b) => Ok(if *b { 1.0 } else { 0.0 }),
            _ => Err(format!("Expected numeric scalar, got {:?}", self)),
        }
    }

    pub fn as_vec(&self) -> Result<&[f64], String> {
        match self {
            Value::Vector(v) => Ok(v.as_slice()),
            _ => Err(format!("Expected vector, got {:?}", self)),
        }
    }

    pub fn as_bool(&self) -> bool {
        match self {
            Value::Bool(b) => *b,
            Value::Scalar(v, _) => *v != 0.0,
            Value::Text(s) => !s.is_empty(),
            Value::Vector(v) => !v.is_empty(),
            Value::Matrix { data, .. } => !data.is_empty(),
            Value::Void => false,
        }
    }

    /// Convert to base SI representation (numerical value scaled to SI)
    pub fn to_si_scalar(&self) -> Result<(f64, Dimensions), String> {
        match self {
            Value::Scalar(v, u) => Ok((v * u.factor, u.dimensions)),
            _ => Err("Only scalars can be converted to SI base".to_string()),
        }
    }

    /// Add two values with dimensional checking
    pub fn add(&self, rhs: &Value) -> Result<Value, String> {
        match (self, rhs) {
            (Value::Scalar(v1, u1), Value::Scalar(v2, u2)) => {
                if u1.dimensions != u2.dimensions {
                    return Err(format!(
                        "Dimensional mismatch in addition: '{}' ({}) vs '{}' ({})",
                        u1.name, u1.dimensions, u2.name, u2.dimensions
                    ));
                }
                // Convert both to base SI, sum, then convert back to u1's unit
                let si1 = v1 * u1.factor;
                let si2 = v2 * u2.factor;
                let res_si = si1 + si2;
                let res_val = if u1.factor != 0.0 { res_si / u1.factor } else { res_si };
                Ok(Value::Scalar(res_val, u1.clone()))
            }
            (Value::Vector(a), Value::Vector(b)) => {
                if a.len() != b.len() {
                    return Err(format!("Vector length mismatch: {} vs {}", a.len(), b.len()));
                }
                let res: Vec<f64> = a.iter().zip(b.iter()).map(|(x, y)| x + y).collect();
                Ok(Value::Vector(res))
            }
            (
                Value::Matrix { data: d1, rows: r1, cols: c1 },
                Value::Matrix { data: d2, rows: r2, cols: c2 },
            ) => {
                if r1 != r2 || c1 != c2 {
                    return Err(format!("Matrix dimension mismatch: {}x{} vs {}x{}", r1, c1, r2, c2));
                }
                let res: Vec<f64> = d1.iter().zip(d2.iter()).map(|(x, y)| x + y).collect();
                Ok(Value::Matrix { data: res, rows: *r1, cols: *c1 })
            }
            (Value::Text(s1), Value::Text(s2)) => Ok(Value::Text(format!("{}{}", s1, s2))),
            (Value::Text(s1), any) => Ok(Value::Text(format!("{}{}", s1, any))),
            (any, Value::Text(s2)) => Ok(Value::Text(format!("{}{}", any, s2))),
            _ => Err(format!("Cannot add {:?} and {:?}", self, rhs)),
        }
    }

    /// Subtract two values with dimensional checking
    pub fn sub(&self, rhs: &Value) -> Result<Value, String> {
        match (self, rhs) {
            (Value::Scalar(v1, u1), Value::Scalar(v2, u2)) => {
                if u1.dimensions != u2.dimensions {
                    return Err(format!(
                        "Dimensional mismatch in subtraction: '{}' ({}) vs '{}' ({})",
                        u1.name, u1.dimensions, u2.name, u2.dimensions
                    ));
                }
                let si1 = v1 * u1.factor;
                let si2 = v2 * u2.factor;
                let res_si = si1 - si2;
                let res_val = if u1.factor != 0.0 { res_si / u1.factor } else { res_si };
                Ok(Value::Scalar(res_val, u1.clone()))
            }
            (Value::Vector(a), Value::Vector(b)) => {
                if a.len() != b.len() {
                    return Err(format!("Vector length mismatch: {} vs {}", a.len(), b.len()));
                }
                let res: Vec<f64> = a.iter().zip(b.iter()).map(|(x, y)| x - y).collect();
                Ok(Value::Vector(res))
            }
            (
                Value::Matrix { data: d1, rows: r1, cols: c1 },
                Value::Matrix { data: d2, rows: r2, cols: c2 },
            ) => {
                if r1 != r2 || c1 != c2 {
                    return Err(format!("Matrix dimension mismatch: {}x{} vs {}x{}", r1, c1, r2, c2));
                }
                let res: Vec<f64> = d1.iter().zip(d2.iter()).map(|(x, y)| x - y).collect();
                Ok(Value::Matrix { data: res, rows: *r1, cols: *c1 })
            }
            _ => Err(format!("Cannot subtract {:?} and {:?}", self, rhs)),
        }
    }

    /// Multiply two values with dimensional algebra
    pub fn mul(&self, rhs: &Value) -> Result<Value, String> {
        match (self, rhs) {
            (Value::Scalar(v1, u1), Value::Scalar(v2, u2)) => {
                let val_si = (v1 * u1.factor) * (v2 * u2.factor);
                let dims = u1.dimensions.mul(&u2.dimensions);
                let unit = Unit::from_dimensions(dims);
                let display_val = val_si / unit.factor;
                Ok(Value::Scalar(display_val, unit))
            }
            (Value::Scalar(s, _), Value::Vector(v)) | (Value::Vector(v), Value::Scalar(s, _)) => {
                let res = v.iter().map(|x| x * s).collect();
                Ok(Value::Vector(res))
            }
            (Value::Scalar(s, _), Value::Matrix { data, rows, cols })
            | (Value::Matrix { data, rows, cols }, Value::Scalar(s, _)) => {
                let res = data.iter().map(|x| x * s).collect();
                Ok(Value::Matrix { data: res, rows: *rows, cols: *cols })
            }
            (
                Value::Matrix { data: d1, rows: r1, cols: c1 },
                Value::Matrix { data: d2, rows: r2, cols: c2 },
            ) => {
                if c1 != r2 {
                    return Err(format!(
                        "Matrix multiplication dimension error: A({}x{}) cannot multiply B({}x{})",
                        r1, c1, r2, c2
                    ));
                }
                let res = crate::native::zig_matrix_mul(d1, d2, *r1, *c1, *c2);
                Ok(Value::Matrix { data: res, rows: *r1, cols: *c2 })
            }
            _ => Err(format!("Cannot multiply {:?} and {:?}", self, rhs)),
        }
    }

    /// Divide two values with dimensional algebra
    pub fn div(&self, rhs: &Value) -> Result<Value, String> {
        match (self, rhs) {
            (Value::Scalar(v1, u1), Value::Scalar(v2, u2)) => {
                if *v2 == 0.0 {
                    return Err("Division by zero in scientific expression".to_string());
                }
                let val_si = (v1 * u1.factor) / (v2 * u2.factor);
                let dims = u1.dimensions.div(&u2.dimensions);
                let unit = Unit::from_dimensions(dims);
                let display_val = val_si / unit.factor;
                Ok(Value::Scalar(display_val, unit))
            }
            (Value::Vector(v), Value::Scalar(s, _)) => {
                if *s == 0.0 {
                    return Err("Division of vector by zero".to_string());
                }
                let res = v.iter().map(|x| x / s).collect();
                Ok(Value::Vector(res))
            }
            (Value::Matrix { data, rows, cols }, Value::Scalar(s, _)) => {
                if *s == 0.0 {
                    return Err("Division of matrix by zero".to_string());
                }
                let res = data.iter().map(|x| x / s).collect();
                Ok(Value::Matrix { data: res, rows: *rows, cols: *cols })
            }
            _ => Err(format!("Cannot divide {:?} by {:?}", self, rhs)),
        }
    }

    /// Exponentiation
    pub fn pow(&self, rhs: &Value) -> Result<Value, String> {
        match (self, rhs) {
            (Value::Scalar(v1, u1), Value::Scalar(v2, u2)) => {
                if !u2.is_dimensionless() {
                    return Err("Exponent must be dimensionless".to_string());
                }
                let p = *v2;
                if u1.is_dimensionless() {
                    Ok(Value::Scalar(v1.powf(p), Unit::dimensionless()))
                } else {
                    if p.fract() != 0.0 {
                        return Err("Non-integer exponent on dimensioned quantity".to_string());
                    }
                    let p_i = p as i8;
                    let val_si = (v1 * u1.factor).powi(p_i as i32);
                    let dims = u1.dimensions.pow(p_i);
                    let unit = Unit::from_dimensions(dims);
                    let display_val = val_si / unit.factor;
                    Ok(Value::Scalar(display_val, unit))
                }
            }
            _ => Err(format!("Cannot raise {:?} to power {:?}", self, rhs)),
        }
    }

    /// Convert to target unit (e.g. `100 km/h to m/s` or `5000 J to kcal`)
    pub fn convert_to(&self, target_unit_name: &str) -> Result<Value, String> {
        match self {
            Value::Scalar(val, u) => {
                let target = crate::units::lookup_unit(target_unit_name)
                    .ok_or_else(|| format!("Unknown target unit '{}'", target_unit_name))?;
                if u.dimensions != target.dimensions {
                    return Err(format!(
                        "Incompatible dimensions for conversion: current '{}' ({}) vs target '{}' ({})",
                        u.name, u.dimensions, target.name, target.dimensions
                    ));
                }
                let si_val = val * u.factor;
                let converted = si_val / target.factor;
                Ok(Value::Scalar(converted, target))
            }
            _ => Err("Only scalar quantities with units can be converted".to_string()),
        }
    }
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Value::Scalar(v, u) => {
                let unit_str = format!("{}", u);
                if unit_str.is_empty() {
                    if v.fract() == 0.0 && v.abs() < 1e12 {
                        write!(f, "{:.0}", v)
                    } else if v.abs() >= 1e9 || (v.abs() < 1e-4 && *v != 0.0) {
                        write!(f, "{:e}", v)
                    } else {
                        write!(f, "{:.6}", v)
                    }
                } else {
                    if v.abs() >= 1e9 || (v.abs() < 1e-4 && *v != 0.0) {
                        write!(f, "{:e} {}", v, unit_str)
                    } else {
                        write!(f, "{:.6} {}", v, unit_str)
                    }
                }
            }
            Value::Vector(v) => {
                let items: Vec<String> = v
                    .iter()
                    .map(|x| {
                        if x.abs() >= 1e7 || (x.abs() < 1e-4 && *x != 0.0) {
                            format!("{:e}", x)
                        } else {
                            format!("{:.4}", x)
                        }
                    })
                    .collect();
                write!(f, "[{}]", items.join(", "))
            }
            Value::Matrix { data, rows, cols } => {
                writeln!(f, "[")?;
                for r in 0..*rows {
                    write!(f, "  [")?;
                    for c in 0..*cols {
                        let idx = r * cols + c;
                        write!(f, "{:8.4}", data[idx])?;
                        if c + 1 < *cols {
                            write!(f, ", ")?;
                        }
                    }
                    writeln!(f, "]")?;
                }
                write!(f, "]")
            }
            Value::Text(s) => write!(f, "{}", s),
            Value::Bool(b) => write!(f, "{}", b),
            Value::Void => write!(f, "()"),
        }
    }
}
