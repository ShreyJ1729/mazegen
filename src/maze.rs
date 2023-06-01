use rand::{rngs::StdRng, Rng, SeedableRng};

pub struct Maze {
    width: usize,
    height: usize,
    hwalls: Vec<Vec<bool>>,
    vwalls: Vec<Vec<bool>>,
    cells: Vec<isize>,
    path_compression: bool,
    rng: StdRng,
}

// (dcol, drow) - up, down, left, right
const DIRECTIONS: [(isize, isize); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];

impl Maze {
    pub fn new(
        width: usize,
        height: usize,
        path_compression: bool,
        show_progress_bar: bool,
    ) -> Self {
        assert!(
            width > 1 && height > 1,
            "Width and height must be at least 2"
        );

        // all of the HxW cells are initially in their own disjoint sets
        let cells = vec![-1; width * height];

        // initialize walls ignoring borders
        // - h-1 rows of w horizontal walls
        // - h rows of w-1 vertical walls
        let hwalls = vec![vec![true; width]; height - 1];
        let mut vwalls = vec![vec![true; width - 1]; height];

        // remove walls at start/end
        vwalls[0][0] = false;
        vwalls[height - 1][width - 2] = false;

        // get curr time in nanos to seed rng
        let currtime = std::time::UNIX_EPOCH.elapsed().unwrap().as_nanos() as u64;

        // init and build maze
        let mut maze = Self {
            width,
            height,
            cells,
            hwalls,
            vwalls,
            path_compression,
            rng: StdRng::seed_from_u64(currtime),
        };
        maze.build(show_progress_bar);
        maze
    }

    /// Builds the maze
    /// ### Arguments
    pub fn build(&mut self, show_progress_bar: bool) {
        while !self.is_maze_connected() {
            // 0. Some logging
            if show_progress_bar {
                print!(
                    "Resolving disjoint sets for {}x{} maze...",
                    self.width, self.height
                );
                print!("{} remaining\r", self.num_disjoint_sets());
            }

            // 1. pick a random cell
            let (row, col) = self.get_random_cell();
            let cell_index = (row * self.width + col) as usize;

            // 2. pick a random direction
            let (dcol, drow) = DIRECTIONS[self.rng.gen_range(0..4)];
            let (new_row, new_col) = (row as isize + drow, col as isize + dcol);

            // 2.1 if direction OOB, retry
            if !(0..self.height as isize).contains(&(new_row))
                || !(0..self.width as isize).contains(&(new_col))
            {
                continue;
            }

            // 3. calculate the adjacent cell
            let adj_cell_index = (new_row * self.width as isize + new_col) as usize;

            // 4. if the roots of the cells are the same, they're already connected, retry
            let cell1_root = self.find(cell_index);
            let cell2_root = self.find(adj_cell_index);
            if cell1_root == cell2_root {
                continue;
            }

            // 5. otherwise connect them

            // 5.1 union the disjoint sets to update the roots
            self.union(cell1_root, cell2_root);

            // 5.2 knock down the walls
            match (drow, dcol) {
                (-1, 0) => {
                    self.hwalls[row - 1][col] = false;
                }
                (1, 0) => {
                    self.hwalls[row][col] = false;
                }
                (0, -1) => {
                    self.vwalls[row][col - 1] = false;
                }
                (0, 1) => {
                    self.vwalls[row][col] = false;
                }
                _ => unreachable!("Invalid direction"),
            }
        }
        if show_progress_bar {
            // clear progress bar
            println!("{}\r", " ".repeat(100));
        }
    }

    /// Returns the (row, col) for a random cell in the maze
    fn get_random_cell(&mut self) -> (usize, usize) {
        let col = self.rng.gen_range(0..self.width);
        let row = self.rng.gen_range(0..self.height);
        (row, col)
    }

    /// Returns whether the maze is connected (i.e. there is only one disjoint set)
    fn is_maze_connected(&self) -> bool {
        self.num_disjoint_sets() == 1
    }

    /// Returns the number of disjoint sets of cells in the maze
    fn num_disjoint_sets(&self) -> usize {
        self.cells.iter().filter(|&cell| *cell < 0).count()
    }

    /// Prints the maze using ascii characters
    /// ### Arguments
    /// * `hwalls` - the horizontal walls (ignoring the top and bottom borders)
    /// * `vwalls` - the vertical walls (ignoring the left and right borders)
    pub fn print(&self) {
        let vwall_rows = self.vwalls.len();
        let vwall_cols = self.vwalls[0].len();
        let hwall_cols = self.hwalls[0].len();

        // top border
        println!("{}+", "+---".repeat(hwall_cols));

        // print the maze
        for row in 0..vwall_rows {
            // left border
            print!("{}", if row == 0 { " " } else { "|" });

            // vertical walls
            for col in 0..vwall_cols {
                print!("   ");
                print!("{}", if self.vwalls[row][col] { "|" } else { " " });
            }
            // right border
            println!("   {}", if row == vwall_rows - 1 { " " } else { "|" });

            // skip bottom border
            if row == vwall_rows - 1 {
                continue;
            }

            // horizontal walls
            print!("+");
            for col in 0..hwall_cols {
                print!("{}+", if self.hwalls[row][col] { "---" } else { "   " });
            }
            println!();
        }

        // bottom border
        print!("{}", "+---".repeat(hwall_cols));
        println!("+");
    }

    /// Unions two disjoint sets using union-by-size
    /// ### Arguments
    /// * `cells` (supplied at construction) - the disjoint set
    /// * `cell1` - the first cell to union
    /// * `cell2` - the second cell to union
    fn union(&mut self, cell1: usize, cell2: usize) {
        if self.cells[cell1] < self.cells[cell2] {
            self.cells[cell1] += self.cells[cell2];
            self.cells[cell2] = cell1 as isize;
        } else {
            self.cells[cell2] += self.cells[cell1];
            self.cells[cell1] = cell2 as isize;
        }
    }

    /// Finds the root of the disjoint set as an index (usize), optionally compressing the path
    /// ### Arguments
    /// * `cells` - the disjoint set
    /// * `target` - the cell to find the root of
    /// * `path_compression` (supplied at construction) - whether to compress the path
    fn find(&mut self, mut target: usize) -> usize {
        // find the root of the disjoint set
        let mut root = target;
        while self.cells[root] >= 0 {
            root = self.cells[root] as usize;
        }

        // path compression
        if self.path_compression {
            while self.cells[target] >= 0 {
                let parent = self.cells[target] as usize;
                self.cells[target] = root as isize;
                target = parent;
            }
        }

        root
    }
}
