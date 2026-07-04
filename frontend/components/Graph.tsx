"use client";

/**
 * Placeholder. The full D3-based graph (constraint lines, shaded feasible
 * region, corner points, pan/zoom/reset) is Step 3 in our build order --
 * this component exists now just so the layout/page structure is final
 * and we're not restructuring `page.tsx` again later.
 */
export default function Graph() {
  return (
    <div className="rounded-2xl border border-dashed border-neutral-300 dark:border-neutral-700 p-8 flex items-center justify-center text-sm text-neutral-400 min-h-[320px]">
      Graph visualization goes here (Step 3 — D3 implementation)
    </div>
  );
}
