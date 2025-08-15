use reqwest::blocking::Client;
use serde::Deserialize;
use serde::Serialize;

use crate::args::SortingOptions::*;
use crate::args::TopRange::*;
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
fn toplist_range() -> Option<String> {
    let args = parse_commands();

    match args.range {
        Some(range) => {
            let range_option = match range {
               ThreeMonths => "3M",
               OneMonth => "1M",
               OneWeek => "1w",
            }.to_string();
            Some(range_option)
        },
        None => None
    }
}
fn request_formatter() -> String {
    let api_url = String::from("https://wallhaven.cc/api/v1/search?");
    let mut final_url = api_url;

    final_url = match sorting_options() {
        Some(sorting) => format!("&{sorting}"),
        None => final_url
    }; 

    final_url = match toplist_range() {
        Some(range) => format!("&{range}"),
        None => final_url
    }; 

    final_url
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
