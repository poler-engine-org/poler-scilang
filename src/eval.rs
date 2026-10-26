//! Evaluator and Execution Engine for POLER-SciLang

use crate::ast::*;
use crate::science::ScienceRegistry;
use crate::units::lookup_unit;
use crate::values::Value;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

#[derive(Clone, Debug)]
pub struct EmittedOutput {
    pub label: Option<String>,
    pub value: Value,
    pub formatted: String,
}

pub struct Environment {
    pub vars: HashMap<String, Value>,
    pub fns: HashMap<String, (Vec<String>, Vec<Stmt>)>,
    pub parent: Option<Rc<RefCell<Environment>>>,
}

impl Environment {
    pub fn new() -> Self {
        Environment {
            vars: HashMap::new(),
            fns: HashMap::new(),
            parent: None,
        }
    }

    pub fn with_parent(parent: Rc<RefCell<Environment>>) -> Self {
        Environment {
            vars: HashMap::new(),
            fns: HashMap::new(),
            parent: Some(parent),
        }
    }

    pub fn get_var(&self, name: &str) -> Option<Value> {
        if let Some(v) = self.vars.get(name) {
            Some(v.clone())
        } else if let Some(ref p) = self.parent {
            p.borrow().get_var(name)
        } else {
            None
        }
    }

    pub fn set_var(&mut self, name: String, val: Value) {
        self.vars.insert(name, val);
    }

    pub fn update_var(&mut self, name: &str, val: Value) -> bool {
        if self.vars.contains_key(name) {
            self.vars.insert(name.to_string(), val);
            true
        } else if let Some(ref p) = self.parent {
            p.borrow_mut().update_var(name, val)
        } else {
            false
        }
    }

    pub fn get_fn(&self, name: &str) -> Option<(Vec<String>, Vec<Stmt>)> {
        if let Some(f) = self.fns.get(name) {
            Some(f.clone())
        } else if let Some(ref p) = self.parent {
            p.borrow().get_fn(name)
        } else {
            None
        }
    }

    pub fn set_fn(&mut self, name: String, params: Vec<String>, body: Vec<Stmt>) {
        self.fns.insert(name, (params, body));
    }
}

pub enum Flow {
    Continue,
    Return(Value),
}

pub struct Interpreter {
    pub science: ScienceRegistry,
    pub global_env: Rc<RefCell<Environment>>,
    pub outputs: Vec<EmittedOutput>,
}

impl Interpreter {
    pub fn new() -> Self {
        Interpreter {
            science: ScienceRegistry::new(),
            global_env: Rc::new(RefCell::new(Environment::new())),
            outputs: Vec::new(),
        }
    }

    pub fn eval_program(&mut self, program: &Program) -> Result<Value, String> {
        let mut last_val = Value::Void;
        for stmt in &program.statements {
            match self.eval_stmt(stmt, self.global_env.clone())? {
                Flow::Continue => {}
                Flow::Return(v) => return Ok(v),
            }
        }
        if let Some(last_output) = self.outputs.last() {
            last_val = last_output.value.clone();
        }
        Ok(last_val)
    }

