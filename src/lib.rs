pub mod dynamic;

mod year_2021;

use std::{fmt, fs, io, num::ParseIntError, path::Path, str::FromStr, string::FromUtf8Error};

use curl::easy::Easy;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("io: {0}")]
    Io(#[from] io::Error),
    #[error("curl: {0}")]
    Curl(#[from] curl::Error),
    #[error("solution not found")]
    SolutionNotFound,
}

macro_rules! impl_error_from {
    ($($ty:ty),+) => {
        $(impl From<$ty> for Error {
            fn from(e: $ty) -> Error {
                Error::Io(io::Error::new(io::ErrorKind::InvalidData, e))
            }
        })+
    };
}

impl_error_from!(FromUtf8Error, ParseIntError);

pub type Result<T, E = Error> = std::result::Result<T, E>;

pub trait Solution<const YEAR: u32, const DAY: u32> {
    fn solve(&mut self) -> Result<()>;
}

pub struct Puzzle {
    input: String,
    output: Vec<String>,
}

impl Puzzle {
    fn new(input: String) -> Puzzle {
        Puzzle {
            input,
            output: vec![],
        }
    }

    pub fn input(&self) -> &str {
        &self.input
    }

    pub fn input_lines(&self) -> impl Iterator<Item = &str> {
        self.input.lines()
    }

    pub fn input_vec<T: FromStr>(&self) -> Result<Vec<T>, T::Err> {
        self.input.lines().map(|l| l.parse()).collect()
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
