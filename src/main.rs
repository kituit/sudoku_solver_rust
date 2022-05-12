use std::io::{self};
const SIZE:usize = 9;

#[derive(Copy, Clone)]
enum Cell {
    Valid(usize),
    Invalid,
}

struct Sudoku {
    grid: [[Cell; SIZE]; SIZE],
}

impl Sudoku {
    fn create() -> Sudoku {
        let mut sudoku = Sudoku {
            grid: [[Cell::Invalid; SIZE] ; SIZE],
        };

        loop {

            let mut a_str = String::new();
            io::stdin().read_line(&mut a_str).expect("read error");
            
            let vec = a_str.split_whitespace()
                .map(|x| x.parse::<usize>().expect("parse error"))
                .collect::<Vec<usize>>();

            let row = vec.get(0);
            let col = vec.get(1);
            let num = vec.get(2);
            if row.is_some() && col.is_some() && num.is_some() {
                let row = row.unwrap().clone();
                let col = col.unwrap().clone();
                let num = num.unwrap().clone();
                sudoku.grid[row][col] = Cell::Valid(num);
            } else {
                return sudoku
            }
            
        }
    }

    fn solve(&mut self) -> bool {
        let mut row = SIZE;
        let mut col = SIZE;

        // Find empty cell
        for test_row in 0..SIZE {
            for test_col in 0..SIZE {
                match self.grid[test_row][test_col] {
                    Cell::Valid(_) => continue,
                    Cell::Invalid => {row = test_row; col = test_col},
                }

            }
        }
        
        // Sudoku is solved, no empty cell
        if row == SIZE || col == SIZE {
            return true
        }

        for num in 1..(SIZE + 1) {
            if self.valid_cell(row, col, num) {
                self.grid[row][col] = Cell::Valid(num);
                let success = self.solve();
                if !success {
                    self.grid[row][col] = Cell::Invalid;
                } else {
                    return success;
                }
            }
        }


        return false
    }

    fn print(&self) {
        for i in 0..(SIZE) {
            for j in 0..(SIZE) {
                match self.grid[i][j] {
                    Cell::Valid(num) => print!("{} ", num),
                    Cell::Invalid => print!("0 "),
                }
            }
            println!("");
        }
    }

    fn valid_cell(&self, row: usize, col: usize, num: usize) -> bool{
        // Test row
        for test_col in 0..SIZE {
            if let Cell::Valid(val) = self.grid[row][test_col] {
                if val == num {
                    return false
                }
            }
        }

        // Test col
        for test_row in 0..SIZE {
            if let Cell::Valid(val) = self.grid[test_row][col] {
                if val == num {
                    return false
                }
            }
        }

        // Test square
        let square_row = (row / 3) * 3;
        let square_col = (col / 3) * 3;
        for test_row in square_row..(square_row + 3) {
            for test_col in square_col..(square_col + 3) {
                if let Cell::Valid(val) = self.grid[test_row][test_col] {
                    if val == num {
                        return false
                    }
                }
            }
        }

        return true
    }
}


fn main() {
    println!("Enter starting grid");
    let mut sudoku = Sudoku::create();
    sudoku.print();

    sudoku.solve();
    
    println!("");

    sudoku.print()

}

