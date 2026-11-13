/**
 * Typed Anchor clients for the ten VCoin programs, generated from the committed IDLs
 * (`src/idl`, synced from `anchor build` by `scripts/sync-idl.mjs`). Every account decoder and
 * instruction encoder comes from these IDLs, so the SDK cannot drift from the programs
 * (audit SDK-03..06, 09..11, 13, 14).
 */
import { Program, type Idl, type Provider } from "./anchor";
import { Connection, PublicKey } from "@solana/web3.js";

import contentRegistryIdl from "./idl/content_registry.json";
import fiveAIdl from "./idl/five_a_protocol.json";
import gaslessIdl from "./idl/gasless_protocol.json";
import identityIdl from "./idl/identity_protocol.json";
import privatePollIdl from "./idl/private_poll.json";
import sscreIdl from "./idl/sscre_protocol.json";
import stakingIdl from "./idl/staking_protocol.json";
import vcoinIdl from "./idl/vcoin_token.json";
import vevcoinIdl from "./idl/vevcoin_token.json";
import vilinkIdl from "./idl/vilink_protocol.json";
import type { ContentRegistry } from "./idl/content_registry";
import type { FiveAProtocol } from "./idl/five_a_protocol";
import type { GaslessProtocol } from "./idl/gasless_protocol";
import type { IdentityProtocol } from "./idl/identity_protocol";
import type { PrivatePoll } from "./idl/private_poll";
import type { SscreProtocol } from "./idl/sscre_protocol";
import type { StakingProtocol } from "./idl/staking_protocol";
import type { VcoinToken } from "./idl/vcoin_token";
import type { VevcoinToken } from "./idl/vevcoin_token";
import type { VilinkProtocol } from "./idl/vilink_protocol";

export type {
  ContentRegistry,
  FiveAProtocol,
  GaslessProtocol,
  IdentityProtocol,
  PrivatePoll,
  SscreProtocol,
  StakingProtocol,
  VcoinToken,
  VevcoinToken,
  VilinkProtocol,
};

/** Raw IDL JSON per program. */
export const IDLS = {
  vcoinToken: vcoinIdl,
  vevcoinToken: vevcoinIdl,
  staking: stakingIdl,
  privatePoll: privatePollIdl,
  sscre: sscreIdl,
  gasless: gaslessIdl,
  fiveA: fiveAIdl,
  identity: identityIdl,
  content: contentRegistryIdl,
  vilink: vilinkIdl,
} as const;

export type ProgramName = keyof typeof IDLS;

/** Program ids, read from the IDLs. */
export const PROGRAM_IDS: Record<ProgramName, PublicKey> = Object.fromEntries(
  Object.entries(IDLS).map(([k, idl]) => [k, new PublicKey(idl.address)]),
) as Record<ProgramName, PublicKey>;

export interface Programs {
  vcoinToken: Program<VcoinToken>;
  vevcoinToken: Program<VevcoinToken>;
  staking: Program<StakingProtocol>;
  privatePoll: Program<PrivatePoll>;
  sscre: Program<SscreProtocol>;
  gasless: Program<GaslessProtocol>;
  fiveA: Program<FiveAProtocol>;
  identity: Program<IdentityProtocol>;
  content: Program<ContentRegistry>;
  vilink: Program<VilinkProtocol>;
}

function program<T extends Idl>(idl: unknown, provider: Provider): Program<T> {
  return new Program<T>(idl as T, provider);
}

/**
 * Clients bound to the caller's connection (the SDK never creates its own). Building
 * instructions needs no wallet; send them with your own signer (see `tx.ts`).
 */
export function createPrograms(connection: Connection): Programs {
  const provider = { connection } as Provider;
  return {
    vcoinToken: program<VcoinToken>(vcoinIdl, provider),
    vevcoinToken: program<VevcoinToken>(vevcoinIdl, provider),
    staking: program<StakingProtocol>(stakingIdl, provider),
    privatePoll: program<PrivatePoll>(privatePollIdl, provider),
    sscre: program<SscreProtocol>(sscreIdl, provider),
    gasless: program<GaslessProtocol>(gaslessIdl, provider),
    fiveA: program<FiveAProtocol>(fiveAIdl, provider),
    identity: program<IdentityProtocol>(identityIdl, provider),
    content: program<ContentRegistry>(contentRegistryIdl, provider),
    vilink: program<VilinkProtocol>(vilinkIdl, provider),
  };
}
