//! POLER-SciLang: Universal Multi-Disciplinary Scientific Language (AVX2 ASM + Zig SIMD + Rust)

mod ast;
mod eval;
mod lexer;
mod native;
mod science;
mod units;
mod values;

use eval::Interpreter;
use lexer::Lexer;
use parser::Parser;
use std::env;
use std::fs;
use std::io::{self, BufRead, Write};

mod parser;

fn main() {
    let args: Vec<String> = env::args().collect();

    let mut json_mode = false;
    let mut eval_code: Option<String> = None;
    let mut script_file: Option<String> = None;

    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            "--json" => {
                json_mode = true;
                i += 1;
            }
            "-e" => {
                if i + 1 < args.len() {
                    eval_code = Some(args[i + 1].clone());
                    i += 2;
                } else {
                    eprintln!("Error: -e requires code string");
                    std::process::exit(1);
                }
            }
            "-h" | "--help" => {
                print_help();
                return;
            }
            "--version" | "-v" => {
                println!("poler-scilang 1.0.0 (AVX2 ASM + Zig 0.14.0 + Rust 1.98.1)");
                return;
            }
            other => {
                if !other.starts_with('-') && script_file.is_none() {
                    script_file = Some(other.to_string());
                }
                i += 1;
            }
        }
    }

    if let Some(code) = eval_code {
        run_source(&code, json_mode);
    } else if let Some(file_path) = script_file {
        match fs::read_to_string(&file_path) {
            Ok(code) => run_source(&code, json_mode),
            Err(e) => {
                if json_mode {
                    print_json_error(&format!("Failed to read file '{}': {}", file_path, e));
                } else {
                    eprintln!("Error reading file '{}': {}", file_path, e);
                }
                std::process::exit(1);
            }
        }
    } else {
        run_repl();
    }
}

