"use client";

import type { UiObjective, VariableNames } from "@/lib/types";

interface Props {
  objective: UiObjective;
  varNames: VariableNames;
  onChange: (objective: UiObjective) => void;
}

export default function ObjectiveEditor({ objective, varNames, onChange }: Props) {
  return (
    <div className="rounded-2xl border border-neutral-200 dark:border-neutral-800 p-4 space-y-3">
      <h2 className="font-semibold text-sm uppercase tracking-wide text-neutral-500">
        Objective
      </h2>
      <div className="flex flex-wrap items-center gap-2 text-sm">
        <select
          className="rounded-md border border-neutral-300 dark:border-neutral-700 bg-transparent px-2 py-1"
          value={objective.maximize ? "max" : "min"}
          onChange={(e) => onChange({ ...objective, maximize: e.target.value === "max" })}
        >
          <option value="max">Maximize</option>
          <option value="min">Minimize</option>
        </select>
        <span>Z =</span>
        <input
          type="number"
          className="w-20 rounded-md border border-neutral-300 dark:border-neutral-700 bg-transparent px-2 py-1"
          value={objective.coeffX}
          onChange={(e) => onChange({ ...objective, coeffX: Number(e.target.value) })}
        />
        <span>{varNames.x} +</span>
        <input
          type="number"
          className="w-20 rounded-md border border-neutral-300 dark:border-neutral-700 bg-transparent px-2 py-1"
          value={objective.coeffY}
          onChange={(e) => onChange({ ...objective, coeffY: Number(e.target.value) })}
        />
        <span>{varNames.y}</span>
      </div>
    </div>
  );
}
