use std::env;

pub enum DisplayStyle {
    LazyK,
    Ecmascript,
}

pub fn display_style() -> DisplayStyle {
    match env::var("TUBER_DISPLAY_STYLE") {
        Ok(s) => match s.as_str() {
            "Lazy_K" => DisplayStyle::LazyK,
            "ECMAScript" => DisplayStyle::Ecmascript,
            _ => DisplayStyle::LazyK,
        },
        Err(_) => DisplayStyle::LazyK,
    }
}

pub fn step_limit() -> usize {
    match env::var("TUBER_STEP_LIMIT") {
        Ok(s) => match s.parse::<usize>() {
            Ok(n) => n,
            Err(_) => 1000,
        },
        Err(_) => 1000,
    }
}