fn print_help() {
    println!(r#"POLER-SciLang: Universal Scientific Scripting Engine
Built with x86_64 AVX2/FMA Assembly + Zig SIMD + Rust Dimensional Engine

USAGE:
    poler-scilang [OPTIONS] [SCRIPT.psc]

OPTIONS:
    -e <code>       Execute inline script code
    --json          Output structured execution results and variables as JSON
    -h, --help      Display this help menu
    -v, --version   Display version information

INTERACTIVE:
    Run without arguments to enter the POLER-SciLang REPL.
"#);
}

fn run_source(source: &str, json_mode: bool) {
    let mut lexer = Lexer::new(source);
    let tokens = match lexer.tokenize() {
        Ok(t) => t,
        Err(e) => {
            if json_mode {
                print_json_error(&format!("Lexer error: {}", e));
            } else {
                eprintln!("Lexer error: {}", e);
            }
            std::process::exit(1);
        }
    };

    let mut parser = Parser::new(tokens);
    let program = match parser.parse_program() {
        Ok(p) => p,
        Err(e) => {
            if json_mode {
                print_json_error(&format!("Parser error: {}", e));
            } else {
                eprintln!("Parser error: {}", e);
            }
            std::process::exit(1);
        }
    };

    let mut interpreter = Interpreter::new();
    match interpreter.eval_program(&program) {
        Ok(_) => {
            if json_mode {
                print_json_success(&interpreter);
            } else {
                for out in &interpreter.outputs {
                    if let Some(ref l) = out.label {
                        println!("{}: {}", l, out.formatted);
                    } else {
                        println!("{}", out.formatted);
                    }
                }
            }
        }
        Err(e) => {
            if json_mode {
                print_json_error(&format!("Runtime error: {}", e));
            } else {
                eprintln!("Runtime error: {}", e);
            }
            std::process::exit(1);
        }
    }
}

fn print_json_error(err_msg: &str) {
    let escaped = escape_json(err_msg);
    println!(
        "{{\"success\": false, \"outputs\": [], \"variables\": {{}}, \"error\": \"{}\"}}",
        escaped
    );
}

fn print_json_success(interpreter: &Interpreter) {
    let mut out_parts = Vec::new();
    for o in &interpreter.outputs {
        let label = o.label.as_deref().unwrap_or("");
        let raw = match &o.value {
            values::Value::Scalar(v, _) => format!("{:.8e}", v),
            values::Value::Bool(b) => format!("{}", b),
            _ => "null".to_string(),
        };
        let unit = match &o.value {
            values::Value::Scalar(_, u) => format!("{}", u),
            _ => "".to_string(),
        };
        out_parts.push(format!(
            "{{\"label\": \"{}\", \"formatted\": \"{}\", \"unit\": \"{}\", \"raw\": {}}}",
            escape_json(label),
            escape_json(&o.formatted),
            escape_json(&unit),
            raw
        ));
    }

    let env = interpreter.global_env.borrow();
    let mut var_parts = Vec::new();
    for (k, v) in &env.vars {
        let fmt = format!("{}", v);
        let raw = match v {
            values::Value::Scalar(s, _) => format!("{:.8e}", s),
            _ => "null".to_string(),
        };
        let unit = match v {
            values::Value::Scalar(_, u) => format!("{}", u),
            _ => "".to_string(),
        };
        var_parts.push(format!(
            "\"{}\": {{\"formatted\": \"{}\", \"unit\": \"{}\", \"raw\": {}}}",
            escape_json(k),
            escape_json(&fmt),
            escape_json(&unit),
            raw
        ));
    }

    println!(
        "{{\"success\": true, \"outputs\": [{}], \"variables\": {{{}}}, \"error\": null}}",
        out_parts.join(", "),
        var_parts.join(", ")
    );
}

fn escape_json(s: &str) -> String {
    s.replace('\\', "\\\\")
        .replace('"', "\\\"")
        .replace('\n', "\\n")
        .replace('\r', "\\r")
        .replace('\t', "\\t")
}

fn run_repl() {
    println!("=== POLER-SciLang Interactive Scientific Engine (AVX2 + Zig + Rust) ===");
    println!("Type scientific equations, unit conversions, or multi-field code. Type 'exit' to quit.\n");

    let stdin = io::stdin();
    let mut stdout = io::stdout();
    let mut interpreter = Interpreter::new();

    loop {
        print!("scilang[Ψ] > ");
        let _ = stdout.flush();

        let mut line = String::new();
        if stdin.lock().read_line(&mut line).is_err() || line.trim() == "exit" || line.trim() == "quit" {
            break;
        }

        let trimmed = line.trim();
        if trimmed.is_empty() {
            continue;
        }

        // If line doesn't start with keyword and has no semicolons, wrap in emit
        let code = if !trimmed.starts_with("let ")
            && !trimmed.starts_with("fn ")
            && !trimmed.starts_with("if ")
            && !trimmed.starts_with("for ")
            && !trimmed.starts_with("emit ")
            && !trimmed.starts_with("print ")
            && !trimmed.contains('=')
        {
            format!("emit {};", trimmed.trim_end_matches(';'))
        } else if !trimmed.ends_with(';') && !trimmed.ends_with('}') {
            format!("{};", trimmed)
        } else {
            trimmed.to_string()
        };

        let mut lexer = Lexer::new(&code);
        let tokens = match lexer.tokenize() {
            Ok(t) => t,
            Err(e) => {
                eprintln!("Lexer error: {}", e);
                continue;
            }
        };

        let mut parser = Parser::new(tokens);
        let program = match parser.parse_program() {
            Ok(p) => p,
            Err(e) => {
                eprintln!("Parser error: {}", e);
                continue;
            }
        };

        let out_count_before = interpreter.outputs.len();
        match interpreter.eval_program(&program) {
            Ok(_) => {
                for out in &interpreter.outputs[out_count_before..] {
                    if let Some(ref l) = out.label {
                        println!("=> {}: {}", l, out.formatted);
                    } else {
                        println!("=> {}", out.formatted);
                    }
                }
            }
            Err(e) => {
                eprintln!("Runtime error: {}", e);
            }
        }
    }
}
