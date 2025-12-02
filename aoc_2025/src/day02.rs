use std::collections::HashSet;
use std::num::ParseIntError;
use std::str::FromStr;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    ParseInt(#[from] ParseIntError),
}

pub fn task1(input: String) -> Result<u64, Error> {
    let mut invalid_sum = 0;
    for part in input.trim().split(',') {
        let (a, b) = part.split_once('-').unwrap();
        let from = u64::from_str(a)?;
        let to = u64::from_str(b)?;

        let from_digits = from.ilog10() + 1;
        let to_digits = to.ilog10() + 1;

        if from_digits == to_digits {
            invalid_sum += find_invalid_numbers_in(from, to);
        } else {
            let midpoint = 10u64.pow(to_digits - 1);
            invalid_sum += find_invalid_numbers_in(from, midpoint - 1);
            invalid_sum += find_invalid_numbers_in(midpoint, to);
        }
    }
    return Ok(invalid_sum);

    #[inline]
    fn find_invalid_numbers_in(from: u64, to: u64) -> u64 {
        let digits = from.ilog10() + 1;
        if digits % 2 == 1 {
            return 0;
        }

        let step = 10u64.pow(digits / 2) + 1;
        let invalid_start = from.div_ceil(step);
        let invalid_end = to / step;
        (invalid_start..=invalid_end).map(|n| step * n).sum()
    }
}

pub fn task2(input: String) -> Result<u64, Error> {
    let mut invalid_sum = 0;
    for part in input.trim().split(',') {
        let (a, b) = part.split_once('-').unwrap();
        let from = u64::from_str(a)?;
        let to = u64::from_str(b)?;

        let from_digits = from.ilog10() + 1;
        let to_digits = to.ilog10() + 1;

        if from_digits == to_digits {
            invalid_sum += find_invalid_numbers_in(from, to);
        } else {
            let midpoint = 10u64.pow(to_digits - 1);
            invalid_sum += find_invalid_numbers_in(from, midpoint - 1);
            invalid_sum += find_invalid_numbers_in(midpoint, to);
        }
    }
    return Ok(invalid_sum);

    #[inline]
    fn find_invalid_numbers_in(from: u64, to: u64) -> u64 {
        let digits = from.ilog10() + 1;

        // Precompute powers of 10 up to 10 digits
        let mut pow10: [u64; 11] = [1; 11];
        for i in 1..=10 {
            pow10[i] = pow10[i - 1] * 10;
        }

        let mut seen = HashSet::new();

        // block with LEN digits should repeat REPS times
        for len in 1..=(digits / 2) {
            if digits % len != 0 {
                continue;
            }
            let reps = digits / len;
            if reps < 2 {
                continue;
            }

            // multiplier M(len, reps) = 1 + 10^len + 10^(2*len) + ...
            let mut mul: u64 = 0;
            let mut factor: u64 = 1;
            let base10_len = 10u64.pow(len);
            for _ in 0..reps {
                mul += factor;
                factor *= base10_len;
            }

            let start = from.div_ceil(mul);
            let end = to / mul;

            for invalid in start..=end {
                seen.insert(invalid * mul);
            }
        }

        seen.into_iter().sum()
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use common::input::Input;

    const INPUT: &[u8] = b"11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";

    #[test]
    fn test_task1() {
        let buf = std::io::BufReader::new(INPUT);
        let result = task1(Input::parse(buf).unwrap());
        let val = result.unwrap();
        assert_eq!(val, 1227775554);
    }
    #[test]
    fn test_task2() {
        let buf = std::io::BufReader::new(INPUT);
        let result = task2(Input::parse(buf).unwrap());
        let val = result.unwrap();
        assert_eq!(val, 4174379265);
    }
}
