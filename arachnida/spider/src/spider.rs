use std::env;

fn parse_arguments(option: &str) {
  if option.starts_with("-r") {
      option_r();
    } else if option.starts_with("-r -l") {
    if let Ok(n) = option[3].parse::<u32>() {
      option_rl();
    }
  } else if option.starts_with("-p") {
    option_p();
  } else {
    eprintln!("Wrong params");
    std::process::exit(1);
  }
}

fn option_r() {
  println!("It's -r");
}

fn option_rl() {
  println!("It's -r -l");
}

fn option_p() {
  println!("It's -p");
}

fn main() {
  let args: Vec<String> = env::args().collect();
  if args.len() != 3 {
    eprintln!("Wrong number of params");
    std::process::exit(1);
  }
  let option: &str = &args[1];
  parse_arguments(&option);
}
