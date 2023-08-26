extern crate glob;
use crate::command::display::ecmascript::ECMAScriptStyle;
use crate::command::display::lazy_k::LazyKStyle;
use crate::command::Command;
use crate::context::Context;
use crate::display_style::DisplayStyle;
use crate::parser::command::ecmascript::parse_command as parse_ecmascript_style_command;
use crate::parser::command::lazy_k::parse_command as parse_lazy_k_style_command;
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

pub fn rebuild_context(file: &File, context: Option<Context>) -> Context {
    let mut context = context.unwrap_or(Context::default());

    for line in std::io::BufReader::new(file).lines() {
        let line = line.unwrap();
        if line.trim().is_empty() {
            continue;
        }
        let command =
            parse_lazy_k_style_command(&line).or_else(|_err| parse_ecmascript_style_command(&line));
        match command {
            Ok(Command::Update(f)) => context.def(f.clone()),
            Ok(Command::Del(i)) => context.del(&i),
            _ => (),
        }
    }

    context
}

pub struct Logger<W: Write>(W, DisplayStyle);

impl<W: Write> Logger<W> {
    pub fn new(writer: W) -> Self {
        Logger(writer, DisplayStyle::get())
    }

    pub fn push(&mut self, command: &Command) {
        match &self.1 {
            DisplayStyle::LazyK => println!("{}", LazyKStyle(command)),
            DisplayStyle::Ecmascript => println!("{}", ECMAScriptStyle(command)),
        }
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

        logger.push(&Command::Update(Func::new(
            "i".into(),
            vec!["x".into()],
            "x".into(),
        )));
        assert_eq!(String::from_utf8(logger.0.clone()).unwrap(), "`ix = x\n");

        logger.push(&Command::Info("i".into()));
        assert_eq!(
            String::from_utf8(logger.0.clone()).unwrap(),
            "`ix = x\n? i\n"
        );
    }
}
