use super::config::CONFIG;
use scraper::{Html, Selector};
use std::fs::File;
use url::Url;

fn extract_url(args: &[String]) -> Url {
    Url::parse(&args[args.len() - 1]).expect("Error: invalid url")
}

fn download_html(args: &[String], client: &reqwest::blocking::Client) -> Html {
    let reponse = client
        .get(args[args.len() - 1].clone())
        .header("User-Agent", "Mozilla 5.0")
        .send()
        .expect("Error: request failed");
    let body = reponse.text().expect("Error: request failed");
    Html::parse_document(&body)
}
fn parse_urls(document: &Html, url_selector: &Selector) {
    for element in document.select(&url_selector) {
        if let Some(href) = element.value().attr("href") {
            println!("Parsed url: {} ", href);
        }
    } 
}

pub fn scraper(args: &Vec<String>) {
    let valid_exts = ["jpg", "jpeg", "png", "gif", "bmp"];
    let config = CONFIG.lock().unwrap();
    let client = reqwest::blocking::Client::new();
    let base_url = extract_url(&args);
    let document = download_html(&args, &client);
    let img_selector = Selector::parse("img").unwrap();
    let url_selector = Selector::parse("a[href]").unwrap();

    parse_urls(&document, &url_selector);

    for element in document.select(&img_selector) {
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
