use args::Args;
use maze::Maze;

mod args;
mod maze;

fn main() {
    let args = Args::parse_args();
    let width = args.columns;
    let height = args.rows;
    let path_compression = args.path_compression;
    let show_progress_bar = args.show_progress_bar;

    Maze::new(width, height, path_compression, show_progress_bar).print();
}
