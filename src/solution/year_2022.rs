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
        // Cursed solution for speed:
        //
        // #[cfg(target_endian = "big")]
        // compile_error!("LE only");
        // // SAFETY: It's fine.
        // let (a, lines, b) = unsafe { self.input.as_bytes().align_to::<u32>() };
        // ensure!(a.is_empty() && b.is_empty());
        // let mut sum = (0, 0);
        // for x in lines {
        //     let t = ((x | x >> 14) & 0xf) * 4;
        //     sum.0 += (0x693025807140u64 >> t) as u32 & 0xf;
        //     sum.1 += (0x798065402130u64 >> t) as u32 & 0xf;
        // }

        let mut sum = (0, 0);
        for line in self.input.lines() {
            let [a @ b'A'..=b'C', _, x @ b'X'..=b'Z'] = line.as_bytes() else {
                err!();
            };
            let (opponent, x) = (a - b'A', x - b'X');

            // me, opponent: 0 = Rock, 1 = Paper, 2 = Scissors
            // outcome: 0 = Loss, 1 = Draw, 2 = Win
            // 1 + me - opponent ≡ outcome (mod 3)
            let me = x;
            let outcome = (4 + me - opponent) % 3;
            sum.0 += (1 + me + outcome * 3) as u32;

            let outcome = x;
            let me = (2 + outcome + opponent) % 3;
            sum.1 += (1 + me + outcome * 3) as u32;
        }

        self.output(sum.0);
        self.output(sum.1);
        Ok(())
    }
}

impl Solution<2022, 3> for Puzzle {
    fn solve(&mut self) -> Result<()> {
        fn priority(common: u64) -> u32 {
            let x = common.trailing_zeros();
            if x < 26 {
                x + 27
            } else {
                x - (b'a' - b'A') as u32 + 1
            }
        }

        let mut lines = self.input.lines().peekable();
        let mut sum = (0, 0);
        while lines.peek().is_some() {
            let mut group = !0;
            for _ in 0..3 {
                let line = lines.next().ok()?.as_bytes();
                let comp_len = line.len() / 2;

                let first = line[..comp_len]
                    .iter()
                    .fold(0, |acc, x| acc | (1u64 << (x - b'A')));
                let second = line[comp_len..]
                    .iter()
                    .fold(0, |acc, x| acc | (1u64 << (x - b'A')));
                sum.0 += priority(first & second);
                group &= first | second;
            }
            sum.1 += priority(group);
        }

        self.output(sum.0);
        self.output(sum.1);
        Ok(())
    }
}
