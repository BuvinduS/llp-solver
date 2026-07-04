"use client";

import { useSolver } from "@/hooks/useSolver";
import ObjectiveEditor from "@/components/ObjectiveEditor";
import ConstraintEditor from "@/components/ConstraintEditor";
import ResultsPanel from "@/components/ResultsPanel";
import VertexTable from "@/components/VertexTable";
import Graph from "@/components/Graph";

export default function Home() {
  const solver = useSolver();

  return (
    <div className="min-h-screen p-6 md:p-10 max-w-6xl mx-auto space-y-6">
      <header>
        <h1 className="text-2xl font-bold">Linear Programming Graphical Solver</h1>
        <p className="text-sm text-neutral-500">
          Enter an objective and constraints — the feasible region, corner points, and optimal
          solution update live.
        </p>
      </header>

      <div className="grid md:grid-cols-2 gap-6">
        <div className="space-y-6">
          <ObjectiveEditor
            objective={solver.objective}
            varNames={solver.varNames}
            onChange={solver.setObjective}
          />
          <ConstraintEditor
            constraints={solver.constraints}
            varNames={solver.varNames}
            restrictXNonneg={solver.restrictXNonneg}
            restrictYNonneg={solver.restrictYNonneg}
            onAdd={solver.addConstraint}
            onUpdate={solver.updateConstraint}
            onRemove={solver.removeConstraint}
            onRestrictXChange={solver.setRestrictXNonneg}
            onRestrictYChange={solver.setRestrictYNonneg}
          />
          <Graph />
        </div>

        <div className="space-y-6">
          <ResultsPanel
            result={solver.result}
            constraints={solver.constraints}
            varNames={solver.varNames}
            isSolving={solver.isSolving}
            error={solver.error}
          />
          <VertexTable result={solver.result} varNames={solver.varNames} />
        </div>
      </div>
    </div>
  );
}
