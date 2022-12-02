use super::*;

// Alternative: collect into a `Vec` and use `{sort, select_nth}_unstable_by`.
impl Solution<2022, 1> for Puzzle {
    fn solve(&mut self) -> Result<()> {
        let mut max = [0; 3];
        // Alternative: use `StrExt::line_blocks`.
        let mut lines = self.input.lines().peekable();
        while lines.peek().is_some() {
            let block = lines.by_ref().take_while(|l| !l.is_empty());
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

impl Solution<2022, 2> for Puzzle {
    fn solve(&mut self) -> Result<()> {
        let mut total = (0, 0);
        for line in self.input.lines() {
            let &[a @ b'A'..=b'C', b' ', x @ b'X'..=b'Z'] = line.as_bytes() else {
                err!();
            };
            let (opp, x) = (a - b'A', x - b'X');

            let you = x;
            let outcome = (4 + you - opp) % 3;
            let score = (you + 1) + outcome * 3;
            total.0 += score as u32;

            let outcome = x;
            let you = (2 + opp + outcome) % 3;
            let score = (you + 1) + outcome * 3;
            total.1 += score as u32;
        }

        self.output(total.0);
        self.output(total.1);
        Ok(())
    }
}
