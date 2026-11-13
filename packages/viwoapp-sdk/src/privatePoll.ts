/**
 * private-poll: secret-ballot polls a Realms proposal can depend on (`assert_poll_passed`).
 *
 * Ballots are produced by the Rust `zk-voting-sdk` (built to wasm with
 * `wasm-pack build packages/zk-voting-sdk --features wasm`), whose `encryptBallot` returns
 * 640 bytes: 192 bytes of ciphertexts followed by the 448-byte proof. This module takes that
 * function as a parameter so the SDK does not bundle wasm.
 */
import { BN } from "./anchor";
import { PublicKey, SystemProgram, TransactionInstruction } from "@solana/web3.js";

import { pda } from "./pda";
import { PROGRAM_IDS, type Programs } from "./programs";
import { toDeposits, toWeightParams, voterWeight } from "./staking";

export const BALLOT_LEN = 192;
export const BALLOT_PROOF_LEN = 448;
/** Poll weight is counted in whole VCoin. */
export const WEIGHT_UNIT = 1_000_000_000n;

export type Choice = 0 | 1 | 2; // For, Against, Abstain

/** Signature of the wasm `encryptBallot` export. */
export type EncryptBallot = (
  programId: Uint8Array,
  poll: Uint8Array,
  voter: Uint8Array,
  electionPk: Uint8Array,
  weight: bigint,
  choice: number,
) => Uint8Array;

export function pollWeight(stakingWeight: bigint): bigint {
  return stakingWeight / WEIGHT_UNIT;
}

/**
 * The weight the program will assign to `voter` in `poll`: staking weight at the poll's end
 * time with the parameters snapshotted at creation, in whole VCoin.
 */
export async function expectedPollWeight(p: Programs, pollAddress: PublicKey, owner: PublicKey): Promise<bigint> {
  const poll = await p.privatePoll.account.poll.fetch(pollAddress);
  const voter = await p.staking.account.voter.fetch(pda.voter(owner));
  const w = voterWeight(toDeposits(voter), toWeightParams(poll.weightParams), BigInt(poll.endTime.toString()));
  return pollWeight(w);
}

export async function castVoteIx(
  p: Programs,
  args: { pollAddress: PublicKey; owner: PublicKey; ballot: Uint8Array; proof: Uint8Array },
): Promise<TransactionInstruction> {
  if (args.ballot.length !== BALLOT_LEN || args.proof.length !== BALLOT_PROOF_LEN) {
    throw new Error(`ballot must be ${BALLOT_LEN} bytes and proof ${BALLOT_PROOF_LEN}`);
  }
  return p.privatePoll.methods
    .castVote(Array.from(args.ballot), Array.from(args.proof))
    .accountsStrict({
      config: pda.pollConfig(),
      poll: args.pollAddress,
      registrar: pda.registrar(),
      stake: pda.voter(args.owner),
      owner: args.owner,
      userScore: pda.userScore(args.owner),
      ballotRecord: pda.ballot(args.pollAddress, args.owner),
      systemProgram: SystemProgram.programId,
    })
    .instruction();
}

/** Encrypts `choice` with the exact on-chain weight and builds the cast instruction. */
export async function encryptAndCastIx(
  p: Programs,
  encryptBallot: EncryptBallot,
  pollAddress: PublicKey,
  owner: PublicKey,
  choice: Choice,
): Promise<TransactionInstruction> {
  const poll = await p.privatePoll.account.poll.fetch(pollAddress);
  const weight = await expectedPollWeight(p, pollAddress, owner);
  if (weight === 0n) throw new Error("no stake locked through this poll's end");
  const out = encryptBallot(
    PROGRAM_IDS.privatePoll.toBytes(),
    pollAddress.toBytes(),
    owner.toBytes(),
    Uint8Array.from(poll.committee.electionPk),
    weight,
    choice,
  );
  return castVoteIx(p, { pollAddress, owner, ballot: out.subarray(0, BALLOT_LEN), proof: out.subarray(BALLOT_LEN) });
}

export function createPollIx(
  p: Programs,
  args: {
    pollId: bigint;
    creator: PublicKey;
    descriptionHash: Uint8Array;
    startTime: number;
    endTime: number;
    consumer: PublicKey;
  },
): Promise<TransactionInstruction> {
  return p.privatePoll.methods
    .createPoll(Array.from(args.descriptionHash), new BN(args.startTime), new BN(args.endTime), args.consumer)
    .accountsStrict({
      config: pda.pollConfig(),
      registrar: pda.registrar(),
      creatorVoter: pda.voter(args.creator),
      creator: args.creator,
      poll: pda.poll(args.pollId),
      systemProgram: SystemProgram.programId,
    })
    .instruction();
}

/** For a Realms proposal transaction, signed by the poll's consumer (the governance PDA). */
export function assertPollPassedIx(p: Programs, pollAddress: PublicKey, consumer: PublicKey): Promise<TransactionInstruction> {
  return p.privatePoll.methods
    .assertPollPassed()
    .accountsStrict({ config: pda.pollConfig(), poll: pollAddress, consumer })
    .instruction();
}
