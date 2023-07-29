mod command;
mod environment;
mod evaluate;
mod expression;
mod function;
mod identifier;
mod parser;

use clap::Parser;
use home_dir::*;
use std::path::Path;

extern crate glob;
use glob::glob;

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

    let dir = Path::new("~/.tuber")
        .expand_home()
        .expect("ホームディレクトリが見つかりませんでした");
    let file_names = glob(&format!("{}/*", dir.to_str().unwrap()))
        .expect("glob pattern error")
        .map(|e| e.unwrap())
        .collect::<Vec<_>>();
    let file = match file_names.iter().max() {
        Some(f) => {
            // ファイルを読み込み
            std::fs::File::open(f).expect("ファイルの読み込みに失敗しました")
        }
        None => {
            // ディレクトリを新規作成
            std::fs::create_dir(dir.clone()).expect("ディレクトリの作成に失敗しました");
            // ファイルを新規作成
            std::fs::File::create(dir.join("0")).expect("ファイルの作成に失敗しました")
        }
    };
}
