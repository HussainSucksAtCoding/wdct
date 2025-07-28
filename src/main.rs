mod args;
use args::*;

mod api_call;
use api_call::*;

fn main() {
    let url = String::from("https://wallhaven.cc/api/v1/search?");
    let args = parse_commands();

    let url = match args.sorting {
        Some(sorting) => format!("{url}q={}&sorting={}", args.search_query, sorting),
        None => format!("{url}q={}", args.search_query),
    };

    //println!("{}", &args.search_query);
    request_wallpapers(&url);
}
