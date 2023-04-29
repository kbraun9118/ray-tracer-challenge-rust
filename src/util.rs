pub(crate) const EPSILON: f64 = 0.00001;

pub fn eq_f64(a: f64, b: f64) -> bool {
    (a - b).abs() < EPSILON
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn eq_f64_equality_difference_less_than_epsilon() {
        assert!(eq_f64(0.1 + 0.2, 0.3));
        assert!(eq_f64(1.0, 1.000001));
        assert!(!eq_f64(1.0, 1.0 + EPSILON));
    }
}
