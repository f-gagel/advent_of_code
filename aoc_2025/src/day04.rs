use common::input::Linewise;

#[derive(Debug, thiserror::Error)]
pub enum Error {}

pub fn task1(input: Linewise<String>) -> Result<u32, Error> {
    let map = input
        .map(|s| {
            s.unwrap()
                .as_bytes()
                .iter()
                .map(|c| *c == b'@')
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let height = map.len();
    let width = map[0].len();

    let mut result = 0;
    for y in 0..height {
        for x in 0..width {
            if !map[y][x] {
                continue;
            }

            result += is_removable(&map, y, x) as u32;
        }
    }

    Ok(result)
}

fn is_removable(map: &Vec<Vec<bool>>, y: usize, x: usize) -> bool {
    let neighbors = [
        (-1, -1),
        (0, -1),
        (1, -1),
        (-1, 0),
        (1, 0),
        (-1, 1),
        (0, 1),
        (1, 1),
    ];

    let neighbor_count = neighbors
        .iter()
        .filter(|(nx, ny)| {
            let (x, over_x) = x.overflowing_add_signed(*nx);
            let (y, over_y) = y.overflowing_add_signed(*ny);
            !over_x && !over_y && map.get(y).is_some_and(|row| row.get(x) == Some(&true))
        })
        .count();
    neighbor_count < 4
}

pub fn task2(input: Linewise<String>) -> Result<usize, Error> {
    let mut map = input
        .map(|s| {
            s.unwrap()
                .as_bytes()
                .iter()
                .map(|c| *c == b'@')
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let height = map.len();
    let width = map[0].len();
    let mut result = 0;
    let mut removed = Vec::new();
    loop {
        for y in 0..height {
            for x in 0..width {
                if !map[y][x] {
                    continue;
                }

                if is_removable(&map, y, x) {
                    removed.push((x, y));
                }
            }
        }

        if removed.is_empty() {
            break;
        }

        result += removed.len();
        for (x, y) in removed.drain(..) {
            map[y][x] = false;
        }
    }

    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;
    use common::input::Input;

    const INPUT: &[u8] = b"\
..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.";

    #[test]
    fn test_task1() {
        let buf = std::io::BufReader::new(INPUT);
        let result = task1(Input::parse(buf).unwrap());
        let val = result.unwrap();
        assert_eq!(val, 13);
    }
    #[test]
    fn test_task2() {
        let buf = std::io::BufReader::new(INPUT);
        let result = task2(Input::parse(buf).unwrap());
        let val = result.unwrap();
        assert_eq!(val, 43);
    }
}
