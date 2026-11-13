# sscre-protocol

Program id `6AJNcQSfoiE2UAeUDyJUBumS9SBwhAdSznoAeYpXrxXZ`. [API](../API.md#sscre-protocol).
Leaf format: [merkle-leaf-format.md](../offchain/merkle-leaf-format.md).

## Model

A per-epoch merkle distributor (Jito/Saber pattern).

- The reserve vault `["reserve-vault"]` holds undistributed rewards.
- **`start_epoch`.** A permissionless crank, at most once per `epoch_duration`.
  - It moves `min(next_epoch_budget, reserve)` into the epoch's own vault `["epoch-vault", i]`,
    so an epoch can never pay out another epoch's funds (audit SSC-02).
  - It then multiplies the next budget by `1 − decay_bps`.
- **`vote_root(root, max_total_claim ≤ budget)`.**
  - Called by registered oracles (24h activation) after the epoch ends.
  - One changeable vote per oracle.
  - `root_quorum` matching votes propose a candidate and start the dispute delay. The DAO can
    `veto_root`, which clears every vote.
- **`activate_root`.** Permissionless after the delay, if the candidate kept its quorum. It must
  happen before `end + claim_window`.
- **`claim(amount, proof)`.**
  - The claimant signs; a separate payer funds `ClaimStatus` `["claim", epoch, claimant]`.
  - Amounts are gross and capped by `max_total_claim`.
  - The fee is snapshotted at activation, rounded up, and paid to the timelocked fee account.
- **`clawback`.**
  - Permissionless after the claim window, or after the root deadline if no root ever
    activated.
  - Returns the remainder to the reserve and closes the epoch vault.
  - `close_claim_status` then refunds rent.
- **Pause.** Blocks start, vote, activate and claim. Every deadline is extended by the time spent
  paused (SSC-05).

## Defaults and bounds

| Parameter | Default | Bounds |
|---|---|---|
| First epoch budget | 10M VCoin | the DAO can only lower it |
| Decay per epoch | 2% | ≤ 50% |
| Epoch duration | 30 days | 1–90 days |
| Claim window | 90 days | 7–365 days |
| Dispute delay | 24h | 1h–7 days |
| Root quorum | 2 | 2..=registered oracles (max 7) |
| Fee | 1% | ≤ 1%, change timelocked 24h |

## Admin

- `initialize_pool`: upgrade authority only; VCoin mint pinned.
- Oracles: `register_oracle` / `remove_oracle`; `set_root_quorum`.
- `set_params`, `set_next_epoch_budget`.
- Fee: `propose_fee` / `apply_fee` / `cancel_fee`.
- `set_paused`.
- Two-step authority.
