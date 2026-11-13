/**
 * User-facing builders for gasless sessions, content-registry, vilink, identity and five-a.
 *
 * Session keys (gasless-protocol) let a short-lived key post / edit content and vouch / follow
 * on ViLink for a wallet; transfers, tips, claims, staking and votes always need the wallet.
 */
import { BN } from "./anchor";
import { PublicKey, SystemProgram, TransactionInstruction } from "@solana/web3.js";

import { pda } from "./pda";
import { PROGRAM_IDS, type Programs } from "./programs";
import { associatedTokenAddress, TOKEN_2022_PROGRAM_ID, vcoinMint } from "./tokens";

// ---------------------------------------------------------------------------- gasless

export const SCOPE_VOUCH = 1 << 1;
export const SCOPE_CONTENT = 1 << 2;
export const SCOPE_FOLLOW = 1 << 7;
/** Sessions last at most 24h. */
export const MAX_SESSION_SECS = 24 * 60 * 60;

/** `authority` and `sessionSigner` both sign; `payer` (e.g. a relayer) funds the rent. */
export function createSessionIx(
  p: Programs,
  args: { authority: PublicKey; sessionSigner: PublicKey; payer: PublicKey; targetProgram: PublicKey; scope: number; validUntil: number },
): Promise<TransactionInstruction> {
  return p.gasless.methods
    .createSession(args.targetProgram, args.scope, new BN(args.validUntil))
    .accountsStrict({
      config: pda.gaslessConfig(),
      session: pda.session(args.targetProgram, args.sessionSigner, args.authority),
      authority: args.authority,
      sessionSigner: args.sessionSigner,
      payer: args.payer,
      systemProgram: SystemProgram.programId,
    })
    .instruction();
}

export function revokeSessionIx(
  p: Programs,
  args: { session: PublicKey; payer: PublicKey; revoker: PublicKey },
): Promise<TransactionInstruction> {
  return p.gasless.methods
    .revokeSession()
    .accountsStrict({ session: args.session, payer: args.payer, revoker: args.revoker })
    .instruction();
}

// ---------------------------------------------------------------------------- content

export const CONTENT_TYPES = { post: 0, article: 1, media: 2, nft: 3, thread: 4 } as const;
export const TIER_THRESHOLDS = [1_000n, 5_000n, 20_000n, 100_000n].map((v) => v * 1_000_000_000n);

/** Energy tier for VCoin locked in staking (0..=4), as the program computes it. */
export function tierForLocked(locked: bigint): number {
  return TIER_THRESHOLDS.filter((t) => locked >= t).length;
}

/**
 * Opens `user`'s energy account. The staking voter address is always passed (the program
 * reads the tier from it; a wallet that never staked has no account there and gets tier 0).
 */
export function initUserEnergyIx(p: Programs, user: PublicKey, payer: PublicKey): Promise<TransactionInstruction> {
  return p.content.methods
    .initUserEnergy()
    .accountsStrict({
      config: pda.registryConfig(),
      user,
      userEnergy: pda.userEnergy(user),
      stake: pda.voter(user),
      payer,
      systemProgram: SystemProgram.programId,
    })
    .instruction();
}

/**
 * Posts as `author`. `signer` is the author or a CONTENT session key for the author. `payer`
 * funds the record's rent and gets it back if the author deletes the post.
 */
export function createContentIx(
  p: Programs,
  args: {
    author: PublicKey;
    signer?: PublicKey;
    payer?: PublicKey;
    trackingId: Uint8Array;
    contentHash: Uint8Array;
    uri: string;
    contentType: number;
  },
): Promise<TransactionInstruction> {
  const signer = args.signer ?? args.author;
  return p.content.methods
    .createContent(Array.from(args.trackingId), Array.from(args.contentHash), args.uri, args.contentType)
    .accountsStrict({
      config: pda.registryConfig(),
      author: args.author,
      signer,
      session: signer.equals(args.author) ? null : pda.session(PROGRAM_IDS.content, signer, args.author),
      userEnergy: pda.userEnergy(args.author),
      stake: pda.voter(args.author),
      content: pda.contentRecord(args.author, args.trackingId),
      payer: args.payer ?? signer,
      systemProgram: SystemProgram.programId,
    })
    .instruction();
}

