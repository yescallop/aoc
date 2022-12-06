use std::mem;

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
        // For speed:
        //
        // #[cfg(target_endian = "big")]
        // compile_error!("LE only");
        // // SAFETY: It's fine.
        // let (a, lines, b) = unsafe { self.input.as_bytes().align_to::<u32>() };
        // ensure!(a.is_empty() && b.is_empty());
        // let mut sum = (0, 0);
        // for x in lines {
        //     let t = ((x | x >> 14) & 0xf) * 4;
        //     sum.0 += (0x693025807140u64 >> t) & 0xf;
        //     sum.1 += (0x798065402130u64 >> t) & 0xf;
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
            if x < 27 {
                x + 26
            } else {
                x - (b'a' - b'A') as u32
            }
        }

        let mut lines = self.input.lines().peekable();
        let mut sum = (0, 0);
        while lines.peek().is_some() {
            let mut group = !0;
            for _ in 0..3 {
                let line = lines.next().ok()?.as_bytes();
                let (l, r) = line.split_at(line.len() / 2);

                // Subtracting with 64 is generally faster than with b'A' (65),
                // since shifts are 6-bit masked on ARM and Intel.
                let bitset = |acc, x| acc | (1u64 << (x - 64));
                let l = l.iter().fold(0, bitset);
                let r = r.iter().fold(0, bitset);

                sum.0 += priority(l & r);
                group &= l | r;
            }
            sum.1 += priority(group);
        }

        self.output(sum.0);
        self.output(sum.1);
        Ok(())
    }
}

impl Solution<2022, 4> for Puzzle {
    fn solve(&mut self) -> Result<()> {
        fn range(s: &str) -> Option<(u32, u32)> {
            let (l, r) = s.split_once('-')?;
            l.parse().ok().zip(r.parse().ok())
        }
        fn contains(a: (u32, u32), b: (u32, u32)) -> bool {
            let (x, y) = (b.0 as i32 - a.0 as i32, b.1 as i32 - a.1 as i32);
            x * y <= 0
        }
        fn overlaps(a: (u32, u32), b: (u32, u32)) -> bool {
            !(b.0 > a.1 || b.1 < a.0)
        }

        let mut cnt = (0, 0);
        for line in self.input.lines() {
            let (l, r) = line.split_once(',').ok()?;
            let (l, r) = range(l).zip(range(r)).ok()?;
            cnt.0 += contains(l, r) as u32;
            cnt.1 += overlaps(l, r) as u32;
        }

        self.output(cnt.0);
        self.output(cnt.1);
        Ok(())
    }
}

impl Solution<2022, 5> for Puzzle {
    fn solve(&mut self) -> Result<()> {
        fn get_two_mut<T>(slice: &mut [T], mut i: usize, mut j: usize) -> (&mut T, &mut T) {
            let rev = i > j;
            if rev {
                mem::swap(&mut i, &mut j);
            }
            let [a, .., b] = &mut slice[i..=j] else {
                panic!("duplicate index");
            };
            if rev {
                (b, a)
            } else {
                (a, b)
            }
        }

        let line_len = self.input.lines().next().ok()?.len();
        let stack_cnt = line_len / 4 + 1;
        ensure!(line_len % 4 == 3 && stack_cnt <= 9);

        const EMPTY_VEC: Vec<u8> = Vec::new();
        let mut stacks = [EMPTY_VEC; 9];

        let mut lines = self.input.lines();
        for line in &mut lines {
            let line = line.as_bytes();
            ensure!(line.len() == line_len);
            if line[1] == b'1' {
                break;
            }

            for i in 0..stack_cnt {
                let x = line[i * 4 + 1];
                if x != b' ' {
                    stacks[i].push(x);
                }
            }
        }

        stacks.iter_mut().for_each(|s| s.reverse());
        let mut stacks_2 = stacks.clone();

        ensure!(lines.next().ok()?.is_empty());
        for line in lines {
            let mut iter = line.split(' ');
            let cnt: usize = iter.nth(1).ok()?.parse()?;

            let mut from_i: usize = iter.nth(1).ok()?.parse()?;
            let mut to_i: usize = iter.nth(1).ok()?.parse()?;
            ensure!(from_i > 0 && to_i > 0);

            from_i -= 1;
            to_i -= 1;
            ensure!(from_i < stack_cnt && to_i < stack_cnt && from_i != to_i);

            // For CrateMover 9000:
            let (from, to) = get_two_mut(&mut stacks, from_i, to_i);
            ensure!(cnt <= from.len());
            to.extend(from.drain(from.len() - cnt..).rev());

            // For CrateMover 9001:
            let (from, to) = get_two_mut(&mut stacks_2, from_i, to_i);
            ensure!(cnt <= from.len());
            to.extend(from.drain(from.len() - cnt..));
        }

        let mut ans = (String::new(), String::new());
        for i in 0..stack_cnt {
            ans.0.push(*stacks[i].last().ok()? as char);
            ans.1.push(*stacks_2[i].last().ok()? as char);
        }
        self.output(ans.0);
        self.output(ans.1);
        Ok(())
    }
}

impl Solution<2022, 6> for Puzzle {
    fn solve(&mut self) -> Result<()> {
        fn all_distinct(bytes: &[u8]) -> bool {
            let mut flags = 0u32;
            for x in bytes {
                // Subtracting with 96 is faster than with b'a' (97).
                // See also: comments in Day 3.
                let mask = 1 << (x - 96);
                if flags & mask != 0 {
                    return false;
                }
                flags |= mask;
            }
            true
        }

        // See also: https://github.com/orlp/aoc2022/blob/master/src/bin/day06.rs
        fn solve_memoized<const N: usize>(bytes: &[u8]) -> Option<usize> {
            if bytes.len() < N {
                return None;
            }
            // In the current N-byte window:
            // The number of bytes with a certain value;
            let mut byte_cnt = [0u8; 256];
            // The number of distinct byte values.
            let mut value_cnt = 0;

            for i in 0..N {
                let in_cnt = &mut byte_cnt[bytes[i] as usize];
                value_cnt += (*in_cnt == 0) as usize;
                *in_cnt += 1;
            }

            let pos = bytes.windows(N + 1).position(|window| {
                let &[out_byte, .., in_byte] = window else {
                    unreachable!();
                };

                let out_cnt = &mut byte_cnt[out_byte as usize];
                *out_cnt -= 1;
                value_cnt -= (*out_cnt == 0) as usize;

                let in_cnt = &mut byte_cnt[in_byte as usize];
                value_cnt += (*in_cnt == 0) as usize;
                *in_cnt += 1;

                value_cnt == N
            });
            pos.map(|i| i + N + 1)
        }

        let bytes = self.input.as_bytes();
        let ans1 = bytes.windows(4).position(all_distinct).ok()? + 4;
        let ans2 = solve_memoized::<14>(bytes).ok()?;

        self.output(ans1);
        self.output(ans2);
        Ok(())
    }
}
