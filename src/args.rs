use clap::{Parser, ValueEnum};

#[derive(Clone, ValueEnum)]
pub enum SortingOptions {
    DateAdded,
    Relevance,
    Random,
    View,
    Favorites,
    Toplist,
}

#[derive(Clone, ValueEnum)]
pub enum TopRange {
    ThreeMonths,
    OneMonth,
    OneWeek,
}

#[derive(Parser)]
pub struct UserArgs {
    ///Wallpaper tags
    #[arg(short, long)]
    pub tags: String,
    ///Shows wallpapers based on a criteria or sorting.
    #[arg(value_enum, short, long)]
    pub sorting: Option<SortingOptions>,

    ///Toplist range. Sorting must be toplist.
    #[arg(value_enum, short, long)]
    pub toprange: Option<TopRange>
}

pub fn parse_commands() -> UserArgs {
    UserArgs::parse()
}
