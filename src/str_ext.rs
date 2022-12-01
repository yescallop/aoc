use std::{
    cell::{Cell, RefCell},
    str::{FromStr, Lines},
};

pub trait StrExt {
    fn parse_lines<T: FromStr>(&self) -> Result<Vec<T>, T::Err>;
    fn line_blocks(&self) -> LineBlocks<'_>;
}

impl StrExt for str {
    fn parse_lines<T: FromStr>(&self) -> Result<Vec<T>, T::Err> {
        self.lines().map(T::from_str).collect()
    }

    fn line_blocks(&self) -> LineBlocks<'_> {
        LineBlocks {
            lines: RefCell::new(self.lines()),
            finished: Cell::new(false),
        }
    }
}

pub struct LineBlocks<'a> {
    lines: RefCell<Lines<'a>>,
    finished: Cell<bool>,
}

impl<'bl, 'a> Iterator for &'bl LineBlocks<'a> {
    type Item = LineBlock<'bl, 'a>;
    fn next(&mut self) -> Option<LineBlock<'bl, 'a>> {
        if !self.finished.get() {
            Some(LineBlock { blocks: self })
        } else {
            None
        }
    }
}

pub struct LineBlock<'bl, 'a> {
    blocks: &'bl LineBlocks<'a>,
}

impl<'bl, 'a> Iterator for LineBlock<'bl, 'a> {
    type Item = &'a str;
    fn next(&mut self) -> Option<&'a str> {
        match self.blocks.lines.borrow_mut().next() {
            Some(line) if line.is_empty() => None,
            Some(line) => Some(line),
            None => {
                self.blocks.finished.set(true);
                None
            }
        }
    }
}
