/**
 * Anchor interop. `@coral-xyz/anchor` resolves to CommonJS under Node, whose named exports
 * ESM cannot detect statically (`import { BN } from "@coral-xyz/anchor"` throws in Node ESM).
 * Reading them from the namespace's default works in both module systems.
 */
import * as anchorNamespace from "@coral-xyz/anchor";

const anchor = ((anchorNamespace as unknown as { default?: typeof anchorNamespace }).default ?? anchorNamespace) as typeof anchorNamespace;

export const BN = anchor.BN;
export type BN = anchorNamespace.BN;
export const Program = anchor.Program;
export type Program<T extends anchorNamespace.Idl> = anchorNamespace.Program<T>;
export type { Idl, Provider } from "@coral-xyz/anchor";
