use crate::sudoku_starter::*;

mod sudoku_starter;

fn main() {
    println!("Hello, world!");
    let board = "53..7....6..195....98....6.8...6...34..8.3..17...2...6.6....28....419..5....8..79";
    let solutions = solve(board);
    println!("Total solutions found: {}", solutions.len());
    println!("Solutions:");
    show_solutions(solutions);
}
