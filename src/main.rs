use std::env;

use aoc::*;

fn main() {
    if let Err(e) = run() {
        eprintln!("err = {e}");
    }
}

fn parse_args() -> Option<(u32, Option<u32>)> {
    let mut args = env::args();
    args.next();

    let year = args.next()?.parse().ok()?;
    let day = args.next().and_then(|arg| arg.parse().ok());
    Some((year, day))
}

fn run() -> Result<()> {
    let sols = dynamic::solutions();
    let Some(sol) = (match parse_args() {
        Some((year, Some(day))) => {
            sols.iter().find(|s| (s.year, s.day) == (year, day))
        }
        Some((year, None)) => sols.iter().rfind(|s| s.year == year),
        None => sols.last()
    }) else {
        return Err(Error::SolutionNotFound);
    };

    println!("(year, day) = ({}, {})", sol.year, sol.day);

    let mut puzzle = fetch_puzzle(sol.year, sol.day)?;
    (sol.solve)(&mut puzzle)?;

    Ok(())
}
