use std::collections::HashSet;
use crate::sudoku_starter::*;
use crate::par_sudoku::*;
use std::time::{Duration, Instant};

mod sudoku_starter;
mod par_sudoku;

fn time_solve(board: &str) -> (HashSet<Vec<Vec<u32>>>, Duration) {
    let starting_point = Instant::now();
    let solutions = solve(board);
    let par_solutions = par_solve(board);
    let elapsed_time = starting_point.elapsed();
    (solutions, elapsed_time)
}

fn main() {
    let board = "53..7....6..195....98....6.8...6...34..8.3..17...2...6.6....28....419..5....8..79";
    //easy board
    // "53..7....6..195....98....6.8...6...34..8.3..17...2...6.6....28....419..5....8..79";

    // "1.....3.8.7.4..............2.3.............958.........5.6...7.....8.2...4.......";
    // "249.6...3.3....2..8.......5.....6......2......1..4.82..9.5..7....4.....1.7...3...";
    // "............1.......8....6.8.......34....3...7...2.....6....28....4.9..5....8..79";
    let (solutions, elapsed_time) = time_solve(board);
    let (par_solutions, elapse_time) = time_solve(board);
    println!("Solutions:");
    show_solutions(solutions.clone());
    show_solutions(par_solutions.clone());
    println!("Total solutions found: {}", solutions.len());
    println!("Total solutions found: {}", par_solutions.len());
    println!("Elapsed time: {:?}", elapse_time);
    println!("Elapsed time: {:?}", elapsed_time);
}