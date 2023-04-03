use crate::sudoku_starter::*;

mod sudoku_starter;

fn main() {
    println!("Hello, world!");
    let board = "4.....8.5.3..........7......2.....6.....8.4......1.......6.3.7.5..2.....1.4......";
    let solutions = solve(board);
    println!("Total solutions found: {}", solutions.len());
    println!("Solutions:");
    show_solutions(solutions);
}
