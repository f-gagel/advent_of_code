use common::input::Linewise;
use std::num::ParseIntError;
use std::str::FromStr;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    ParseInt(#[from] ParseIntError),
}

pub fn task1(input: Linewise<String>) -> Result<u32, Error> {
    let mut zeros = 0;
    let mut position = 50;
    for line in input {
        let line = line.unwrap();
        let (a, b) = line.split_at(1);
        let mut change = i32::from_str(b)?;
        if a == "L" {
            change *= -1;
        }

        position = (position + change).rem_euclid(100);
        zeros += (position == 0) as u32;
    }
    Ok(zeros)
}

fn div_floor(base: i32, div: i32) -> i32 {
    let mut result = base / div;
    if base < 0 && base % div != 0 {
        result -= 1;
    }
    result
}

pub fn task2(input: Linewise<String>) -> Result<i32, Error> {
    let mut clicks = 0;
    let mut position = 50;
    for line in input {
        let line = line.unwrap();
        let (a, b) = line.split_at(1);
        let mut change = i32::from_str(b)?;
        if a == "L" {
            change *= -1;
        }

        let start = position;
        let end = position + change;

        clicks += if change > 0 {
            // multiples of 100 in (start, end]
            div_floor(end, 100) - div_floor(start, 100)
        } else {
            // multiples of 100 in [end, start)
            div_floor(start - 1, 100) - div_floor(end - 1, 100)
        };

        position = (position + change).rem_euclid(100);
    }
    Ok(clicks)
}

#[cfg(test)]
mod tests {
    use super::*;
    use common::input::Input;

    const INPUT: &[u8] = b"\
L68
L30
R48
L5
R60
L55
L1
L99
R14
L82
";

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
        assert_eq!(val, 6);
    }
}
