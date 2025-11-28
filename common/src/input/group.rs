use super::Input;
use std::convert::Infallible;
use std::io::BufRead;
use std::iter::*;
use std::marker::PhantomData;
use std::mem::MaybeUninit;
use std::str::FromStr;

pub fn parse_lines<E>(
    reader: &mut impl BufRead,
    mut f: impl FnMut(&str) -> Result<(), E>,
) -> Result<(), E>
where
    E: std::error::Error + From<std::io::Error>,
{
    let mut buf = String::with_capacity(256);
    while reader.read_line(&mut buf)? > 0 {
        let s = buf.trim();
        f(s)?;
        buf.clear();
    }
    Ok(())
}

pub struct Chunked<'a, T: FromStr, const N: usize, const PADDED: bool> {
    read: Box<dyn 'a + BufRead>,
    string: String,
    _t: PhantomData<T>,
}

impl<'a, T: FromStr, const N: usize, const PADDED: bool> Input<'a> for Chunked<'a, T, N, PADDED> {
    type Error = Infallible;

    fn parse<R: 'a + BufRead>(read: R) -> Result<Self, Self::Error> {
        Ok(Self {
            read: Box::new(read),
            string: String::with_capacity(256),
            _t: PhantomData::default(),
        })
    }
}

impl<T: FromStr, const N: usize, const PADDED: bool> Iterator for Chunked<'_, T, N, PADDED> {
    type Item = Result<[T; N], T::Err>;

    fn next(&mut self) -> Option<Self::Item> {
        let mut array: [MaybeUninit<T>; N] = std::array::from_fn(|_| MaybeUninit::uninit());
        for i in 0..N {
            let read = self.read.read_line(&mut self.string).unwrap();
            if read == 0 {
                return None;
            }
            let res = T::from_str(self.string.trim());
            self.string.clear();
            let t = match res {
                Ok(t) => t,
                Err(e) => return Some(Err(e)),
            };
            array[i].write(t);
        }

        if PADDED {
            let _ = self.read.read_line(&mut self.string);
            self.string.clear();
        }

        Some(Ok(array.map(|x| unsafe { MaybeUninit::assume_init(x) })))
    }
}

pub struct Grouped<'a, T: FromStr> {
    read: Box<dyn 'a + BufRead>,
    string: String,
    _t: PhantomData<T>,
}

impl<'a, T: FromStr> Input<'a> for Grouped<'a, T> {
    type Error = Infallible;

    fn parse<R: 'a + BufRead>(read: R) -> Result<Self, Self::Error> {
        Ok(Self {
            read: Box::new(read),
            string: String::with_capacity(256),
            _t: PhantomData::default(),
        })
    }
}

impl<T: FromStr> Iterator for Grouped<'_, T> {
    type Item = Result<Vec<T>, T::Err>;

    fn next(&mut self) -> Option<Self::Item> {
        let mut vec = Vec::new();
        loop {
            let read = self.read.read_line(&mut self.string).unwrap();
            let trimmed = self.string.trim();
            if read == 0 || trimmed.len() == 0 {
                break;
            }

            let res = T::from_str(trimmed);
            self.string.clear();
            let t = match res {
                Ok(t) => t,
                Err(e) => return Some(Err(e)),
            };
            vec.push(t);
        }

        if vec.is_empty() { None } else { Some(Ok(vec)) }
    }
}
