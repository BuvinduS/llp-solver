"use client";

import type { SolverResult, VariableNames } from "@/lib/types";

interface Props {
  result: SolverResult | null;
  varNames: VariableNames;
}

export default function VertexTable({ result, varNames }: Props) {
  if (!result || result.status !== "optimal" || result.feasible_vertices.length === 0) {
    return null;
  }

  const optimal = result.optimal_point;

  return (
    <div className="rounded-2xl border border-neutral-200 dark:border-neutral-800 p-4">
      <h2 className="font-semibold text-sm uppercase tracking-wide text-neutral-500 mb-3">
        Corner points
      </h2>
      <table className="w-full text-sm">
        <thead>
          <tr className="text-left text-neutral-500 text-xs uppercase tracking-wide">
            <th className="pb-2">{varNames.x}</th>
            <th className="pb-2">{varNames.y}</th>
            <th className="pb-2">Z</th>
          </tr>
        </thead>
        <tbody>
          {result.feasible_vertices.map((v, i) => {
            const isOptimal =
              optimal !== null &&
              Math.abs(v.point.x - optimal.x) < 1e-6 &&
              Math.abs(v.point.y - optimal.y) < 1e-6;
            return (
              <tr
                key={i}
                className={
                  isOptimal
                    ? "bg-green-500/10 text-green-700 dark:text-green-400 font-semibold"
                    : ""
                }
              >
                <td className="py-1 font-mono">{round(v.point.x)}</td>
                <td className="py-1 font-mono">{round(v.point.y)}</td>
                <td className="py-1 font-mono">{round(v.objective_value)}</td>
              </tr>
            );
          })}
        </tbody>
      </table>
    </div>
  );
}

function round(n: number): number {
  return Math.round(n * 1000) / 1000;
}
