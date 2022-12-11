use std::{
    env,
    time::{Duration, Instant},
};

use aoc::{dynamic::DynSolution, *};

const WARMUP_RUNS: u32 = 100;
const BENCH_TIME: Duration = Duration::from_secs(5);

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
    for _ in 0..WARMUP_RUNS {
        (sol.solve)(&mut puzzle)?;
    }
    let estimated_time = start.elapsed() / WARMUP_RUNS;

    let estimated_runs = (BENCH_TIME.as_secs_f32() / estimated_time.as_secs_f32()) as u32;
    let start = Instant::now();
    for _ in 0..estimated_runs {
        (sol.solve)(&mut puzzle)?;
    }
    println!("{:?}", start.elapsed() / estimated_runs);

    Ok(())
}
