# private-poll

Program id `3R256kBN9iXozjypQFRAmegBhd6HJqXWqdNG7Th78HYe` (the former governance-protocol).
[API](../API.md#private-poll).

Binding governance runs on Realms. This program runs **secret-ballot polls** that a Realms proposal
can require.

## Cryptography

All of it lives in `crates/zk-poll-core`, shared with `packages/zk-voting-sdk`.

- **Ballot.** Exponential ElGamal on Ristretto255: `(R, C) = (r·G, r·PK + v·H)` per choice
  (For, Against, Abstain), with `v ∈ {0, w}` and `Σv = w`.
- **Proof (448 bytes).** Three CDS 1-of-2 OR proofs plus a Chaum–Pedersen sum proof. Every
  challenge is SHA-256 masked to 252 bits, over a transcript that binds program, poll, voter,
  election key, H and weight (audit ZK-01/02/06).
- **Committee.** From a Pedersen DKG.
  - Registered by the DAO with `set_committee`.
  - Checked by `verify_committee`: 3 ≤ t ≤ n ≤ 5, non-identity key, distinct members and
    authorities, and a Feldman check for every member key (ZK-03).
  - Each poll snapshots the committee.
- **Partial decryptions.** Batched DLEQ proofs bound to program, poll and member.
- **Tally check.** The tally must satisfy `t·H == C − Σλ_i·D_i` and must sum to the weight cast.

## Lifecycle

1. **`create_poll(description_hash, start, end, consumer)`.**
   - The creator needs a lock-weighted stake of at least `min_creator_weight`.
   - It snapshots the weight parameters, the quorum (bps of max weight), `pass_bps` and the
     committee.
2. **`cast_vote(ballot, proof)`.**
   - Allowed in `[start, end)`.
   - Weight = staking weight at `end`, in whole VCoin, so stake must stay locked through the poll.
   - The 5A gate applies.
   - One ballot per voter. About 97k CU.
3. **`submit_share(member_index, shares, proof)`.** Allowed in `[end, end + reveal_window)`, one
   per member.
4. **`finalize_poll(tally, lagrange)`** (one-shot).
   - Needs at least t shares.
   - Passed iff `total ≥ quorum` and `For > pass_bps·(For + Against)`.
   - **`expire_poll`** instead, if the reveal window closed without enough shares. An expired
     poll can never pass.
5. **`assert_poll_passed`.** Consumable once, by the named consumer (a governance PDA, inside a
   Realms proposal transaction).
6. **Cleanup.** `close_ballot` and `close_poll`, once settled.

## Admin

- `initialize_config`: upgrade authority only.
- `set_committee`.
- `set_poll_params`:

  | Parameter | Bounds |
  |---|---|
  | quorum | ≤ 100% |
  | pass threshold | 50%–<100% |
  | voting window | 1–60 days |
  | reveal window | 1–30 days |

- `set_paused`.
- Two-step authority.
