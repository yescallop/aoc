use std::iter;

use super::*;

impl Solution<2022, 1> for Puzzle {
    fn solve(&mut self) -> Result<()> {
        let mut sum = 0;
        let mut vec = vec![];
        for line in self.input_lines().chain(iter::once("")) {
            if line.is_empty() {
                vec.push(sum);
                sum = 0;
            } else {
                sum += line.parse::<u32>()?;
            }
        }

        ensure!(vec.len() >= 3);
        vec.sort_unstable_by(|a, b| b.cmp(a));

        self.output(vec[0]);
        self.output(vec[..3].iter().sum::<u32>());
        Ok(())
    }
}
