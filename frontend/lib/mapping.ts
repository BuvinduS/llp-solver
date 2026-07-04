/**
 * Maps UI-facing constraint/objective shapes to the solver's a/b/c/op
 * form. This is intentionally trivial (no math, no LP logic) -- it just
 * repackages fields. Keeping it separate from lib/types.ts and
 * solverClient.ts means the UI is free to grow richer editing affordances
 * (reordering, per-row validation, etc.) without ever touching how data
 * reaches the solver.
 */

import type { Constraint, Objective, UiConstraint, UiObjective } from "./types";

export function toSolverConstraint(uiConstraint: UiConstraint): Constraint {
  return {
    a: uiConstraint.coeffX,
    b: uiConstraint.coeffY,
    op: uiConstraint.op,
    c: uiConstraint.rhs,
    label: null, // frontend builds display labels itself via formatConstraint()
  };
}

export function toSolverObjective(uiObjective: UiObjective): Objective {
  return {
    a: uiObjective.coeffX,
    b: uiObjective.coeffY,
    maximize: uiObjective.maximize,
  };
}

const OPERATOR_SYMBOLS: Record<UiConstraint["op"], string> = {
  le: "\u2264",
  ge: "\u2265",
  eq: "=",
};

/** Human-readable "2x + 5y ≤ 100" style label, using the current variable names. */
export function formatConstraint(
  c: UiConstraint,
  varNames: { x: string; y: string }
): string {
  const parts: string[] = [];
  if (c.coeffX !== 0) parts.push(`${formatCoeff(c.coeffX)}${varNames.x}`);
  if (c.coeffY !== 0) parts.push(`${formatCoeff(c.coeffY, parts.length > 0)}${varNames.y}`);
  const lhs = parts.length > 0 ? parts.join(" ") : "0";
  return `${lhs} ${OPERATOR_SYMBOLS[c.op]} ${c.rhs}`;
}

function formatCoeff(value: number, needsSign = false): string {
  const abs = Math.abs(value);
  const magnitude = abs === 1 ? "" : String(abs);
  if (needsSign) {
    return value < 0 ? ` - ${magnitude}` : ` + ${magnitude}`;
  }
  return value < 0 ? `-${magnitude}` : magnitude;
}
