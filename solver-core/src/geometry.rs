//! Low-level geometry helpers: line intersection and float comparisons.
//!
//! Every constraint `a*x + b*y <op> c` has a boundary *line*
//! `a*x + b*y = c`. Two boundary lines intersect at exactly one point
//! unless they're parallel. We find that point with Cramer's rule for a
//! 2x2 linear system -- the same thing you'd do by hand solving two
//! simultaneous equations.

use crate::types::{Constraint, Point, EPS};

/// Solve the 2x2 system:
///   a1*x + b1*y = c1
///   a2*x + b2*y = c2
///
/// Returns `None` if the lines are parallel (or identical) -- Rust's
/// `Option<T>` is how we represent "a value, or nothing" without using a
/// null/sentinel value.
pub fn line_intersection(c1: &Constraint, c2: &Constraint) -> Option<Point> {
    // Determinant of the 2x2 coefficient matrix. If it's ~0, the two lines
    // have the same slope (parallel) or are the same line -- no single
    // intersection point exists.
    let det = c1.a * c2.b - c2.a * c1.b;

    if det.abs() < EPS {
        return None;
    }

    let x = (c1.c * c2.b - c2.c * c1.b) / det;
    let y = (c1.a * c2.c - c2.a * c1.c) / det;

    Some(Point { x, y })
}

/// Are two points "the same" within a given tolerance? Used to
/// de-duplicate vertices reached via different pairs of constraint lines
/// (e.g. three lines meeting at a single point produce the same
/// intersection three times).
pub fn points_close(p1: &Point, p2: &Point, tol: f64) -> bool {
    (p1.x - p2.x).abs() < tol && (p1.y - p2.y).abs() < tol
}

/// The value of `a*x + b*y` for a given point against a given constraint's
/// line -- the core building block for every feasibility check.
pub fn evaluate(c: &Constraint, p: &Point) -> f64 {
    c.a * p.x + c.b * p.y
}
