# Integration guide

Use the TypeScript SDK (`packages/viwoapp-sdk`). Its README has the code for every flow. This
page lists what an integrating app must get right.

## Accounts and addresses

- Derive every address with `pda.*` (the seeds come from the IDLs). Never hard-code PDAs.
- The VCoin mint is `vcoinMint()` (a PDA of vcoin-token). It is Token-2022 with 9 decimals, so
  use `TOKEN_2022_PROGRAM_ID` and Token-2022 associated token accounts.
- Staking receipts (veVCoin) live only in the owner's associated token account. Create it with
  `createAtaIdempotentIx(payer, owner, vevcoinMint())` before the first deposit.

## Governance

- Votes happen in Realms. Prepend the plugin update with `withVoterWeight(...)` in the same
  transaction as the Realms instruction; the records expire after the current slot.
- Before `withdrawIx`, relinquish the user's Realms votes. Withdrawal is refused while any vote
  is unrelinquished.
- If the DAO set a 5A vote gate, users below it cannot vote: show the gate from the registrar
  (`minVoteScore`).

## Private polls

- Ballots need the Rust ZK SDK (the wasm build, or native Rust).
- The weight is fixed by the user's stake at the poll's end time (`expectedPollWeight`). Stake
  that unlocks within 4 days of the end does not count.
- One ballot per wallet per poll.

## Rewards

- Show users their gross amount and the fee (`claimFee(amount, epoch.feeBps)`).
- Claims close 90 days after activation, plus any time the pool was paused.
- A relayer can pay the claim-status rent: `claimIx({ payer })`.

## Sessions

- Create sessions with the narrowest scope (`SCOPE_CONTENT`, `SCOPE_VOUCH`, `SCOPE_FOLLOW`)
  and the shortest validity that works (at most 24h).
- Revoke them when the user logs out.
- Sessions cannot move tokens.

## Errors

Every program error code and message is in `docs/API.md`. Codes from `vcoin-common` start at
9000 and are shared by every program: `NotAuthority`, `NotUpgradeAuthority`, timelock errors and
mint validation.
