use std::env;
use once_cell::sync::Lazy; // initialise la config a la premiere utilisation
use std::sync::Mutex;
use url::Url;
use std::fs; // creation de dossier
use std::fs::File;
///
mod config;
use config::Config;
mod display;
use display::display_parsing_error;
use display::display_values;
use scraper::{Html, Selector};

static CONFIG: Lazy<Mutex<Config>> = Lazy::new(|| {
	Mutex::new(Config {
		recursive: false,
		path: String::from("./data/"),
		depth: 5,
	})
});

fn check_url(url: String) {
	if Url::parse(&url).is_ok() {
		println!("The url format is conform");
	} else {
		eprintln!("Error: please enter a valid url");
		std::process::exit(1);
	}
}

fn create_directory() {
	let config = CONFIG.lock().unwrap();
	if let Err(error) = fs::create_dir_all(config.path.clone()) {
		eprintln!("Error: failed to create the directory. Reason: {}", error);
	}
}

fn parse_options(args: &Vec<String>) {
	let mut config = CONFIG.lock().unwrap();
	let mut i = 1;
	if args.len() > 2 {
		while i < args.len() {
			if args[i].starts_with("-r") && args[i].len() == 2 {
				config.recursive = true;
			} else if args[i].starts_with("-l") && args[i].len() == 2 && args.get(i + 1).is_some() && args[i + 1].parse::<u32>().is_ok() {
				config.depth = args[i + 1].parse::<u32>().unwrap();
				i += 1;
			} else if args[i].starts_with("-p") && args[i].len() == 2 {
				if args[i + 1].starts_with("./") {
					config.path = args[i + 1].clone();
				}
				i += 1;
			} else if args[i].starts_with("http") && i == (args.len() - 1) {
				check_url(args[i].clone());
			} else {
				display_parsing_error();
			}
			i += 1;
		}
	} else {
		display_parsing_error();
	}
}

fn main() {
	let args: Vec<String> = env::args().collect();
	parse_options(&args);
	display_values();
	create_directory();
	let client = reqwest::blocking::Client::new();
	let base_url = Url::parse(&args[args.len() - 1]).expect("Error: failed to create the full_url");
	let reponse = client.get(args[args.len() - 1].clone()).header("User-Agent", "Mozilla 5.0").send().expect("Error: request failed");
	let body = reponse.text().expect("Request error");
	let document = Html::parse_document(&body);
	let selector = Selector::parse("img").unwrap();
	for element in document.select(&selector) {
		if let Some(src) = element.value().attr("data-src").or_else(|| element.value().attr("src")) {
			if let Ok(full_url) = base_url.join(src) {
				let mut reponse = client.get(full_url.clone()).send().expect("Error: request failed");
				if let Some(filename) = full_url.path_segments().and_then(|segments| segments.last()) {
					let path = format!("./data/{}", filename);
					let mut file = File::create(path).expect("Error: file creation failed");
					reponse.copy_to(&mut file).expect("Error: copy failed");
				}
			}
		}
	}
}

