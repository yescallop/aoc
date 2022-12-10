use aoc::*;

fn main() {
    if let Err(e) = run() {
        eprintln!("err = {e}");
    }
}

fn run() -> Result<()> {
    let Some(sol) = dynamic::solution_by_args() else {
        return Err(Error::SolutionNotFound);
    };

    println!("(year, day) = ({}, {})", sol.year, sol.day);

    let mut puzzle = fetch_puzzle(sol.year, sol.day)?;
    (sol.solve)(&mut puzzle)?;

    for (i, out) in puzzle.outputs().enumerate() {
        println!("out[{i}] = {out}");
    }

    Ok(())
}
