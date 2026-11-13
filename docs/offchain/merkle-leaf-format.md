# SSCRE claim trees: leaf format and oracle procedure

`sscre-protocol` pays epoch rewards from a merkle root that oracles agree on. The v2 docs gave a
leaf recipe that differed from the program (big-endian amount, no epoch), so an epoch built from
the docs could never be claimed (audit SSC-03). This file is the single source of truth. The
program (`programs/sscre-protocol/src/merkle.rs`) and its frozen test vector enforce it.

## Format v1

```
leaf = keccak256( "SSCRE_CLAIM_V1"          // 14 ASCII bytes
                ‖ claimant                  // 32-byte public key
                ‖ amount  as u64 little-endian
                ‖ epoch   as u64 little-endian )
node = keccak256( min(a, b) ‖ max(a, b) )  // byte-wise comparison; proofs carry no directions
```

- The leaf preimage is 62 bytes and a node preimage is 64 bytes, so a leaf can never pass for an
  internal node (no second-preimage trick).
- An odd node at the end of a level is promoted unchanged, with no sibling.
- `amount` is **gross**: the program deducts the fee (≤ 1%, rounded up) when paying.
- `epoch` is the epoch index, so a leaf from one epoch never verifies in another.

## Frozen test vector

| Field | Value |
|---|---|
| claimant | 32 bytes of `0x07` |
| amount | `1234567890` |
| epoch | `3` |
| leaf | `5867436a6f21c025885194461701203778b53c261bd3f449af326ca8b9cf0083` |

`cargo test -p sscre-protocol leaf_matches` recomputes it with an independent Keccak. Any SDK must
reproduce this hash before it builds a tree.

## Building a tree (oracles)

Use the reference builder `sscre_protocol::merkle::builder::Tree` (Rust, off-chain) or an
equivalent implementation that passes the vector. It rejects trees the program could not honour
(audit SSC-12):

- the claim list is empty;
- an amount is 0;
- a claimant appears twice (only one claim per epoch is possible);
- the amounts sum to more than the `max_total_claim` being voted.

## Getting a root accepted

1. After `end_ts`, each oracle calls `vote_root(root, max_total_claim)`. `max_total_claim` is the
   sum of the leaves, and must be ≤ the epoch's budget. Votes can be changed until activation.
2. When `root_quorum` (default 2) oracles agree on the same `(root, max_total_claim)`, the
   dispute delay (default 24h) starts. The DAO can `veto_root` during it, which clears all votes.
3. Anyone calls `activate_root` after the delay, provided the candidate still has quorum (removed
   oracles no longer count). It must happen before `end_ts + claim_window`, or the budget can only
   be clawed back.
4. Claims stay open for `claim_window` (default 90 days), extended by any time the pool was paused.
   Afterwards anyone calls `clawback` to return the unclaimed remainder to the reserve.

Publish each tree (leaves and proofs) alongside the root so claimants and the DAO can audit it
during the dispute delay.
