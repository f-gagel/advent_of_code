use common::bit_grid::BitGrid;
use common::input::Linewise;
use std::mem::swap;

#[derive(Debug, thiserror::Error)]
pub enum Error {}

pub fn task1(lines: Linewise<String>) -> Result<u64, Error> {
    let lines = lines.map(|res| res.unwrap()).collect::<Vec<_>>();
    let width = lines[0].len();
    let height = lines.len();
    let mut east_grid = BitGrid::new(width, height);
    let mut south_grid = BitGrid::new(width, height);
    for (y, line) in lines.into_iter().enumerate() {
        for (x, char) in line.bytes().enumerate() {
            match char {
                b'>' => east_grid.set(x, y, true),
                b'v' => south_grid.set(x, y, true),
                _ => continue,
            }
        }
    }

    let mut write_east_grid = BitGrid::new(width, height);
    let mut write_south_grid = BitGrid::new(width, height);

    let mut changed = true;
    let mut turns = 0;
    while changed {
        changed = false;
        for (x, y) in east_grid.set_positions() {
            let target_x = (x + 1) % east_grid.width();
            if east_grid.get(target_x, y) || south_grid.get(target_x, y) {
                write_east_grid.set(x, y, true);
            } else {
                write_east_grid.set(target_x, y, true);
                changed = true;
            }
        }
        for (x, y) in south_grid.set_positions() {
            let target_y = (y + 1) % south_grid.height();
            // NOTE: south movement considers the already updated east move state
            if south_grid.get(x, target_y) || write_east_grid.get(x, target_y) {
                write_south_grid.set(x, y, true);
            } else {
                write_south_grid.set(x, target_y, true);
                changed = true;
            }
        }

        turns += 1;
        swap(&mut write_east_grid, &mut east_grid);
        write_east_grid.fill(false);
        swap(&mut write_south_grid, &mut south_grid);
        write_south_grid.fill(false);
        if cfg!(debug_assertions) {
            print_board(turns, &east_grid, &south_grid);
        }
    }

    Ok(turns)
}

fn print_board(turns: u64, east: &BitGrid, south: &BitGrid) {
    let mut line = String::new();
    println!("After {turns} steps:");
    for y in 0..east.height() {
        line.clear();
        for x in 0..east.width() {
            let c = if east.get(x, y) {
                '>'
            } else if south.get(x, y) {
                'v'
            } else {
                '.'
            };
            line.push(c);
        }
        println!("{line}");
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use common::input::Input;

    const INPUT: &[u8] = b"\
v...>>.vv>
.vv>>.vv..
>>.>v>...v
>>v>>.>.v.
v>v.vv.v..
>.>>..v...
.vv..>.>v.
v.v..>>v.v
....v..v.>";

    #[test]
    fn test_task1() {
        let buf = std::io::BufReader::new(INPUT);
        let result = task1(Input::parse(buf).unwrap());
        let val = result.unwrap();
        assert_eq!(val, 58);
    }
}
