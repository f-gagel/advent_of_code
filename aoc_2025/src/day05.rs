use common::input::{LineSeparated, Linewise};
use common::iter_ext::TryIterator;
use std::cmp::{max, min};
use std::num::ParseIntError;
use std::ops::Range;
use std::str::FromStr;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Invalid format for input range")]
    InvalidRange,
    #[error(transparent)]
    ParseInt(#[from] ParseIntError),
}

pub struct InputRange(Range<u64>);

impl FromStr for InputRange {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (a, b) = s.split_once('-').ok_or(Error::InvalidRange)?;
        let from = u64::from_str(a)?;
        let to = u64::from_str(b)?;
        // transform inclusive range to exclusive
        Ok(Self(from..(to + 1)))
    }
}

pub fn task1<'a>(
    input: LineSeparated<'a, Linewise<'static, InputRange>, Linewise<'a, u64>>,
) -> Result<u64, Error> {
    let (ranges, ids) = input.into_inner();
    let mut ranges = ranges.try_collect2::<Vec<_>>()?;
    ranges.sort_by_key(|r| r.0.start);

    let mut invalid = 0;
    for item in ids {
        let id = item?;
        invalid += ranges.iter().any(|r| r.0.contains(&id)) as u64;
    }

    Ok(invalid)
}

pub fn task2<'a>(
    input: LineSeparated<'a, Linewise<'static, InputRange>, Linewise<'a, u64>>,
) -> Result<u64, Error> {
    let (ranges, _ids) = input.into_inner();
    let mut add_ranges = Vec::<Range<u64>>::new();
    let mut new_add_ranges = Vec::<Range<u64>>::new();
    let mut sub_ranges = Vec::<Range<u64>>::new();
    let mut new_sub_ranges = Vec::<Range<u64>>::new();

    for range in ranges {
        let range = range?.0;

        for other in &add_ranges {
            let overlap_start = max(range.start, other.start);
            let overlap_end = min(range.end, other.end);
            let overlap = overlap_start..overlap_end;
            if !overlap.is_empty() {
                new_sub_ranges.push(overlap)
            }
        }
        for other in &sub_ranges {
            let overlap_start = max(range.start, other.start);
            let overlap_end = min(range.end, other.end);
            let overlap = overlap_start..overlap_end;
            if !overlap.is_empty() {
                new_add_ranges.push(overlap)
            }
        }

        add_ranges.push(range);
        add_ranges.extend(new_add_ranges.drain(..));
        sub_ranges.extend(new_sub_ranges.drain(..));
    }

    let add = add_ranges.iter().map(|r| r.end - r.start).sum::<u64>();
    let sub = sub_ranges.iter().map(|r| r.end - r.start).sum::<u64>();
    Ok(add - sub)
}

#[cfg(test)]
mod tests {
    use super::*;
    use common::input::Input;

    const INPUT: &[u8] = b"\
3-5
10-14
16-20
12-18

1
5
8
11
17
32";

    #[test]
    fn test_task1() {
        let buf = std::io::BufReader::new(INPUT);
        let result = task1(Input::parse(buf).unwrap());
        let val = result.unwrap();
        assert_eq!(val, 3);
    }
    #[test]
    fn test_task2() {
        let buf = std::io::BufReader::new(INPUT);
        let result = task2(Input::parse(buf).unwrap());
        let val = result.unwrap();
        assert_eq!(val, 14);
    }
}
