//! Integration tests: exercise `solve()` end-to-end against known LP
//! problems, including the worked example from the project brief.
//!
//! Rust note: files under `tests/` are compiled as separate crates that
//! link against our library via its normal public API (`use solver_core::...`)
//! -- exactly like an external caller (e.g. the wasm bindings) would use it.
//! This is a good sanity check that the public API is actually usable, not
//! just the internals.

use solver_core::types::{Constraint, Objective, Operator, SolutionStatus};
use solver_core::{solve, types::EPS};

fn le(a: f64, b: f64, c: f64) -> Constraint {
    Constraint { a, b, op: Operator::Le, c, label: None }
}
fn ge(a: f64, b: f64, c: f64) -> Constraint {
    Constraint { a, b, op: Operator::Ge, c, label: None }
}

/// The worked example from the project brief:
///   Maximize Z = 200x + 500y
///   Subject to: x<=400, y<=300, x+2y<=800, 4x+5y<=2300, 5x+4y<=2500,
///               y<=2x, x>=0, y>=0
/// Hand-solved expected optimum: (x, y) = (200, 300), Z = 190000.
/// (At that point, y<=300, x+2y<=800, and 4x+5y<=2300 are all
/// simultaneously binding -- three lines meeting at one vertex, a good
/// stress test for point de-duplication.)
#[test]
fn worked_example_matches_hand_solved_optimum() {
    let constraints = vec![
        le(1.0, 0.0, 400.0),   // x <= 400
        le(0.0, 1.0, 300.0),   // y <= 300
        le(1.0, 2.0, 800.0),   // x + 2y <= 800
        le(4.0, 5.0, 2300.0),  // 4x + 5y <= 2300
        le(5.0, 4.0, 2500.0),  // 5x + 4y <= 2500
        le(-2.0, 1.0, 0.0),    // y <= 2x  =>  -2x + y <= 0
    ];
    let objective = Objective { a: 200.0, b: 500.0, maximize: true };

    let result = solve(&constraints, &objective, true, true);

    assert_eq!(result.status, SolutionStatus::Optimal);
    let opt = result.optimal_point.expect("expected an optimal point");
    assert!((opt.x - 200.0).abs() < 1e-4, "expected x=200, got {}", opt.x);
    assert!((opt.y - 300.0).abs() < 1e-4, "expected y=300, got {}", opt.y);
    assert!((result.objective_value.unwrap() - 190000.0).abs() < 1e-3);

    // Three constraints (y<=300, x+2y<=800, 4x+5y<=2300) should all be
    // active/binding at this vertex.
    assert!(result.active_constraints.len() >= 3);
}

/// Two directly contradictory constraints on x (x>=5 and x<=2) leave no
/// feasible point at all.
#[test]
fn contradictory_constraints_are_infeasible() {
    let constraints = vec![ge(1.0, 0.0, 5.0), le(1.0, 0.0, 2.0)];
    let objective = Objective { a: 1.0, b: 0.0, maximize: true };

    let result = solve(&constraints, &objective, true, true);

    assert_eq!(result.status, SolutionStatus::Infeasible);
    assert!(result.optimal_point.is_none());
    assert!(result.feasible_vertices.is_empty());
}

/// With only x>=0, y>=0 and no upper bounds, the feasible region is the
/// entire first quadrant -- maximizing x+y grows without bound.
#[test]
fn unbounded_region_is_detected() {
    let constraints: Vec<Constraint> = vec![]; // only the x>=0/y>=0 restrictions apply
    let objective = Objective { a: 1.0, b: 1.0, maximize: true };

    let result = solve(&constraints, &objective, true, true);

    assert_eq!(result.status, SolutionStatus::Unbounded);
    assert!(result.optimal_point.is_none());
}

/// A bounded region where *minimizing* still has a unique finite optimum
/// -- checks the `maximize: false` path.
#[test]
fn minimize_picks_the_lowest_vertex() {
    let constraints = vec![
        le(1.0, 0.0, 10.0), // x <= 10
        le(0.0, 1.0, 10.0), // y <= 10
    ];
    let objective = Objective { a: 1.0, b: 1.0, maximize: false };

    let result = solve(&constraints, &objective, true, true);

    assert_eq!(result.status, SolutionStatus::Optimal);
    let opt = result.optimal_point.unwrap();
    // Minimizing x+y over [0,10]x[0,10] bottoms out at the origin.
    assert!((opt.x - 0.0).abs() < EPS.sqrt());
    assert!((opt.y - 0.0).abs() < EPS.sqrt());
    assert!((result.objective_value.unwrap() - 0.0).abs() < 1e-6);
}

/// Parallel constraints (same slope, different offsets) must not produce
/// a bogus intersection point.
#[test]
fn parallel_constraints_do_not_crash_or_fabricate_points() {
    let constraints = vec![
        le(1.0, 1.0, 10.0), // x + y <= 10
        le(1.0, 1.0, 20.0), // x + y <= 20  (parallel to the line above)
    ];
    let objective = Objective { a: 1.0, b: 1.0, maximize: true };

    let result = solve(&constraints, &objective, true, true);

    // Should still resolve fine using the x>=0/y>=0 boundaries and the
    // binding x+y<=10 line -- just confirms nothing panics or produces
    // garbage from the parallel pair.
    assert_eq!(result.status, SolutionStatus::Optimal);
    assert!((result.objective_value.unwrap() - 10.0).abs() < 1e-6);
}
