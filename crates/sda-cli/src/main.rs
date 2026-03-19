use clap::{Args, Parser, Subcommand};
use std::io::{IsTerminal, Read};

#[derive(Parser)]
#[command(name = "sda", about = "Structured Data Algebra evaluator")]
struct Cli {
    #[command(subcommand)]
    command: Option<Command>,

    /// Legacy shorthand: evaluate this SDA expression against stdin or null.
    expression: Option<String>,

    /// Legacy shorthand: input JSON file. Reads stdin if omitted.
    input_file: Option<std::path::PathBuf>,

    /// Legacy shorthand: identifier bound to the host input value.
    #[arg(long, default_value = "input")]
    bind: String,
}

#[derive(Subcommand)]
enum Command {
    /// Evaluate SDA source against JSON input.
    Eval(EvalArgs),
    /// Parse and validate SDA source without evaluating it.
    Check(SourceArgs),
}

#[derive(Args)]
struct SourceArgs {
    /// Inline SDA expression.
    #[arg(short = 'e', long = "expr", conflicts_with = "file")]
    expr: Option<String>,

    /// SDA source file.
    #[arg(short = 'f', long = "file", conflicts_with = "expr")]
    file: Option<std::path::PathBuf>,
}

#[derive(Args)]
struct EvalArgs {
    /// Inline SDA expression.
    #[arg(short = 'e', long = "expr", conflicts_with = "file")]
    expr: Option<String>,

    /// SDA source file.
    #[arg(short = 'f', long = "file", conflicts_with = "expr")]
    file: Option<std::path::PathBuf>,

    /// Input JSON file. Reads stdin if omitted.
    #[arg(short = 'i', long = "input")]
    input: Option<std::path::PathBuf>,

    /// Identifier bound to the host input value.
    #[arg(long, default_value = "input")]
    bind: String,

    /// Emit compact JSON instead of pretty JSON.
    #[arg(long)]
    compact: bool,
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Some(Command::Eval(args)) => eval_command(args),
        Some(Command::Check(args)) => check_command(args),
        None => legacy_eval(cli),
    }
}

fn legacy_eval(cli: Cli) {
    let expr = cli.expression.unwrap_or_else(|| {
        eprintln!("Error: missing expression. Use `sda eval -e ...` or provide the legacy positional expression.");
        std::process::exit(2);
    });

    let input_json = read_input_json(cli.input_file, true);
    let result = sda_core::run_with_input_binding(&expr, &cli.bind, input_json).unwrap_or_else(|error| {
        eprintln!("Error: {error}");
        std::process::exit(1);
    });

    print_json(&result, false);
}

fn eval_command(args: EvalArgs) {
    let source = read_source(args.expr, args.file);
    let input_json = read_input_json(args.input, true);
    let result = sda_core::run_with_input_binding(&source, &args.bind, input_json).unwrap_or_else(|error| {
        eprintln!("Error: {error}");
        std::process::exit(1);
    });

    print_json(&result, args.compact);
}

fn check_command(args: SourceArgs) {
    let source = read_source(args.expr, args.file);
    sda_core::check(&source).unwrap_or_else(|error| {
        eprintln!("Error: {error}");
        std::process::exit(1);
    });
    println!("ok");
}

fn read_source(expr: Option<String>, file: Option<std::path::PathBuf>) -> String {
    if let Some(expr) = expr {
        expr
    } else if let Some(path) = file {
        std::fs::read_to_string(path).unwrap_or_else(|error| {
            eprintln!("Error: failed to read source file: {error}");
            std::process::exit(1);
        })
    } else {
        eprintln!("Error: provide either `-e/--expr` or `-f/--file`.");
        std::process::exit(2);
    }
}

fn read_input_json(path: Option<std::path::PathBuf>, default_null_if_tty: bool) -> serde_json::Value {
    let input_str = if let Some(path) = path {
        std::fs::read_to_string(path).expect("Failed to read file")
    } else if default_null_if_tty && std::io::stdin().is_terminal() {
        "null".to_string()
    } else {
        let mut buffer = String::new();
        std::io::stdin()
            .read_to_string(&mut buffer)
            .expect("Failed to read stdin");
        buffer
    };

    serde_json::from_str(&input_str).unwrap_or(serde_json::Value::Null)
}

fn print_json(value: &serde_json::Value, compact: bool) {
    if compact {
        println!("{}", serde_json::to_string(value).unwrap());
    } else {
        println!("{}", serde_json::to_string_pretty(value).unwrap());
    }
}
