use std::env;
use once_cell::sync::Lazy;
use std::sync::Mutex;
use url::Url;

struct Config {
    recursive: bool,
    path: String,
    depth: u32,
}

static CONFIG: Lazy<Mutex<Config>> = Lazy::new(|| {
  Mutex::new(Config {
    recursive: false,
    path: String::from("/data"),
    depth: 5,
  })
});

fn display_parsing_error() {
    eprintln!("Error: please use the spaced '| -r | -l [depth] | -p [path] |' options, followed by a valid url");
    std::process::exit(1);
}

fn check_url(url: String) {
	if Url::parse(&url).is_ok() {
		println!("The url format is conform");
	} else {
		eprintln!("Error: please enter a valid url");
		std::process::exit(1);
	}
}

fn parse_options(args: &Vec<String>) {
  let mut config = CONFIG.lock().unwrap();
  let mut i = 1;
  if args.len() > 1 {
    while i < args.len() {
      if args[i].starts_with("-r") && args[i].len() == 2 {
        config.recursive = true;
      } else if args[i].starts_with("-l") && args[i].len() == 2 && args.get(i + 1).is_some() && args[i + 1].parse::<u32>().is_ok() {
        config.depth = args[i + 1].parse::<u32>().unwrap();
        i += 1;
      } else if args[i].starts_with("-p") && args[i].len() == 2 && args.get(i + 1).is_some() && args[i + 1].starts_with("/") {
        config.path = args[i + 1].clone();
        i += 1;
      } else if args[args.len() - 1].starts_with("http") {
				check_url(args[args.len() - 1].clone());
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

  let config = CONFIG.lock().unwrap();

  println!("Recursive is: {}", config.recursive);
  println!("Path is: {}", config.path);
  println!("Depth is: {}", config.depth);

}

