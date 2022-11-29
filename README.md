# aoc

This is my solutions to [Advent of Code](https://adventofcode.com/) puzzles, written in Rust, licensed under the [MIT License](/LICENSE).

## Boilerplate Free

Insert your solution anywhere you like[^1], `cargo run`, and you're done.

The *latest* solution is run by default[^2], against the puzzle input fetched from the website[^3] or local cache.

The following is an example of inserted code.

```rust
impl Solution<2022, 1> for Puzzle {
    fn solve(&mut self) -> Result<()> {
        // input via self.input and friends
        // output via self.output
        Ok(())
    }
}
```

Typically you won't need to modify anything else, thanks to [autoref specialization](https://github.com/dtolnay/case-studies/blob/master/autoref-specialization/README.md) implemented in the [`dynamic`](/src/dynamic.rs) module, except that the `push_sols` macro call therein needs to be updated before each new year's work.

[^1]: In the module tree, to be exact.
[^2]: To run a specific solution, run the binary with arguments `year day`.
You may omit the day to run the latest solution in a specific year.
[^3]: Cookie file (`cookie.txt`) is needed, with contents `session=xxx`.
