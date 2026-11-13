/**
 * Convenience wrapper: typed programs plus fetchers for the accounts apps read most.
 * Decoding always goes through the IDL coder.
 */
import { Connection, PublicKey } from "@solana/web3.js";

import { pda } from "./pda";
import { createPrograms, type Programs } from "./programs";
import { lockedAmount, toDeposits, toWeightParams, voterWeight } from "./staking";

export class ViWoClient {
  readonly programs: Programs;

  /** Uses the caller's connection (v2 silently created its own, audit SDK). */
  constructor(readonly connection: Connection) {
    this.programs = createPrograms(connection);
  }

  fetchRegistrar() {
    return this.programs.staking.account.registrar.fetch(pda.registrar());
  }

  fetchVoter(owner: PublicKey) {
    return this.programs.staking.account.voter.fetchNullable(pda.voter(owner));
  }

  /** Current Realms voting weight (base units) and VCoin still locked, computed locally. */
  async votingPower(owner: PublicKey, at = BigInt(Math.floor(Date.now() / 1000))): Promise<{ weight: bigint; locked: bigint }> {
    const [registrar, voter] = await Promise.all([this.fetchRegistrar(), this.fetchVoter(owner)]);
    if (!voter) return { weight: 0n, locked: 0n };
    const deposits = toDeposits(voter);
    return { weight: voterWeight(deposits, toWeightParams(registrar.weight), at), locked: lockedAmount(deposits, at) };
  }

  fetchUserScore(user: PublicKey) {
    return this.programs.fiveA.account.userScore.fetchNullable(pda.userScore(user));
  }

  fetchIdentity(owner: PublicKey) {
    return this.programs.identity.account.identity.fetchNullable(pda.identity(owner));
  }

  fetchUserEnergy(user: PublicKey) {
    return this.programs.content.account.userEnergy.fetchNullable(pda.userEnergy(user));
  }

  fetchEpoch(index: bigint | number) {
    return this.programs.sscre.account.epoch.fetchNullable(pda.epoch(index));
  }

  fetchPoll(pollId: bigint | number) {
    return this.programs.privatePoll.account.poll.fetchNullable(pda.poll(pollId));
  }

  fetchAction(creator: PublicKey, nonce: bigint | number) {
    return this.programs.vilink.account.viLinkAction.fetchNullable(pda.action(creator, nonce));
  }
}
