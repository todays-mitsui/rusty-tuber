extern crate glob;
use crate::command::Command;
use glob::glob;
use home_dir::*;
use std::io::Write;
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

    println!("{:?}", file_names);

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

struct History {
    history: Vec<Command>,
    writer: std::io::BufWriter<std::fs::File>,
}

impl History {
    fn new(file: std::fs::File) -> History {
        History {
            history: vec![],
            writer: std::io::BufWriter::new(file),
        }
    }

    fn push(&mut self, command: Command) {
        write!(self.writer, "{}\n", command).expect("ファイルの書き込みに失敗しました");
        self.history.push(command);
    }
}
