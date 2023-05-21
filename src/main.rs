use std::io::{self};

const SIZE:usize = 9;

type Sudoku = [[Option<u8>; SIZE]; SIZE];

fn create() -> Sudoku {
    let mut sudoku: Sudoku = [[None; SIZE] ; SIZE];

    let mut line = String::new();

    loop {
        match io::stdin().read_line(&mut line) {
            Ok(0) => {
                break
            }
            Ok(_) => {
                let vec = line.split_whitespace()
                    .map(|x| x.parse::<usize>().expect("parse error"))
                    .collect::<Vec<usize>>();

                assert!(vec.len() == 3);

                let (row, col, num) = (vec[0], vec[1], vec[2] as u8);
                sudoku[row][col] = Some(num);

            }
            Err(error) => {
                eprintln!("Error: {error}");
            }   
        }
        line.clear();

    }

    return sudoku

}

fn solve(sudoku: &mut Sudoku) -> bool {
    let mut row = SIZE;
    let mut col = SIZE;

    // Find empty cell
    for test_row in 0..SIZE {
        for test_col in 0..SIZE {
            match sudoku[test_row][test_col] {
                Some(_) => continue,
                None => {row = test_row; col = test_col},
            }

        }
    }
    
    // Sudoku is solved, no empty cell
    if row == SIZE || col == SIZE {
        return true
    }

    for num in 1..(SIZE as u8 + 1) {
        // let num = num as u8;
        if valid_cell(sudoku, row, col, num) {
            sudoku[row][col] = Some(num);
            let success = solve(sudoku);
            if !success {
                sudoku[row][col] = None;
            } else {
                return success;
            }
        }
    }


    return false
}

fn print_sudoku(sudoku: &Sudoku) {
    for row in sudoku {
        for cell in row {
            match cell {
                Some(num) => print!("{} ", num),
                None => print!("0 "),
            }
        }
        println!("");
    }
}

fn valid_cell(sudoku: &Sudoku, row: usize, col: usize, num: u8) -> bool {
    // Test row
    for test_col in 0..SIZE {
        if let Some(val) = sudoku[row][test_col] {
            if val == num {
                return false
            }
        }
    }

    // Test col
    for test_row in 0..SIZE {
        if let Some(val) = sudoku[test_row][col] {
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
            if let Some(val) = sudoku[test_row][test_col] {
                if val == num {
                    return false
                }
            }
        }
    }

    return true
}


fn main() {
    println!("Enter starting sudoku:");
    let mut sudoku = create();
    print_sudoku(&sudoku);
    solve(&mut sudoku);
    println!("");
    print_sudoku(&sudoku);

}

