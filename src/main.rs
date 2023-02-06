pub mod zvec;
use crate::zvec::ZVec;
pub mod lcp;
mod sais;
use crate::lcp::*;


fn naive_edit_distance<'a>(a: &'a [u8], b: &'a [u8]) -> usize {
    let m = b.len();
    let mut row: Vec<usize> = (0..=m).collect();
    let mut nrow = vec![usize::MAX; m+1];
    for i in 0..a.len() {
        nrow[0] = i+1;
        for j in 0..m {
            if a[i] == b[j] {
                nrow[j+1] = row[j];
            } else {
                nrow[j+1] = std::cmp::min(nrow[j], std::cmp::min(row[j], row[j+1])) + 1;
            }
        }
        std::mem::swap(&mut row, &mut nrow);
        //eprintln!("{i} {row:?}");
    }
    row[m]
}


fn edit_distance<'a, LCP:Lcp<'a>>(a: &'a [u8], b: &'a [u8]) -> usize {
    let n = a.len();
    let m = b.len();
    let lcp:LCP = LCP::make(a, b);
    let init = lcp.at(0, 0);
    if init == n && init == m {
        return 0
    }
    let mut diags: ZVec<isize> = ZVec::with_diam(0, init as isize);

    //eprintln!("a={:?} b={:?}, init={}", a, b, init);


    // ints[int][x] = the row (length of prefix of a) where the distance is x for the last time on int
    // diag is the difference between pos in b and pos in a
    let mut dist: isize = 1;
    let enddiag = m as isize - n as isize;
    loop {
        let mut ndiags: ZVec<isize> = ZVec::with_diam(dist as usize, -1);
        //eprintln!("dist={}", dist);
        //let lodiag = std::cmp::max(-dist, -(n as isize));
        //let hidiag = std::cmp::min(dist, m as isize);
        for d in -dist..=dist {
            let mut row = -1;
            for (dd, drow) in [(1, 1), (0, 1), (-1, 0)] {
                if !(-dist < d+dd && d+dd < dist) { continue }
                let prev_row = diags[d+dd];
                if prev_row >= 0 {
                    if prev_row + drow > n as isize { continue }
                    if ((prev_row + drow) as isize + d) as usize > m { continue }
                    row = std::cmp::max(row, prev_row + drow);
                }
            }
            if row < 0 { continue };
            let col = (row as isize + d) as usize;
            let ext_row = row as usize + lcp.at(row as usize, col); // go down the diagonal while letters are equal
            ndiags[d] = ext_row as isize;
        }
        if enddiag.abs() <= dist && ndiags[enddiag] == n as isize {
            return dist as usize
        }
        //eprintln!("ndiags={:?}", ndiags.0);
        diags = ndiags;
        dist += 1;
        assert!((dist as usize) <= n+m);
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let file1 = std::fs::read(args.get(1).expect("first arg (filename) missing")).unwrap();
    let file2 = std::fs::read(&args.get(2).expect("second arg (filename) missing")).unwrap();
    let ed = match &args[3].parse::<i32>().unwrap() {
        0 => naive_edit_distance(&file1, &file2),
        1 => edit_distance::<NaiveLcp>(&file1, &file2),
        2 => edit_distance::<SaLcp>(&file1, &file2),
        _ => panic!("invalid third argument")
    };
    println!("{ed}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_edit_distance() {
        for i in 0..4 {
            let s = &vec![0u8; i];
            assert_eq!(edit_distance::<NaiveLcp>(s, s), 0);
            assert_eq!(naive_edit_distance(s, s), 0);
        }
        for i in 0..4 {
            let s1 = &vec![0u8; i];
            let s2 = &vec![1u8; i];
            assert_eq!(edit_distance::<NaiveLcp>(s1, s2), i);
            assert_eq!(naive_edit_distance(s1, s2), i);
        }
        for i in 0..4 {
            let s = &vec![0u8; i];
            assert_eq!(edit_distance::<NaiveLcp>(s, &[]), i);
            assert_eq!(edit_distance::<NaiveLcp>(&[], s), i);
            assert_eq!(naive_edit_distance(s, &[]), i);
            assert_eq!(naive_edit_distance(&[], s), i);
        }
    }
}
