extern crate glob;
use glob::glob;
use home_dir::*;
use std::path::Path;

pub fn load_or_create_history_file() -> std::fs::File {
    let dir = Path::new("~/.tuber")
        .expand_home()
        .expect("ホームディレクトリが見つかりませんでした");

    let file_names = glob(&format!("{}/*", dir.to_str().unwrap()))
        .expect("glob pattern error")
        .map(|e| e.unwrap())
        .collect::<Vec<_>>();

    return match file_names.iter().max() {
        Some(f) => std::fs::File::open(f).expect("ファイルの読み込みに失敗しました"),

        None => {
            std::fs::create_dir(dir.clone()).expect("ディレクトリの作成に失敗しました");
            std::fs::File::create(dir.join("0")).expect("ファイルの作成に失敗しました")
        }
    };
}
