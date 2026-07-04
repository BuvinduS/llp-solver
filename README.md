# Linear Programming Graphical Solver

An educational tool for solving and visualizing two-variable linear
programming problems using the graphical method: enter an objective and
constraints, see the feasible region, corner points, and optimal solution
update live.

## Repo layout

```
solver-core/     Rust crate — the actual LP solving logic (geometry,
                  feasibility, optimization). Pure Rust, no UI dependency,
                  tested with `cargo test`. Compiled to WebAssembly for
                  use in the browser.
frontend/        Next.js (App Router, TypeScript, Tailwind CSS) app.
                  Static export — no backend/server at runtime.
.github/
  workflows/
    build-wasm.yml   Compiles solver-core to WASM on every push and
                      commits the output into frontend/lib/wasm/, so the
                      frontend never needs Rust installed locally.
```

## Why Rust compiled to WebAssembly?

The solver (line intersections, feasibility checks, polygon construction,
optimization) is pure numeric/geometric computation with zero UI
dependency — a natural fit for Rust, and for being compiled once to a
small `.wasm` file the frontend calls into like any other JS module. It's
still a fully static, backend-free app: the `.wasm` file is just another
static asset served alongside the JS/CSS.

## Local development

You only need **Node.js** installed locally — not Rust:

```bash
cd frontend
npm install
npm run dev
```

This works because `frontend/lib/wasm/` (the compiled solver) is checked
into the repo, kept up to date automatically by the GitHub Actions
workflow described below. You never need to run `wasm-pack` yourself
unless you're specifically editing the Rust code.

### If you do want to touch the Rust solver

You'll need `rustc`/`cargo` (via [rustup](https://rustup.rs)) and the
`wasm32-unknown-unknown` target:

```bash
rustup target add wasm32-unknown-unknown
cargo install wasm-pack

cd solver-core
cargo test          # run the solver's own test suite

wasm-pack build --target web --out-dir ../frontend/lib/wasm --out-name solver_core
```

Or simpler: just edit `solver-core/`, run `cargo test` locally to check
your logic, then push — GitHub Actions will do the WASM build and commit
step for you.

## CI/CD pipeline

1. **Push to `main`** touching `solver-core/**` →
   `.github/workflows/build-wasm.yml` runs on GitHub's servers: installs
   Rust + the wasm32 target, runs `cargo test` as a safety gate, runs
   `wasm-pack build`, and commits the compiled output back into
   `frontend/lib/wasm/`.
2. **Cloudflare Pages** (connected via their dashboard's Git integration,
   not a workflow file) auto-deploys on every push:
   - Framework preset: **Next.js (Static HTML Export)**
   - Root directory: `frontend`
   - Build command: `npm run build`
   - Build output directory: `out`

Because the frontend is a static export (`output: "export"` in
`next.config.ts`), Cloudflare serves it as plain static files — no
server-side Next.js runtime, no Cloudflare Workers/edge functions needed.

## Solver architecture

See `solver-core/src/`:

- `types.rs` — shared data types (`Constraint`, `Objective`, `Point`,
  `SolverResult`, etc.)
- `geometry.rs` — line intersection math, float-tolerance comparisons
- `parser.rs` — expands user constraints + `x≥0`/`y≥0` into the full
  internal constraint list
- `intersections.rs` — enumerates every pairwise boundary-line
  intersection (candidate corner points)
- `feasibility.rs` — filters candidates against every constraint, orders
  the feasible region into a drawable polygon
- `optimizer.rs` — evaluates the objective at each feasible vertex, picks
  the optimum
- `lib.rs` — orchestrates the full `solve()` pipeline; also hosts the
  `#[wasm_bindgen]`-exported JS-facing function (compiled only when
  targeting `wasm32`)

`solver-core/tests/solver_tests.rs` has integration tests including the
project's worked example (Maximize `Z = 200x + 500y`, several constraints)
verified against a hand-solved optimum of `(200, 300)`, `Z = 190000`.
