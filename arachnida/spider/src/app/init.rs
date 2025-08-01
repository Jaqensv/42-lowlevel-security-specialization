use super::config::CONFIG;
use std::fs;
use url::Url;

pub fn check_url(url: String) {
    if Url::parse(&url).is_ok() {
        println!("The url format is conform");
    } else {
        eprintln!("Error: please enter a valid url");
        std::process::exit(1);
    }
}

pub fn create_directory() {
    let config = CONFIG.lock().unwrap();
    if let Err(error) = fs::create_dir_all(config.path.clone()) {
        eprintln!("Error: failed to create the directory. Reason: {}", error);
    }
}
