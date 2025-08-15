use reqwest::blocking::Client;
use serde::Deserialize;
use serde::Serialize;

use crate::args::*;

#[derive(Serialize, Deserialize)]
pub struct WallhavenResponse {
    data: Vec<Wallpaper>,
}

#[derive(Serialize, Deserialize)]
struct Wallpaper {
    path: String,
}

pub fn request_formatter() -> String {
    let args = parse_commands();
    let api_url = String::from("https://wallhaven.cc/api/v1/search?");

    let base_url = format!("{}q={}&", api_url, args.tags);

    match args.sorting {
        Some(sorting) => {
            let sorting_url = match sorting {
                SortingOptions::DateAdded => "date_added",
                SortingOptions::Relevance => "relevance",
                SortingOptions::Random => "random",
                SortingOptions::View => "view",
                SortingOptions::Favorites => "favorites",
                SortingOptions::Toplist => "toplist",
            };
            format!("{base_url}sorting={sorting_url}")
        }
        None => base_url,
    }
}

pub fn api_call() -> WallhavenResponse {
    let client = Client::new();
    let url = request_formatter();

    println!("url: {url}");
    let request = client
        .get(url)
        .header("User-Agent", "my-app")
        .send()
        .unwrap()
        .text()
        .unwrap();
    let urls: WallhavenResponse = serde_json::from_str(&request).unwrap();

    for n in &urls.data {
        println!("{}", n.path);
    }
    urls
}
