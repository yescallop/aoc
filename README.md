# aoc

This repository holds my solutions to [Advent of Code](https://adventofcode.com/) puzzles, written in Rust, licensed under the [MIT License](/LICENSE).

## Boilerplate Free

Insert your solution in the codebase, `cargo run`, and you're done.

The *latest* solution is run by default[^1], against the puzzle input fetched from the website[^2] or local cache.

The following is an example of solution code.

```rust
impl Solution<2077, 1> for Puzzle {
    fn solve(&mut self) -> Result<()> {
        Ok(self.output(self.input().to_owned()))
    }
}
```

Typically you won't need to modify anything else, thanks to *specialization* implemented in the [`dynamic`](/src/dynamic.rs) module, except that the `push_sols` macro call therein needs to be updated before a new year's work.

[^1]: To run a specific solution, run the binary with arguments `year day`.
You may omit the day to run the latest solution in a specific year.
[^2]: Cookie file (`cookie.txt`) is needed, with contents `session=xxx`.
