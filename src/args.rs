use clap::Parser;

#[derive(Parser)]
pub struct Args {
    ///Wallpaper tags
    #[arg(short, long)]
    pub search_query: String,
    ///Shows wallpapers based on a criteria or sorting.
    ///
    ///{date_added(default), relevance, random, views, favorites, toplist}
    pub sorting: Option<String>,
}

pub fn parse_commands() -> Args {
    Args::parse()
}
