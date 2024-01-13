use super::math::*;

/// Returns the sum of [whole] and [fraction] as two 64-bit floats.
///
/// The arithmetic is all done with exact floating point operations so no
/// precision is lost to rounding error. It is assumed the sum is less
/// than about 1E16, otherwise the remainder will be greater than 1.0.
pub fn normalize(mut whole: f64, mut fraction: f64, divisor: f64) -> (f64, f64) {
    (whole, fraction) = two_sum(whole, fraction);

    if divisor > 0.0 && divisor.is_finite() {
        let a = whole / divisor;
        let (b, c) = two_product(a, divisor);
        let (d, e) = two_sum(whole, -b);
        (whole, fraction) = two_sum(a, (d + e + fraction - c) / divisor);
    }

    let mut day = whole.round();
    let (extra, mut frac) = two_sum(whole, -day);
    frac += extra + fraction;

    // Our fraction can now have gotten >0.5 or <-0.5, which means we would
    // loose one bit of precision. So, correct for that.
    day += frac.round();
    let (extra, mut frac) = two_sum(whole, -day);
    frac += extra + fraction;

    return (day, frac);
}

#[cfg(test)]
mod tests {
    use crate::time;
    use googletest::prelude::*;

    #[test]
    fn normalize() {
        let (a, b) = time::normalize(2400000.5, 50123.9999, 0.0);
        assert_that!(a, eq(2450124.0));
        assert_that!(b, near(0.4999, 1e-11));

        let (a, b) = time::normalize(2459946.0, 0.016795405092592586, 0.0);
        assert_that!(a, eq(2459946.0));
        assert_that!(b, eq(0.016795405092592586));
    }
}
