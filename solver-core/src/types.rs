//! Core data types shared across the solver.
//!
//! Rust notes for readers new to the language:
//! - `pub` makes an item visible outside this file/module.
//! - `#[derive(...)]` auto-generates trait implementations. `Debug` lets
//!   you `println!("{:?}", value)`. `Clone` lets you explicitly copy a
//!   value. `Serialize`/`Deserialize` (from serde) auto-generate the
//!   to/from-JSON-like-data code -- this is what lets these structs cross
//!   the Rust <-> JavaScript boundary as plain objects later.
//! - `Option<T>` is Rust's null-safety type: a value is either `Some(x)`
//!   or `None`, and the compiler forces you to handle both cases. No nulls.

use serde::{Deserialize, Serialize};

/// Floating point tolerance used everywhere we compare two numbers "close
/// enough to be equal" -- necessary because line-intersection math
/// produces results like 299.99999999997 instead of exactly 300.0.
pub const EPS: f64 = 1e-7;

/// Comparison operator used in a constraint: a*x + b*y <op> c
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Operator {
    Le, // <=
    Ge, // >=
    Eq, // =
}

/// A single linear constraint: a*x + b*y <op> c
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Constraint {
    pub a: f64,
    pub b: f64,
    pub op: Operator,
    pub c: f64,
    /// Human-readable label (e.g. "4x + 5y <= 2300") for UI display and
    /// Educational Mode explanations. The frontend can generate this from
    /// variable names, so it's optional here.
    pub label: Option<String>,
}

/// The objective function: maximize/minimize Z = a*x + b*y
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Objective {
    pub a: f64,
    pub b: f64,
    pub maximize: bool,
}

/// A 2D point. `Copy` (in addition to `Clone`) means it's cheap enough
/// (just two f64s) that Rust will implicitly copy it by value wherever
/// needed, instead of making us think about ownership/borrowing for it.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Point {
    pub x: f64,
    pub y: f64,
}

/// A vertex (corner point) of the feasible region, plus bookkeeping used
/// by the results panel and Educational Mode.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Vertex {
    pub point: Point,
    /// Indices into the solved constraint list of every constraint whose
    /// boundary line passes exactly through this point.
    pub active_constraint_indices: Vec<usize>,
    pub objective_value: f64,
}

/// Overall status of a solve attempt.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum SolutionStatus {
    Optimal,
    Infeasible,
    Unbounded,
}

/// A candidate corner point that got rejected, and which constraint it
/// violated -- powers Educational Mode's "why this intersection doesn't
/// count" explanations.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RejectedPoint {
    pub point: Point,
    pub violated_constraint_index: usize,
}

/// Full result returned by the solver.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SolverResult {
    pub status: SolutionStatus,
    pub feasible_vertices: Vec<Vertex>,
    pub optimal_point: Option<Point>,
    pub objective_value: Option<f64>,
    /// Constraint indices active (binding) at the optimal point.
    pub active_constraints: Vec<usize>,
    /// Feasible region vertices ordered into a closed polygon, ready for
    /// the frontend to shade directly.
    pub feasible_polygon: Vec<Point>,
    pub rejected_points: Vec<RejectedPoint>,
}
