"use client";

import type { SolverResult, UiConstraint, VariableNames } from "@/lib/types";
import { formatConstraint } from "@/lib/mapping";

interface Props {
  result: SolverResult | null;
  constraints: UiConstraint[];
  varNames: VariableNames;
  isSolving: boolean;
  error: string | null;
}

export default function ResultsPanel({ result, constraints, varNames, isSolving, error }: Props) {
  return (
    <div className="rounded-2xl border border-neutral-200 dark:border-neutral-800 p-4 space-y-3">
      <h2 className="font-semibold text-sm uppercase tracking-wide text-neutral-500">
        Results {isSolving && <span className="normal-case text-neutral-400">(solving…)</span>}
      </h2>

      {error && <p className="text-sm text-red-500">{error}</p>}

      {!error && result?.status === "infeasible" && (
        <p className="text-sm text-amber-500">
          No feasible region — these constraints can&apos;t all be satisfied at once.
        </p>
      )}

      {!error && result?.status === "unbounded" && (
        <p className="text-sm text-amber-500">
          The feasible region is unbounded in a direction where the objective keeps improving —
          there&apos;s no finite optimum.
        </p>
      )}

      {!error && result?.status === "optimal" && result.optimal_point && (
        <div className="space-y-2 text-sm">
          <p>
            {varNames.x} = <span className="font-mono">{round(result.optimal_point.x)}</span>
          </p>
          <p>
            {varNames.y} = <span className="font-mono">{round(result.optimal_point.y)}</span>
          </p>
          <p className="font-semibold">
            Optimal Z = <span className="font-mono">{round(result.objective_value ?? 0)}</span>
          </p>

          {result.active_constraints.length > 0 && (
            <div className="pt-2">
              <p className="text-neutral-500 uppercase text-xs tracking-wide mb-1">
                Binding constraints
              </p>
              <ul className="space-y-0.5 font-mono text-xs">
                {result.active_constraints.map((idx) => (
                  <li key={idx}>{describeConstraintIndex(idx, constraints, varNames)}</li>
                ))}
              </ul>
            </div>
          )}
        </div>
      )}
    </div>
  );
}

function round(n: number): number {
  return Math.round(n * 1000) / 1000;
}

/**
 * Constraint indices from the solver refer to the *expanded* internal
 * list (user constraints, then x>=0, then y>=0 if enabled) -- see
 * solver-core's parser::build_constraint_list. We reconstruct a label for
 * user-authored constraints by index, and fall back to a generic label
 * for the synthetic non-negativity constraints appended after them.
 */
function describeConstraintIndex(
  idx: number,
  constraints: UiConstraint[],
  varNames: VariableNames
): string {
  if (idx < constraints.length) {
    return formatConstraint(constraints[idx], varNames);
  }
  const offset = idx - constraints.length;
  return offset === 0 ? `${varNames.x} \u2265 0` : `${varNames.y} \u2265 0`;
}
