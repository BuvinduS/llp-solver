"use client";

import type { UiConstraint, VariableNames } from "@/lib/types";

interface Props {
  constraints: UiConstraint[];
  varNames: VariableNames;
  restrictXNonneg: boolean;
  restrictYNonneg: boolean;
  onAdd: () => void;
  onUpdate: (id: string, patch: Partial<UiConstraint>) => void;
  onRemove: (id: string) => void;
  onRestrictXChange: (value: boolean) => void;
  onRestrictYChange: (value: boolean) => void;
}

export default function ConstraintEditor({
  constraints,
  varNames,
  restrictXNonneg,
  restrictYNonneg,
  onAdd,
  onUpdate,
  onRemove,
  onRestrictXChange,
  onRestrictYChange,
}: Props) {
  return (
    <div className="rounded-2xl border border-neutral-200 dark:border-neutral-800 p-4 space-y-3">
      <div className="flex items-center justify-between">
        <h2 className="font-semibold text-sm uppercase tracking-wide text-neutral-500">
          Constraints
        </h2>
        <button
          onClick={onAdd}
          className="text-sm rounded-md bg-green-500/10 text-green-600 dark:text-green-400 px-3 py-1 hover:bg-green-500/20 transition-colors"
        >
          + Add constraint
        </button>
      </div>

      <div className="space-y-2">
        {constraints.map((c) => (
          <div key={c.id} className="flex flex-wrap items-center gap-2 text-sm">
            <input
              type="number"
              className="w-16 rounded-md border border-neutral-300 dark:border-neutral-700 bg-transparent px-2 py-1"
              value={c.coeffX}
              onChange={(e) => onUpdate(c.id, { coeffX: Number(e.target.value) })}
            />
            <span>{varNames.x} +</span>
            <input
              type="number"
              className="w-16 rounded-md border border-neutral-300 dark:border-neutral-700 bg-transparent px-2 py-1"
              value={c.coeffY}
              onChange={(e) => onUpdate(c.id, { coeffY: Number(e.target.value) })}
            />
            <span>{varNames.y}</span>
            <select
              className="rounded-md border border-neutral-300 dark:border-neutral-700 bg-transparent px-2 py-1"
              value={c.op}
              onChange={(e) => onUpdate(c.id, { op: e.target.value as UiConstraint["op"] })}
            >
              <option value="le">&le;</option>
              <option value="ge">&ge;</option>
              <option value="eq">=</option>
            </select>
            <input
              type="number"
              className="w-20 rounded-md border border-neutral-300 dark:border-neutral-700 bg-transparent px-2 py-1"
              value={c.rhs}
              onChange={(e) => onUpdate(c.id, { rhs: Number(e.target.value) })}
            />
            <button
              onClick={() => onRemove(c.id)}
              className="ml-auto text-neutral-400 hover:text-red-500 transition-colors"
              aria-label="Delete constraint"
            >
              ✕
            </button>
          </div>
        ))}
        {constraints.length === 0 && (
          <p className="text-sm text-neutral-500">No constraints yet — add one above.</p>
        )}
      </div>

      <div className="pt-2 border-t border-neutral-200 dark:border-neutral-800 flex gap-4 text-sm">
        <label className="flex items-center gap-2">
          <input
            type="checkbox"
            checked={restrictXNonneg}
            onChange={(e) => onRestrictXChange(e.target.checked)}
          />
          {varNames.x} &ge; 0
        </label>
        <label className="flex items-center gap-2">
          <input
            type="checkbox"
            checked={restrictYNonneg}
            onChange={(e) => onRestrictYChange(e.target.checked)}
          />
          {varNames.y} &ge; 0
        </label>
      </div>
    </div>
  );
}
