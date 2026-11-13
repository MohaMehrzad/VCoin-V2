#!/usr/bin/env node
// Copies the Anchor-generated IDLs and TypeScript types of the ten programs into the SDK.
// Run after `anchor build`; CI fails if the committed copies differ (audit TOOL-10/SDK).
import { copyFileSync, existsSync, mkdirSync, rmSync } from "node:fs";
import { join } from "node:path";

const PROGRAMS = [
  "vcoin_token",
  "vevcoin_token",
  "staking_protocol",
  "private_poll",
  "sscre_protocol",
  "gasless_protocol",
  "five_a_protocol",
  "identity_protocol",
  "content_registry",
  "vilink_protocol",
];
const out = "packages/viwoapp-sdk/src/idl";
rmSync(out, { recursive: true, force: true });
mkdirSync(out, { recursive: true });
for (const p of PROGRAMS) {
  for (const [dir, ext] of [["target/idl", "json"], ["target/types", "ts"]]) {
    const src = join(dir, `${p}.${ext}`);
    if (!existsSync(src)) {
      console.error(`missing ${src}: run \`anchor build\` first`);
      process.exit(1);
    }
    copyFileSync(src, join(out, `${p}.${ext}`));
  }
}
console.log(`synced ${PROGRAMS.length} IDLs into ${out}`);
