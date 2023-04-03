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

    let mut domains = initialize_domains(&grid);
    let mut books = HashSet::new();
    solve_all_sol_n(&mut grid, &mut domains, &mut books);
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

// Add this function to find the next empty cell with the fewest legal values
fn find_most_constrained_variable(grid: &Vec<Vec<u32>>) -> Option<(usize, usize)> {
    let grid_size = grid.len();
    let mut min_options = 10;
    let mut mcv = None;

    for row in 0..grid_size {
        for col in 0..grid_size {
            if grid[row][col] == 0 {
                let mut options = 0;

                for num in 1..=9 {
                    if is_safe(grid, num, row, col) {
                        options += 1;
                    }
                }

                if options < min_options {
                    min_options = options;
                    mcv = Some((row, col));
                }
            }
        }
    }

    mcv
}

// Add this function to find the least constraining values for a given cell
fn least_constraining_values(grid: &Vec<Vec<u32>>, row: usize, col: usize) -> Vec<u32> {
    let mut values_with_constraints = Vec::new();

    for num in 1..=9 {
        if is_safe(grid, num, row, col) {
            let mut constraints = 0;

            for i in 0..9 {
                if i != row && is_safe(grid, num, i, col) {
                    constraints += 1;
                }

                if i != col && is_safe(grid, num, row, i) {
                    constraints += 1;
                }
            }

            values_with_constraints.push((num, constraints));
        }
    }

    values_with_constraints.sort_unstable_by_key(|&(_, constraints)| constraints);
    values_with_constraints.into_iter().map(|(num, _)| num).collect()
}

// Modify the solve_all_sol_n function to use both MCV and LCV heuristics
fn solve_all_sol_n<'a>(
    grid: &'a mut Vec<Vec<u32>>,
    domains: &'a mut Vec<Vec<HashSet<u32>>>,
    result_set: &'a mut HashSet<Vec<Vec<u32>>>,
) {
    if let Some((row, col)) = find_most_constrained_variable(grid) {
        let lcv_values = least_constraining_values(grid, row, col);

        for num in lcv_values {
            if domains[row][col].contains(&num) {
                let old_domains = domains.clone();
                grid[row][col] = num;
                update_domains(domains, grid, row, col);
                solve_all_sol_n(grid, domains, result_set);
                grid[row][col] = 0;
                *domains = old_domains;
            }
        }
    } else {
        result_set.insert(grid.clone());
    }
}


fn initialize_domains(grid: &Vec<Vec<u32>>) -> Vec<Vec<HashSet<u32>>> {
    let size = grid.len();
    let mut domains: Vec<Vec<HashSet<u32>>> = vec![vec![HashSet::new(); size]; size];

    for row in 0..size {
        for col in 0..size {
            if grid[row][col] == 0 {
                for num in 1..=9 {
                    if is_safe(grid, num, row, col) {
                        domains[row][col].insert(num);
                    }
                }
            }
        }
    }

    domains
}

fn update_domains(domains: &mut Vec<Vec<HashSet<u32>>>, grid: &Vec<Vec<u32>>, row: usize, col: usize) {
    let size = grid.len();
    let num = grid[row][col];

    for i in 0..size {
        domains[row][i].remove(&num);
        domains[i][col].remove(&num);
    }

    let (row_start, col_start) = (row / 3 * 3, col / 3 * 3);
    for i in 0..3 {
        for j in 0..3 {
            domains[row_start + i][col_start + j].remove(&num);
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
            if j != size - 1 {
                print!(" ");
            }
        }
        println!();
    }
    println!();
}