extern crate glob;
use crate::command::Command;
use crate::environment::Env;
use crate::parser::command::parse_command;
use glob::glob;
use home_dir::*;
use std::fs::File;
#[allow(unused_imports)]
use std::io::{self, BufRead, Write};
use std::path::Path;
use ulid::Ulid;

pub fn open_or_create_history_file() -> File {
    let dir = Path::new("~/.tuber")
        .expand_home()
        .expect("ホームディレクトリが見つかりませんでした");

    let file_names = glob(&format!("{}/*.txt", dir.to_str().unwrap()))
        .expect("glob pattern error")
        .map(|e| e.unwrap())
        .collect::<Vec<_>>();

    return match file_names.iter().max() {
        Some(f) => std::fs::OpenOptions::new()
            .read(true)
            .write(true)
            .append(true)
            .open(f)
            .expect("ファイルの読み込みに失敗しました"),

        None => {
            if !Path::new(&dir).is_dir() {
                std::fs::create_dir(&dir).expect("ディレクトリの作成に失敗しました");
            }
            let filename = format!("{}.txt", Ulid::new().to_string());
            File::create(dir.join(filename)).expect("ファイルの作成に失敗しました")
        }
    };
}

pub fn rebuild_env(file: &File, env: Option<Env>) -> Env {
    let mut env = env.unwrap_or(Env::new());

    for line in std::io::BufReader::new(file).lines() {
        let line = line.unwrap();
        if line.trim().is_empty() {
            continue;
        }
        let command = parse_command(&line).unwrap();
        match command {
            Command::Update(i, f) => env.def(i.clone(), f.clone()),
            _ => (),
        }
    }

    env
}

pub struct Logger<W: Write>(W);

impl<W: Write> Logger<W> {
    pub fn new(writer: W) -> Self {
        Logger(writer)
    }

    pub fn push(&mut self, command: &Command) {
        writeln!(self.0, "{}", command).expect("ログの書き込みに失敗しました");
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::command::Command;
    use crate::function::Func;

    #[test]
    fn test_logger() {
        let dist: Vec<u8> = Vec::new();
        let mut logger = Logger::new(dist);

        logger.push(&Command::Update(
            "i".into(),
            Func::new(vec!["x".into()], "x".into()),
        ));
        assert_eq!(String::from_utf8(logger.0.clone()).unwrap(), "`ix = x\n");

        logger.push(&Command::Info("i".into()));
        assert_eq!(
            String::from_utf8(logger.0.clone()).unwrap(),
            "`ix = x\n? i\n"
        );
    }
}
