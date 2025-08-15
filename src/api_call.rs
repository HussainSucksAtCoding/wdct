use std::ops::Add;

use reqwest::blocking::Client;
use serde::Deserialize;
use serde::Serialize;

use crate::args::SortingOptions::*;
use crate::args::*;

#[derive(Serialize, Deserialize)]
pub struct WallhavenResponse {
    data: Vec<Wallpaper>,
}

#[derive(Serialize, Deserialize)]
struct Wallpaper {
    path: String,
}

fn sorting_options() -> Option<String> {
    let args = parse_commands();

    match args.sorting {
        Some(sorting) => {
            let sorting_option = match sorting {
                DateAdded => "date_added",
                Relevance => "relevance",
                Random => "random",
                View => "view",
                Favorites => "favorites",
                Toplist => "toplist",
            }.to_string();
            Some(sorting_option)
        }
        None => None,
    }
}

pub fn request_formatter() -> String {
    let api_url = String::from("https://wallhaven.cc/api/v1/search?");
    let final_url = api_url;

    match sorting_options() {
        Some(sorting) => final_url.add(&sorting),
        None => final_url
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
