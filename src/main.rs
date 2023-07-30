mod command;
mod engine;
mod environment;
mod evaluate;
mod expression;
mod function;
mod history;
mod identifier;
mod parser;

use clap::Parser;

use engine::Engine;
use environment::Env;
use history::{open_or_create_history_file, History};
use parser::command::parse_command;

/// An interpreter that evaluates λ-calculations step by step.
#[derive(Parser, Debug)]
#[command(version)]
struct Args {
    /// コマンド
    command: String,
}

fn main() {
    let args = Args::parse();
    let file = open_or_create_history_file();
    let mut history = History::new(file.try_clone().unwrap(), file);
    let env = history.build_env(Env::default());
    let command = args.command;
    match parse_command(&command) {
        Ok(command) => {
            Engine::new(env).run(&command);
            history.push(command);
        }
        Err(e) => println!("{}", e),
    }
}
