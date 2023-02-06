use std::ops::{Index, IndexMut};

pub struct ZVec<T>(pub Vec<T>);

fn nat_of_int(int: isize) -> usize {
    ((int * 2).abs() - (if int < 0 { 1 } else { 0 })).try_into().unwrap()
}

/*
fn int_of_nat(nat: usize) -> isize {
    let inat: isize = (nat / 2).try_into().unwrap();
    if nat % 2 == 0 { inat } else { -inat }
}
*/

impl<T> ZVec<T> {
    pub fn with_diam(diam: usize, value: T) -> Self where T: Clone {
        ZVec(vec![value; 2*diam+1])
    }
}


impl<T> Index<isize> for ZVec<T> {
    type Output = T;

    fn index(&self, int: isize) -> &Self::Output {
        &self.0[nat_of_int(int)]
    }
}

impl<T> IndexMut<isize> for ZVec<T> {
    fn index_mut(&mut self, int: isize) -> &mut Self::Output {
        &mut self.0[nat_of_int(int)]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_nat_of_int() {
        assert_eq!(nat_of_int(0), 0);
        assert_eq!(nat_of_int(-1), 1);
        assert_eq!(nat_of_int(1), 2);
        assert_eq!(nat_of_int(-2), 3);
        assert_eq!(nat_of_int(2), 4);
    }

    /*
    #[test]
    fn test_inverse() {
        for i in -10..10 {
            assert_eq!(int_of_nat(nat_of_int(i)), i);
        }
        for i in 0..21 {
            assert_eq!(nat_of_int(int_of_nat(i)), i);
        }
    }
    */
}
