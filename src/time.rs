use crate::math::{two_product, two_sum};

/// Returns the sum of [whole] and [fraction] as two 64-bit floats.
///
/// The arithmetic is all done with exact floating point operations so no
/// precision is lost to rounding error. It is assumed the sum is less
/// than about 1E16, otherwise the remainder will be greater than 1.0.
pub fn normalize(whole: f64, fraction: f64, divisor: f64) -> (f64, f64) {
    let (mut sum, mut err) = two_sum(whole, fraction);

    if divisor > 0.0 && divisor.is_finite() {
        let q1 = sum / divisor;
        let (p1, p2) = two_product(q1, divisor);
        let (d1, mut d2) = two_sum(sum, -p1);
        d2 += err;
        d2 -= p2;
        let q2 = (d1 + d2) / divisor; // 3-part float fine here; nothing can be lost.;
        (sum, err) = two_sum(q1, q2);
    }

    let mut day = sum.round();
    let (extra, mut frac) = two_sum(sum, -day);
    frac += extra + err;

    // Our fraction can now have gotten >0.5 or <-0.5, which means we would
    // loose one bit of precision. So, correct for that.
    day += frac.round();
    let (extra1, mut frac1) = two_sum(sum, -day);
    frac1 += extra1 + err;

    return (day, frac1);
}

#[cfg(test)]
mod tests {

    #[test]
    fn normalize() {
        let (a, b) = crate::time::normalize(2400000.5, 50123.9999, 0.0);
        assert_eq!(2450124.0, a);
        assert!(b >= 0.4999 && b < 0.49991);

        let (a, b) = crate::time::normalize(2459946.0, 0.016795405092592586, 0.0);
        assert_eq!(2459946.0, a);
        assert_eq!(0.016795405092592586, b);
    }
}
