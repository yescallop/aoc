use super::*;

impl Solution<2022, 1> for Puzzle {
    fn solve(&mut self) -> Result<()> {
        let mut vec = self
            .input
            .line_blocks()
            .map(|b| b.map(|l| l.parse::<u32>()).sum())
            .collect::<Result<Vec<u32>, _>>()?;

        ensure!(vec.len() >= 3);
        vec.sort_unstable_by(|a, b| b.cmp(a));

        self.output(vec[0]);
        self.output(vec[..3].iter().sum::<u32>());
        Ok(())
    }
}
