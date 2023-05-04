use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// width in cells
    #[arg(short, long, default_value_t = 10)]
    pub columns: usize,

    /// height in cells
    #[arg(short, long, default_value_t = 10)]
    pub rows: usize,

    /// implement path compression (only useful for large mazes)
    #[arg(short, long, default_value_t = false)]
    pub path_compression: bool,

    /// show progress bar
    #[arg(short, long, default_value_t = false)]
    pub show_progress_bar: bool,
}

impl Args {
    pub fn parse_args() -> Self {
        Self::parse()
    }
}
