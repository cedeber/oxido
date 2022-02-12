import { threads, simd } from "wasm-feature-detect";

async function main() {
  // no SharedArrayBuffer -> no Threads
  console.log("SharedArrayBuffer", !!window.SharedArrayBuffer);

  const hasThreads = await threads();
  const hasSimd = await simd();

  console.log("Threads", hasThreads, navigator.hardwareConcurrency ?? 4);
  console.log("SIMD", hasSimd);

  new Worker(new URL("worker.js", import.meta.url), { type: "module" });
}

main();
