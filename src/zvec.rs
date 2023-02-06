use std::ops::{Index, IndexMut};

pub struct ZVec<T>(pub Vec<T>, usize);

impl<T> ZVec<T> {
    pub fn with_diam(diam: usize, value: T) -> Self where T: Clone {
        ZVec(vec![value; 2*diam+1], diam)
    }
}


impl<T> Index<isize> for ZVec<T> {
    type Output = T;

    fn index(&self, int: isize) -> &Self::Output {
        &self.0[(int + self.1 as isize) as usize]
    }
}

impl<T> IndexMut<isize> for ZVec<T> {
    fn index_mut(&mut self, int: isize) -> &mut Self::Output {
        &mut self.0[(int + self.1 as isize) as usize]
    }
}
