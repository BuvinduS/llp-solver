//! Crate entry point: wires parsing -> intersections -> feasibility ->
//! optimization into one `solve()` call, and (only when compiled for
//! wasm32) exposes it to JavaScript via wasm-bindgen.
//!
//! Rust note: `pub mod foo;` says "this crate has a module living in
//! `foo.rs`, and it's visible to code outside this crate too" -- that's
//! how `solve()` below can reach into `parser::`, `geometry::`, etc., and
//! how our integration tests (in `tests/`) can reach into the crate.

pub mod feasibility;
pub mod geometry;
pub mod intersections;
pub mod optimizer;
pub mod parser;
pub mod types;

use types::{
    Constraint, Objective, Point, RejectedPoint, SolutionStatus, SolverResult, Vertex, EPS,
};

/// The main solver entry point. Pure Rust, no JS/wasm dependency -- which
/// is exactly what makes it directly testable with plain `cargo test`.
pub fn solve(
    user_constraints: &[Constraint],
    objective: &Objective,
    restrict_x_nonneg: bool,
    restrict_y_nonneg: bool,
) -> SolverResult {
    // Step 1: expand into the full internal constraint list.
    let constraints =
        parser::build_constraint_list(user_constraints, restrict_x_nonneg, restrict_y_nonneg);

    // Step 2: every pairwise intersection of constraint boundary lines --
    // our candidate corner points. Most will fail the feasibility check
    // in step 3; that's expected and normal.
    let candidates = intersections::all_pairwise_intersections(&constraints);

    // Step 3: keep only points that satisfy every constraint, recording
    // *why* each rejected candidate failed (drives Educational Mode).
    let mut feasible_points: Vec<Point> = Vec::new();
    let mut rejected_points: Vec<RejectedPoint> = Vec::new();

    for p in &candidates {
        match feasibility::check_all(&constraints, p) {
            Ok(()) => feasible_points.push(*p),
            Err(violated_idx) => rejected_points.push(RejectedPoint {
                point: *p,
                violated_constraint_index: violated_idx,
            }),
        }
    }

    if feasible_points.is_empty() {
        return SolverResult {
            status: SolutionStatus::Infeasible,
            feasible_vertices: vec![],
            optimal_point: None,
            objective_value: None,
            active_constraints: vec![],
            feasible_polygon: vec![],
            rejected_points,
        };
    }

    // Step 4: unboundedness check. We sample many directions around a
    // very large circle; if the feasible region contains a point out
    // there that both (a) still satisfies every constraint and (b) beats
    // every known finite vertex's objective value, the objective grows
    // without bound over the feasible region.
    //
    // This is a pragmatic numerical approximation rather than a fully
    // rigorous extreme-ray analysis (which would need to inspect the
    // polygon's unbounded edges directly) -- it's easy to reason about
    // and test, and works well for the polygon shapes this app targets.
    // A documented spot for future improvement if edge cases turn up.
    let best_known = optimizer::find_optimal_index(objective, &feasible_points)
        .map(|i| optimizer::evaluate_objective(objective, &feasible_points[i]))
        .unwrap_or(f64::NEG_INFINITY);

    if is_unbounded(&constraints, objective, best_known) {
        return SolverResult {
            status: SolutionStatus::Unbounded,
            feasible_vertices: vec![],
            optimal_point: None,
            objective_value: None,
            active_constraints: vec![],
            feasible_polygon: feasibility::order_polygon(&feasible_points),
            rejected_points,
        };
    }

    // Step 5: build full Vertex records (point + which constraints are
    // active there + objective value).
    let vertices: Vec<Vertex> = feasible_points
        .iter()
        .map(|p| {
            let active: Vec<usize> = constraints
                .iter()
                .enumerate()
                .filter(|(_, c)| (geometry::evaluate(c, p) - c.c).abs() <= EPS)
                .map(|(i, _)| i)
                .collect();
            Vertex {
                point: *p,
                active_constraint_indices: active,
                objective_value: optimizer::evaluate_objective(objective, p),
            }
        })
        .collect();

    // Step 6: pick the optimal vertex.
    let optimal_idx = optimizer::find_optimal_index(objective, &feasible_points).unwrap();
    let optimal_vertex = &vertices[optimal_idx];

    SolverResult {
        status: SolutionStatus::Optimal,
        feasible_vertices: vertices.clone(),
        optimal_point: Some(optimal_vertex.point),
        objective_value: Some(optimal_vertex.objective_value),
        active_constraints: optimal_vertex.active_constraint_indices.clone(),
        feasible_polygon: feasibility::order_polygon(&feasible_points),
        rejected_points,
    }
}

fn is_unbounded(constraints: &[Constraint], objective: &Objective, best_known: f64) -> bool {
    const DIRECTIONS: usize = 360;
    const RADIUS: f64 = 1.0e7;

    for i in 0..DIRECTIONS {
        let theta = (i as f64) * std::f64::consts::TAU / (DIRECTIONS as f64);
        let far_point = Point {
            x: RADIUS * theta.cos(),
            y: RADIUS * theta.sin(),
        };

        if feasibility::check_all(constraints, &far_point).is_ok() {
            let val = optimizer::evaluate_objective(objective, &far_point);
            let improves = if objective.maximize {
                val > best_known
            } else {
                val < best_known
            };
            if improves {
                return true;
            }
        }
    }
    false
}

// ---------------------------------------------------------------------
// WASM bindings. `#[cfg(target_arch = "wasm32")]` means "only compile
// this module when building for the wasm32 target". On a normal native
// `cargo build`/`cargo test` (what runs in this sandbox), this block is
// skipped entirely -- which is exactly why the rest of this crate is
// fully testable here without the wasm32 toolchain installed. When you
// (or CI) run `wasm-pack build` on a machine with the wasm32 target
// added, this module compiles in and becomes the JS-callable surface.
// ---------------------------------------------------------------------
#[cfg(target_arch = "wasm32")]
mod wasm_api {
    use super::*;
    use wasm_bindgen::prelude::*;

    /// JS-facing entry point. Accepts/returns JsValue (via serde-wasm-
    /// bindgen) so the TypeScript side works with plain objects instead
    /// of raw wasm memory offsets.
    #[wasm_bindgen]
    pub fn solve_lpp(
        constraints_js: JsValue,
        objective_js: JsValue,
        restrict_x_nonneg: bool,
        restrict_y_nonneg: bool,
    ) -> Result<JsValue, JsValue> {
        let constraints: Vec<Constraint> = serde_wasm_bindgen::from_value(constraints_js)
            .map_err(|e| JsValue::from_str(&format!("bad constraints: {e}")))?;
        let objective: Objective = serde_wasm_bindgen::from_value(objective_js)
            .map_err(|e| JsValue::from_str(&format!("bad objective: {e}")))?;

        let result = solve(&constraints, &objective, restrict_x_nonneg, restrict_y_nonneg);

        serde_wasm_bindgen::to_value(&result)
            .map_err(|e| JsValue::from_str(&format!("serialize error: {e}")))
    }
}
