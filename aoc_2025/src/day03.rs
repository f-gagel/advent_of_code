use common::input::Linewise;

#[derive(Debug, thiserror::Error)]
pub enum Error {}

pub fn task1(input: Linewise<String>) -> Result<u32, Error> {
    let mut total = 0;
    for line in input {
        let line = line.unwrap();
        debug_assert!(line.is_ascii());

        let bytes = line.as_bytes();
        let last = bytes.len() - 1;
        let (tenner, tenner_idx) = max_with_index(&bytes[..last]);
        let (onner, _) = max_with_index(&bytes[(tenner_idx + 1)..]);
        total += (tenner as u32 * 10) + (onner as u32);
    }
    Ok(total)
}

fn max_with_index<'a>(iter: impl IntoIterator<Item = &'a u8>) -> (u8, usize) {
    let mut iter = iter.into_iter().enumerate();
    let (mut max_idx, mut max_val) = iter.next().expect("used empty range");

    while let Some((idx, val)) = iter.next() {
        if val > max_val {
            max_val = val;
            max_idx = idx;
        }
    }

    // NOTE: values are ascii characters, but we need numeric values
    (max_val - b'0', max_idx)
}

pub fn task2(input: Linewise<String>) -> Result<u64, Error> {
    let mut total = 0;
    for line in input {
        let line = line.unwrap();
        debug_assert!(line.is_ascii());

        let bytes = line.as_bytes();
        let mut start = 0;
        let mut end = bytes.len() - 11;
        let mut joltage = 0;

        for _ in 0..12 {
            let (digit, idx) = max_with_index(&bytes[start..end]);
            joltage = joltage * 10 + digit as u64;
            start = start + idx + 1;
            end += 1;
        }

        total += joltage;
    }
    Ok(total)
}

#[cfg(test)]
mod tests {
    use super::*;
    use common::input::Input;

    const INPUT: &[u8] = b"\
987654321111111
811111111111119
234234234234278
818181911112111";

    #[test]
    fn test_task1() {
        let buf = std::io::BufReader::new(INPUT);
        let result = task1(Input::parse(buf).unwrap());
        let val = result.unwrap();
        assert_eq!(val, 357);
    }
    #[test]
    fn test_task2() {
        let buf = std::io::BufReader::new(INPUT);
        let result = task2(Input::parse(buf).unwrap());
        let val = result.unwrap();
        assert_eq!(val, 3121910778619);
    }
}
