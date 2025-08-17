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

fn tags() -> String {
    let args = parse_commands();

    args.tags
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

    let tags = tags();
    let tags = format!("q={tags}");

    let sorting = match sorting_options() {
        Some(sorting) => format!("&sorting={sorting}"),
        None => "".to_string()
    }; 

    let range = match toplist_range() {
        Some(range) => {
            if sorting != "".to_string() && sorting == "&sorting=toplist" {
                format!("&range={range}")
            } else {
                panic!("ERROR: range can only be applied if sorting is set to \"toplist\"");
            }
        },
        None => "".to_string(),
    }; 

    format!("{api_url}{tags}{sorting}{range}")
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
