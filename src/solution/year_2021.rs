use super::*;

impl Solution<2021, 1> for Puzzle {
    fn solve(&mut self) -> Result<()> {
        let data = self.input.parse_lines::<u32>()?;
        self.output(data.windows(2).filter(|w| w[1] > w[0]).count());
        self.output(data.windows(4).filter(|w| w[3] > w[0]).count());
        Ok(())
    }
}

impl Solution<2021, 2> for Puzzle {
    fn solve(&mut self) -> Result<()> {
        let (mut h_pos, mut depth) = (0, 0);
        for line in self.input.lines() {
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
        for line in self.input.lines() {
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
        let bits = self.input.lines().next().ok()?.len();
        let mut sums = vec![0; bits];
        let mut data = vec![];

        for line in self.input.lines() {
            ensure!(line.len() == bits);

            // `u32::from_str_radix` allows leading '+', so...
            let num = line
                .bytes()
                .enumerate()
                .try_fold(0, |acc, (i, x)| {
                    let x = (x as char).to_digit(2)?;
                    sums[i] += if x == 1 { 1 } else { -1 };
                    Some((acc << 1) | x)
                })
                .ok()?;
            data.push(num);
        }

        let gamma = sums.iter().fold(0, |acc, &x| (acc << 1) | (x >= 0) as u32);
        let epsilon = gamma ^ ((1 << bits) - 1);

        self.output(gamma * epsilon);

        fn rating(mut data: Vec<u32>, bits: usize, rate: u32, rev: bool) -> Option<u32> {
            let mut mask = 1 << (bits - 1);
            let mut crit = rate & mask;

            while mask != 0 {
                let next_mask = mask >> 1;
                let mut sum = 0;

                data.retain(|x| {
                    if x & mask == crit {
                        sum += if x & next_mask != 0 { 1 } else { -1 };
                        true
                    } else {
                        false
                    }
                });
                if data.len() == 1 {
                    return Some(data[0]);
                }

                mask = next_mask;
                crit = ((sum >= 0) != rev) as u32 * mask;
            }
            None
        }

        let oxygen_gen = rating(data.clone(), bits, gamma, false).ok()?;
        let co2_scrub = rating(data, bits, epsilon, true).ok()?;

        self.output(oxygen_gen * co2_scrub);
        Ok(())
    }
}

impl Solution<2021, 4> for Puzzle {
    fn solve(&mut self) -> Result<()> {
        let mut lines = self.input.lines();
        let nums_to_mark = lines
            .next()
            .ok()?
            .split(',')
            .map(u8::from_str)
            .collect::<Result<Vec<_>, _>>()?;

        let mut boards = Vec::with_capacity(100);
        while let Some(_blank) = lines.next() {
            let mut board = [0u8; 25];
            let mut i = 0;
            for _ in 0..5 {
                let mut iter = lines.next().ok()?.split_ascii_whitespace();
                for _ in 0..5 {
                    board[i] = iter.next().ok()?.parse()?;
                    i += 1;
                }
            }
            boards.push((board, 0u32));
        }
        let boards_cnt = boards.len();

        #[allow(clippy::unusual_byte_groupings)]
        fn mask(i: usize) -> (u32, u32) {
            let (row, col) = (i / 5, i % 5);
            (0b11111 << (row * 5), 0b1_00001_00001_00001_00001 << col)
        }

        let mut boards_won = 0;
        for &num_to_mark in &nums_to_mark {
            for (board, marked) in &mut boards {
                if (*marked as i32) < 0 {
                    // Already won.
                    continue;
                }
                let Some(mark_i) = board.iter().position(|&x| x == num_to_mark) else {
                    continue;
                };
                *marked |= 1 << mark_i;

                let mask = mask(mark_i);
                if *marked & mask.0 != mask.0 && *marked & mask.1 != mask.1 {
                    continue;
                }

                // Mark as won.
                *marked |= 1 << 31;
                boards_won += 1;

                if boards_won > 1 && boards_won < boards_cnt {
                    continue;
                }

                let mut x = *marked;
                let mut sum = 0u32;
                for num in board {
                    if x & 1 == 0 {
                        sum += *num as u32;
                    }
                    x >>= 1;
                }
                self.output(sum * num_to_mark as u32);

                if boards_won == boards_cnt {
                    return Ok(());
                }
            }
        }
        err!();
    }
}
