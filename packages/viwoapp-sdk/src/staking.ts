/**
 * staking-protocol: VCoin lockups (1 week – 4 years), 1:1 veVCoin receipts, and the Realms
 * voter-weight plugin. Weight math mirrors `programs/staking-protocol/src/weight.rs` exactly
 * (bigint, same rounding) so clients can predict the on-chain weight (audit SDK-12, ZK-10).
 */
import { BN } from "./anchor";
import { PublicKey, SystemProgram, TransactionInstruction } from "@solana/web3.js";

import { pda } from "./pda";
import { PROGRAM_IDS, type Programs } from "./programs";
import { associatedTokenAddress, receiptAta, TOKEN_2022_PROGRAM_ID, vcoinMint, vevcoinMint } from "./tokens";

export const FACTOR_SCALE = 1_000_000_000n;
export const MIN_LOCK_SECS = 7 * 24 * 60 * 60;
export const MAX_LOCK_SECS = 4 * 365 * 24 * 60 * 60;
export const MAX_DEPOSITS = 8;

export interface WeightParams {
  baselineFactor: bigint;
  maxExtraFactor: bigint;
  saturationSecs: bigint;
  minLockRemainingToVote: bigint;
}

export const DEFAULT_WEIGHT_PARAMS: WeightParams = {
  baselineFactor: FACTOR_SCALE,
  maxExtraFactor: FACTOR_SCALE,
  saturationSecs: BigInt(MAX_LOCK_SECS),
  minLockRemainingToVote: BigInt(4 * 24 * 60 * 60),
};

export interface Deposit {
  amount: bigint;
  lockEnd: bigint;
  isUsed: boolean;
}

type BnLike = BN | bigint | number;
const big = (v: BnLike): bigint => (BN.isBN(v) ? BigInt(v.toString()) : BigInt(v));

/** Converts a decoded `Voter` / `WeightParams` (BN fields) into plain bigints. */
export function toDeposits(voter: { deposits: { amount: BnLike; lockEnd: BnLike; isUsed: boolean }[] }): Deposit[] {
  return voter.deposits.map((d) => ({ amount: big(d.amount), lockEnd: big(d.lockEnd), isUsed: d.isUsed }));
}

export function toWeightParams(p: {
  baselineFactor: BnLike;
  maxExtraFactor: BnLike;
  saturationSecs: BnLike;
  minLockRemainingToVote: BnLike;
}): WeightParams {
  return {
    baselineFactor: big(p.baselineFactor),
    maxExtraFactor: big(p.maxExtraFactor),
    saturationSecs: big(p.saturationSecs),
    minLockRemainingToVote: big(p.minLockRemainingToVote),
  };
}

/** Weight of one deposit at unix time `t`. */
export function depositWeight(d: Deposit, p: WeightParams, t: bigint): bigint {
  if (!d.isUsed || d.amount === 0n) return 0n;
  const remaining = d.lockEnd - t;
  if (remaining <= 0n || remaining < p.minLockRemainingToVote) return 0n;
  const capped = remaining < p.saturationSecs ? remaining : p.saturationSecs;
  const baseline = (d.amount * p.baselineFactor) / FACTOR_SCALE;
  const extra = (d.amount * p.maxExtraFactor * capped) / (p.saturationSecs * FACTOR_SCALE);
  return baseline + extra;
}

/** Total weight of a voter's deposits at unix time `t`. */
export function voterWeight(deposits: Deposit[], p: WeightParams, t: bigint): bigint {
  return deposits.reduce((acc, d) => acc + depositWeight(d, p, t), 0n);
}

/** VCoin still locked at `t` (content-registry energy tiers use this). */
export function lockedAmount(deposits: Deposit[], t: bigint): bigint {
  return deposits.filter((d) => d.isUsed && d.lockEnd > t).reduce((acc, d) => acc + d.amount, 0n);
}

/** Accounts shared by deposit and withdraw. */
function moveStakeAccounts(owner: PublicKey, ownerVcoin: PublicKey) {
  return {
    registrar: pda.registrar(),
    voter: pda.voter(owner),
    owner,
    vault: pda.vault(),
    vcoinMint: vcoinMint(),
    ownerVcoin,
    vevcoinMint: vevcoinMint(),
    ownerVevcoin: receiptAta(owner),
    vevcoinConfig: pda.vevcoinConfig(),
    vevcoinProgram: PROGRAM_IDS.vevcoinToken,
    tokenProgram: TOKEN_2022_PROGRAM_ID,
  };
}

