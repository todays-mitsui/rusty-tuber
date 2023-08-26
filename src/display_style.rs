use std::env;

pub enum DisplayStyle {
    LazyK,
    Ecmascript,
}

impl DisplayStyle {
    pub fn get() -> Self {
        match env::var("TUBER_DISPLAY_STYLE") {
            Ok(s) => match s.as_str() {
                "Lazy_K" => DisplayStyle::LazyK,
                "ECMAScript" => DisplayStyle::Ecmascript,
                _ => DisplayStyle::LazyK,
            },
            Err(_) => DisplayStyle::LazyK,
        }
    }
}