    pub fn eval_stmt(&mut self, stmt: &Stmt, env: Rc<RefCell<Environment>>) -> Result<Flow, String> {
        match stmt {
            Stmt::Let { name, init } => {
                let val = self.eval_expr(init, env.clone())?;
                env.borrow_mut().set_var(name.clone(), val);
                Ok(Flow::Continue)
            }
            Stmt::Assign { name, expr } => {
                let val = self.eval_expr(expr, env.clone())?;
                let updated = env.borrow_mut().update_var(name, val.clone());
                if !updated {
                    env.borrow_mut().set_var(name.clone(), val);
                }
                Ok(Flow::Continue)
            }
            Stmt::Emit { label, expr } => {
                let val = self.eval_expr(expr, env)?;
                let formatted = format!("{}", val);
                self.outputs.push(EmittedOutput {
                    label: label.clone(),
                    value: val,
                    formatted,
                });
                Ok(Flow::Continue)
            }
            Stmt::Expr(expr) => {
                let _ = self.eval_expr(expr, env)?;
                Ok(Flow::Continue)
            }
            Stmt::If { cond, then_branch, else_branch } => {
                let c_val = self.eval_expr(cond, env.clone())?;
                if c_val.as_bool() {
                    let local_env = Rc::new(RefCell::new(Environment::with_parent(env)));
                    for s in then_branch {
                        if let Flow::Return(v) = self.eval_stmt(s, local_env.clone())? {
                            return Ok(Flow::Return(v));
                        }
                    }
                } else if let Some(eb) = else_branch {
                    let local_env = Rc::new(RefCell::new(Environment::with_parent(env)));
                    for s in eb {
                        if let Flow::Return(v) = self.eval_stmt(s, local_env.clone())? {
                            return Ok(Flow::Return(v));
                        }
                    }
                }
                Ok(Flow::Continue)
            }
            Stmt::For { var, start, end, body } => {
                let s_val = self.eval_expr(start, env.clone())?.as_f64()? as i64;
                let e_val = self.eval_expr(end, env.clone())?.as_f64()? as i64;
                for i in s_val..e_val {
                    let local_env = Rc::new(RefCell::new(Environment::with_parent(env.clone())));
                    local_env.borrow_mut().set_var(var.clone(), Value::scalar_raw(i as f64));
                    for s in body {
                        if let Flow::Return(v) = self.eval_stmt(s, local_env.clone())? {
                            return Ok(Flow::Return(v));
                        }
                    }
                }
                Ok(Flow::Continue)
            }
            Stmt::While { cond, body } => {
                let mut iter_count = 0;
                while self.eval_expr(cond, env.clone())?.as_bool() {
                    let local_env = Rc::new(RefCell::new(Environment::with_parent(env.clone())));
                    for s in body {
                        if let Flow::Return(v) = self.eval_stmt(s, local_env.clone())? {
                            return Ok(Flow::Return(v));
                        }
                    }
                    iter_count += 1;
                    if iter_count > 1_000_000 {
                        return Err("Infinite loop detected: exceeded 1,000,000 iterations".to_string());
                    }
                }
                Ok(Flow::Continue)
            }
            Stmt::FnDef { name, params, body } => {
                env.borrow_mut().set_fn(name.clone(), params.clone(), body.clone());
                Ok(Flow::Continue)
            }
            Stmt::Return(opt_expr) => {
                let val = match opt_expr {
                    Some(e) => self.eval_expr(e, env)?,
                    None => Value::Void,
                };
                Ok(Flow::Return(val))
            }
        }
    }

