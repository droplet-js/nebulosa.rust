/// Adds [a] and [b] exactly, returning the result as two 64-bit floats.
///
/// Uses the procedure of Shewchuk, 1997,
/// Discrete & Computational Geometry 18(3):305-363
///
/// See [Paper](http://www.cs.berkeley.edu/~jrs/papers/robustr.pdf)
pub fn two_sum(a: f64, b: f64) -> (f64, f64) {
    let x = a + b;
    let mut eb = x - a;
    let mut ea = x - eb;
    eb = b - eb;
    ea = a - ea;
    return (x, ea + eb);
}

/// Splits 64-bit float in two aligned parts.
///
/// Uses the procedure of Shewchuk, 1997,
/// Discrete & Computational Geometry 18(3):305-363
///
/// See [Paper](http://www.cs.berkeley.edu/~jrs/papers/robustr.pdf)
pub fn split_aligned(a: f64) -> (f64, f64) {
    let c = 134217729.0 * a;
    let abig = c - a;
    let ah = c - abig;
    let al = a - ah;
    return (ah, al);
}

/// Multiples [a] and [b] exactly, returning the result as two 64-bit floats.
/// The first is the approximate product (with some floating point error)
/// and the second is the error of the float64 product.
///
/// Uses the procedure of Shewchuk, 1997,
/// Discrete & Computational Geometry 18(3):305-363
///
/// See [Paper](http://www.cs.berkeley.edu/~jrs/papers/robustr.pdf)
pub fn two_product(a: f64, b: f64) -> (f64, f64) {
    let x = a * b;
    let (ah, al) = split_aligned(a);
    let (bh, bl) = split_aligned(b);
    let y1 = ah * bh;
    let mut y = x - y1;
    let y2 = al * bh;
    y -= y2;
    let y3 = ah * bl;
    y -= y3;
    let y4 = al * bl;
    y = y4 - y;
    return (x, y);
}
