use core::num;
use std::collections::HashSet;
use std::fs::File;
use std::io::*;
use std::ptr::null;
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
    books = solveAllSoln(&mut grid, 0, 0, &mut books).to_owned();
    return books;
}

fn is_safe(mut grid: &Vec<Vec<u32>>, num: u32, row: usize, col: usize) -> bool {
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

    let rowChop = row - row % 3; //3*3 block
    let colChop = col - col % 3;
    // e.g. row = 2 col = 5
    // rowchop = 2-2 = 0 colchop = 5 - 2 = 3
    // then the search block would be block [0,1]
    // and the search cells would be row 0-> 2 and col 3->5

    let mut i = rowChop;
    while i <= rowChop + 2 {
        let mut j = colChop;
        while j <= colChop + 2 {
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

fn solveAllSoln<'a>(
    grid: &'a mut Vec<Vec<u32>>,
    mut row: usize,
    mut col: usize,
    mut resultSet: &'a mut HashSet<Vec<Vec<u32>>>,
) -> &'a HashSet<Vec<Vec<u32>>> {
    let grid_size: usize = grid.len();
    // base case check if we reach the last cell i.e. row = 8 and col = 8
    if row == grid_size - 1 && col == grid_size - 1 {
        //grid_size = 8 for 9*9

        if grid[row][col] > 0 {
            //if the cell in the last index has valid number then we save it to our resultSet

            resultSet.insert(grid.clone());

            return resultSet;
        } else {
            //when the last cell is blank then we check for possible num to be in the cell
            for num in 1..=9 {
                if is_safe(grid, num, row, col) {
                    grid[row][col] = num;

                    resultSet.insert(grid.to_owned());

                    grid[row][col] = 0; // reset the grid[row][col] to 0 (unfilled) so, that we can search for other possible soln
                }
            }
        }
        return resultSet;
    }

    if col == grid_size {
        return solveAllSoln(grid, row + 1, 0, resultSet);
    }
    if grid[row][col] == 0 {
        //if cell is empty then check for all possible number
        for num in 1..=9 {
            if is_safe(grid, num, row, col) {
                grid[row][col] = num;

                solveAllSoln(grid, row, col + 1, resultSet);
                grid[row][col] = 0;
            }
        }
    } else {
        solveAllSoln(grid, row, col + 1, resultSet);
    }
    return resultSet;
}

fn shows_sol(inputs: HashSet<Vec<Vec<u32>>>) {
    for big in inputs {
        for line in big {
            for cell in line {
                print!("{}, ", cell);
            }
            println!("");
        }
        println!("");
    }
}