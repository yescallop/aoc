use std::collections::VecDeque;

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
                (i, j) = (j, i);
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
            for in_byte in &bytes[..N] {
                // Subtracting with 96 (b'a' - 1) is no-op.
                // See also: comments in Day 3.
                flags ^= 1 << (in_byte - 96);
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
        let mut visible: Vec<bool> = Vec::with_capacity(width * width);
        let mut vis_row = vec![false; width];

        let mut max_from_top = vec![0u8; width];

        for row in self.input.lines().map(str::as_bytes) {
            ensure!(row.len() == width);

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
            visible.extend_from_slice(&vis_row);
            vis_row.fill(false);
        }

        let mut max_from_bottom = vec![0u8; width];
        let mut cols_done = 0;

        for (row, vis_row) in rows.iter().rev().zip(visible.rchunks_mut(width)) {
            assert!(row.len() == width);

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

        let visible_cnt = visible.iter().filter(|&&b| b).count();

        fn viewing_dis(h: u8, mut iter: impl Iterator<Item = u8>) -> u32 {
            let mut dis = 0;
            iter.find(|&cur_h| {
                dis += 1;
                cur_h >= h
            });
            dis
        }

        let mut max_score = 0;

        for y in 0..rows.len() {
            let row = rows[y];
            for x in 0..width {
                let h = row[x];
                let get_h = |row: &&[u8]| row[x];

                let up = viewing_dis(h, rows[..y].iter().rev().map(get_h));
                let left = viewing_dis(h, row[..x].iter().rev().copied());
                let down = viewing_dis(h, rows[y + 1..].iter().map(get_h));
                let right = viewing_dis(h, row[x + 1..].iter().copied());

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

impl Solution<2022, 9> for Puzzle {
    fn solve(&mut self) -> Result<()> {
        fn zigzag_encode(x: i32) -> u32 {
            ((x << 1) ^ (x >> 31)) as u32
        }

        #[cfg(not(target_arch = "x86_64"))]
        fn interleave(x: u32, y: u32) -> u64 {
            fn interleave_with_zeros(input: u32) -> u64 {
                let mut word = input as u64;
                word = (word ^ (word << 16)) & 0x0000ffff0000ffff;
                word = (word ^ (word << 8)) & 0x00ff00ff00ff00ff;
                word = (word ^ (word << 4)) & 0x0f0f0f0f0f0f0f0f;
                word = (word ^ (word << 2)) & 0x3333333333333333;
                word = (word ^ (word << 1)) & 0x5555555555555555;
                word
            }
            interleave_with_zeros(x) | (interleave_with_zeros(y) << 1)
        }

        #[cfg(target_arch = "x86_64")]
        fn interleave(x: u32, y: u32) -> u64 {
            use std::arch::x86_64::_pdep_u64;
            unsafe {
                _pdep_u64(x as u64, 0x5555555555555555) | _pdep_u64(y as u64, 0xaaaaaaaaaaaaaaaa)
            }
        }

        #[derive(Clone, Copy, Default, PartialEq, Eq, Hash)]
        struct Point {
            x: i32,
            y: i32,
        }

        impl Point {
            fn index(self) -> usize {
                let x = zigzag_encode(self.x);
                let y = zigzag_encode(self.y);
                interleave(x, y) as usize
            }

            fn set_in(self, bitset: &mut Vec<u64>) {
                let index = self.index();
                let (word_i, bit_i) = (index / 64, index % 64);
                if word_i >= bitset.len() {
                    bitset.resize(bitset.len() * 2, 0);
                }
                bitset[word_i] |= 1 << bit_i;
            }
        }

        let mut knots = [Point::default(); 10];

        let mut knot_1_track = vec![0u64; 4096];
        knot_1_track[0] = 1;
        let mut knot_9_track = vec![0u64; 4096];
        knot_9_track[0] = 1;

        for line in self.input.lines() {
            let (direction, step) = line.split_once(' ').ok()?;
            let (dx, dy) = match direction {
                "U" => (0, 1),
                "R" => (1, 0),
                "D" => (0, -1),
                "L" => (-1, 0),
                _ => err!(),
            };
            let step: u32 = step.parse()?;

            for _ in 0..step {
                knots[0].x += dx;
                knots[0].y += dy;

                let mut i = 1;
                while i < 10 {
                    let (front, back) = (knots[i - 1], &mut knots[i]);
                    let diff_x = front.x - back.x;
                    let diff_y = front.y - back.y;

                    if diff_x.abs() > 1 || diff_y.abs() > 1 {
                        back.x += diff_x.signum();
                        back.y += diff_y.signum();
                    } else {
                        break;
                    }
                    i += 1;
                }

                if i > 1 {
                    knots[1].set_in(&mut knot_1_track);
                }
                if i > 9 {
                    knots[9].set_in(&mut knot_9_track);
                }
            }
        }

        let ans1: u32 = knot_1_track.iter().map(|x| x.count_ones()).sum();
        let ans2: u32 = knot_9_track.iter().map(|x| x.count_ones()).sum();

        self.output(ans1);
        self.output(ans2);
        Ok(())
    }
}

impl Solution<2022, 10> for Puzzle {
    fn solve(&mut self) -> Result<()> {
        #[derive(Clone, Copy)]
        enum Instr {
            Noop,
            AddX(i32),
        }

        struct Cpu<I> {
            instrs: I,
            cur_instr: Option<Instr>,
            cycles_countdown: i32,
            x: i32,
        }

        impl<I> Cpu<I>
        where
            I: Iterator<Item = Instr>,
        {
            fn from_instrs(instrs: I) -> Cpu<I> {
                Cpu {
                    instrs,
                    cur_instr: None,
                    cycles_countdown: 0,
                    x: 1,
                }
            }
        }

        impl<I> Iterator for Cpu<I>
        where
            I: Iterator<Item = Instr>,
        {
            type Item = i32;

            /// Yields the value of the `X` register during each cycle.
            fn next(&mut self) -> Option<i32> {
                if self.cycles_countdown == 0 {
                    match self.cur_instr.take() {
                        None | Some(Instr::Noop) => (),
                        Some(Instr::AddX(imm)) => self.x += imm,
                    }

                    if let Some(instr) = self.instrs.next() {
                        self.cur_instr = Some(instr);
                        self.cycles_countdown = match instr {
                            Instr::Noop => 1,
                            Instr::AddX(_) => 2,
                        };
                    }
                }
                self.cycles_countdown -= 1;
                Some(self.x)
            }
        }

        let instrs = self.input.lines().map(|line| {
            line.strip_prefix("addx ")
                .and_then(|imm| imm.parse().map(Instr::AddX).ok())
                .unwrap_or(Instr::Noop)
        });

        let mut signal_strength_sum = 0;
        let mut image = String::with_capacity(41 * 6);

        for (index, x) in Cpu::from_instrs(instrs).enumerate().take(240) {
            let col = (index % 40) as i32;
            if col == 0 {
                image.push('\n');
            } else if col == 19 {
                signal_strength_sum += (index as i32 + 1) * x;
            }

            let sprite = x - 1..=x + 1;
            image.push(if sprite.contains(&col) { '#' } else { '.' });
        }

        self.output(signal_strength_sum);
        self.output(image);
        Ok(())
    }
}

impl Solution<2022, 11> for Puzzle {
    fn solve(&mut self) -> Result<()> {
        #[derive(Clone)]
        struct Monkey {
            items: Vec<u64>,
            op: Op,
            divisor: u64,
            throw_to: [usize; 2],
            items_inspected: u64,
        }

        #[derive(Clone, Copy)]
        enum Op {
            Add(u64),
            Mul(u64),
            Square,
        }

        let mut lines = self.input.lines();
        let mut monkeys = Vec::<Monkey>::new();

        while let Some(_monkey) = lines.next() {
            let (_, items) = lines.next().ok()?.split_once(": ").ok()?;
            let items = items
                .split(", ")
                .map(u64::from_str)
                .collect::<Result<Vec<_>, _>>()?;

            let (_, op) = lines.next().ok()?.split_once("old ").ok()?;
            let op = match op.split_once(' ').ok()? {
                ("+", imm) => Op::Add(imm.parse()?),
                ("*", "old") => Op::Square,
                ("*", imm) => Op::Mul(imm.parse()?),
                _ => err!(),
            };

            let mut rextract = || Some(lines.next()?.rsplit_once(' ')?.1);

            let divisor = rextract().ok()?.parse()?;
            let throw_to_if_true = rextract().ok()?.parse()?;
            let throw_to_if_false = rextract().ok()?.parse()?;

            monkeys.push(Monkey {
                items,
                op,
                divisor,
                throw_to: [throw_to_if_false, throw_to_if_true],
                items_inspected: 0,
            });

            let _blank = lines.next();
        }

        fn play_round(monkeys: &mut [Monkey], reduce_item: impl Fn(u64) -> u64) {
            for i in 0..monkeys.len() {
                let items_len = monkeys[i].items.len();
                monkeys[i].items_inspected += items_len as u64;

                for item_i in 0..items_len {
                    let monkey = &monkeys[i];
                    let mut item = monkey.items[item_i];
                    match monkey.op {
                        Op::Add(imm) => item += imm,
                        Op::Mul(imm) => item *= imm,
                        Op::Square => item *= item,
                    }

                    let item = reduce_item(item);

                    let divisible = item % monkey.divisor == 0;

                    let to = monkey.throw_to[divisible as usize];
                    monkeys[to].items.push(item);
                }
                monkeys[i].items.clear();
            }
        }

        fn monkey_business_level(monkeys: &[Monkey]) -> u64 {
            let mut inspected: Vec<_> = monkeys.iter().map(|m| m.items_inspected).collect();
            inspected.select_nth_unstable_by(1, |a, b| b.cmp(a));
            inspected[0] * inspected[1]
        }

        ensure!(monkeys.len() > 2);
        for (i, monkey) in monkeys.iter().enumerate() {
            ensure!(monkey
                .throw_to
                .iter()
                .all(|&to| to != i && to < monkeys.len()));
        }
        let divisors_prod: u64 = monkeys.iter().map(|m| m.divisor).product();

        let mut relieving_monkeys = monkeys.clone();
        for _ in 0..20 {
            play_round(&mut relieving_monkeys, |item| item / 3);
        }
        let ans1 = monkey_business_level(&relieving_monkeys);

        for _ in 0..10000 {
            play_round(&mut monkeys, |item| item % divisors_prod);
        }
        let ans2 = monkey_business_level(&monkeys);

        self.output(ans1);
        self.output(ans2);
        Ok(())
    }
}

impl Solution<2022, 12> for Puzzle {
    fn solve(&mut self) -> Result<()> {
        let width = self.input.lines().next().ok()?.len();

        let mut map = Vec::with_capacity(width * width);
        let mut start = None;
        let mut end = None;

        let mut y = 0;
        for line in self.input.lines().map(str::as_bytes) {
            ensure!(line.len() == width);
            if start.is_none() {
                start = line.iter().position(|&b| b == b'S').zip(Some(y));
            }
            if end.is_none() {
                end = line.iter().position(|&b| b == b'E').zip(Some(y));
            }
            map.extend_from_slice(line);
            y += 1;
        }

        let height = y;
        let (start, end) = start.zip(end).ok()?;

        // We're going in reverse!
        fn bfs_min_steps(
            map: &mut [u8],
            width: usize,
            height: usize,
            queue: &mut VecDeque<((usize, usize), u8, u32)>,
            is_destination: impl Fn((usize, usize), u8) -> bool,
        ) -> Option<u32> {
            const DELTAS: [(isize, isize); 4] = [(0, 1), (0, -1), (1, 0), (-1, 0)];

            while let Some((cur_pos, cur_h, steps)) = queue.pop_front() {
                if is_destination(cur_pos, cur_h) {
                    // Push this state back so we can reuse the queue.
                    queue.push_front((cur_pos, cur_h, steps));
                    return Some(steps);
                }

                for (dx, dy) in DELTAS {
                    let next_x = cur_pos.0.wrapping_add(dx as usize);
                    let next_y = cur_pos.1.wrapping_add(dy as usize);

                    if next_x >= width || next_y >= height {
                        continue;
                    }
                    let next_h = &mut map[next_y * width + next_x];

                    if *next_h >= cur_h - 1 {
                        queue.push_back(((next_x, next_y), *next_h, steps + 1));
                        *next_h = 0;
                    }
                }
            }
            None
        }

        let mut queue = VecDeque::with_capacity(width * height);
        queue.push_back((end, b'z', 0));

        map[end.1 * width + end.0] = 0;
        map[start.1 * width + start.0] = b'a';

        let ans2 = bfs_min_steps(&mut map, width, height, &mut queue, |_, h| h == b'a');
        let ans1 = bfs_min_steps(&mut map, width, height, &mut queue, |pos, _| pos == start);

        self.output(ans1.ok()?);
        self.output(ans2.ok()?);
        Ok(())
    }
}
