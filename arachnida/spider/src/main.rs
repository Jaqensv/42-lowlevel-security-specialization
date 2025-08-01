mod app;
mod ui;

use app::args::parse_args;
use app::init::create_directory;
use app::scraper::scraper;
use ui::display::display_values;

use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    parse_args(&args);
    display_values();
    create_directory();
    scraper(&args);
}
