use crate::{Puzzle, Result, Solution};

use once_cell::sync::OnceCell;

static SOLS: OnceCell<Vec<DynSolution>> = OnceCell::new();

pub struct DynSolution {
    pub year: u32,
    pub day: u32,
    pub solve: fn(&mut Puzzle) -> Result<()>,
}

struct Helper<const Y: u32, const D: u32>;

trait Blanket {
    fn push_sol(&self, _: &mut Vec<DynSolution>) {}
}

impl<T> Blanket for T {}

impl<const Y: u32, const D: u32> Helper<Y, D>
where
    Puzzle: Solution<Y, D>,
{
    fn push_sol(&self, vec: &mut Vec<DynSolution>) {
        vec.push(DynSolution {
            year: Y,
            day: D,
            solve: Solution::solve,
        })
    }
}

macro_rules! push_sols {
    ($vec:expr, [$($year:expr),+]) => {
        $(push_sols!($vec, $year,
            1, 2, 3, 4, 5,
            6, 7, 8, 9, 10,
            11, 12, 13, 14, 15,
            16, 17, 18, 19, 20,
            21, 22, 23, 24, 25
        ));+
    };
    ($vec:expr, $year:expr, $($day:expr),+) => {
        $(Helper::<$year, $day>.push_sol(&mut $vec));+
    }
}

pub fn solutions() -> &'static [DynSolution] {
    SOLS.get_or_init(|| {
        let mut vec = vec![];
        push_sols!(vec, [2021, 2022]);
        vec
    })
}

fn parse_args() -> Option<Option<(u32, Option<u32>)>> {
    let mut args = std::env::args();
    args.next();

    let Some(year) = args.next() else {
        return Some(None);
    };
    let year = year.parse().ok()?;

    let Some(day) = args.next() else {
        return Some(Some((year, None)));
    };
    let day = day.parse().ok()?;

    Some(Some((year, Some(day))))
}

pub fn solution_by_args() -> Option<&'static DynSolution> {
    let sols = solutions();

    match parse_args()? {
        Some((year, Some(day))) => sols.iter().find(|s| (s.year, s.day) == (year, day)),
        Some((day, None)) if day <= 25 => sols.iter().rfind(|s| s.day == day),
        Some((year, None)) => sols.iter().rfind(|s| s.year == year),
        None => sols.last(),
    }
}
