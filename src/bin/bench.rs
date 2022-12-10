use std::{env, time::Instant};

use aoc::{dynamic::DynSolution, *};

const RUNS: u32 = 10000;

fn main() -> Result<()> {
    let sols = dynamic::solutions();

    if let Some("all") = env::args().nth(1).as_deref() {
        sols.iter().try_for_each(bench)
    } else if let Some(sol) = dynamic::solution_by_args() {
        bench(sol)
    } else {
        bench(sols.last().ok_or(Error::SolutionNotFound)?)
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
