use clap::Parser;
use std::io::{IsTerminal, Read};

#[derive(Parser)]
#[command(name = "sda", about = "Structured Data Algebra evaluator")]
struct Cli {
    /// SDA expression to evaluate.
    expression: Option<String>,
    /// Input JSON file. Reads stdin if omitted.
    file: Option<std::path::PathBuf>,
}

fn main() {
    let cli = Cli::parse();

    let input_str = if let Some(path) = cli.file {
        std::fs::read_to_string(path).expect("Failed to read file")
    } else if cli.expression.is_some() && std::io::stdin().is_terminal() {
        "null".to_string()
    } else {
        let mut buffer = String::new();
        std::io::stdin()
            .read_to_string(&mut buffer)
            .expect("Failed to read stdin");
        buffer
    };

    let input_json: serde_json::Value =
        serde_json::from_str(&input_str).unwrap_or(serde_json::Value::Null);

    let result = if let Some(expr) = cli.expression {
        sda_core::run(&expr, input_json).unwrap_or_else(|error| {
            eprintln!("Error: {error}");
            std::process::exit(1);
        })
    } else {
        input_json
    };

    println!("{}", serde_json::to_string_pretty(&result).unwrap());
}
