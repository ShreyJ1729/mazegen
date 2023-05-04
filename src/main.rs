use rand::Rng;

// todo
// 1. add command line arguments for width and height
// 2. add command line argument for path compression
// 3. add command line argument for showing solution
// 4. code cleanup
// 5. add doc comments
// 6. finish readme

fn main() {
    let args: Vec<String> = std::env::args().collect();

    const DEFAULT_WIDTH: usize = 100;
    const DEFAULT_HEIGHT: usize = 100;

    const DIRECTIONS: [(isize, isize); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];

    // width/height in cells
    let width: usize = args[1].parse().unwrap_or(DEFAULT_WIDTH);
    let height: usize = args[2].parse().unwrap_or(DEFAULT_HEIGHT);

    // cells, horizontal walls, vertical walls
    let mut cells = vec![0isize; width * height];
    let mut hwalls = vec![vec![true; width]; height - 1];
    let mut vwalls = vec![vec![true; width - 1]; height];

    // start is topleft, end is bottomright
    vwalls[0][0] = false;
    vwalls[height - 1][width - 2] = false;

    let mut rng = rand::thread_rng();

    // fill cells with -1 (all cells disjoint)
    cells.fill(-1);

    // while there are disjoint cells, pick a random cell and knock down a random wall
    while cells.iter().filter(|&cell| *cell < 0).count() > 1 {
        // pick a random cell
        let cell1 = rng.gen_range(0..width * height);
        let (col, row) = (cell1 % width, cell1 / width);

        // pick a random direction
        let direction = rng.gen_range(0..4);
        let (dcol, drow) = DIRECTIONS[direction];

        // if out of bounds, pick another cell
        if col as isize + dcol < 0
            || col as isize + dcol >= width as isize
            || row as isize + drow < 0
            || row as isize + drow >= height as isize
        {
            continue;
        }

        // calculate the adjacent cell
        let cell2 = ((row as isize + drow) * width as isize + (col as isize + dcol)) as usize;

        // find roots of the disjoint sets
        let cell1_root = find(&mut cells, cell1);
        let cell2_root = find(&mut cells, cell2);

        // if the roots are the same, the cells are already connected
        if cell1_root == cell2_root {
            continue;
        }

        // otherwise connect
        // 1. union the disjoint sets
        union(&mut cells, cell1_root, cell2_root);

        // 2. knock down the walls
        if dcol == -1 {
            vwalls[row][col - 1] = false;
        } else if dcol == 1 {
            vwalls[row][col] = false;
        } else if drow == -1 {
            hwalls[row - 1][col] = false;
        } else if drow == 1 {
            hwalls[row][col] = false;
        } else {
            panic!("invalid direction");
        }
    }

    print_maze(&hwalls, &vwalls);

    println!("{} {}", width, height);
    println!("{:?}", cells);
}

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

fn union(cells: &mut [isize], cell1: usize, cell2: usize) {
    if cells[cell1] < cells[cell2] {
        cells[cell1] += cells[cell2];
        cells[cell2] = cell1 as isize;
    } else {
        cells[cell2] += cells[cell1];
        cells[cell1] = cell2 as isize;
    }
}

fn find(cells: &mut [isize], mut cell: usize) -> usize {
    // find the root of the disjoint set
    let mut root = cell;
    while cells[root] >= 0 {
        root = cells[root] as usize;
    }

    // path compression
    while cells[cell] >= 0 {
        let parent = cells[cell] as usize;
        cells[cell] = root as isize;
        cell = parent;
    }

    root
}
