use std::{env, time::Instant};

use aoc::{dynamic::DynSolution, *};

const RUNS: u32 = 10000;

fn parse_args() -> Option<(u32, u32)> {
    let mut args = env::args();
    args.next();

    let year = args.next()?.parse().ok()?;
    let day = args.next()?.parse().ok()?;
    Some((year, day))
}

fn main() -> Result<()> {
    let sols = dynamic::solutions();

    if let Some((year, day)) = parse_args() {
        let sol = sols
            .iter()
            .find(|s| (s.year, s.day) == (year, day))
            .ok_or(Error::SolutionNotFound)?;
        bench(sol)
    } else if let Some("all") = env::args().nth(1).as_deref() {
        sols.iter().try_for_each(bench)
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
