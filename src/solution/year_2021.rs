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
        for line in self.input_lines() {
            let (dir, x) = line.split_once(' ').ok()?;
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
        for line in self.input_lines() {
            let (dir, x) = line.split_once(' ').ok()?;
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

impl Solution<2021, 3> for Puzzle {
    fn solve(&mut self) -> Result<()> {
        let bits = self.input_lines().next().ok()?.len();
        let mut sums = vec![0; bits];
        let mut vec = vec![];

        for line in self.input_lines() {
            ensure!(line.len() == bits);
            vec.push(u32::from_str_radix(line, 2)?);

            for (i, x) in line.bytes().enumerate() {
                sums[i] += if x == b'1' { 1 } else { -1 };
            }
        }

        let gamma = sums.iter().fold(0, |acc, &x| (acc << 1) | (x >= 0) as u32);
        let epsilon = gamma ^ ((1 << bits) - 1);

        self.output(gamma * epsilon);

        fn rating(mut vec: Vec<u32>, bits: usize, rate: u32, rev: bool) -> Option<u32> {
            let mut mask = 1 << (bits - 1);
            let mut crit = rate & mask;

            while mask != 0 {
                let next_mask = mask >> 1;
                let mut sum = 0;
                vec.retain(|x| {
                    if x & mask == crit {
                        sum += if x & next_mask != 0 { 1 } else { -1 };
                        true
                    } else {
                        false
                    }
                });

                if vec.len() == 1 {
                    break;
                }
                mask = next_mask;
                crit = ((sum >= 0) != rev) as u32 * mask;
            }

            (vec.len() == 1).then(|| vec[0])
        }

        let oxygen_gen = rating(vec.clone(), bits, gamma, false).ok()?;
        let co2_scrub = rating(vec, bits, epsilon, true).ok()?;

        self.output(oxygen_gen * co2_scrub);
        Ok(())
    }
}