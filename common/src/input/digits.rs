use std::str::FromStr;

use super::Input;

#[derive(Debug)]
pub struct DigitMap<T>(Vec<Vec<T>>);

impl<T> DigitMap<T> {
    pub fn into_inner(self) -> Vec<Vec<T>> {
        self.0
    }
}

impl<'a, T: FromStr> Input<'a> for DigitMap<T>
where
    T::Err: 'static + std::error::Error,
{
    type Error = T::Err;

    fn parse<R: 'a + std::io::BufRead>(mut read: R) -> Result<Self, Self::Error> {
        let mut buf = String::new();
        let mut lines = Vec::new();
        loop {
            // try to read the next line
            let count = read.read_line(&mut buf).unwrap();
            if count == 0 {
                break;
            }

            // trim trailing whitespace and ensure we have only digits
            let s = buf.trim_end();
            debug_assert!(
                s.chars().all(|c| char::is_ascii_digit(&c)),
                "Line contains non-digit characters"
            );

            // parse the digits
            let mut line = Vec::with_capacity(s.len());
            for i in 0..s.len() {
                let char = &s[i..=i];
                line.push(T::from_str(char)?);
            }

            lines.push(line);
            buf.clear();
        }

        Ok(Self(lines))
    }
}
