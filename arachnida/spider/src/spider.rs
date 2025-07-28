use std::env;
use once_cell::sync::Lazy;
use std::sync::Mutex;

struct Config {
    recursive: bool,
    path: String,
    depth: u32,
}

static CONFIG: Lazy<Mutex<Config>> = Lazy(|| {
  Mutex::new(Config {
    recursive: false,
    path: String::from("/data"),
    depth: 5,
  })
});

fn init_config(args: &Vec<String>) {
  let recursive_opt: bool;
  if args[1].starts_with("-r") {
    recursive_opt = true;
  } else {
    recursive_opt = false;
  }

  Config {
      recursive: recursive_opt,
      path: "test".to_string(),
      depth: 3,
  };
}

// fn parse_arguments(args: &Vec<String>) {

//   //TMP//
//   let str_len = args.len();
//   println!("str_len = {}", str_len);
//   //
//   if args[1].starts_with("-r") {
//     println!("Option -r");
//   }

// }

fn main() {
  let args: Vec<String> = env::args().collect();
  init_config(&args);

  println!("Recursive is: {}", Config::recursive);
  println!("Path is: {}", Config::path);
  println!("Depth is: {}", Config::depth);
  //let args: &str = env::args().collect();
  // parse_arguments(&args);
  // if args.len() != 3 {
  //   eprintln!("Wrong number of params");
  //   std::process::exit(1);
  // }
  
  // let option: Option<Value> = match args.get(2) {
  //   Some(s) => {
  //       if let Ok(n) = s.parse::<i32>() {
  //           Some(Value::Depth(n))
  //       } else {
  //           Some(Value::Path(s.clone()))
  //       }
  //   }
  //   None => None,
  // };

  // match option {
  //   Some(Value::Depth(n)) => println!("C'est un entier : {}", n),
  //   Some(Value::Path(p)) => println!("C'est une chaîne : {}", p),
  //   None => println!("Pas de valeur"),
  // }

}
