use std::{cmp::Reverse, collections::HashSet, fmt::Debug};

use super::*;
use thiserror::Error;

#[derive(Debug)]
struct PartialPath<'a, W: World<'a>, S: Score> {
    world: &'a W,
    positions: Vec<W::Index>,
    start_distance: S,
}

impl<'a, W: World<'a>, S: Score> Clone for PartialPath<'a, W, S> {
    fn clone(&self) -> Self {
        PartialPath {
            world: self.world,
            positions: self.positions.clone(),
            start_distance: self.start_distance.clone(),
        }
    }
}
impl<'a, W: World<'a>, S: Score> PartialPath<'a, W, S> {
    fn append(&mut self, point: W::Index, distance: S) {
        self.positions.push(point);
        self.start_distance += distance;
    }
}

impl<'a, W: World<'a>, S: Score> PartialOrd for PartialPath<'a, W, S> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let a = self.start_distance.clone();
        let b = other.start_distance.clone();
        a.partial_cmp(&b)
    }
}

impl<'a, W: World<'a>, S: Score> Ord for PartialPath<'a, W, S> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let a = self.start_distance.clone();
        let b = other.start_distance.clone();
        a.cmp(&b)
    }
}

impl<'a, W: World<'a>, S: Score> PartialEq for PartialPath<'a, W, S> {
    fn eq(&self, other: &Self) -> bool {
        self.positions == other.positions
    }
}

impl<'a, W: World<'a>, S: Score> Eq for PartialPath<'a, W, S> {}

#[derive(Debug, Error)]
pub enum Error {
    #[error("No path is possible")]
    NoPossiblePath,
}

pub struct Algorithm;

impl<'a, W, A> super::Algorithm<'a, W, A> for Algorithm
where
    W: World<'a>,
    A: Agent<'a, W>,
{
    type Error = Error;

    fn try_get_path(
        &self,
        world: &'a W,
        agent: &A,
        start: W::Index,
        target: W::Index,
        max_steps: Option<u32>,
    ) -> Result<Path<'a, W>, Self::Error> {
        if start == target {
            return Ok(Path {
                world,
                positions: vec![start],
            });
        }

        let mut paths = Vec::new();
        let mut visited = HashSet::new();
        visited.insert(start.clone());

        paths.push(Reverse(PartialPath {
            world,
            positions: vec![start.clone()],
            start_distance: A::Cost::default(),
        }));
        let mut step = 0;
        while Some(step) != max_steps {
            step += 1;
            let shortest = paths.pop().ok_or(Error::NoPossiblePath)?;
            let shortest = shortest.0;

            let head = shortest.positions.last().unwrap();

            for neighbor in world.get_neighbors(head) {
                if visited.contains(&neighbor) {
                    continue;
                }

                let dist = match agent.get_cost(world, &head, &neighbor) {
                    Some(x) => x,
                    None => continue,
                };

                let mut path = shortest.clone();
                path.append(neighbor.clone(), dist);

                if neighbor == target {
                    let path = Path {
                        world,
                        positions: path.positions,
                    };
                    return Ok(path);
                }

                let insert = Reverse(path);
                let index = paths.binary_search(&insert).unwrap_or_else(|i| i);
                paths.insert(index, insert);
                visited.insert(neighbor);
            }
        }

        Err(Error::NoPossiblePath)
    }
}
