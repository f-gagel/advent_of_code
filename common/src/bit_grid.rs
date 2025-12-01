#[derive(Eq, PartialEq)]
pub struct BitGrid {
    bits: Vec<u8>,
    width: usize,
    height: usize,
}

impl BitGrid {
    pub fn new(width: usize, height: usize) -> Self {
        let bytes = (width * height).div_ceil(8);
        Self {
            bits: vec![0; bytes],
            width,
            height,
        }
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }

    pub fn get(&self, x: usize, y: usize) -> bool {
        let bit = y * self.width + x;
        let mask = 1 << (bit % 8);
        self.bits[bit / 8] & mask != 0
    }

    pub fn set(&mut self, x: usize, y: usize, set: bool) {
        let bit = y * self.width + x;
        let mask = 1 << (bit % 8);
        let bucket = &mut self.bits[bit / 8];
        if set {
            *bucket |= mask;
        } else {
            *bucket &= !mask;
        }
    }

    pub fn fill(&mut self, set: bool) {
        self.bits.fill(if set { 0xff } else { 0 })
    }

    pub fn set_positions(&self) -> SetIter<'_> {
        SetIter { grid: self, pos: 0 }
    }
}

pub struct SetIter<'a> {
    grid: &'a BitGrid,
    pos: usize,
}

impl<'a> Iterator for SetIter<'a> {
    type Item = (usize, usize);

    fn next(&mut self) -> Option<Self::Item> {
        while self.pos < self.grid.width * self.grid.height {
            let i = self.pos;
            self.pos += 1;
            let byte = self.grid.bits[i / 8];
            let mask = 1 << (i % 8);
            if byte & mask != 0 {
                let x = i % self.grid.width;
                let y = i / self.grid.width;
                return Some((x, y));
            }
        }
        None
    }
}
