//! Feasibility checks and polygon construction.

use crate::geometry::evaluate;
use crate::types::{Constraint, Operator, Point, EPS};

/// Does point `p` satisfy a single constraint, within tolerance?
pub fn satisfies(c: &Constraint, p: &Point) -> bool {
    let lhs = evaluate(c, p);
    match c.op {
        Operator::Le => lhs <= c.c + EPS,
        Operator::Ge => lhs >= c.c - EPS,
        Operator::Eq => (lhs - c.c).abs() <= EPS,
    }
}

/// Does `p` satisfy every constraint in the list?
///
/// Rust note: `Result<(), usize>` reads as "either success with no extra
/// data (`Ok(())`), or failure carrying a `usize`". Here the `usize` is
/// the index of the first constraint that got violated, which is exactly
/// what Educational Mode needs to explain a rejection.
pub fn check_all(constraints: &[Constraint], p: &Point) -> Result<(), usize> {
    for (i, c) in constraints.iter().enumerate() {
        if !satisfies(c, p) {
            return Err(i);
        }
    }
    Ok(())
}

/// Order a set of convex-polygon corner points into a proper walk-around
/// loop, by sorting them by angle around their centroid. SVG/D3 need
/// points in this order to draw a correctly-filled shape instead of a
/// self-intersecting scribble.
pub fn order_polygon(points: &[Point]) -> Vec<Point> {
    if points.len() < 3 {
        return points.to_vec();
    }

    let cx = points.iter().map(|p| p.x).sum::<f64>() / points.len() as f64;
    let cy = points.iter().map(|p| p.y).sum::<f64>() / points.len() as f64;

    let mut sorted = points.to_vec();
    // `atan2(dy, dx)` gives the angle of each point relative to the
    // centroid; sorting by that angle walks around the polygon in order.
    sorted.sort_by(|a, b| {
        let angle_a = (a.y - cy).atan2(a.x - cx);
        let angle_b = (b.y - cy).atan2(b.x - cx);
        angle_a.partial_cmp(&angle_b).unwrap()
    });
    sorted
}
