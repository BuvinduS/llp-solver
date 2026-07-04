"use client";

/**
 * Thin wrapper around the WASM solver module. This is the ONLY file that
 * touches the raw wasm-bindgen exports directly -- everything else in the
 * app (the useSolver hook, components) goes through the typed functions
 * below instead. That keeps the wasm-loading/init quirkiness contained to
 * one place.
 *
 * "use client": this module calls browser APIs (fetch, WebAssembly) that
 * don't exist during Next.js's server-side rendering pass, so anything
 * importing it must run client-side.
 */

import init, { solve_lpp } from "./wasm/solver_core";
import type { Constraint, Objective, SolverResult } from "./types";

// wasm modules only need to be fetched + instantiated once per page load.
// We cache the in-flight/completed init promise so concurrent callers
// (e.g. several components re-rendering) all await the same instance
// instead of re-initializing the module repeatedly.
let initPromise: Promise<unknown> | null = null;

function ensureWasmReady(): Promise<unknown> {
  if (!initPromise) {
    initPromise = init();
  }
  return initPromise;
}

/**
 * Runs the LP solver. Safe to call as often as you like (e.g. on every
 * keystroke via the debounced useSolver hook) -- WASM init only happens
 * once, and each call after that is a fast synchronous-feeling function
 * call into compiled Rust.
 */
export async function runSolver(
  constraints: Constraint[],
  objective: Objective,
  restrictXNonneg: boolean,
  restrictYNonneg: boolean
): Promise<SolverResult> {
  await ensureWasmReady();
  // The Rust side returns exactly the SolverResult shape (see
  // types.rs / lib.rs's wasm_api::solve_lpp) -- this cast is safe as long
  // as lib/types.ts stays in sync with the Rust struct definitions.
  return solve_lpp(
    constraints,
    objective,
    restrictXNonneg,
    restrictYNonneg
  ) as SolverResult;
}
