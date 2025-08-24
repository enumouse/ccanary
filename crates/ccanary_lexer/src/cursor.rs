use std::str::Chars;

/// Character used to indicate end of file
pub const EOF_CHAR: char = '\0';

/// A cursor that can 'peek' and shift over a sequence of chars.
pub struct Cursor<'a> {
    chars: Chars<'a>,
    prev: char,
}

impl<'a> Cursor<'a> {
    /// Create a cursor from an input string
    pub fn new(input: &'a str) -> Cursor<'a> {
        Self {
            chars: input.chars(),
            prev: EOF_CHAR,
        }
    }

    pub fn first(&self) -> char {
        self.chars.clone().next().unwrap_or(EOF_CHAR)
    }

    pub fn second(&self) -> char {
        let mut iter = self.chars.clone();
        iter.next();
        iter.next().unwrap_or(EOF_CHAR)
    }

    pub fn third(&self) -> char {
        let mut iter = self.chars.clone();
        iter.next();
        iter.next();
        iter.next().unwrap_or(EOF_CHAR)
    }

    pub fn bump(&mut self) -> Option<char> {
        let c = self.chars.next()?;
        Some(c)
    }
}
