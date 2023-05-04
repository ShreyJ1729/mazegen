use std::{mem::size_of_val, path};

use clap::Parser;
use maze::Maze;

mod maze;
// todo
// add seedable rng and time=seed
// 4. code cleanup
// 6. finish readme
// 7. under some threshold (ratio of total cells to disjoint sets), start only knocking down walls that connect disjoint sets (do this by only picking -1s)
// 8. for above could even j do this for all rnadom picking

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// width
    #[arg(short, long, default_value_t = 10)]
    columns: usize,

    /// height
    #[arg(short, long, default_value_t = 10)]
    rows: usize,

    /// path compression
    #[arg(short, long, default_value_t = false)]
    path_compression: bool,
}

fn main() {
    const DIRECTIONS: [(isize, isize); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];

    // width/height in cells
    let args = Args::parse();
    let width = args.columns;
    let height = args.rows;
    let path_compression = args.path_compression;

    println!("Building {}x{} maze...", width, height);

    Maze::new(width, height, path_compression).print();
}
