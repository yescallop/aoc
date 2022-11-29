use crate::{Puzzle, Result, Solution};

impl Solution<2021, 1> for Puzzle {
    fn solve(&mut self) -> Result<()> {
        let vec = self.input_vec::<u32>()?;
        self.output(vec.windows(2).filter(|w| w[1] > w[0]).count());
        self.output(vec.windows(4).filter(|w| w[3] > w[0]).count());
        Ok(())
    }
}
