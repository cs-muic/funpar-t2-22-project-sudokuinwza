use std::collections::HashSet;
use std::{cmp};

#[allow(dead_code)]
pub fn par_solve(board: &str) -> HashSet<Vec<Vec<u32>>> {
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

    let mut solutions = HashSet::new();
    par_solve_all_sol_n(&mut grid, &mut solutions);
    solutions
}


// Check if the provided number can be placed in the given row and column of the grid
fn is_par_safe(grid: &Vec<Vec<u32>>, num: u32, row: usize, col: usize) -> bool {
    // Calculate the starting row and column of the 3x3 block that contains the current cell
    let (row_start, col_start) = (row / &3 * 3, col / &3 * 3);
    let size = grid.len();
    // Iterate through all cells in the current row, column, and 3x3 block
    for i in 0..size {
        // Check if the number already exists in the current row, column, or 3x3 block
        if grid[row][i] == num || grid[i][col] == num || grid[row_start + i % 3][col_start + i / 3] == num {
            return false;
        }
    }
    // If the number doesn't exist in the row, column, and 3x3 block, it is safe to place it in the cell
    true
}

// Add this function to find the next empty cell with the fewest legal values
fn par_most_constraining_value(grid: &Vec<Vec<u32>>) -> Option<(usize, usize)> {
    let grid_size = grid.len();
    let mut min_options = 10;
    let mut mcv = None;

    (0..grid_size).into_iter().for_each(|row| {
        (0..grid_size).into_iter().for_each(|col| {
            if grid[row][col] == 0 {
                let mut options = 0;

                (1..=9).into_iter().for_each(|num| {
                    if is_par_safe(grid, num, row, col) {
                        options += 1;
                    }
                });

                if options < min_options {
                    min_options = options;
                    mcv = Some((row, col));
                }
            }
        });
    });

    mcv
}

fn par_least_constraining_value(grid: &Vec<Vec<u32>>, row: usize, col: usize) -> Vec<u32> {
    let mut lcv_values = (1..=9)
        .into_iter()
        .filter(|&num| is_par_safe(grid, num, row, col))
        .map(|num| {
            let constraints = (0..9)
                .into_iter()
                .filter(|&i| i != row && is_par_safe(grid, num, i, col) || i != col && is_par_safe(grid, num, row, i))
                .count();
            (num, constraints)
        })
        .collect::<Vec<(u32, usize)>>();

    // Sort by constraints (ascending order)
    lcv_values.sort_unstable_by_key(|&(_, constraints)| constraints);

    // Extract values without constraints
    let mut values = Vec::with_capacity(lcv_values.len());
    for (value, _) in lcv_values {
        values.push(value);
    }

    values
}

fn par_solve_all_sol_n<'a>(
    grid: &'a mut Vec<Vec<u32>>,
    result_set: &'a mut HashSet<Vec<Vec<u32>>>,
) {
    if let Some((row, col)) = par_most_constraining_value(grid) {
        let lcv_values = par_least_constraining_value(grid, row, col);

        lcv_values.iter().for_each(|&num| {
            if is_par_safe(grid, num, row, col) {
                let mut grid_copy = grid.clone();
                grid_copy[row][col] = num;
                par_solve_all_sol_n(&mut grid_copy, result_set);
            }
        });
    } else {
        result_set.insert(grid.clone());
    }
}

pub fn par_show_solutions(solutions: HashSet<Vec<Vec<u32>>>) {
    for solution in solutions {
        par_print_solution(&solution);
    }
}
fn par_print_solution(grid: &Vec<Vec<u32>>) {
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