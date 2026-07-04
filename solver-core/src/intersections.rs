//! Enumerate every candidate vertex: pairwise intersections of all
//! constraint boundary lines. Parallel pairs are skipped automatically
//! (see `geometry::line_intersection`). This step does NOT check
//! feasibility -- that's deliberately kept separate (see `feasibility.rs`)
//! so this stays a pure, easily-testable geometry function.

use crate::geometry::{line_intersection, points_close};
use crate::types::{Constraint, Point};

pub fn all_pairwise_intersections(constraints: &[Constraint]) -> Vec<Point> {
    let mut points: Vec<Point> = Vec::new();

    // Nested loop over every unordered pair (i, j) with i < j, i.e. every
    // pair of constraints considered exactly once.
    for i in 0..constraints.len() {
        for j in (i + 1)..constraints.len() {
            if let Some(p) = line_intersection(&constraints[i], &constraints[j]) {
                // Skip if this point (within tolerance) is already in our
                // list -- happens whenever 3+ boundary lines pass through
                // the same corner.
                let already_have = points.iter().any(|existing| points_close(existing, &p, 1e-6));
                if !already_have {
                    points.push(p);
                }
            }
        }
    }

    points
}
