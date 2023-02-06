extern {
    fn libsais(t: *const u8, sa: *mut i32, n: i32, fs: i32, freq: *mut i32) -> i32;
    fn libsais_plcp(t: *const u8, sa: *const i32, plcp: *mut i32, n: i32) -> i32;
    fn libsais_lcp(plcp: *const i32, sa: *const i32, lcp: *mut i32, n: i32) -> i32;
}

/// libsais apparently sets first element of lcp to zero
pub fn sa_lcp(t: &[u8]) -> (Vec<i32>, Vec<i32>) {
    let n = t.len();
    let mut sa = vec![0i32; n];
    unsafe { libsais(t.as_ptr(), sa.as_mut_ptr(), n as i32, 0, std::ptr::null_mut()) };
    let mut plcp = vec![0i32; n];
    unsafe { libsais_plcp(t.as_ptr(), sa.as_mut_ptr(), plcp.as_mut_ptr(), n as i32) };
    let mut lcp = vec![0i32; n];
    unsafe { libsais_lcp(plcp.as_ptr(), sa.as_ptr(), lcp.as_mut_ptr(), n as i32) };
    (sa, lcp)
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lcp() {
        assert_eq!(sa_lcp(b"banana").1, vec![0, 1, 3, 0, 0, 2]); // wiki example
        assert_eq!(sa_lcp(b"abaab").1, vec![0, 1, 2, 0, 1]); // wiki example

        assert_eq!(sa_lcp(&[]).1, vec![]);
        assert_eq!(sa_lcp(&[0]).1, vec![0]);
        assert_eq!(sa_lcp(&[0,0]).1, vec![0, 1]);
        assert_eq!(sa_lcp(&[0,0,0]).1, vec![0, 1, 2]);

        assert_eq!(sa_lcp(b"banana").0, vec![5, 3, 1, 0, 4, 2]); // wiki example
        assert_eq!(sa_lcp(b"abaab").0, vec![2, 3, 0, 4, 1]); // wiki example

        assert_eq!(sa_lcp(&[]).0, vec![]);
        assert_eq!(sa_lcp(&[0]).0, vec![0]);
        assert_eq!(sa_lcp(&[0,0]).0, vec![1, 0]);
        assert_eq!(sa_lcp(&[0,0,0]).0, vec![2, 1, 0]);
    }
}
