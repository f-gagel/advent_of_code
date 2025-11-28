use std::mem::MaybeUninit;
use std::ops::{AddAssign, MulAssign};

pub fn try_flatten<T, E, TS, I>(iter: I) -> TryFlatten<T, E, TS, I>
where
    TS: IntoIterator<Item = T>,
    I: Iterator<Item = Result<TS, E>>,
{
    TryFlatten {
        super_iter: iter,
        current_iter: None,
    }
}

pub struct TryFlatten<T, E, TS, I>
where
    TS: IntoIterator<Item = T>,
    I: Iterator<Item = Result<TS, E>>,
{
    super_iter: I,
    current_iter: Option<ResultIter<TS::IntoIter, E>>,
}

enum ResultIter<I: Iterator, E> {
    Ok(I),
    Err(std::option::IntoIter<E>),
}

impl<T, E, TS, I> Iterator for TryFlatten<T, E, TS, I>
where
    TS: IntoIterator<Item = T>,
    I: Iterator<Item = Result<TS, E>>,
{
    type Item = Result<T, E>;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let mut iter = match &mut self.current_iter {
                // a subiterator is present try to continue with it
                Some(iter) => iter,
                // subiterator is missing, try creating a new one
                None => match self.super_iter.next() {
                    // there was at least A result that should be yielded
                    Some(res) => {
                        let iter: ResultIter<TS::IntoIter, E> = match res {
                            Ok(ok) => ResultIter::Ok(ok.into_iter()),
                            Err(err) => ResultIter::Err(Some(err).into_iter()),
                        };
                        self.current_iter = Some(iter);
                        self.current_iter.as_mut().unwrap()
                    }
                    // sub- and super-iterator are exhausted: were done
                    None => return None,
                },
            };

            // try continuing with the current sub iterator
            match &mut iter {
                ResultIter::Ok(ok) => match ok.next() {
                    Some(x) => return Some(Ok(x)),
                    None => {
                        self.current_iter = None;
                        continue;
                    }
                },
                ResultIter::Err(err) => match err.next() {
                    Some(x) => return Some(Err(x)),
                    None => {
                        self.current_iter = None;
                        continue;
                    }
                },
            }
        }
    }
}

pub unsafe trait UnlimitedIterator: Iterator {
    fn next_unlimited(&mut self) -> Self::Item {
        unsafe { self.next().unwrap_unchecked() }
    }
}

unsafe impl<I> UnlimitedIterator for std::iter::Cycle<I> where I: Iterator + Clone {}

pub trait TrySum<A = Self>: Sized {
    fn try_sum<E>(i: impl TryIterator<A, E>) -> Result<Self, E>;
}

pub trait TryProduct<A = Self>: Sized {
    fn try_product<E>(i: impl TryIterator<A, E>) -> Result<Self, E>;
}

pub trait TryCollect<A>: Sized {
    fn try_collect<E>(i: impl TryIterator<A, E>) -> Result<Self, E>;
}

impl<A, S: Default + AddAssign<A>> TrySum<A> for S {
    fn try_sum<E>(i: impl TryIterator<A, E>) -> Result<Self, E> {
        let mut s = S::default();
        for elem in i {
            s += elem?;
        }
        Ok(s)
    }
}

impl<A, S: Default + MulAssign<A>> TryProduct<A> for S {
    fn try_product<E>(i: impl TryIterator<A, E>) -> Result<Self, E> {
        let mut s = S::default();
        for elem in i {
            s *= elem?;
        }
        Ok(s)
    }
}

impl<A, S: Default + Extend<A>> TryCollect<A> for S {
    fn try_collect<E>(i: impl TryIterator<A, E>) -> Result<Self, E> {
        let mut s = S::default();
        for elem in i {
            s.extend([elem?]);
        }
        Ok(s)
    }
}

pub trait TryIterator<T, E>: Iterator<Item = Result<T, E>> {
    fn try_sum<S>(self) -> Result<S, E>
    where
        Self: Sized,
        S: TrySum<T>,
    {
        TrySum::try_sum(self)
    }
    fn try_product<S>(self) -> Result<S, E>
    where
        Self: Sized,
        S: TryProduct<T>,
    {
        TryProduct::try_product(self)
    }
    fn try_collect2<S>(self) -> Result<S, E>
    where
        Self: Sized,
        S: TryCollect<T>,
    {
        TryCollect::try_collect(self)
    }
}

impl<T, E, I: Iterator<Item = Result<T, E>>> TryIterator<T, E> for I {}

struct CollectFixedArray<T, const N: usize> {
    array: MaybeUninit<[T; N]>,
    cursor: usize,
}

impl<T, const N: usize> Drop for CollectFixedArray<T, N> {
    fn drop(&mut self) {
        let array = unsafe { self.array.assume_init_mut() };
        for i in 0..self.cursor {
            unsafe { std::ptr::drop_in_place(&mut array[i] as *mut _) };
        }
    }
}

impl<T, const N: usize> CollectFixedArray<T, N> {
    fn new() -> Self {
        Self {
            array: MaybeUninit::<[T; N]>::uninit(),
            cursor: 0,
        }
    }
    fn insert(&mut self, val: T) {
        assert!(self.cursor < N, "Attempted to insert too many element");
        unsafe {
            self.array.assume_init_mut()[self.cursor] = val;
        }
        self.cursor += 1;
    }
    fn get_array(mut self) -> [T; N] {
        assert_eq!(self.cursor, N, "Attempted collect array before it was full");
        let moved = unsafe { self.array.assume_init_read() };
        // reset the cursor to prevent dropping element after theyre moved out
        self.cursor = 0;
        moved
    }
}

pub trait IterExt: Iterator {
    fn collect_fixed<const N: usize>(&mut self) -> [Self::Item; N] {
        let mut col = CollectFixedArray::<Self::Item, N>::new();
        for _ in 0..N {
            col.insert(self.next().expect("Iterator was exhausted prematurely"))
        }
        col.get_array()
    }
}

impl<S: Iterator> IterExt for S {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn collect_fixed() {
        let x = [1, 2, 3].into_iter().collect_fixed::<2>();
        assert_eq!(x, [1, 2]);
    }

    #[test]
    fn collect_fixed_zero() {
        let x = [1, 2, 3].into_iter().collect_fixed::<0>();
        assert_eq!(x, []);
    }

    #[test]
    #[should_panic]
    fn collect_fixed_too_many() {
        [1, 2, 3].into_iter().collect_fixed::<4>();
    }
}
