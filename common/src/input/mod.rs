pub use self::{chars::*, group::*, lines::*, separated::*};
use std::io::BufRead;

pub mod chars;
pub mod digits;
pub mod group;
pub mod lines;
pub mod separated;

pub trait Input<'a>: Sized {
    type Error: 'static + std::error::Error;
    fn parse<R: 'a + BufRead>(read: R) -> Result<Self, Self::Error>;
}

impl Input<'_> for String {
    type Error = std::io::Error;
    fn parse<R: BufRead>(mut read: R) -> Result<Self, Self::Error> {
        let mut s = String::new();
        read.read_to_string(&mut s)?;
        let len = s.trim_end().len();
        s.truncate(len);
        Ok(s)
    }
}

impl Input<'_> for () {
    type Error = std::convert::Infallible;

    fn parse<R: BufRead>(_: R) -> Result<Self, Self::Error> {
        Ok(())
    }
}
