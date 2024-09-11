use itertools::iterate;
use p3_circle::{CircleDomain, Point};
use p3_mersenne_31::Mersenne31;

#[test]
fn test_domain() {
    // D =  Q·G_n = Q·G_{n-1} U Q^{-1}·G_{n-1}

    type F = Mersenne31;
    let log_n = 5;
    let d = CircleDomain::<F>::standard(log_n);

    let g = Point::generator(log_n);
    let shift = Point::generator(log_n + 1);

    assert_eq!(g, shift.double());

    let points = iterate(shift, move |&p| p + g)
        .take(1 << log_n)
        .collect::<Vec<_>>();

    assert_eq!(d.points().collect::<Vec<_>>(), points);
}
