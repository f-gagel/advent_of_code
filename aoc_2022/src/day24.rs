use std::{
    cell::{RefCell, RefMut},
    mem::swap,
    ops::Deref,
    str::FromStr, convert::Infallible,
};

use  common::pathfinding as pf;
use grid::Grid;

bitflags::bitflags! {
    #[derive(Debug, Copy, Clone, PartialEq, Eq)]
    struct Tile: u8 {
        const LEFT = 1;
        const RIGHT = 2;
        const UP = 4;
        const DOWN = 8;
        const WALL = 16;
    }
}

impl Default for Tile {
    fn default() -> Self {
        Self::empty()
    }
}

struct Map(RefCell<Vec<Grid<Tile>>>);

impl FromStr for Map {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let tiles = s
            .chars()
            .filter_map(|c| match c {
                '.' => Some(Tile::empty()),
                '<' => Some(Tile::LEFT),
                '>' => Some(Tile::RIGHT),
                '^' => Some(Tile::UP),
                'v' => Some(Tile::DOWN),
                '#' => Some(Tile::WALL),
                _ => None,
            })
            .collect::<Vec<_>>();

        let width = s.lines().next().unwrap().len();
        let grid = Grid::from_vec(tiles, width);

        Ok(Self(RefCell::new(vec![grid])))
    }
}

impl pf::World<'_> for Map {
    type Index = Point;
    type Neighbors = std::vec::IntoIter<Self::Index>;

    fn get_neighbors(&self, origin: &Self::Index) -> Self::Neighbors {
        let mut neighbors = Vec::with_capacity(5);

        let vec = self.0.borrow();

        let cols = vec[0].cols();
        let rows = vec[0].rows();

        let x = origin.x;
        let y = origin.y;
        let time = origin.time.map(|i| i + 1);

        if x > 0 {
            neighbors.push(Point { x: x - 1, y, time });
        }
        if y > 0 {
            neighbors.push(Point { x, y: y - 1, time });
        }

        if x < cols - 1 {
            neighbors.push(Point { x: x + 1, y, time });
        }
        if y < rows - 1 {
            neighbors.push(Point { x, y: y + 1, time });
        }

        neighbors.push(Point { x, y, time });

        neighbors.into_iter()
    }
}

struct SnapshotRef<'a> {
    vec: RefMut<'a, Vec<Grid<Tile>>>,
    index: usize,
}

impl Deref for SnapshotRef<'_> {
    type Target = Grid<Tile>;

    fn deref(&self) -> &Self::Target {
        &self.vec[self.index]
    }
}

impl Map {
    fn get_snapshot(&self, time: usize) -> SnapshotRef<'_> {
        let mut vec = self.0.borrow_mut();
        if time >= vec.len() {
            for t in (vec.len() - 1)..time {
                let previous = &vec[t];
                let grid = make_next(previous);
                // print_map(&grid);
                vec.push(grid);
            }
        };
        SnapshotRef { vec, index: time }
    }
}

fn make_next(previous: &Grid<Tile>) -> Grid<Tile> {
    let rows = previous.rows();
    let cols = previous.cols();
    let mut new = Grid::<Tile>::new(rows, cols);

    for y in 0..rows {
        for x in 0..cols {
            let tile = previous.get(y, x).unwrap();
            if tile.contains(Tile::WALL) {
                new[y][x] |= Tile::WALL;
                continue;
            }
            for wind in [Tile::LEFT, Tile::RIGHT, Tile::UP, Tile::DOWN] {
                if tile.contains(wind) {
                    let (tile, nx, ny) = step_wind(previous, x, y, wind);
                    new[ny][nx] |= tile;
                }
            }
        }
    }
    new
}

fn step_wind(grid: &Grid<Tile>, x: usize, y: usize, wind: Tile) -> (Tile, usize, usize) {
    let (nx, ny) = match wind {
        Tile::LEFT => (x - 1, y),
        Tile::RIGHT => (x + 1, y),
        Tile::DOWN => (x, y + 1),
        Tile::UP => (x, y - 1),
        _ => panic!("Unexpected wind {wind:?}"),
    };

    if grid[ny][nx] != Tile::WALL {
        return (wind, nx, ny);
    }
    let nx = if nx == 0 {
        grid.cols() - 2
    } else if nx == grid.cols() - 1 {
        1
    } else {
        nx
    };
    let ny = if ny == 0 {
        grid.rows() - 2
    } else if ny == grid.rows() - 1 {
        1
    } else {
        ny
    };
    (wind, nx, ny)
}

struct Agent;

impl pf::Agent<'_, Map> for Agent {
    type Cost = u32;

    fn get_cost(
        &self,
        world: &Map,
        _start: &<Map as pf::World>::Index,
        destination: &<Map as pf::World>::Index,
    ) -> Option<Self::Cost> {
        let x = destination.x;
        let y = destination.y;
        let time = destination.time.unwrap();
        let grid = world.get_snapshot(time);
        let tile = &grid[y][x];

        if tile.is_empty() {
            Some(1)
        } else {
            None
        }
    }
}

#[derive(Debug, Clone, Copy, Hash)]
struct Point {
    x: usize,
    y: usize,
    time: Option<usize>,
}

impl Eq for Point {}
impl PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

pub fn task1(s: String) -> Result<usize, pf::djikstra::Error> {
    let map = Map::from_str(&s).unwrap();
    let alg = pf::djikstra::Algorithm;
    let start = Point {
        x: 1,
        y: 0,
        time: Some(0),
    };

    let end = Point {
        x: 150,
        y: 21,
        time: None,
    };
    let path = pf::Algorithm::get_path(&alg, &map, &Agent, start, end)?;
    Ok(path.len() - 1)
}

pub fn task2(s: String) -> Result<usize, pf::djikstra::Error> {
    let map = Map::from_str(&s).unwrap();
    let alg = pf::djikstra::Algorithm;
    let mut start = Point {
        x: 1,
        y: 0,
        time: Some(0),
    };
    let mut end = Point {
        x: 150,
        y: 21,
        time: None,
    };

    // path start -> end
    let path = pf::Algorithm::get_path(&alg, &map, &Agent, start, end)?;
    let duration_1 = path.len() - 1;

    // swap points
    swap(&mut start, &mut end);
    end.time = None;
    start.time = Some(duration_1);

    // path orig end -> orig start
    let path = pf::Algorithm::get_path(&alg, &map, &Agent, start, end)?;
    let duration_2 = path.len() - 1;

    // swap points
    swap(&mut start, &mut end);
    end.time = None;
    start.time = Some(duration_1 + duration_2);

    // path start -> end
    let path = pf::Algorithm::get_path(&alg, &map, &Agent, start, end)?;
    let duration_3 = path.len() - 1;

    // sum up the total time taken
    let sum = duration_1 + duration_2 + duration_3;
    Ok(sum)
}
