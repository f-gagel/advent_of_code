use std::{
    convert::Infallible,
    error::Error,
    fmt::Display,
    io::{BufRead, Cursor},
    marker::PhantomData,
    str::FromStr,
};

use super::Input;

pub type CommaSeparated<'a, T> = CharSeparated<'a, T, ','>;
pub type SpaceSeparated<'a, T> = CharSeparated<'a, T, ' '>;

pub struct CharSeparated<'a, T: 'a + FromStr, const C: char> {
    input: Box<dyn 'a + BufRead>,
    buffer: String,
    cursor: usize,
    _t: PhantomData<T>,
}

impl<'a, T: FromStr, const C: char> FromStr for CharSeparated<'a, T, C> {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self {
            input: Box::new([0u8; 0].as_slice()),
            buffer: s.to_string(),
            cursor: 0,
            _t: PhantomData,
        })
    }
}

impl<'a, T: 'a + FromStr, const C: char> Input<'a> for CharSeparated<'a, T, C> {
    type Error = Infallible;

    fn parse<R: 'a + BufRead>(read: R) -> Result<Self, Self::Error> {
        Ok(Self {
            input: Box::new(read),
            buffer: String::new(),
            cursor: 0,
            _t: PhantomData,
        })
    }
}

impl<'a, T: 'a + FromStr, const C: char> Iterator for CharSeparated<'a, T, C> {
    type Item = Result<T, T::Err>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.cursor >= self.buffer.len() {
            self.buffer.clear();
            self.cursor = 0;
            let n = self.input.read_line(&mut self.buffer).unwrap();
            if n == 0 {
                return None;
            }
        }

        let read = &self.buffer[self.cursor..];
        let len = read
            .chars()
            .take_while(|c| {
                debug_assert!(c.is_ascii(), "Cannot handle non ascii input");
                *c != C
            })
            .count();

        let start = self.cursor;
        let end = self.cursor + len;
        // advance the cursor PAST the separator
        self.cursor += len + 1;

        Some(T::from_str(&self.buffer[start..end]))
    }
}

#[derive(Debug)]
pub struct LineSeparatedError(Box<dyn Error>);

impl Display for LineSeparatedError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Error in LineSeparated: {}", self.0)
    }
}

impl Error for LineSeparatedError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        Some(self.0.as_ref())
    }
}

pub struct LineSeparated<'a, A: Input<'static>, B: 'a + Input<'a>>(A, B, PhantomData<&'a ()>);

impl<'a, A: Input<'static>, B: 'a + Input<'a>> LineSeparated<'a, A, B> {
    pub fn into_inner(self) -> (A, B) {
        (self.0, self.1)
    }
}

impl<'a, A: Input<'static>, B: 'a + Input<'a>> Input<'a> for LineSeparated<'a, A, B> {
    type Error = LineSeparatedError;

    fn parse<R: 'a + BufRead>(mut read: R) -> Result<Self, Self::Error> {
        let mut buf = String::new();
        loop {
            let len = read.read_line(&mut buf).unwrap();
            let prev = buf.len() - len;
            let new = &buf[prev..];
            if new.chars().all(|c| c.is_ascii_whitespace()) {
                buf.truncate(prev);
                break;
            }
        }

        let a =
            A::parse(Cursor::new(buf.into_bytes())).map_err(|e| LineSeparatedError(Box::new(e)))?;
        let b = B::parse(read).map_err(|e| LineSeparatedError(Box::new(e)))?;

        Ok(Self(a, b, PhantomData))
    }
}
