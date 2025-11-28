use std::fmt::{Binary, Debug, Formatter};
use std::ops::{BitXor, BitXorAssign};

#[derive(Clone)]
pub struct BitSet {
    bits: Vec<u8>,
    len: usize,
}

impl Debug for BitSet {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let bin = format!("{self:b}");
        f.debug_struct("BitSet")
            .field("bits", &bin)
            .field("len", &self.len)
            .finish()
    }
}

impl Binary for BitSet {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for val in &self.bits {
            let val = val.reverse_bits();
            f.write_fmt(format_args!("{val:0>8b}"))?;
        }
        Ok(())
    }
}

impl PartialEq for BitSet {
    fn eq(&self, other: &Self) -> bool {
        self.len.eq(&other.len) && self.bits.eq(&other.bits)
    }
}
impl Eq for BitSet {}

impl BitXor for BitSet {
    type Output = Self;
    fn bitxor(mut self, rhs: Self) -> Self::Output {
        assert_eq!(self.len, rhs.len);
        self ^= rhs;
        self
    }
}

impl BitXorAssign for BitSet {
    fn bitxor_assign(&mut self, rhs: Self) {
        for (i, other) in rhs.bits.into_iter().enumerate() {
            self.bits[i] ^= other;
        }
    }
}

impl BitSet {
    pub fn new(length: usize) -> Self {
        let mut cap = length / 8;
        if length % 8 > 0 {
            cap += 1;
        }
        Self {
            bits: vec![0; cap],
            len: length,
        }
    }

    pub fn set(&mut self, i: usize) {
        assert!(i < self.len, "Index exceeded set length");

        let bucket = i / 8;
        let bit = i % 8;
        let mask = 1u8 << bit;
        self.bits[bucket] |= mask;
    }

    pub fn unset(&mut self, i: usize) {
        assert!(i < self.len, "Index exceeded set length");

        let bucket = i / 8;
        let bit = i % 8;
        let mask = 1u8 << bit;
        self.bits[bucket] &= !mask;
    }

    pub fn toggle(&mut self, i: usize) {
        if self.get(i) {
            self.unset(i);
        } else {
            self.set(i);
        }
    }

    pub fn get(&self, i: usize) -> bool {
        assert!(i < self.len, "Index {i} exceeded set length {}", self.len);

        let bucket = i / 8;
        let bit = i % 8;
        let mask = 1u8 << bit;
        self.bits[bucket] & mask == mask
    }

    pub fn iter(&self) -> Iter<'_> {
        Iter { set: self, pos: 0 }
    }

    pub fn count_ones(&self) -> u32 {
        self.bits.iter().map(|i| i.count_ones()).sum()
    }

    pub fn count_zeros(&self) -> u32 {
        self.bits.iter().map(|i| i.count_zeros()).sum()
    }

    pub fn clear(&mut self) {
        for i in self.bits.iter_mut() {
            *i = 0;
        }
    }
}

pub struct Iter<'a> {
    set: &'a BitSet,
    pos: usize,
}

impl Iterator for Iter<'_> {
    type Item = bool;
    fn next(&mut self) -> Option<Self::Item> {
        if self.pos >= self.set.len {
            return None;
        }

        let val = self.set.get(self.pos);
        self.pos += 1;
        Some(val)
    }
}
