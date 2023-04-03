use std::collections::HashSet;
use std::{cmp, io};

#[allow(dead_code)]
pub fn solve(board: &str) -> HashSet<Vec<Vec<u32>>> {
    let mut lines = vec![];
    let mut cur = board;

    while !cur.is_empty() {
        let (chunk, rest) = cur.split_at(cmp::min(9, cur.len()));
        lines.push(chunk);
        cur = rest;
    }
    let mut grid = vec![];

    for mut line in lines {
        //get line from the file
        let mut row = Vec::new(); //create new vector
        let lst: String = line.to_ascii_lowercase();
        for c in lst.chars() {
            //get each of the char in that line
            if c != '.' {
                let x = c.to_digit(10).unwrap(); //convert char to u32
                row.push(x);
            } else {
                row.push(0);
            }
        }

        grid.push(row); //add the row vector to the grid
    }

    let mut books = HashSet::new();
    solve_all_sol_n(&mut grid, 0, 0, &mut books);
    return books;
}

// Check if the provided number can be placed in the given row and column of the grid
fn is_safe(grid: &Vec<Vec<u32>>, num: u32, row: usize, col: usize) -> bool {
    // Calculate the starting row and column of the 3x3 block that contains the current cell
    let (row_start, col_start) = (row / 3 * 3, col / 3 * 3);
    let size = grid.len();
    // Iterate through all cells in the current row, column, and 3x3 block
    for i in 0..size {
        // Check if the number already exists in the current row, column, or 3x3 block
        if grid[row][i] == num || grid[i][col] == num || grid[row_start + i / 3][col_start + i % 3] == num {
            return false;
        }
    }
    // If the number doesn't exist in the row, column, and 3x3 block, it is safe to place it in the cell
    true
}

// Recursive function to find all possible solutions for the given Sudoku grid
fn solve_all_sol_n<'a>(
    grid: &'a mut Vec<Vec<u32>>,
    row: usize,
    col: usize,
    result_set: &'a mut HashSet<Vec<Vec<u32>>>,
) {
    let grid_size: usize = grid.len();

    // Base case: if the row index reaches the grid size, a solution is found
    if row == grid_size {
        result_set.insert(grid.clone());
        return;
    }

    // Calculate the next row and column indices
    let next_row = if col + 1 == grid_size { row + 1 } else { row };
    let next_col = if col + 1 == grid_size { 0 } else { col + 1 };

    // If the current cell already contains a number, move to the next cell
    if grid[row][col] != 0 {
        solve_all_sol_n(grid, next_row, next_col, result_set);
    } else {
        // Iterate through all possible numbers from 1 to 9
        for num in 1..=9 {
            // Check if the current number can be placed in the current cell
            if is_safe(grid, num, row, col) {
                grid[row][col] = num;
                // Recursively call the function for the next cell
                solve_all_sol_n(grid, next_row, next_col, result_set);
                // Reset the current cell to 0 (unfilled) to search for other possible solutions
                grid[row][col] = 0;
            }
        }
    }
}

pub fn show_solutions(solutions: HashSet<Vec<Vec<u32>>>) {
    for solution in solutions {
        print_solution(&solution);
    }
}

fn print_solution(grid: &Vec<Vec<u32>>) {
    let size = grid.len();
    for i in 0..size {
        if i % 3 == 0 && i != 0 {
            println!("- - - + - - - + - - -");
        }
        for j in 0..size {
            if j % 3 == 0 && j != 0 {
                print!("| ");
            }
            print!("{}", grid[i][j]);
            if j != size-1 {
                print!(" ");
            }
        }
        println!();
    }
    println!();
}