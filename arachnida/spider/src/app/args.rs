use super::config::CONFIG;
use super::init::check_url;
use crate::ui::display::display_parsing_error;

pub fn parse_args(args: &[String]) {
    let mut config = CONFIG.lock().unwrap();
    let mut index = 1;
    if args.len() > 2 {
        while index < args.len() {
            if args[index].starts_with("-r") && args[index].len() == 2 {
                config.recursive = true;
            } else if args[index].starts_with("-l")
                && args[index].len() == 2
                && args.get(index + 1).is_some()
                && args[index + 1].parse::<u32>().is_ok()
            {
                config.depth = args[index + 1].parse::<u32>().unwrap();
                index += 1;
            } else if args[index].starts_with("-p") && args[index].len() == 2 {
                if args[index + 1].starts_with("./") {
                    config.path = args[index + 1].clone();
                }
                index += 1;
            } else if args[index].starts_with("http") && index == (args.len() - 1) {
                check_url(args[index].clone());
            } else {
                display_parsing_error();
            }
            index += 1;
        }
    } else {
        display_parsing_error();
    }
}
