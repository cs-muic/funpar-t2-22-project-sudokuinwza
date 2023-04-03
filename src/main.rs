use std::collections::HashSet;
use crate::sudoku_starter::*;
use std::time::{Duration, Instant};

mod sudoku_starter;

fn time_solve(board: &str) -> (HashSet<Vec<Vec<u32>>>, Duration) {
    let starting_point = Instant::now();
    let solutions = solve(board);
    let elapsed_time = starting_point.elapsed();
    (solutions, elapsed_time)
}

fn main() {
    let board = "53..7....6..195....98....6.8...6...34..8.3..17...2...6.6....28....419..5....8..79";
    let (solutions, elapsed_time) = time_solve(board);
    println!("Solutions:");
    show_solutions(solutions.clone());
    println!("Total solutions found: {}", solutions.len());
    println!("Elapsed time: {:?}", elapsed_time);
}