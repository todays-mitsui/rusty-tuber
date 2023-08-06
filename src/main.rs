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
use history::{open_or_create_history_file, rebuild_env, Logger};
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
    let env = rebuild_env(&file, None);
    let mut logger = Logger::new(file);
    let command = args.command;
    match parse_command(&command) {
        Ok(command) => {
            logger.push(&command);
            Engine::new(env).run(&command);
        }
        Err(e) => println!("{}", e),
    }
}
