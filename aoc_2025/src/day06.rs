use std::str::FromStr;

#[derive(Debug, thiserror::Error)]
pub enum Error {}

pub fn task1(input: String) -> Result<u64, Error> {
    let mut lines = input.lines().collect::<Vec<_>>();
    let mut operators = lines[lines.len() - 1].split_ascii_whitespace();
    let mut inputs = lines
        .drain(..lines.len() - 1)
        .map(str::split_ascii_whitespace)
        .collect::<Vec<_>>();

    let mut total = 0;
    loop {
        let Some(op) = operators.next() else {
            break;
        };
        match op {
            "+" => {
                let mut sum = 0;
                for cursor in &mut inputs {
                    sum += u64::from_str(cursor.next().unwrap()).unwrap();
                }
                total += sum;
            }
            "*" => {
                let mut product = 1;
                for cursor in &mut inputs {
                    product *= u64::from_str(cursor.next().unwrap()).unwrap();
                }
                total += product;
            }
            _ => unreachable!(),
        }
    }

    Ok(total)
}

pub fn task2(input: String) -> Result<u64, Error> {
    debug_assert!(input.is_ascii());
    let mut lines = input.lines().collect::<Vec<_>>();
    let operators = lines.remove(lines.len() - 1).as_bytes();
    let values = lines.into_iter().map(str::as_bytes).collect::<Vec<_>>();
    let mut total = 0;
    let mut cursor = 0;
    let max_line_len = values.iter().map(|row| row.len()).max().unwrap();
    while cursor < max_line_len {
        // Starting at the cursor, in each line, find the longest sequence of non-spaces
        // "123 " -> 3
        // " 45 " -> 0
        // "  6 " -> 0
        let number_len = values
            .iter()
            .map(|v| v[cursor..].iter().take_while(|c| **c != b' ').count())
            .max()
            .unwrap();

        // Construct the number top to bottom, converting the ascii codes to numbers
        let mut sum = 0;
        let mut product = 1;
        for i in 0..number_len {
            let mut number = 0;
            for row in &values {
                let digit = match row.get(cursor + i) {
                    None | Some(b' ') => continue,
                    Some(digit) => *digit - b'0',
                };
                number = number * 10 + digit as u64;
            }
            sum += number;
            product *= number;
        }

        total += if operators[cursor] == b'+' {
            sum
        } else {
            product
        };
        cursor += number_len + 1;
    }
    Ok(total)
}

#[cfg(test)]
mod tests {
    use super::*;
    use common::input::Input;

    const INPUT: &[u8] = b"\
123 328  51 64
 45 64  387 23
  6 98  215 314
*   +   *   +  ";

    #[test]
    fn test_task1() {
        let buf = std::io::BufReader::new(INPUT);
        let result = task1(Input::parse(buf).unwrap());
        let val = result.unwrap();
        assert_eq!(val, 4277556);
    }
    #[test]
    fn test_task2() {
        let buf = std::io::BufReader::new(INPUT);
        let result = task2(Input::parse(buf).unwrap());
        let val = result.unwrap();
        assert_eq!(val, 3263827);
    }
}
