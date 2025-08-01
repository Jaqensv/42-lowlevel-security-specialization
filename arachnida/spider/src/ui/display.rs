use crate::app::config::CONFIG;

pub fn display_parsing_error() {
    eprintln!("Error: please use the spaced '| -r | -l [depth] | -p [path] |' options, followed by a valid url");
    std::process::exit(1);
}

pub fn display_values() {
    let config = CONFIG.lock().unwrap();
    println!("Recursive is: {}", config.recursive);
    println!("Path is: {}", config.path);
    println!("Depth is: {}", config.depth);
}
