import type { NextConfig } from "next";

const nextConfig: NextConfig = {
  // Static export: emits plain HTML/JS/CSS/WASM into `out/`, no Node
  // server required at runtime. This is what makes Cloudflare Pages
  // hosting (and GitHub Pages, if we ever switch) free and simple --
  // we're just serving static files, matching the "no backend" requirement.
  output: "export",

  // next/image's optimizer needs a running server by default. Static
  // export can't provide one, so we opt out and let the browser load
  // images as-is (fine for this app -- we're not using next/image for
  // anything perf-critical).
  images: {
    unoptimized: true,
  },
};

export default nextConfig;
