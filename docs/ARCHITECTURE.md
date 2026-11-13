# Architecture

## Components

```
                 ┌──────────────────────── Realms (SPL Governance) ────────────────────────┐
                 │ proposals · thresholds · hold-up · council veto · delegation · treasury │
                 └───────▲──────────────────────────────┬──────────────────────────────────┘
       voter weight      │                              │ governance PDA signs: params, pause,
       (VWR / MaxVWR)    │                              │ oracle/attester sets, slashing, upgrades
┌─────────────┐  CPI  ┌──┴──────────────┐  reads   ┌────▼──────────┐
│ vcoin-token │◄──────│ staking-protocol │◄─────────│ private-poll  │── assert_poll_passed ──► Realms tx
│ (mint PDA)  │       │ deposits, weight │          │ ZK ballots    │
└─────────────┘       │ slashing         │          └───────────────┘
┌─────────────┐  CPI  │                  │  reads Voter   ┌──────────────────┐
│ vevcoin     │◄──────│                  │◄───────────────│ content-registry │◄── session keys ──┐
│ (receipts)  │       └───────▲──────────┘                └──────────────────┘                    │
└─────────────┘               │ reads UserScore (gate)                                ┌──────────┴───────┐
                       ┌──────┴──────┐        ┌────────────┐   ┌──────────────┐      │ gasless-protocol │
                       │ five-a      │        │ sscre      │   │ vilink       │◄─────│ sessions, fees   │
                       │ oracles     │        │ distributor│   │ tip/vouch/   │      └──────────────────┘
                       └─────────────┘        └────────────┘   │ follow       │
                       ┌─────────────┐                          └──────────────┘
                       │ identity    │
                       └─────────────┘
```

Shared crates:

- **`vcoin-common`:** authority timelock, upgrade-authority gate, mint validation.
- **`zk-poll-core`:** the ZK verifier, used by both private-poll and `zk-voting-sdk`.

## Flows

**Staking and voting.**

- A user locks VCoin in one of 8 deposit slots for 1 week to 4 years, and receives veVCoin 1:1
  in their associated token account (non-transferable).
- Voting weight = `amount × (1 + min(remaining, 4y) / 4y)`, computed live. It is 0 once fewer
  than 4 days of lock remain. Splitting tokens across wallets gains nothing.
- To vote in Realms, a client bundles `update_voter_weight_record(CastVote, proposal)` and
  `update_max_voter_weight_record` before `CastVote`. The records expire after the current slot.
- `withdraw` reads the user's TokenOwnerRecord and refuses while any vote is unrelinquished.
- The 5A score gates participation (separate vote and propose gates), but never scales weight.

**Slashing.**

- The DAO proposes a slash with an evidence hash, and the voter's withdrawals freeze at once.
- After a 48h appeal window anyone can execute it: `min(amount, deposits)` moves from the vault
  to the DAO treasury, and the receipts are burned.
- The DAO can cancel it at any time before. Unexecuted requests expire after 7 days.

**Private polls.**

1. The DAO registers a committee from a Pedersen DKG (3 ≤ t ≤ n ≤ 5, Feldman-verified).
2. Each poll snapshots the committee and the weight parameters, and names its consumer
   (usually a governance PDA).
3. Voters cast exponential-ElGamal ballots with a 448-byte CDS proof that exactly one choice
   carries their weight: their staking weight at the poll's end time, in whole VCoin.
4. After voting ends, members post DLEQ-proven partial decryptions.
5. Anyone finalizes once with a tally the program verifies. A Realms proposal can include
   `assert_poll_passed`, which the governance PDA signs, so its other instructions only run if
   the poll passed.

**Rewards (SSCRE).**

1. Once per epoch, a crank moves the scheduled budget into a dedicated epoch vault. The budget
   decays 2% per epoch. Only the authority may start an epoch the reserve cannot fund in full.
2. After the epoch ends, oracles vote on `(root, max_total_claim ≤ budget)`.
3. When the quorum (at least 2, and a majority of the active oracles) agrees, a 24h dispute
   window starts. The DAO or the emergency guardian may veto during it.
4. Once activated, users claim gross amounts (a fee of up to 1%, rounded up, into the
   program's fee vault) for 90 days. The window is extended by any pause.
5. The remainder returns to the reserve.

**Reputation (5A).**

- Up to 7 oracles, each active 24h after registration.
- A user's score changes only when `required_consensus` (≥ 2) of them, and a majority of the
  active ones, submit identical values within an hour of each other. Competing values are
  tallied separately.
- Vouching is reputation-only. An established user vouches for a newcomer, an oracle records the
  outcome at 90 days, and unevaluated vouches expire after a year. The newcomer can decline.

**Identity.**

- A unique username and a DID hash per wallet.
- Verification levels come from attestations co-signed by trusted attesters, and lapse on
  expiry or attester removal. One attestation binds to one wallet.
- USDC subscriptions extend, prorate upgrades (never dropping prepaid value) and refuse
  downgrades while active.

**Content.**

- Proof-of-existence records keyed by `(author, tracking_id)`.
- Energy tiers follow staked VCoin on every post, edit and refund, and regeneration keeps its
  partial progress.
- A post's rent returns to whoever paid it (the author or a sponsoring relayer).
- Engagement reported by an oracle refunds up to 100% of the energy, within 7 days.
- A session key may post and edit.

**ViLink.**

- Tip, vouch and follow links: one execution per executor, bounded by `max_executions`, with
  the fee fixed at creation.
- A session key may vouch and follow; tips need the wallet.

**Gasless.**

- Session keys are scoped to one program and to content, vouch or follow actions, for ≤ 24h.
- An off-chain relayer (Kora) pays SOL fees and rent and may charge VCoin, up to the user-signed
  maximum.

## Off-chain components

| Component | Feeds | Doc |
|---|---|---|
| Activity indexer | 5A oracles, SSCRE trees | `docs/offchain/activity-indexer.md` |
| 5A oracles (≥ 2) | `five_a::submit_score`, `evaluate_vouch` | this page |
| SSCRE oracles (≥ 2) | `sscre::vote_root` | `docs/offchain/merkle-leaf-format.md` |
| Engagement oracle | `content_registry::update_engagement` | this page |
| Decryption committee | `private_poll::submit_share` | `packages/zk-voting-sdk` |
| Relayer | fee payer, sessions | `docs/offchain/relayer-kora.md` |

## Authority

Every program's config holds an `AdminState`:

- it is initialized by the upgrade authority;
- it moves to a new key with a 24h two-step transfer;
- at launch it moves to the Realms governance PDA (see the deploy runbook).

Fee, treasury, oracle and payment changes are additionally timelocked by 24h inside the programs.

## Emergency guardian

Once the authorities belong to the DAO, every admin action takes a vote plus a hold-up, which
is days. Some stops cannot wait that long. Each program's config therefore has an optional
`guardian` key next to the authority (`set_guardian`, authority only; unset by default):

| The guardian can | The guardian cannot |
|---|---|
| Pause the program (`set_paused(true)`) | Unpause |
| Veto a proposed rewards root (`sscre::veto_root`) | Move funds, change parameters, oracles, fees or attesters |
| | Change the authority or itself |

The authority can unpause and replace or remove the guardian at any time, so the guardian can
delay the protocol but not capture it. Pauses never block withdrawals of stake, and deadlines
that matter (reward claims, poll reveals) are extended by the time spent paused. A council
multisig is the intended holder.
