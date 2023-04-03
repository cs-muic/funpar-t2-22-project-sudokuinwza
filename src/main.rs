use crate::sudoku_starter::*;

mod sudoku_starter;

fn main() {
    println!("Hello, world!");
    let board = "1.....3.8.7.4..............2.3.............958.........5.6...7.....8.2...4.......";
    let solutions = solve(board);
    println!("Solutions:");
    show_solutions(solutions.clone());
    println!("Total solutions found: {}", solutions.len());
}