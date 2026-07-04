"use client";

import { useEffect, useMemo, useRef, useState } from "react";
import { runSolver } from "@/lib/solverClient";
import { toSolverConstraint, toSolverObjective } from "@/lib/mapping";
import type { SolverResult, UiConstraint, UiObjective, VariableNames } from "@/lib/types";

let nextId = 1;
function makeId(): string {
  return `c${nextId++}`;
}

const DEBOUNCE_MS = 150;

export interface UseSolverState {
  constraints: UiConstraint[];
  objective: UiObjective;
  varNames: VariableNames;
  restrictXNonneg: boolean;
  restrictYNonneg: boolean;
  result: SolverResult | null;
  isSolving: boolean;
  error: string | null;
}

export interface UseSolverActions {
  addConstraint: () => void;
  updateConstraint: (id: string, patch: Partial<UiConstraint>) => void;
  removeConstraint: (id: string) => void;
  setObjective: (objective: UiObjective) => void;
  setVarNames: (names: VariableNames) => void;
  setRestrictXNonneg: (value: boolean) => void;
  setRestrictYNonneg: (value: boolean) => void;
}

export function useSolver(): UseSolverState & UseSolverActions {
  const [constraints, setConstraints] = useState<UiConstraint[]>(() => [
    { id: makeId(), coeffX: 1, coeffY: 2, op: "le", rhs: 800 },
    { id: makeId(), coeffX: 4, coeffY: 5, op: "le", rhs: 2300 },
  ]);
  const [objective, setObjective] = useState<UiObjective>({
    coeffX: 200,
    coeffY: 500,
    maximize: true,
  });
  const [varNames, setVarNames] = useState<VariableNames>({ x: "x", y: "y" });
  const [restrictXNonneg, setRestrictXNonneg] = useState(true);
  const [restrictYNonneg, setRestrictYNonneg] = useState(true);

  const [result, setResult] = useState<SolverResult | null>(null);
  const [isSolving, setIsSolving] = useState(false);
  const [error, setError] = useState<string | null>(null);

  // `useRef` for the debounce timer: we want to mutate this without
  // triggering a re-render, unlike state.
  const debounceTimer = useRef<ReturnType<typeof setTimeout> | null>(null);

  // Re-solve whenever any input changes, debounced so rapid typing
  // (e.g. dragging through digits) doesn't fire a solve on every
  // keystroke.
  useEffect(() => {
    if (debounceTimer.current) clearTimeout(debounceTimer.current);

    debounceTimer.current = setTimeout(() => {
      let cancelled = false;
      setIsSolving(true);
      setError(null);

      const solverConstraints = constraints.map(toSolverConstraint);
      const solverObjective = toSolverObjective(objective);

      runSolver(solverConstraints, solverObjective, restrictXNonneg, restrictYNonneg)
        .then((r) => {
          if (!cancelled) setResult(r);
        })
        .catch((e: unknown) => {
          if (!cancelled) {
            setError(e instanceof Error ? e.message : "Solver failed unexpectedly.");
            setResult(null);
          }
        })
        .finally(() => {
          if (!cancelled) setIsSolving(false);
        });

      // Note: this inner cancellation flag protects against a stale
      // response overwriting a newer one if solves resolve out of order;
      // the debounce above handles the common "don't solve every
      // keystroke" case.
      return () => {
        cancelled = true;
      };
    }, DEBOUNCE_MS);

    return () => {
      if (debounceTimer.current) clearTimeout(debounceTimer.current);
    };
  }, [constraints, objective, restrictXNonneg, restrictYNonneg]);

  const actions = useMemo<UseSolverActions>(
    () => ({
      addConstraint: () =>
        setConstraints((prev) => [...prev, { id: makeId(), coeffX: 1, coeffY: 1, op: "le", rhs: 100 }]),
      updateConstraint: (id, patch) =>
        setConstraints((prev) => prev.map((c) => (c.id === id ? { ...c, ...patch } : c))),
      removeConstraint: (id) => setConstraints((prev) => prev.filter((c) => c.id !== id)),
      setObjective,
      setVarNames,
      setRestrictXNonneg,
      setRestrictYNonneg,
    }),
    []
  );

  return {
    constraints,
    objective,
    varNames,
    restrictXNonneg,
    restrictYNonneg,
    result,
    isSolving,
    error,
    ...actions,
  };
}
