/**
 * Program-derived addresses. Every seed is read from the IDL constants, so a seed changed in a
 * program shows up here on the next IDL sync (audit SDK-08).
 */
import { PublicKey } from "@solana/web3.js";

import { IDLS, PROGRAM_IDS, type ProgramName } from "./programs";

interface IdlConstant {
  name: string;
  type: unknown;
  value: string;
}

/** A `bytes` seed constant from a program's IDL. */
export function idlSeed(program: ProgramName, name: string): Buffer {
  const constants = (IDLS[program] as { constants?: IdlConstant[] }).constants ?? [];
  const c = constants.find((x) => x.name === name);
  if (!c) throw new Error(`seed ${name} not found in the ${program} IDL`);
  return Buffer.from(JSON.parse(c.value) as number[]);
}

export const SEEDS = {
  vcoinConfig: idlSeed("vcoinToken", "VCOIN_CONFIG_SEED"),
  vcoinMint: idlSeed("vcoinToken", "VCOIN_MINT_SEED"),
  vevcoinConfig: idlSeed("vevcoinToken", "VEVCOIN_CONFIG_SEED"),
  vevcoinMint: idlSeed("vevcoinToken", "VEVCOIN_MINT_SEED"),
  registrar: idlSeed("staking", "REGISTRAR_SEED"),
  vault: idlSeed("staking", "VAULT_SEED"),
  voter: idlSeed("staking", "VOTER_SEED"),
  voterWeightRecord: idlSeed("staking", "VOTER_WEIGHT_RECORD_SEED"),
  maxVoterWeightRecord: idlSeed("staking", "MAX_VOTER_WEIGHT_RECORD_SEED"),
  slash: idlSeed("staking", "SLASH_SEED"),
  pollConfig: idlSeed("privatePoll", "CONFIG_SEED"),
  poll: idlSeed("privatePoll", "POLL_SEED"),
  ballot: idlSeed("privatePoll", "BALLOT_SEED"),
  poolConfig: idlSeed("sscre", "POOL_CONFIG_SEED"),
  reserveVault: idlSeed("sscre", "RESERVE_VAULT_SEED"),
  epoch: idlSeed("sscre", "EPOCH_SEED"),
  epochVault: idlSeed("sscre", "EPOCH_VAULT_SEED"),
  claim: idlSeed("sscre", "CLAIM_SEED"),
  gaslessConfig: idlSeed("gasless", "GASLESS_CONFIG_SEED"),
  feeVault: idlSeed("gasless", "FEE_VAULT_SEED"),
  session: idlSeed("gasless", "SESSION_SEED"),
  fiveAConfig: idlSeed("fiveA", "FIVE_A_CONFIG_SEED"),
  userScore: idlSeed("fiveA", "USER_SCORE_SEED"),
  pendingScore: idlSeed("fiveA", "PENDING_SCORE_SEED"),
  vouchRecord: idlSeed("fiveA", "VOUCH_RECORD_SEED"),
  vouchStatus: idlSeed("fiveA", "VOUCH_STATUS_SEED"),
  voucherStats: idlSeed("fiveA", "VOUCHER_STATS_SEED"),
  identityConfig: idlSeed("identity", "IDENTITY_CONFIG_SEED"),
  identity: idlSeed("identity", "IDENTITY_SEED"),
  username: idlSeed("identity", "USERNAME_SEED"),
  attestation: idlSeed("identity", "ATTESTATION_SEED"),
  subscription: idlSeed("identity", "SUBSCRIPTION_SEED"),
  registryConfig: idlSeed("content", "REGISTRY_CONFIG_SEED"),
  userEnergy: idlSeed("content", "USER_ENERGY_SEED"),
  contentRecord: idlSeed("content", "CONTENT_RECORD_SEED"),
  vilinkConfig: idlSeed("vilink", "CONFIG_SEED"),
  action: idlSeed("vilink", "ACTION_SEED"),
  userStats: idlSeed("vilink", "USER_STATS_SEED"),
  receipt: idlSeed("vilink", "RECEIPT_SEED"),
} as const;

export function u64Le(n: bigint | number): Buffer {
  const b = Buffer.alloc(8);
  b.writeBigUInt64LE(BigInt(n));
  return b;
}

function find(seeds: (Buffer | Uint8Array)[], program: PublicKey): PublicKey {
  return PublicKey.findProgramAddressSync(seeds, program)[0];
}

const id = PROGRAM_IDS;

