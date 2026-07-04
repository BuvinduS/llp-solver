/**
 * TypeScript mirrors of the Rust types in `solver-core/src/types.rs`.
 *
 * Important: this file contains ONLY type shapes, never computation. All
 * actual solving logic lives in Rust/WASM -- duplicating that logic here
 * in JS would violate the "no duplicated logic" requirement and risk the
 * two implementations drifting apart. This file exists purely so
 * TypeScript can type-check the JSON-shaped data that crosses the
 * Rust -> JS boundary via serde-wasm-bindgen.
 *
 * Field names and casing must exactly match what serde produces on the
 * Rust side (see the `#[serde(rename_all = "lowercase")]` attributes in
 * types.rs) -- if the Rust struct changes, this file needs a matching
 * update.
 */

export type Operator = "le" | "ge" | "eq";

/** A single linear constraint: a*x + b*y <op> c */
export interface Constraint {
  a: number;
  b: number;
  op: Operator;
  c: number;
  label?: string | null;
}

/** The objective function: maximize/minimize Z = a*x + b*y */
export interface Objective {
  a: number;
  b: number;
  maximize: boolean;
}

export interface Point {
  x: number;
  y: number;
}

export interface Vertex {
  point: Point;
  active_constraint_indices: number[];
  objective_value: number;
}

export type SolutionStatus = "optimal" | "infeasible" | "unbounded";

export interface RejectedPoint {
  point: Point;
  violated_constraint_index: number;
}

export interface SolverResult {
  status: SolutionStatus;
  feasible_vertices: Vertex[];
  optimal_point: Point | null;
  objective_value: number | null;
  active_constraints: number[];
  feasible_polygon: Point[];
  rejected_points: RejectedPoint[];
}

/**
 * A constraint as the user edits it in the UI, before being mapped to the
 * internal a/b/c/op form the solver expects. Keeping the display-facing
 * shape (with a stable `id` for React keys and editing) separate from the
 * solver-facing `Constraint` shape means we can add variable-name
 * flexibility (Xa vs Xb, etc.) later without touching solver code.
 */
export interface UiConstraint {
  id: string;
  coeffX: number;
  coeffY: number;
  op: Operator;
  rhs: number;
}

export interface UiObjective {
  coeffX: number;
  coeffY: number;
  maximize: boolean;
}

export interface VariableNames {
  x: string;
  y: string;
}
