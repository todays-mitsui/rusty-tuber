extern crate glob;
use crate::command::Command;
use crate::parser::command::parse_command;
use glob::glob;
use home_dir::*;
use std::io::{self, BufRead, Read, Write};
use std::path::Path;
use ulid::Ulid;

pub fn open_or_create_history_file() -> std::fs::File {
    let dir = Path::new("~/.tuber")
        .expand_home()
        .expect("ホームディレクトリが見つかりませんでした");

    let file_names = glob(&format!("{}/*.txt", dir.to_str().unwrap()))
        .expect("glob pattern error")
        .map(|e| e.unwrap())
        .collect::<Vec<_>>();

    return match file_names.iter().max() {
        Some(f) => std::fs::File::open(f).expect("ファイルの読み込みに失敗しました"),

        None => {
            if !Path::new(&dir).is_dir() {
                std::fs::create_dir(&dir).expect("ディレクトリの作成に失敗しました");
            }
            let filename = format!("{}.txt", Ulid::new().to_string());
            std::fs::File::create(dir.join(filename)).expect("ファイルの作成に失敗しました")
        }
    };
}

pub struct History<W: Write> {
    history: Vec<Command>,
    writer: W,
}

impl<W: Write> History<W> {
    pub fn new(reader: impl Read, writer: W) -> Self {
        let mut histories = Vec::new();
        for line in std::io::BufReader::new(reader).lines() {
            let command = parse_command(&line.unwrap()).unwrap();
            histories.push(command);
        }

        History {
            history: histories,
            writer,
        }
    }

    pub fn push(&mut self, command: Command) {
        writeln!(self.writer, "{}", command).expect("ファイルの書き込みに失敗しました");
        self.history.push(command);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::command::Command;
    use crate::function::Func;

    #[test]
    fn test_history() {
        let input = io::empty();
        let output: Vec<u8> = Vec::new();
        let mut history = History::new(input, output);

        history.push(Command::Update(
            "i".into(),
            Func::new(vec!["x".into()], "x".into()),
        ));
        assert_eq!(String::from_utf8(history.writer.clone()).unwrap(), "`ix = x\n");

        history.push(Command::Info("i".into()));
        assert_eq!(
            String::from_utf8(history.writer.clone()).unwrap(),
            "`ix = x\n? i\n"
        );
    }
}
