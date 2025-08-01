use once_cell::sync::Lazy;
use std::sync::Mutex;

pub struct Config {
    pub recursive: bool,
    pub path: String,
    pub depth: u32,
}

pub static CONFIG: Lazy<Mutex<Config>> = Lazy::new(|| {
    Mutex::new(Config {
        recursive: false,
        path: String::from("../data/"),
        depth: 5,
    })
});
