use reqwest::blocking::Client;
use serde::Deserialize;
use serde::Serialize;

use crate::Sorting;
use crate::args::parse_commands;

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

    let final_url = match args.sorting {
        Some(sorting) => {
            let sorting_url = match sorting {
                Sorting::DateAdded => "date_added",
                Sorting::Relevance => "relevance",
                Sorting::Random => "random",
                Sorting::View => "view",
                Sorting::Favorites => "favorites",
                Sorting::Toplist => "toplist",
            };
            format!("{}sorting={}", base_url, sorting_url)
        }
        None => base_url,
    };
    final_url
}

pub fn api_call() -> WallhavenResponse {
    let client = Client::new();
    let url = request_formatter();

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
