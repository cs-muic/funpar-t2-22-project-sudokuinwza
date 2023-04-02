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
    books = solve_all_sol_n(&mut grid, 0, 0, &mut books).to_owned();
    return books;
}

fn is_safe(grid: &Vec<Vec<u32>>, num: u32, row: usize, col: usize) -> bool {
    let size = grid[0].len(); //grid size i.e. 9

    if grid[row].contains(&num) {
        //if the number already exists within that row then return false
        return false;
    }

    for j in 0..size {
        if grid[j][col] == num {
            //if the number already exists within that column then return false
            return false;
        }
    }

    let row_chop = row - row % 3; //3*3 block
    let col_chop = col - col % 3;
    // e.g. row = 2 col = 5
    // row_chop = 2-2 = 0 col_chop = 5 - 2 = 3
    // then the search block would be block [0,1]
    // and the search cells would be row 0-> 2 and col 3->5

    let mut i = row_chop;
    while i <= row_chop + 2 {
        let mut j = col_chop;
        while j <= col_chop + 2 {
            if grid[i][j] == num {
                //if the number already exists within that block then return false
                return false;
            }
            j += 1;
        }
        i += 1;
    }

    return true;
}

fn solve_all_sol_n<'a>(
    grid: &'a mut Vec<Vec<u32>>,
    mut row: usize,
    mut col: usize,
    mut result_set: &'a mut HashSet<Vec<Vec<u32>>>,
) -> &'a HashSet<Vec<Vec<u32>>> {
    let grid_size: usize = grid.len();
    // base case check if we reach the last cell i.e. row = 8 and col = 8
    if row == grid_size - 1 && col == grid_size - 1 {
        //grid_size = 8 for 9*9

        if grid[row][col] > 0 {
            //if the cell in the last index has valid number then we save it to our result_set

            result_set.insert(grid.clone());

            return result_set;
        } else {
            //when the last cell is blank then we check for possible num to be in the cell
            for num in 1..=9 {
                if is_safe(grid, num, row, col) {
                    grid[row][col] = num;

                    result_set.insert(grid.to_owned());

                    grid[row][col] = 0; // reset the grid[row][col] to 0 (unfilled) so, that we can search for other possible sol_n
                }
            }
        }
        return result_set;
    }

    if col == grid_size {
        return solve_all_sol_n(grid, row + 1, 0, result_set);
    }
    if grid[row][col] == 0 {
        //if cell is empty then check for all possible number
        for num in 1..=9 {
            if is_safe(grid, num, row, col) {
                grid[row][col] = num;

                solve_all_sol_n(grid, row, col + 1, result_set);
                grid[row][col] = 0;
            }
        }
    } else {
        solve_all_sol_n(grid, row, col + 1, result_set);
    }
    return result_set;
}

pub fn show_solutions(solutions: HashSet<Vec<Vec<u32>>>) {
    for solution in solutions {
        print_solution(&solution);
    }
}

fn print_solution(grid: &Vec<Vec<u32>>) {
    for i in 0..9 {
        if i % 3 == 0 && i != 0 {
            println!("- - - + - - - + - - -");
        }
        for j in 0..9 {
            if j % 3 == 0 && j != 0 {
                print!("| ");
            }
            print!("{}", grid[i][j]);
            if j != 8 {
                print!(" ");
            }
        }
        println!();
    }
    println!();
}