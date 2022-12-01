use super::*;

// Alternative: collect into a `Vec` and use `sort_unstable_by`.
impl Solution<2022, 1> for Puzzle {
    fn solve(&mut self) -> Result<()> {
        let mut max = [0; 3];
        for block in &self.input.line_blocks() {
            let sum = block.map(u32::from_str).sum::<Result<_, _>>()?;
            // This should be fairly good, I suppose.
            for i in 0..3 {
                // Numbers may equal.
                if sum >= max[i] {
                    max.copy_within(i..2, i + 1);
                    max[i] = sum;
                    break;
                }
            }
        }

        self.output(max[0]);
        self.output(max.iter().sum::<u32>());
        Ok(())
    }
}