export function createVoterIx(p: Programs, owner: PublicKey, payer: PublicKey = owner): Promise<TransactionInstruction> {
  return p.staking.methods
    .createVoter()
    .accountsStrict({ registrar: pda.registrar(), voter: pda.voter(owner), owner, payer, systemProgram: SystemProgram.programId })
    .instruction();
}

/**
 * Locks `amount` base units in slot `index` for `lockSecs`. The receipt ATA must exist
 * (see `createAtaIdempotentIx(payer, owner, vevcoinMint())`).
 */
export function depositIx(
  p: Programs,
  args: { owner: PublicKey; index: number; amount: bigint; lockSecs: number; ownerVcoin?: PublicKey },
): Promise<TransactionInstruction> {
  const ownerVcoin = args.ownerVcoin ?? associatedTokenAddress(args.owner, vcoinMint());
  return p.staking.methods
    .deposit(args.index, new BN(args.amount.toString()), new BN(args.lockSecs))
    .accountsStrict(moveStakeAccounts(args.owner, ownerVcoin))
    .instruction();
}

export function extendLockIx(p: Programs, owner: PublicKey, index: number, lockSecs: number): Promise<TransactionInstruction> {
  return p.staking.methods
    .extendLock(index, new BN(lockSecs))
    .accountsStrict({ registrar: pda.registrar(), voter: pda.voter(owner), owner })
    .instruction();
}

/**
 * Withdraws unlocked VCoin. Pass the owner's Realms TokenOwnerRecord (see `governance.ts`);
 * the program refuses while it has unrelinquished votes, so relinquish first.
 */
export function withdrawIx(
  p: Programs,
  args: { owner: PublicKey; index: number; amount: bigint; tokenOwnerRecord: PublicKey; ownerVcoin?: PublicKey },
): Promise<TransactionInstruction> {
  const ownerVcoin = args.ownerVcoin ?? associatedTokenAddress(args.owner, vcoinMint());
  return p.staking.methods
    .withdraw(args.index, new BN(args.amount.toString()))
    .accountsStrict({ stake: moveStakeAccounts(args.owner, ownerVcoin), tokenOwnerRecord: args.tokenOwnerRecord })
    .instruction();
}

export function closeVoterIx(p: Programs, owner: PublicKey): Promise<TransactionInstruction> {
  return p.staking.methods.closeVoter().accountsStrict({ voter: pda.voter(owner), owner }).instruction();
}

/** Realms actions a voter-weight record can be written for. */
export type VoterWeightAction = "castVote" | "commentProposal" | "createGovernance" | "createProposal" | "signOffProposal";

export function createVoterWeightRecordIx(p: Programs, owner: PublicKey, payer: PublicKey = owner): Promise<TransactionInstruction> {
  return p.staking.methods
    .createVoterWeightRecord()
    .accountsStrict({
      registrar: pda.registrar(),
      voterWeightRecord: pda.voterWeightRecord(owner),
      owner,
      payer,
      systemProgram: SystemProgram.programId,
    })
    .instruction();
}

/**
 * Writes the owner's weight for one Realms action, valid for the current slot only: put it in
 * the same transaction as the Realms instruction, right before it.
 */
export function updateVoterWeightRecordIx(
  p: Programs,
  owner: PublicKey,
  action: VoterWeightAction,
  target: PublicKey | null,
): Promise<TransactionInstruction> {
  return p.staking.methods
    .updateVoterWeightRecord({ [action]: {} } as never, target)
    .accountsStrict({
      registrar: pda.registrar(),
      voter: pda.voter(owner),
      voterWeightRecord: pda.voterWeightRecord(owner),
      userScore: pda.userScore(owner),
    })
    .instruction();
}

export function updateMaxVoterWeightRecordIx(p: Programs): Promise<TransactionInstruction> {
  return p.staking.methods
    .updateMaxVoterWeightRecord()
    .accountsStrict({ registrar: pda.registrar(), maxVoterWeightRecord: pda.maxVoterWeightRecord() })
    .instruction();
}
