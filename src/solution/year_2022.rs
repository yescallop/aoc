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

        ensure!(lines.next() == Some(""));
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
        fn solve_xor<const N: usize>(bytes: &[u8]) -> Option<usize> {
            if bytes.len() < N {
                return None;
            }

            // flag = number of a byte value in the current window
            //    0 = zero or even
            //    1 = one or odd
            // current window contains no duplicates iff `flags.count_ones() == N`
            let mut flags = 0u32;
            for i in 0..N {
                // Subtracting with 96 (b'a' - 1) is no-op.
                // See also: comments in Day 3.
                flags ^= 1 << (bytes[i] - 96);
            }
            if flags.count_ones() == N as u32 {
                return Some(4);
            }

            let pos = bytes.windows(N + 1).position(|one_larger_window| {
                let &[out_byte, .., in_byte] = one_larger_window else {
                    unreachable!();
                };
                flags ^= 1 << (out_byte - 96);
                flags ^= 1 << (in_byte - 96);
                flags.count_ones() == N as u32
            });
            pos.map(|i| i + N + 1)
        }

        let bytes = self.input.as_bytes();
        let ans1 = solve_xor::<4>(bytes).ok()?;
        let ans2 = solve_xor::<14>(bytes).ok()?;

        self.output(ans1);
        self.output(ans2);
        Ok(())
    }
}

impl Solution<2022, 7> for Puzzle {
    fn solve(&mut self) -> Result<()> {
        struct Dir {
            parent: i32,
            size: u32,
        }

        let mut dirs = Vec::<Dir>::new();
        let mut cur_i = -1;

        for line in self.input.lines() {
            if line.starts_with('$') {
                match line.strip_prefix("$ cd ") {
                    Some("..") => {
                        ensure!(cur_i > 0);
                        cur_i = dirs[cur_i as usize].parent;
                    }
                    Some(_) => {
                        let next_i = dirs.len() as i32;
                        dirs.push(Dir {
                            parent: cur_i,
                            size: 0,
                        });
                        cur_i = next_i;
                    }
                    None => {}
                }
                continue;
            }

            let (attr, _) = line.split_once(' ').ok()?;
            if let Ok(size) = attr.parse::<u32>() {
                let mut i = cur_i;
                while i >= 0 {
                    dirs[i as usize].size += size;
                    i = dirs[i as usize].parent;
                }
            }
        }

        let ans1: u32 = dirs
            .iter()
            .map(|dir| dir.size)
            .filter(|&size| size <= 100000)
            .sum();
        self.output(ans1);

        const TOTAL_SPACE: u32 = 70000000;
        const REQUIRED_SPACE: u32 = 30000000;

        ensure!(!dirs.is_empty());
        let used = dirs[0].size;
        ensure!(used <= TOTAL_SPACE);
        let unused = TOTAL_SPACE - used;
        ensure!(unused < REQUIRED_SPACE);
        let min_to_delete = REQUIRED_SPACE - unused;

        let ans2 = dirs
            .iter()
            .map(|dir| dir.size)
            .filter(|&size| size >= min_to_delete)
            .min();
        self.output(ans2.unwrap());

        Ok(())
    }
}

impl Solution<2022, 8> for Puzzle {
    fn solve(&mut self) -> Result<()> {
        let width = self.input.lines().next().ok()?.len();

        let mut rows = Vec::with_capacity(width);
        let mut visible: Vec<Vec<bool>> = Vec::with_capacity(width);

        let mut max_from_top = vec![0u8; width];

        for row in self.input.lines().map(str::as_bytes) {
            ensure!(row.len() == width);
            let mut vis_row = vec![false; width];

            let mut max_from_left = 0;
            for i in 0..width {
                let h = row[i];
                if h > max_from_left {
                    vis_row[i] = true;
                    max_from_left = h;
                }
                if h > max_from_top[i] {
                    vis_row[i] = true;
                    max_from_top[i] = h;
                }
            }

            let mut max_from_right = 0;
            for i in (0..width).rev() {
                let h = row[i];
                if h > max_from_right {
                    vis_row[i] = true;
                    max_from_right = h;
                    if h == max_from_left {
                        break;
                    }
                }
            }

            rows.push(row);
            visible.push(vis_row);
        }

        let mut max_from_bottom = vec![0u8; width];
        let mut cols_done = 0;

        for (row, vis_row) in rows.iter().zip(visible.iter_mut()).rev() {
            assert!(row.len() == width && vis_row.len() == width);

            for i in 0..width {
                let h = row[i];
                if h > max_from_bottom[i] {
                    vis_row[i] = true;
                    max_from_bottom[i] = h;
                    cols_done += (h == max_from_top[i]) as usize;
                }
            }
            if cols_done == width {
                break;
            }
        }

        let visible_cnt = visible.iter().flatten().filter(|&&b| b).count();

        let height = rows.len();
        let mut max_score = 0;

        for y in 0..height {
            let row = rows[y];
            for x in 0..width {
                let h = row[x];

                let up = rows[..y]
                    .iter()
                    .rev()
                    .position(|r| r[x] >= h)
                    .map(|i| i + 1)
                    .unwrap_or(y);
                let left = row[..x]
                    .iter()
                    .rev()
                    .position(|&v| v >= h)
                    .map(|i| i + 1)
                    .unwrap_or(x);
                let down = rows[y + 1..]
                    .iter()
                    .position(|r| r[x] >= h)
                    .map(|i| i + 1)
                    .unwrap_or(height - y - 1);
                let right = row[x + 1..]
                    .iter()
                    .position(|&v| v >= h)
                    .map(|i| i + 1)
                    .unwrap_or(width - x - 1);

                let score = up * left * down * right;
                if score > max_score {
                    max_score = score;
                }
            }
        }

        self.output(visible_cnt);
        self.output(max_score);
        Ok(())
    }
}
