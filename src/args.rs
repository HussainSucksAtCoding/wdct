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
    #[arg(value_enum, short, long)]
    pub sorting: Option<Sorting>,
}

pub fn parse_commands() -> UserArgs {
    UserArgs::parse()
}
