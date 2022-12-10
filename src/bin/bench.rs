use std::{env, time::Instant};

use aoc::{dynamic::DynSolution, *};

const RUNS: u32 = 10000;

fn main() -> Result<()> {
    if let Some("all") = env::args().nth(1).as_deref() {
        dynamic::solutions().iter().try_for_each(bench)
    } else {
        bench(dynamic::solution_by_args().ok_or(Error::SolutionNotFound)?)
    }
}

fn bench(sol: &DynSolution) -> Result<()> {
    print!("{}d{}: ", sol.year, sol.day);

    let mut puzzle = fetch_puzzle(sol.year, sol.day)?;
    puzzle.suppress_output();

    let start = Instant::now();
    for _ in 0..RUNS {
        (sol.solve)(&mut puzzle)?;
    }
    println!("{:?}", start.elapsed() / RUNS);

    Ok(())
}
