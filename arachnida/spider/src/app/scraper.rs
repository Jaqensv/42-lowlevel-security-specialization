use super::config::CONFIG;
use scraper::{Html, Selector};
use std::fs::File;
use url::Url;

pub fn scraper(args: &[String]) {
    let valid_exts = ["jpg", "jpeg", "png", "gif", "bmp"];
    let config = CONFIG.lock().unwrap();
    let client = reqwest::blocking::Client::new();
    let base_url = Url::parse(&args[args.len() - 1]).expect("Error: failed to create the full_url");
    let reponse = client
        .get(args[args.len() - 1].clone())
        .header("User-Agent", "Mozilla 5.0")
        .send()
        .expect("Error: request failed");
    let body = reponse.text().expect("Request error");
    let document = Html::parse_document(&body);
    let selector = Selector::parse("img").unwrap();
    for element in document.select(&selector) {
        if let Some(src) = element
            .value()
            .attr("data-src")
            .or_else(|| element.value().attr("src"))
        {
            if let Ok(full_url) = base_url.join(src) {
                let mut reponse = client
                    .get(full_url.clone())
                    .send()
                    .expect("Error: request failed");
                if let Some(filename) = full_url
                    .path_segments()
                    .and_then(|segments| segments.last())
                {
                    let filename = filename.to_lowercase();
                    if valid_exts.iter().any(|ext| filename.ends_with(ext)) {
                        let path = format!("{}{}", config.path, filename);
                        let mut file = File::create(path).expect("Error: file creation failed");
                        reponse.copy_to(&mut file).expect("Error: copy failed");
                    }
                }
            }
        }
    }
}
