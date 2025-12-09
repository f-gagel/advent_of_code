use common::input::Linewise;
use common::iter_ext::TryIterator;
use std::str::FromStr;
use std::{
    borrow::Borrow,
    cmp::{max, min},
    collections::BTreeSet,
};

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Input contained invalid point")]
    InvalidPoint,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct Point {
    x: i32,
    y: i32,
}

impl FromStr for Point {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (left, right) = s.split_once(',').ok_or(Error::InvalidPoint)?;
        Ok(Self {
            x: left.parse().map_err(|_| Error::InvalidPoint)?,
            y: right.parse().map_err(|_| Error::InvalidPoint)?,
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Edge {
    start: i32,
    end: i32,
    fixed: i32,
}

impl Borrow<i32> for Edge {
    fn borrow(&self) -> &i32 {
        &self.fixed
    }
}

impl PartialOrd for Edge {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(Self::cmp(self, other))
    }
}

impl Ord for Edge {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.fixed
            .cmp(&other.fixed)
            .then_with(|| self.start.cmp(&other.start))
            .then_with(|| self.end.cmp(&other.end))
    }
}

pub fn task1(input: Linewise<Point>) -> Result<u64, Error> {
    let mut max_area = 0;
    let mut points = Vec::<Point>::new();

    for point in input {
        let point = point?;
        for other in &points {
            let area = calc_area(&point, other);
            max_area = max(max_area, area);
        }
        points.push(point);
    }

    Ok(max_area)
}

#[inline]
fn calc_area(a: &Point, b: &Point) -> u64 {
    let x_diff = (a.x - b.x).abs() as u64 + 1;
    let y_diff = (a.y - b.y).abs() as u64 + 1;
    x_diff * y_diff
}

pub fn task2(input: Linewise<Point>) -> Result<u64, Error> {
    let points = input.try_collect2::<Vec<_>>()?;
    let mut horizontal = BTreeSet::<Edge>::new();
    let mut vertical = BTreeSet::<Edge>::new();

    for pair in points.windows(2) {
        let [a, b] = pair else { unreachable!() };
        make_edge(a, b, &mut horizontal, &mut vertical);
    }
    make_edge(
        &points[0],
        points.last().unwrap(),
        &mut horizontal,
        &mut vertical,
    );

    let mut max_area = 0;
    for (i, from) in points.iter().enumerate() {
        for to in &points[..i] {
            let area = calc_area(&from, to);
            if area > max_area && is_valid_rect(from, to, &horizontal, &vertical) {
                max_area = area;
            }
        }
    }

    Ok(max_area)
}

#[inline]
fn make_edge(p1: &Point, p2: &Point, horizontal: &mut BTreeSet<Edge>, vertical: &mut BTreeSet<Edge>) {
    if p1.x == p2.x {
        // x is equal -> line is vertical
        let edge = Edge {
            start: min(p1.y, p2.y),
            end: max(p1.y, p2.y),
            fixed: p1.x,
        };
        vertical.insert(edge);
    } else {
        // y is equal -> line is horizontal
        let edge = Edge {
            start: min(p1.x, p2.x),
            end: max(p1.x, p2.x),
            fixed: p1.y,
        };
        horizontal.insert(edge);
    }
}

fn is_valid_rect(
    p1: &Point,
    p2: &Point,
    horizontal: &BTreeSet<Edge>,
    vertical: &BTreeSet<Edge>,
) -> bool {
    let min_x = min(p1.x, p2.x);
    let max_x = max(p1.x, p2.x);
    let min_y = min(p1.y, p2.y);
    let max_y = max(p1.y, p2.y);

    // technically these rects _could_ be valid
    // but will never be big enough to be the largest rect
    // ... also they would break the later code :3
    if max_x - min_x < 2 || max_y - min_y < 2 {
        return false;
    }

    if vertical
        // Find each vertical edge in the range of the rectangle
        .range::<i32, _>((min_x + 1)..=(max_x - 1))
        // check for any edge intersecting the "ceiling" or "floor" of the rect
        // note that edges fully inside the rect invalidate it too
        .any(|edge| edge.start < max_y && edge.end > min_y)
    {
        return false;
    }

    // check horizontal edges the same way
    if horizontal
        .range::<i32, _>((min_y + 1)..=(max_y - 1))
        .any(|edge| edge.start < max_x && edge.end > min_x)
    {
        return false;
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;
    use common::input::Input;

    const INPUT: &[u8] = b"\
7,1
11,1
11,7
9,7
9,5
2,5
2,3
7,3";

    #[test]
    fn test_task1() {
        let buf = std::io::BufReader::new(INPUT);
        let result = task1(Input::parse(buf).unwrap());
        let val = result.unwrap();
        assert_eq!(val, 50);
    }
    #[test]
    fn test_task2() {
        let buf = std::io::BufReader::new(INPUT);
        let result = task2(Input::parse(buf).unwrap());
        let val = result.unwrap();
        assert_eq!(val, 24);
    }
}
