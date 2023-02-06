use range_minimum_query::Rmq;
use crate::sais;

pub trait Lcp<'a> {
    fn make(a: &'a [u8], b: &'a [u8]) -> Self;
    fn at(&self, i: usize, j: usize) -> usize;
}

pub struct NaiveLcp<'a>(&'a [u8], &'a [u8]);

impl<'a> Lcp<'a> for NaiveLcp<'a> {
    fn make(a: &'a [u8], b: &'a [u8]) -> Self {
        NaiveLcp(a, b)
    }

    fn at(&self, i: usize, j: usize) -> usize {
        let mut result = 0;
        for (x, y) in self.0[i..].iter().zip(self.1[j..].iter()) {
            if x == y {
                result += 1
            } else {
                break
            }
        }
        return result
    }

}

pub struct SaLcp {
    n: usize,
    rmq: Rmq,
    lcp: Vec<i32>,
    invsa: Vec<i32>,
}

impl Lcp<'_> for SaLcp {
    fn make(a: &[u8], b: &[u8]) -> Self {
        let ab = Vec::from([a, b].concat());
        let n = a.len();
        let nm = n + b.len();
        let (sa, lcp) = sais::sa_lcp(&ab);
        // TODO: can you invert in place? probably
        let mut invsa: Vec<i32> = vec![0; nm];
        for (i, s) in sa.iter().enumerate() {
            invsa[*s as usize] = i as i32;
        }
        let rmq = Rmq::from_iter(lcp.iter());
        SaLcp { n, rmq, lcp, invsa }
    }

    fn at(&self, i: usize, j: usize) -> usize {
        if i == self.n || self.n+j == self.invsa.len() {
            return 0
        }
        let si = self.invsa[i];
        let sj = self.invsa[self.n+j];
        assert!(si != sj);
        let (li, lj) = if si < sj { (si+1, sj) } else { (sj+1, si) };
        let (li, lj) = (li as usize, lj as usize);
        let l = self.lcp[self.rmq.range_minimum(li..=lj).unwrap()];
        let l = std::cmp::min(l as usize, self.n - i);
        l
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_salcp1() {
        let salcp = SaLcp::make(b"a", b"a");
        assert_eq!(salcp.at(0, 0), 1);
    }

    #[test]
    fn test_salcp_dynstr() {
        let s1 = vec!['a' as u8];
        let s2 = vec!['b' as u8];
        let salcp = SaLcp::make(&s1, &s2);
        assert_eq!(salcp.at(0, 0), 0);
    }

    #[test]
    fn test_salcp2() {
        let salcp = SaLcp::make(b"banana", b"nirvana");
        assert_eq!(salcp.at(0, 0), 0);
        assert_eq!(salcp.at(1, 4), 3);
        assert_eq!(salcp.at(3, 4), 3);
        assert_eq!(salcp.at(2, 5), 2);
        assert_eq!(salcp.at(4, 5), 2);

        let salcp = SaLcp::make(b"bandana", b"ananas");
        assert_eq!(salcp.at(1, 0), 2);
        assert_eq!(salcp.at(2, 1), 1);
        assert_eq!(salcp.at(3, 2), 0);
        assert_eq!(salcp.at(4, 0), 3);
    }



}