export const pda = {
  // vcoin / vevcoin
  vcoinConfig: () => find([SEEDS.vcoinConfig], id.vcoinToken),
  vcoinMint: () => find([SEEDS.vcoinMint], id.vcoinToken),
  vevcoinConfig: () => find([SEEDS.vevcoinConfig], id.vevcoinToken),
  vevcoinMint: () => find([SEEDS.vevcoinMint], id.vevcoinToken),
  // staking
  registrar: () => find([SEEDS.registrar], id.staking),
  vault: () => find([SEEDS.vault], id.staking),
  voter: (owner: PublicKey) => find([SEEDS.voter, owner.toBuffer()], id.staking),
  voterWeightRecord: (owner: PublicKey) => find([SEEDS.voterWeightRecord, owner.toBuffer()], id.staking),
  maxVoterWeightRecord: () => find([SEEDS.maxVoterWeightRecord], id.staking),
  slashRequest: (voter: PublicKey, nonce: bigint | number) =>
    find([SEEDS.slash, voter.toBuffer(), u64Le(nonce)], id.staking),
  // private poll
  pollConfig: () => find([SEEDS.pollConfig], id.privatePoll),
  poll: (pollId: bigint | number) => find([SEEDS.poll, u64Le(pollId)], id.privatePoll),
  ballot: (poll: PublicKey, voter: PublicKey) => find([SEEDS.ballot, poll.toBuffer(), voter.toBuffer()], id.privatePoll),
  // sscre
  poolConfig: () => find([SEEDS.poolConfig], id.sscre),
  reserveVault: () => find([SEEDS.reserveVault], id.sscre),
  epoch: (index: bigint | number) => find([SEEDS.epoch, u64Le(index)], id.sscre),
  epochVault: (index: bigint | number) => find([SEEDS.epochVault, u64Le(index)], id.sscre),
  claimStatus: (epoch: PublicKey, claimant: PublicKey) =>
    find([SEEDS.claim, epoch.toBuffer(), claimant.toBuffer()], id.sscre),
  // gasless
  gaslessConfig: () => find([SEEDS.gaslessConfig], id.gasless),
  feeVault: () => find([SEEDS.feeVault], id.gasless),
  session: (targetProgram: PublicKey, sessionSigner: PublicKey, authority: PublicKey) =>
    find([SEEDS.session, targetProgram.toBuffer(), sessionSigner.toBuffer(), authority.toBuffer()], id.gasless),
  // five-a
  fiveAConfig: () => find([SEEDS.fiveAConfig], id.fiveA),
  userScore: (user: PublicKey) => find([SEEDS.userScore, user.toBuffer()], id.fiveA),
  pendingScore: (user: PublicKey) => find([SEEDS.pendingScore, user.toBuffer()], id.fiveA),
  vouchRecord: (voucher: PublicKey, vouchee: PublicKey) =>
    find([SEEDS.vouchRecord, voucher.toBuffer(), vouchee.toBuffer()], id.fiveA),
  vouchStatus: (vouchee: PublicKey) => find([SEEDS.vouchStatus, vouchee.toBuffer()], id.fiveA),
  voucherStats: (voucher: PublicKey) => find([SEEDS.voucherStats, voucher.toBuffer()], id.fiveA),
  // identity
  identityConfig: () => find([SEEDS.identityConfig], id.identity),
  identity: (owner: PublicKey) => find([SEEDS.identity, owner.toBuffer()], id.identity),
  username: (name: string) => find([SEEDS.username, Buffer.from(name).subarray(0, 32)], id.identity),
  attestation: (attestationId: Uint8Array) => find([SEEDS.attestation, attestationId], id.identity),
  subscription: (user: PublicKey) => find([SEEDS.subscription, user.toBuffer()], id.identity),
  // content
  registryConfig: () => find([SEEDS.registryConfig], id.content),
  userEnergy: (user: PublicKey) => find([SEEDS.userEnergy, user.toBuffer()], id.content),
  contentRecord: (author: PublicKey, trackingId: Uint8Array) =>
    find([SEEDS.contentRecord, author.toBuffer(), trackingId], id.content),
  // vilink
  vilinkConfig: () => find([SEEDS.vilinkConfig], id.vilink),
  action: (creator: PublicKey, nonce: bigint | number) => find([SEEDS.action, creator.toBuffer(), u64Le(nonce)], id.vilink),
  userStats: (user: PublicKey) => find([SEEDS.userStats, user.toBuffer()], id.vilink),
  receipt: (action: PublicKey, executor: PublicKey) =>
    find([SEEDS.receipt, action.toBuffer(), executor.toBuffer()], id.vilink),
};
