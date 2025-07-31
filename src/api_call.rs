use reqwest::blocking::Client;
use serde::Deserialize;
use serde::Serialize;

#[derive(Serialize, Deserialize)]
pub struct WallhavenResponse {
    data: Vec<Wallpaper>,
}

#[derive(Serialize, Deserialize)]
struct Wallpaper {
    path: String,
}

pub fn request_wallpapers(url: &String) -> WallhavenResponse {
    let client = Client::new();

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
