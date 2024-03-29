use super::*;
use std::{convert::Infallible, marker::PhantomData, str::FromStr};

/// Adapter iterator reading from an underlying stream converting each line individually.
/// [`Iterator::next()`] may yield a [`Result::Err`] after which further iteration may become unstable.
/// (Though this will never lead to UB)
pub struct Linewise<'a, T: FromStr> {
    read: Box<dyn 'a + BufRead>,
    string: String,
    _t: PhantomData<T>,
}

impl<'a, T: FromStr> Input<'a> for Linewise<'a, T> {
    type Error = Infallible;

    fn parse<R: 'a + BufRead>(read: R) -> Result<Self, Self::Error> {
        Ok(Self {
            read: Box::new(read),
            string: String::with_capacity(256),
            _t: PhantomData::default(),
        })
    }
}

impl<T: FromStr> Iterator for Linewise<'_, T> {
    type Item = Result<T, T::Err>;

    fn next(&mut self) -> Option<Self::Item> {
        let read = self.read.read_line(&mut self.string).unwrap();
        if read == 0 {
            return None;
        }
        let t = T::from_str(self.string.trim());
        self.string.clear();
        return Some(t);
    }
}

pub struct Multiline<'a, T: FromStr, const N: usize, const PADDED: bool> {
    read: Box<dyn 'a + BufRead>,
    string: String,
    _t: PhantomData<T>,
}

impl<'a, T: FromStr, const N: usize, const PADDED: bool> Input<'a> for Multiline<'a, T, N, PADDED> {
    type Error = Infallible;

    fn parse<R: 'a + BufRead>(read: R) -> Result<Self, Self::Error> {
        Ok(Self {
            read: Box::new(read),
            string: String::with_capacity(256),
            _t: PhantomData::default(),
        })
    }
}

impl<T: FromStr, const N: usize, const PADDED: bool> Iterator for Multiline<'_, T, N, PADDED> {
    type Item = Result<T, T::Err>;

    fn next(&mut self) -> Option<Self::Item> {
        self.string.clear();
        for _ in 0..N {
            let read = self.read.read_line(&mut self.string).unwrap();
            if read == 0 {
                return None;
            }
        }

        let res = T::from_str(self.string.trim());

        if PADDED {
            let _ = self.read.read_line(&mut self.string);
            self.string.clear();
        }

        Some(res)
    }
}