    pub fn eval_expr(&mut self, expr: &Expr, env: Rc<RefCell<Environment>>) -> Result<Value, String> {
        match expr {
            Expr::Number { val, unit } => {
                let u = match unit {
                    Some(u_str) => lookup_unit(u_str)
                        .ok_or_else(|| format!("Unknown physical unit '{}'", u_str))?,
                    None => crate::units::Unit::dimensionless(),
                };
                Ok(Value::Scalar(*val, u))
            }
            Expr::Str(s) => Ok(Value::Text(s.clone())),
            Expr::Bool(b) => Ok(Value::Bool(*b)),
            Expr::Ident(name) => {
                // Check environment variables first
                if let Some(v) = env.borrow().get_var(name) {
                    return Ok(v);
                }
                // Check science constants (c, G, h, k_B, M_sun, etc.)
                if let Some(c) = self.science.get_constant(name) {
                    return Ok(c);
                }
                // Check physical units as standalone unit quantities (e.g. mol, kg, s, m, J, etc.)
                if let Some(u) = lookup_unit(name) {
                    return Ok(Value::Scalar(1.0, u));
                }
                Err(format!("Undefined variable or constant '{}'", name))
            }
            Expr::Vector(elements) => {
                let mut data = Vec::with_capacity(elements.len());
                for e in elements {
                    let val = self.eval_expr(e, env.clone())?;
                    data.push(val.as_f64()?);
                }
                Ok(Value::Vector(data))
            }
            Expr::Matrix { rows, cols, elements } => {
                let mut data = Vec::with_capacity(elements.len());
                for e in elements {
                    let val = self.eval_expr(e, env.clone())?;
                    data.push(val.as_f64()?);
                }
                Ok(Value::Matrix {
                    data,
                    rows: *rows,
                    cols: *cols,
                })
            }
            Expr::Unary { op, expr } => {
                let val = self.eval_expr(expr, env)?;
                match op {
                    UnaryOp::Neg => match val {
                        Value::Scalar(v, u) => Ok(Value::Scalar(-v, u)),
                        Value::Vector(v) => Ok(Value::Vector(v.iter().map(|x| -x).collect())),
                        Value::Matrix { data, rows, cols } => {
                            Ok(Value::Matrix { data: data.iter().map(|x| -x).collect(), rows, cols })
                        }
                        _ => Err(format!("Cannot negate non-numeric value {:?}", val)),
                    },
                    UnaryOp::Not => Ok(Value::Bool(!val.as_bool())),
                }
            }
            Expr::Binary { op, left, right } => {
                if let BinaryOp::ConvertTo(target_unit) = op {
                    let left_val = self.eval_expr(left, env)?;
                    return left_val.convert_to(target_unit);
                }

                let l_val = self.eval_expr(left, env.clone())?;
                let r_val = self.eval_expr(right, env.clone())?;

                match op {
                    BinaryOp::Add => l_val.add(&r_val),
                    BinaryOp::Sub => l_val.sub(&r_val),
                    BinaryOp::Mul => l_val.mul(&r_val),
                    BinaryOp::Div => l_val.div(&r_val),
                    BinaryOp::Pow => l_val.pow(&r_val),
                    BinaryOp::Mod => {
                        let l_num = l_val.as_f64()?;
                        let r_num = r_val.as_f64()?;
                        Ok(Value::scalar_raw(l_num % r_num))
                    }
                    BinaryOp::Eq => Ok(Value::Bool(l_val == r_val)),
                    BinaryOp::Neq => Ok(Value::Bool(l_val != r_val)),
                    BinaryOp::Lt => Ok(Value::Bool(l_val.as_f64()? < r_val.as_f64()?)),
                    BinaryOp::Lte => Ok(Value::Bool(l_val.as_f64()? <= r_val.as_f64()?)),
                    BinaryOp::Gt => Ok(Value::Bool(l_val.as_f64()? > r_val.as_f64()?)),
                    BinaryOp::Gte => Ok(Value::Bool(l_val.as_f64()? >= r_val.as_f64()?)),
                    BinaryOp::And => Ok(Value::Bool(l_val.as_bool() && r_val.as_bool())),
                    BinaryOp::Or => Ok(Value::Bool(l_val.as_bool() || r_val.as_bool())),
                    BinaryOp::ConvertTo(_) => unreachable!(),
                }
            }
            Expr::Call { callee, args } => {
                let mut evaluated_args = Vec::with_capacity(args.len());
                for a in args {
                    evaluated_args.push(self.eval_expr(a, env.clone())?);
                }

                // Check user-defined functions
                if let Some((params, body)) = env.borrow().get_fn(callee) {
                    if params.len() != evaluated_args.len() {
                        return Err(format!(
                            "Function '{}' expects {} arguments, received {}",
                            callee, params.len(), evaluated_args.len()
                        ));
                    }
                    let call_env = Rc::new(RefCell::new(Environment::with_parent(self.global_env.clone())));
                    for (p, v) in params.iter().zip(evaluated_args.into_iter()) {
                        call_env.borrow_mut().set_var(p.clone(), v);
                    }
                    for stmt in &body {
                        if let Flow::Return(ret_val) = self.eval_stmt(stmt, call_env.clone())? {
                            return Ok(ret_val);
                        }
                    }
                    return Ok(Value::Void);
                }

                // Check multi-science builtin functions (astrophysics, quantum, chemistry, AVX2, Zig)
                self.science.call_function(callee, &evaluated_args)
            }
            Expr::Index { target, index } => {
                let t_val = self.eval_expr(target, env.clone())?;
                let idx = self.eval_expr(index, env)?.as_f64()? as usize;
                match t_val {
                    Value::Vector(v) => {
                        if idx < v.len() {
                            Ok(Value::scalar_raw(v[idx]))
                        } else {
                            Err(format!("Vector index out of bounds: {} >= {}", idx, v.len()))
                        }
                    }
                    _ => Err("Indexing is only supported on vectors".to_string()),
                }
            }
        }
    }
}
