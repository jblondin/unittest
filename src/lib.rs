
#[macro_export]
macro_rules! assert_fp_eq {
    ($lhs:expr, $rhs:expr, $eps:expr) => {{
        assert!((($lhs - $rhs) as f64).abs() < $eps);
    }};
    ($lhs:expr, $rhs:expr) => {{
        const DEFAULT_EPS: f64 = 1.0e-6;
        assert_fp_eq!($lhs, $rhs, DEFAULT_EPS);
    }};
}
#[macro_export]
macro_rules! assert_fpvec_eq {
    ($lhs:expr, $rhs:expr, $eps:expr) => {{
        use std::f64;
        assert!(($lhs.iter().zip($rhs.iter())).fold(f64::NEG_INFINITY, |acc, (l, r)| {
                acc.max((l.clone() as f64 - r.clone() as f64).abs())
        }) < $eps);
    }};
    ($lhs:expr, $rhs:expr) => {{
        const DEFAULT_EPS: f64 = 1.0e-6;
        assert_fpvec_eq!($lhs, $rhs, DEFAULT_EPS);
    }};
}
#[macro_export]
macro_rules! assert_fpvec_neq {
    ($lhs:expr, $rhs:expr, $eps:expr) => {{
        use std::f64;
        assert!(($lhs.iter().zip($rhs.iter())).fold(f64::NEG_INFINITY, |acc, (l, r)| {
                acc.max((l.clone() as f64 - r.clone() as f64).abs())
        }) > $eps);
    }};
    ($lhs:expr, $rhs:expr) => {{
        const DEFAULT_EPS: f64 = 1.0e-6;
        assert_fpvec_neq!($lhs, $rhs, DEFAULT_EPS);
    }};
}


#[cfg(test)]
mod tests {

    #[test]
    fn test_assert_fp_eq() {
        assert_fp_eq!(5.1000001, 5.1000002);
        assert_fp_eq!(5.1000001, 5.1000002, 1e-4);
    }
    #[test]
    #[should_panic(expected = "assertion failed")]
    fn test_assert_fp_eq_fail() {
        assert_fp_eq!(5.1000001, 5.1000002, 1.0e-8);
    }

    #[test]
    fn test_assert_fpvec_eq() {
        assert_fpvec_eq!([5.1000001, 6.2000002], [5.1000002, 6.2000005]);
        assert_fpvec_eq!([5.1000001, 6.2000002], [5.1000002, 6.2000005], 1e-4);
    }

    #[test]
    #[should_panic(expected = "assertion failed")]
    fn test_assert_fpvec_eq_fail() {
        assert_fpvec_eq!([5.100000001, 6.200000002], [5.1000002, 6.2000005], 1e-8);
    }

    #[test]
    fn test_assert_fpvec_neq() {
        assert_fpvec_neq!([5.100001, 6.200002], [5.1000002, 6.2000005]);
        assert_fpvec_neq!([5.100000001, 6.200000002], [5.1000002, 6.2000005], 1e-8);
    }

    #[test]
    #[should_panic(expected = "assertion failed")]
    fn test_assert_fpvec_neq_fail() {
        assert_fpvec_neq!([5.1000001, 6.2000002], [5.1000002, 6.2000005]);
    }
}
