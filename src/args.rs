use clap::{Parser, ValueEnum};

#[derive(Clone, ValueEnum)]
pub enum Sorting {
    DateAdded,
    Relevance,
    Random,
    View,
    Favorites,
    Toplist,
}

#[derive(Parser)]
pub struct UserArgs {
    ///Wallpaper tags
    #[arg(short, long)]
    pub tags: String,
    ///Shows wallpapers based on a criteria or sorting.
    ///
    ///{date_added(default), relevance, random, views, favorites, toplist}
    #[arg(value_enum)]
    pub sorting: Option<Sorting>,
}

pub fn parse_commands() -> UserArgs {
    UserArgs::parse()
}
