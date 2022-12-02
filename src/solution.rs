macro_rules! err {
    () => {
        return None.ok()
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

use std::str::FromStr;

use crate::{error::Track, str_ext::StrExt, Puzzle, Result, Solution};
