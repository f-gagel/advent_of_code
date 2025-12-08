use common::input::Linewise;
use common::iter_ext::TryIterator;
use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::str::FromStr;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Line was not a valid point")]
    InvalidPoint,
}

#[derive(Debug, Copy, Clone)]
pub struct Point {
    x: i32,
    y: i32,
    z: i32,
}

impl Point {
    fn sqr_distance(a: &Point, b: &Point) -> i64 {
        let x_diff = (a.x - b.x) as i64;
        let y_diff = (a.y - b.y) as i64;
        let z_diff = (a.z - b.z) as i64;
        x_diff * x_diff + y_diff * y_diff + z_diff * z_diff
    }
}

impl FromStr for Point {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut values = [0; 3];
        for (i, part) in s.splitn(3, ',').enumerate() {
            values[i] = part.parse().map_err(|_| Error::InvalidPoint)?;
        }
        Ok(Self {
            x: values[0],
            y: values[1],
            z: values[2],
        })
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
struct Edge {
    sqr_distance: i64,
    from_idx: usize,
    to_idx: usize,
}

impl PartialOrd for Edge {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(Self::cmp(self, other))
    }
}

impl Ord for Edge {
    fn cmp(&self, other: &Self) -> Ordering {
        self.sqr_distance.cmp(&other.sqr_distance)
    }
}

struct DSU {
    parent: Vec<usize>,
    size: Vec<usize>,
}

impl DSU {
    fn new(n: usize) -> Self {
        Self {
            parent: (0..n).collect(),
            size: vec![1; n],
        }
    }

    fn root_of(&mut self, x: usize) -> usize {
        if self.parent[x] != x {
            let root = self.root_of(self.parent[x]);
            self.parent[x] = root;
        }
        self.parent[x]
    }

    fn union(&mut self, a: usize, b: usize) -> bool {
        // find to which graph each belongs
        let root_a = self.root_of(a);
        let root_b = self.root_of(b);
        if root_a == root_b {
            return false;
        }

        // make A a parent of B
        self.parent[root_b] = root_a;
        // A inherits all children of B
        self.size[root_a] += self.size[root_b];
        true
    }
}

pub fn task1(input: Linewise<Point>) -> Result<usize, Error> {
    task1_core(input, 1000)
}

pub fn task1_core(input: Linewise<Point>, steps: usize) -> Result<usize, Error> {
    let mut points = Vec::new();

    // find the N shortest edges in the system
    let mut edges = BinaryHeap::with_capacity(steps);
    for (new_idx, new_point) in input.enumerate() {
        let new_point = new_point?;
        for (other_idx, other) in points.iter().enumerate() {
            let distance = Point::sqr_distance(&new_point, other);
            let edge = Edge {
                sqr_distance: distance,
                from_idx: other_idx,
                to_idx: new_idx,
            };

            if edges.len() < steps {
                edges.push(edge);
            } else if let Some(top) = edges.peek() {
                if distance < top.sqr_distance {
                    edges.pop();
                    edges.push(edge);
                }
            }
        }
        points.push(new_point);
    }

    // build a graph connecting the points by their index
    let n = points.len();
    let mut dsu = DSU::new(n);
    for edge in edges {
        dsu.union(edge.from_idx, edge.to_idx);
    }

    let mut sizes = (0..n)
        // consider only graph roots
        .filter(|idx| dsu.parent[*idx] == *idx)
        // take the size of the whole graph
        .map(|idx| dsu.size[idx])
        .collect::<Vec<_>>();
    // sort in descending order
    sizes.sort_unstable_by(|a, b| b.cmp(a));
    let result = sizes[..3].iter().product::<usize>();
    Ok(result)
}

pub fn task2(input: Linewise<Point>) -> Result<i64, Error> {
    let points = input.try_collect2::<Vec<_>>()?;
    let n = points.len();
    let mut edges = BinaryHeap::with_capacity(n * (n - 1) / 2);

    for i in 0..n {
        for j in (i + 1)..n {
            let d = Point::sqr_distance(&points[i], &points[j]);
            edges.push(Edge {
                sqr_distance: d,
                from_idx: i,
                to_idx: j,
            });
        }
    }

    // create an iterator yielding the shortest connections
    let mut edge_iter = edges.into_sorted_vec().into_iter();

    // use Kruskal's algorithm to merge until everything is connected
    let mut dsu = DSU::new(n);
    // assume only one node is in the network initially
    let mut out_of_network = n - 1;
    let last = loop {
        let e = edge_iter.next().expect("Ran out of edges");
        out_of_network -= dsu.union(e.from_idx, e.to_idx) as usize;
        // break once network is complete
        if out_of_network == 0 {
            break e;
        }
    };

    // multiply X coords of that last pair
    let x1 = points[last.from_idx].x as i64;
    let x2 = points[last.to_idx].x as i64;
    Ok(x1 * x2)
}

#[cfg(test)]
mod tests {
    use super::*;
    use common::input::Input;

    const INPUT: &[u8] = b"\
162,817,812
57,618,57
906,360,560
592,479,940
352,342,300
466,668,158
542,29,236
431,825,988
739,650,466
52,470,668
216,146,977
819,987,18
117,168,530
805,96,715
346,949,466
970,615,88
941,993,340
862,61,35
984,92,344
425,690,689";

    #[test]
    fn test_task1() {
        let buf = std::io::BufReader::new(INPUT);
        let result = task1_core(Input::parse(buf).unwrap(), 10);
        let val = result.unwrap();
        assert_eq!(val, 40);
    }
    #[test]
    fn test_task2() {
        let buf = std::io::BufReader::new(INPUT);
        let result = task2(Input::parse(buf).unwrap());
        let val = result.unwrap();
        assert_eq!(val, 25272);
    }
}
