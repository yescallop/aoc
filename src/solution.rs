use super::{Puzzle, Result, Solution};

macro_rules! err_msg {
    () => {
        concat!(
            "failure at ",
            file!(),
            ":",
            line!(),
            ":",
            column!(),
        )
    };
}

macro_rules! err {
    () => {
        return Err(crate::Error::from(err_msg!()))
    };
}

macro_rules! ok {
    ($opt:expr) => {
        $opt.ok_or(crate::Error::from(err_msg!()))?
    };
}

mod year_2021;
