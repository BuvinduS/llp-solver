//! Evaluate the objective function at feasible vertices and pick the
//! optimum.

use crate::types::{Objective, Point};

pub fn evaluate_objective(obj: &Objective, p: &Point) -> f64 {
    obj.a * p.x + obj.b * p.y
}

/// Given a list of feasible points, return the index of the one that
/// optimizes the objective (max or min, per `obj.maximize`).
/// Returns `None` if the list is empty -- there's nothing to pick from.
pub fn find_optimal_index(obj: &Objective, points: &[Point]) -> Option<usize> {
    if points.is_empty() {
        return None;
    }

    let mut best_idx = 0;
    let mut best_val = evaluate_objective(obj, &points[0]);

    // `.iter().enumerate().skip(1)`: walk the remaining points (we already
    // used index 0 as our starting "best").
    for (i, p) in points.iter().enumerate().skip(1) {
        let val = evaluate_objective(obj, p);
        let better = if obj.maximize { val > best_val } else { val < best_val };
        if better {
            best_val = val;
            best_idx = i;
        }
    }

    Some(best_idx)
}
