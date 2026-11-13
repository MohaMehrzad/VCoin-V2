// Resolves the built package by its own name (through "exports") in both module systems,
// the check v2's broken ESM/subpath exports would have failed (audit SDK-02, TOOL-10).
import { createRequire } from "node:module";

const require = createRequire(import.meta.url);
const cjs = require("@viwoapp/sdk");
const esm = await import("@viwoapp/sdk");
for (const [kind, mod] of [["cjs", cjs], ["esm", esm]]) {
  for (const name of ["createPrograms", "pda", "depositIx", "MerkleTree", "computeLeaf", "PROGRAM_IDS"]) {
    if (!(name in mod)) throw new Error(`${kind} build is missing ${name}`);
  }
}
const leaf = esm.computeLeaf(new esm.PROGRAM_IDS.staking.constructor(new Uint8Array(32).fill(7)), 1234567890n, 3n);
if (leaf.toString("hex") !== "5867436a6f21c025885194461701203778b53c261bd3f449af326ca8b9cf0083") {
  throw new Error("merkle vector mismatch in the built package");
}
console.log("smoke ok: cjs + esm entry points resolve and agree with the frozen vector");
