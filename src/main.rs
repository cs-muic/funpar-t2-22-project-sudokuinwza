use std::collections::HashSet;
// use crate::sudoku_starter::*;
use crate::par_sudoku::*;
use std::time::{Duration, Instant};

mod sudoku_starter;
mod par_sudoku;

fn time_solve(board: &str) -> (HashSet<Vec<Vec<u32>>>, Duration) {
    let starting_point = Instant::now();
    let solutions = solve(board);
    let elapsed_time = starting_point.elapsed();
    (solutions, elapsed_time)
}

fn main() {
    let board = "............1.......8....6.8.......34....3...7...2.....6....28....4.9..5....8..79";
    let (solutions, elapsed_time) = time_solve(board);
    println!("Solutions:");
    show_solutions(solutions.clone());
    println!("Total solutions found: {}", solutions.len());
    println!("Elapsed time: {:?}", elapsed_time);
}