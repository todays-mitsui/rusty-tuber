mod command;
mod environment;
mod evaluate;
mod expression;
mod function;
mod history;
mod identifier;
mod parser;

use clap::Parser;

use history::open_or_create_history_file;

/// An interpreter that evaluates λ-calculations step by step.
#[derive(Parser, Debug)]
#[command(version)]
struct Args {
    /// コマンド
    command: String,
}

fn main() {
    let args = Args::parse();
    println!("{}", args.command);

    let file = open_or_create_history_file();
}
