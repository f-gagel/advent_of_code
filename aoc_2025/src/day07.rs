use common::input::Linewise;
use common::iter_ext::TryIterator;
use std::collections::HashMap;
use std::mem::swap;

#[derive(Debug, thiserror::Error)]
pub enum Error {}

pub fn task1(mut input: Linewise<String>) -> Result<i32, Error> {
    let mut active_columns = vec![];
    let start = input.next().unwrap().unwrap().find('S').unwrap();
    active_columns.push(start);
    let mut splits = 0;
    let mut next_columns = vec![];

    for line in input {
        let line = line.unwrap();
        let bytes = line.as_bytes();
        for col in active_columns.drain(..) {
            match bytes[col] {
                b'.' => insert_if_new(&mut next_columns, col),
                b'^' => {
                    insert_if_new(&mut next_columns, col - 1);
                    insert_if_new(&mut next_columns, col + 1);
                    splits += 1;
                }
                _ => unreachable!(),
            }
        }
        swap(&mut next_columns, &mut active_columns);
    }

    Ok(splits)
}

pub fn insert_if_new(vec: &mut Vec<usize>, num: usize) {
    if let Err(idx) = vec.binary_search(&num) {
        vec.insert(idx, num)
    }
}

pub fn task2(input: Linewise<String>) -> Result<u64, Error> {
    let lines = input.try_collect2::<Vec<_>>().unwrap();
    let map = lines.iter().map(String::as_bytes).collect::<Vec<_>>();
    let mut cache = HashMap::new();
    let start = map[0].iter().position(|c| *c == b'S').unwrap();
    let timelines = recurse_map(&map, start, 1, &mut cache);
    Ok(timelines)
}

fn recurse_map(
    map: &[&[u8]],
    x: usize,
    y: usize,
    cache: &mut HashMap<(usize, usize), u64>,
) -> u64 {
    if let Some(result) = cache.get(&(x, y)) {
        return *result;
    }

    let mut mov_y = y;
    let result = loop {
        let Some(row) = map.get(mov_y) else {
            break 1;
        };
        let Some(char) = row.get(x) else {
            break 1;
        };

        match *char {
            b'.' => mov_y += 1,
            b'^' => {
                let mut split = 0;
                split += recurse_map(map, x - 1, mov_y + 1, cache);
                split += recurse_map(map, x + 1, mov_y + 1, cache);
                break split;
            }
            _ => unreachable!(),
        }
    };
    cache.insert((x, y), result);
    result
}

#[cfg(test)]
mod tests {
    use super::*;
    use common::input::Input;

    const INPUT: &[u8] = b"\
.......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
...............";

    #[test]
    fn test_task1() {
        let buf = std::io::BufReader::new(INPUT);
        let result = task1(Input::parse(buf).unwrap());
        let val = result.unwrap();
        assert_eq!(val, 21);
    }
    #[test]
    fn test_task2() {
        let buf = std::io::BufReader::new(INPUT);
        let result = task2(Input::parse(buf).unwrap());
        let val = result.unwrap();
        assert_eq!(val, 40);
    }
}
