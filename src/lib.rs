pub mod dynamic;

mod error;
pub use error::Error;

pub type Result<T, E = Error> = std::result::Result<T, E>;

mod solution;
mod str_ext;

use std::{fmt, fs, path::Path};

use curl::easy::Easy;

pub trait Solution<const YEAR: u32, const DAY: u32> {
    fn solve(&mut self) -> Result<()>;
}

#[readonly::make]
pub struct Puzzle {
    pub input: String,
    output: Vec<String>,
}

impl Puzzle {
    fn new(input: String) -> Puzzle {
        Puzzle {
            input,
            output: vec![],
        }
    }

    pub fn output<T: fmt::Display>(&mut self, t: T) {
        self.output.push(t.to_string())
    }
}

impl fmt::Display for Puzzle {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for (i, x) in self.output.iter().enumerate() {
            writeln!(f, "out[{i}] = {x}")?;
        }
        Ok(())
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
