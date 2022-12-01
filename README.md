# aoc

This repository holds my solutions to [Advent of Code](https://adventofcode.com/) puzzles, written in Rust, licensed under the [MIT License](/LICENSE).

## Using this crate as a template

Replace the [solutions](/src/solution/) with your own, `cargo run`, and you're done.

The *latest* solution is run by default[^1], against the puzzle input fetched from the website[^2] or local cache.

Here is an example of solution code (typically located at `/src/solution/year_2077.rs`):

```rust
use super::*;

impl Solution<2077, 1> for Puzzle {
    fn solve(&mut self) -> Result<()> {
        self.output(self.input().len());
        self.output(self.input_lines().count());
        Ok(())
    }
}
```

Running this solution against an input of `100` bytes and `10` lines, the program would print the following:

```text
(year, day) = (2077, 1)
out[0] = 100
out[1] = 10
```

Typically you won't need to modify anything else, except for some small initialization[^3] before a new year's work.

[^1]: To run a specific solution, pass in the arguments `year day`.
You may omit the day to run the latest solution in a specific year.
[^2]: Cookie file (`cookie.txt`) is needed, with contents `session=<obtained from your browser>`.
[^3]: This includes updating the [module declarations][1] and the [`push_sols` macro call][2].

[1]: https://github.com/yescallop/aoc/blob/main/src/solution.rs#L17
[2]: https://github.com/yescallop/aoc/blob/main/src/dynamic.rs#L52
