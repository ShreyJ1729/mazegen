use std::{mem::size_of_val, path};

use clap::Parser;
use maze::*;
use rand::Rng;

mod maze;
// todo
// add seedable rng and time=seed
// 4. code cleanup
// 6. finish readme
// 7. under some threshold (ratio of total cells to disjoint sets), start only knocking down walls that connect disjoint sets (do this by only picking -1s)

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

    let maze = Maze::new(width, height, path_compression);
    maze.print(true);

    // // cells, horizontal walls, vertical walls
    // let mut cells = vec![-1isize; width * height];
    // let mut hwalls = vec![vec![true; width]; height - 1];
    // let mut vwalls = vec![vec![true; width - 1]; height];

    // // start is topleft, end is bottomright
    // vwalls[0][0] = false;
    // vwalls[height - 1][width - 2] = false;

    // let mut rng = rand::thread_rng();

    // // while there are disjoint cells, pick a random cell and knock down a random wall
    // while cells.iter().filter(|&cell| *cell < 0).count() > 1 {
    //     // pick a random cell
    //     let cell1 = rng.gen_range(0..width * height);
    //     let (col, row) = (cell1 % width, cell1 / width);

    //     // pick a random direction
    //     let direction = rng.gen_range(0..4);
    //     let (dcol, drow) = DIRECTIONS[direction];

    //     // if out of bounds, pick another cell
    //     if col as isize + dcol < 0
    //         || col as isize + dcol >= width as isize
    //         || row as isize + drow < 0
    //         || row as isize + drow >= height as isize
    //     {
    //         continue;
    //     }

    //     // calculate the adjacent cell
    //     let cell2 = ((row as isize + drow) * width as isize + (col as isize + dcol)) as usize;

    //     // find roots of the disjoint sets
    //     let cell1_root = find(&mut cells, cell1, path_compression);
    //     let cell2_root = find(&mut cells, cell2, path_compression);

    //     // if the roots are the same, the cells are already connected
    //     if cell1_root == cell2_root {
    //         continue;
    //     }

    //     // otherwise connect
    //     // 1. union the disjoint sets
    //     union(&mut cells, cell1_root, cell2_root);

    //     // 2. knock down the walls
    //     if dcol == -1 {
    //         vwalls[row][col - 1] = false;
    //     } else if dcol == 1 {
    //         vwalls[row][col] = false;
    //     } else if drow == -1 {
    //         hwalls[row - 1][col] = false;
    //     } else if drow == 1 {
    //         hwalls[row][col] = false;
    //     } else {
    //         panic!("invalid direction");
    //     }
    // }

    // print_maze(&hwalls, &vwalls);
}

/// Prints the maze
/// ### Arguments
/// * `hwalls` - the horizontal walls (ignoring the top and bottom borders)
/// * `vwalls` - the vertical walls (ignoring the left and right borders)
fn print_maze(hwalls: &[Vec<bool>], vwalls: &[Vec<bool>]) {
    // print the top border
    print!("+");
    for _ in 0..vwalls.len() {
        print!("--+");
    }
    println!();

    for row in 0..hwalls.len() {
        // print the left border if not first row
        print!("{}", if row == 0 { " " } else { "|" });

        // print the vertical walls
        for col in 0..vwalls[row].len() {
            print!("  ");
            print!("{}", if vwalls[row][col] { "|" } else { " " });
        }
        println!("  |");

        // print the horizontal walls
        print!("+");
        for col in 0..hwalls[row].len() {
            print!("{}", if hwalls[row][col] { "--" } else { "  " });
            print!("+");
        }

        println!();
    }

    // print the bottom border
    print!("|");
    for col in 0..vwalls[0].len() {
        print!("  ");
        print!(
            "{}",
            if vwalls[vwalls.len() - 1][col] {
                "|"
            } else {
                " "
            }
        );
    }
    println!();
    for _ in 0..hwalls[0].len() {
        print!("+--");
    }
    println!();
}

/// Unions two disjoint sets using union-by-size
/// ### Arguments
/// * `cells` - the disjoint set
/// * `cell1` - the first cell to union
/// * `cell2` - the second cell to union
fn union(cells: &mut [isize], cell1: usize, cell2: usize) {
    if cells[cell1] < cells[cell2] {
        cells[cell1] += cells[cell2];
        cells[cell2] = cell1 as isize;
    } else {
        cells[cell2] += cells[cell1];
        cells[cell1] = cell2 as isize;
    }
}

/// Finds the root of the disjoint set, optionally compressing the path
/// ### Arguments
/// * `cells` - the disjoint set
/// * `target` - the cell to find the root of
/// * `path_compression` - whether to compress the path
///
/// ### Returns
/// * the root of the disjoint set containing `target` as a usize (index of the root in `cells`)
fn find(cells: &mut [isize], mut target: usize, path_compression: bool) -> usize {
    // find the root of the disjoint set
    let mut root = target;
    while cells[root] >= 0 {
        root = cells[root] as usize;
    }

    // path compression
    if path_compression {
        while cells[target] >= 0 {
            let parent = cells[target] as usize;
            cells[target] = root as isize;
            target = parent;
        }
    }

    root
}
