/**
 * Realms (SPL Governance) integration. Proposals, votes, delegation and treasury execution run
 * in Realms; staking-protocol supplies voter weight. Build the Realms instructions with
 * `@solana/spl-governance` and wrap them with the helpers below so the plugin records are
 * fresh in the same transaction.
 */
import { PublicKey, TransactionInstruction } from "@solana/web3.js";

import type { Programs } from "./programs";
import { updateMaxVoterWeightRecordIx, updateVoterWeightRecordIx, type VoterWeightAction } from "./staking";

/** The SPL Governance program VCoin's realm runs on (mainnet and devnet). */
export const SPL_GOVERNANCE_PROGRAM_ID = new PublicKey("GovER5Lthms3bLBqWub97yVrMmEogzX7xNjdXpPPCVZw");
const GOVERNANCE_SEED = Buffer.from("governance");

export function realmAddress(name: string, programId = SPL_GOVERNANCE_PROGRAM_ID): PublicKey {
  return PublicKey.findProgramAddressSync([GOVERNANCE_SEED, Buffer.from(name)], programId)[0];
}

/** The owner's TokenOwnerRecord, which `withdrawIx` must be given. */
export function tokenOwnerRecordAddress(
  realm: PublicKey,
  governingTokenMint: PublicKey,
  owner: PublicKey,
  programId = SPL_GOVERNANCE_PROGRAM_ID,
): PublicKey {
  return PublicKey.findProgramAddressSync(
    [GOVERNANCE_SEED, realm.toBuffer(), governingTokenMint.toBuffer(), owner.toBuffer()],
    programId,
  )[0];
}

/**
 * Instructions to place before a Realms instruction that needs voter weight:
 * `[updateVoterWeightRecord(action, target), updateMaxVoterWeightRecord?, realmsIx]`.
 * Targets: CastVote → proposal, CreateProposal → governance, CreateGovernance → realm.
 * CastVote and FinalizeVote also read the max record, so include it for those.
 */
export async function withVoterWeight(
  p: Programs,
  owner: PublicKey,
  action: VoterWeightAction,
  target: PublicKey | null,
  realmsIx: TransactionInstruction,
  includeMax = action === "castVote",
): Promise<TransactionInstruction[]> {
  const ixs = [await updateVoterWeightRecordIx(p, owner, action, target)];
  if (includeMax) ixs.push(await updateMaxVoterWeightRecordIx(p));
  ixs.push(realmsIx);
  return ixs;
}
