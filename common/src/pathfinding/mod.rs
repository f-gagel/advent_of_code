use std::{
    hash::Hash,
    ops::{AddAssign, Deref},
};

pub mod astar;
pub mod djikstra;

pub trait Score: Default + AddAssign + Clone + Ord {}

macro_rules! impl_score {
    ($($ty:ident,)*) => {
        $(impl Score for $ty {})*
    };
}

impl_score!(
    u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize, NonNanF32, NonNanF64,
);

#[derive(Debug, Copy, Clone, Default, PartialEq, PartialOrd)]
pub struct NonNanF32(f32);

#[derive(Debug)]
pub struct WasNan;

impl TryFrom<f32> for NonNanF32 {
    type Error = WasNan;
    fn try_from(value: f32) -> Result<Self, Self::Error> {
        if f32::is_nan(value) {
            Err(WasNan)
        } else {
            Ok(Self(value))
        }
    }
}
impl Deref for NonNanF32 {
    type Target = f32;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl AddAssign for NonNanF32 {
    fn add_assign(&mut self, rhs: Self) {
        self.0 += rhs.0;
    }
}
impl Eq for NonNanF32 {}
impl Ord for NonNanF32 {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.0
            .partial_cmp(&other.0)
            .expect("Impossible nan comparison")
    }
}
#[derive(Debug, Copy, Clone, Default, PartialEq, PartialOrd)]
pub struct NonNanF64(f64);
impl Deref for NonNanF64 {
    type Target = f64;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl AddAssign for NonNanF64 {
    fn add_assign(&mut self, rhs: Self) {
        self.0 += rhs.0;
    }
}
impl Ord for NonNanF64 {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.0
            .partial_cmp(&other.0)
            .expect("Impossible nan comparison")
    }
}
impl Eq for NonNanF64 {}

#[derive(Debug, Clone, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct Score2d<A: Score, B: Score>(pub A, pub B);
impl<A: Score, B: Score> Score for Score2d<A, B> {}
impl<A: Score, B: Score> AddAssign for Score2d<A, B> {
    fn add_assign(&mut self, rhs: Self) {
        self.0 += rhs.0;
        self.1 += rhs.1;
    }
}

#[derive(Debug, Clone, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct Score3d<A: Score, B: Score, C: Score>(pub A, pub B, pub C);
impl<A: Score, B: Score, C: Score> Score for Score3d<A, B, C> {}
impl<A: Score, B: Score, C: Score> AddAssign for Score3d<A, B, C> {
    fn add_assign(&mut self, rhs: Self) {
        self.0 += rhs.0;
        self.1 += rhs.1;
        self.2 += rhs.2;
    }
}

pub trait World<'a> {
    type Index: Eq + Clone + Hash;
    type Neighbors: 'a + IntoIterator<Item = Self::Index>;
    fn get_neighbors(&'a self, origin: &Self::Index) -> Self::Neighbors;
}

pub trait Agent<'a, W: World<'a>> {
    type Cost: Score;
    fn get_cost(&self, world: &W, start: &W::Index, destination: &W::Index) -> Option<Self::Cost>;
}

pub trait Algorithm<'a, W: World<'a>, A: Agent<'a, W>> {
    type Error: std::error::Error;
    fn get_path(
        &self,
        world: &'a W,
        agent: &A,
        start: W::Index,
        target: W::Index,
    ) -> Result<Path<'a, W>, Self::Error> {
        self.try_get_path(world, agent, start, target, None)
    }
    fn try_get_path(
        &self,
        world: &'a W,
        agent: &A,
        start: W::Index,
        target: W::Index,
        max_steps: Option<u32>,
    ) -> Result<Path<'a, W>, Self::Error>;
}

#[derive(Debug)]
pub struct Path<'a, W: World<'a>> {
    world: &'a W,
    positions: Vec<W::Index>,
}

impl<'a, W: World<'a>> Path<'a, W> {
    pub fn world(&self) -> &'a W {
        self.world
    }
    pub fn positions(&self) -> &[W::Index] {
        &self.positions
    }
    pub fn len(&self) -> usize {
        self.positions.len()
    }
}

impl<'a, W: World<'a>> IntoIterator for Path<'a, W> {
    type Item = W::Index;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.positions.into_iter()
    }
}
