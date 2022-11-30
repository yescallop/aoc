use crate::{error::Track, Puzzle, Result, Solution};

macro_rules! err {
    () => {
        None.ok()?
    };
}

mod year_2021;
