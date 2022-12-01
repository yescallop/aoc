use crate::{error::Track, Puzzle, Result, Solution};

macro_rules! err {
    () => {
        None.ok()?
    };
}

macro_rules! ensure {
    ($cond:expr) => {
        if (!$cond) {
            err!();
        }
    };
}

mod year_2021;
mod year_2022;