export function editContentIx(
  p: Programs,
  args: { author: PublicKey; signer?: PublicKey; trackingId: Uint8Array; contentHash: Uint8Array; uri: string },
): Promise<TransactionInstruction> {
  const signer = args.signer ?? args.author;
  return p.content.methods
    .editContent(Array.from(args.contentHash), args.uri)
    .accountsStrict({
      config: pda.registryConfig(),
      author: args.author,
      signer,
      session: signer.equals(args.author) ? null : pda.session(PROGRAM_IDS.content, signer, args.author),
      userEnergy: pda.userEnergy(args.author),
      stake: pda.voter(args.author),
      content: pda.contentRecord(args.author, args.trackingId),
    })
    .instruction();
}

/**
 * Deletes a post (main wallet only). `rentPayer` must be the account that paid for the
 * record: read it from `ContentRecord.rentPayer`. It is the author, a session key or the
 * relayer that sponsored the post, so there is no safe default.
 */
export function deleteContentIx(
  p: Programs,
  author: PublicKey,
  trackingId: Uint8Array,
  rentPayer: PublicKey,
): Promise<TransactionInstruction> {
  return p.content.methods
    .deleteContent()
    .accountsStrict({ content: pda.contentRecord(author, trackingId), author, rentPayer })
    .instruction();
}

export function claimRefundIx(p: Programs, author: PublicKey, trackingId: Uint8Array): Promise<TransactionInstruction> {
  return p.content.methods
    .claimRefund()
    .accountsStrict({
      config: pda.registryConfig(),
      author,
      userEnergy: pda.userEnergy(author),
      stake: pda.voter(author),
      content: pda.contentRecord(author, trackingId),
    })
    .instruction();
}

// ---------------------------------------------------------------------------- vilink

export const ACTION_TYPES = { tip: 0, vouch: 1, follow: 2 } as const;

/** Tip fee the program takes (rounded up, capped at the amount). */
export function tipFee(amount: bigint, feeBps: number): bigint {
  const fee = (amount * BigInt(feeBps) + 9_999n) / 10_000n;
  return fee < amount ? fee : amount;
}

/** `nonce` must equal the creator's `UserActionStats.actionsCreated` (0 for a new user). */
export function createActionIx(
  p: Programs,
  args: {
    creator: PublicKey;
    nonce: bigint;
    actionType: number;
    amount: bigint;
    target: PublicKey;
    metadataHash?: Uint8Array;
    expirySecs: number;
    maxExecutions: number;
  },
): Promise<TransactionInstruction> {
  return p.vilink.methods
    .createAction(
      args.actionType,
      new BN(args.amount.toString()),
      args.target,
      Array.from(args.metadataHash ?? new Uint8Array(32)),
      new BN(args.expirySecs),
      args.maxExecutions,
      new BN(args.nonce.toString()),
    )
    .accountsStrict({
      config: pda.vilinkConfig(),
      creator: args.creator,
      creatorStats: pda.userStats(args.creator),
      action: pda.action(args.creator, args.nonce),
      systemProgram: SystemProgram.programId,
    })
    .instruction();
}

/** Tips move tokens: the executor's wallet signs. */
export function executeTipIx(
  p: Programs,
  args: { action: PublicKey; target: PublicKey; executor: PublicKey; treasury: PublicKey; executorToken?: PublicKey; targetToken?: PublicKey },
): Promise<TransactionInstruction> {
  const mint = vcoinMint();
  return p.vilink.methods
    .executeTip()
    .accountsStrict({
      config: pda.vilinkConfig(),
      action: args.action,
      executor: args.executor,
      executorToken: args.executorToken ?? associatedTokenAddress(args.executor, mint),
      targetToken: args.targetToken ?? associatedTokenAddress(args.target, mint),
      treasury: args.treasury,
      vcoinMint: mint,
      receipt: pda.receipt(args.action, args.executor),
      executorStats: pda.userStats(args.executor),
      targetStats: pda.userStats(args.target),
      tokenProgram: TOKEN_2022_PROGRAM_ID,
      systemProgram: SystemProgram.programId,
    })
    .instruction();
}

