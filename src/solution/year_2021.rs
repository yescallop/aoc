use super::*;

impl Solution<2021, 1> for Puzzle {
    fn solve(&mut self) -> Result<()> {
        let vec = self.input_vec::<u32>()?;
        self.output(vec.windows(2).filter(|w| w[1] > w[0]).count());
        self.output(vec.windows(4).filter(|w| w[3] > w[0]).count());
        Ok(())
    }
}

impl Solution<2021, 2> for Puzzle {
    fn solve(&mut self) -> Result<()> {
        let (mut h_pos, mut depth) = (0, 0);
        for l in self.input_lines() {
            let (dir, x) = l.split_once(' ').ok()?;
            let x = x.parse::<u32>()?;
            match dir {
                "forward" => h_pos += x,
                "down" => depth += x,
                "up" => depth -= x,
                _ => err!(),
            }
        }

        self.output(h_pos * depth);

        let (mut h_pos, mut depth, mut aim) = (0, 0, 0);
        for l in self.input_lines() {
            let (dir, x) = l.split_once(' ').ok()?;
            let x = x.parse::<u32>()?;
            match dir {
                "down" => aim += x,
                "up" => aim -= x,
                "forward" => {
                    h_pos += x;
                    depth += aim * x;
                }
                _ => err!(),
            }
        }

        self.output(h_pos * depth);
        Ok(())
    }
}
