pub mod dynamic;

mod error;
pub use error::Error;

pub type Result<T, E = Error> = std::result::Result<T, E>;

mod solution;
mod str_ext;

use std::{cell::Cell, fmt, fs, path::Path};

use curl::easy::Easy;

pub trait Solution<const YEAR: u32, const DAY: u32> {
    fn solve(&self) -> Result<()>;
}

pub struct Puzzle {
    input: String,
    out_index: Cell<u32>,
}

impl Puzzle {
    fn new(input: String) -> Puzzle {
        Puzzle {
            input,
            out_index: Cell::new(0),
        }
    }

    fn output<T: fmt::Display>(&self, t: T) {
        let index = self.out_index.get();
        self.out_index.set(index + 1);
        println!("out[{index}] = {t}");
    }
}

pub fn fetch_puzzle(year: u32, day: u32) -> Result<Puzzle> {
    let path = format!("cache/{year}/{day}.txt");
    let path = Path::new(&path);

    if let Ok(data) = fs::read_to_string(path) {
        Ok(Puzzle::new(data))
    } else {
        let mut data = Vec::new();
        let mut handle = Easy::new();

        handle.url(&format!("https://adventofcode.com/{year}/day/{day}/input"))?;

        let cookie = fs::read_to_string("cookie.txt")?;
        handle.cookie(cookie.trim_end())?;

        let mut transfer = handle.transfer();
        transfer.write_function(|new_data| {
            data.extend_from_slice(new_data);
            Ok(new_data.len())
        })?;
        transfer.perform()?;
        drop(transfer);

        fs::create_dir_all(path.parent().unwrap())?;
        fs::write(path, &data)?;

        Ok(Puzzle::new(String::from_utf8(data)?))
    }
}