/** Vouch or follow; `signer` is the executor or its VOUCH / FOLLOW session key. */
export function executeSocialIx(
  p: Programs,
  kind: "vouch" | "follow",
  args: { action: PublicKey; target: PublicKey; executor: PublicKey; signer?: PublicKey; payer?: PublicKey },
): Promise<TransactionInstruction> {
  const signer = args.signer ?? args.executor;
  const accounts = {
    config: pda.vilinkConfig(),
    action: args.action,
    executor: args.executor,
    signer,
    session: signer.equals(args.executor) ? null : pda.session(PROGRAM_IDS.vilink, signer, args.executor),
    receipt: pda.receipt(args.action, args.executor),
    executorStats: pda.userStats(args.executor),
    targetStats: pda.userStats(args.target),
    payer: args.payer ?? signer,
    systemProgram: SystemProgram.programId,
  };
  return kind === "vouch"
    ? p.vilink.methods.executeVouch().accountsStrict(accounts).instruction()
    : p.vilink.methods.executeFollow().accountsStrict(accounts).instruction();
}

// ---------------------------------------------------------------------------- identity

/** Claim bits an attester can certify. */
export const CLAIMS = { email: 1, phone: 2, social: 4, kyc: 8, biometric: 16, uniqueHuman: 32 } as const;

/** Level for a claims bitmap; KYC levels require the KYC claim. */
export function levelForClaims(claims: number): number {
  const has = (b: number) => (claims & b) !== 0;
  if (has(CLAIMS.kyc) && has(CLAIMS.biometric) && has(CLAIMS.uniqueHuman)) return 4;
  if (has(CLAIMS.kyc) && has(CLAIMS.biometric)) return 3;
  if (has(CLAIMS.kyc)) return 2;
  if (has(CLAIMS.email) && has(CLAIMS.phone)) return 1;
  return 0;
}

export function isValidUsername(name: string): boolean {
  return /^[a-z0-9_]{3,32}$/.test(name);
}

export function createIdentityIx(p: Programs, owner: PublicKey, didHash: Uint8Array, username: string): Promise<TransactionInstruction> {
  if (!isValidUsername(username)) throw new Error("usernames are 3-32 characters of a-z, 0-9 and _");
  return p.identity.methods
    .createIdentity(Array.from(didHash), username)
    .accountsStrict({
      config: pda.identityConfig(),
      owner,
      identity: pda.identity(owner),
      usernameRecord: pda.username(username),
      systemProgram: SystemProgram.programId,
    })
    .instruction();
}

/** Owner and a trusted attester both sign. */
export function linkAttestationIx(
  p: Programs,
  args: { owner: PublicKey; attester: PublicKey; attestationId: Uint8Array; claims: number; expiresAt: number },
): Promise<TransactionInstruction> {
  return p.identity.methods
    .linkAttestation(Array.from(args.attestationId), args.claims, new BN(args.expiresAt))
    .accountsStrict({
      config: pda.identityConfig(),
      identity: pda.identity(args.owner),
      owner: args.owner,
      attester: args.attester,
      attestationLink: pda.attestation(args.attestationId),
      systemProgram: SystemProgram.programId,
    })
    .instruction();
}

/** `maxPayment` caps the USDC charge (base units, 6 decimals). */
export function subscribeIx(
  p: Programs,
  args: { user: PublicKey; tier: 1 | 2 | 3; maxPayment: bigint; usdcMint: PublicKey; treasuryToken: PublicKey; userUsdc: PublicKey; tokenProgram: PublicKey },
): Promise<TransactionInstruction> {
  return p.identity.methods
    .subscribe(args.tier, new BN(args.maxPayment.toString()))
    .accountsStrict({
      config: pda.identityConfig(),
      subscription: pda.subscription(args.user),
      user: args.user,
      userUsdc: args.userUsdc,
      treasuryToken: args.treasuryToken,
      usdcMint: args.usdcMint,
      tokenProgram: args.tokenProgram,
      systemProgram: SystemProgram.programId,
    })
    .instruction();
}

// ---------------------------------------------------------------------------- five-a

/** An established user (≥ 60%) vouches for a new or low-score user. */
export function vouchForUserIx(p: Programs, voucher: PublicKey, vouchee: PublicKey): Promise<TransactionInstruction> {
  return p.fiveA.methods
    .vouchForUser()
    .accountsStrict({
      config: pda.fiveAConfig(),
      voucher,
      voucherScore: pda.userScore(voucher),
      vouchee,
      voucheeScore: pda.userScore(vouchee),
      vouchRecord: pda.vouchRecord(voucher, vouchee),
      reverseVouch: pda.vouchRecord(vouchee, voucher),
      voucherStats: pda.voucherStats(voucher),
      voucheeStatus: pda.vouchStatus(vouchee),
      systemProgram: SystemProgram.programId,
    })
    .instruction();
}
