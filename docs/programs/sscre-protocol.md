# sscre-protocol

Program id `6AJNcQSfoiE2UAeUDyJUBumS9SBwhAdSznoAeYpXrxXZ`. [API](../API.md#sscre-protocol).
Leaf format: [merkle-leaf-format.md](../offchain/merkle-leaf-format.md).

## Model

A per-epoch merkle distributor (Jito/Saber pattern).

- The reserve vault `["reserve-vault"]` holds undistributed rewards.
- **`start_epoch`.** A crank, at most once per `epoch_duration`.
  - Anyone may call it when the reserve covers the whole scheduled budget. A smaller epoch
    from a part-funded reserve needs the authority's signature, so a stranger cannot spend a
    schedule step on a tiny budget. The authority signs as a separate optional account
    (`authority`), not as the rent payer: a governance PDA can sign through a proposal but
    cannot pay rent.
  - It needs at least `root_quorum` registered oracles, or the epoch could never get a root.
  - It moves the budget into the epoch's own vault `["epoch-vault", i]`, so an epoch can never
    pay out another epoch's funds (audit SSC-02).
  - It snapshots the claim window and dispute delay into the epoch. Later parameter changes
    do not move a running epoch's deadlines.
  - It then multiplies the next budget by `1 − decay_bps`.
- **`vote_root(root, max_total_claim ≤ budget)`.**
  - Called by registered oracles (24h activation) after the epoch ends.
  - One changeable vote per oracle.
  - A candidate is proposed, and the dispute delay starts, when it has matching votes from
    `root_quorum` oracles **and** from a strict majority of the active oracles, whichever is
    larger. Two candidates can never both qualify.
  - A proposal is valid only under the oracle set and quorum that produced it. If an oracle
    is added or removed, the quorum changes, or the candidate loses a vote, the proposal is
    dropped and must be proposed again with a fresh dispute delay.
  - A removed oracle's vote stops counting, also if the same key is registered again later.
- **`veto_root`.** The authority (DAO) or the emergency guardian clears the proposal and every
  vote. Oracles must vote again. The guardian exists because a DAO vote takes longer than the
  dispute delay; see [Guardian](#guardian). Not blocked by a pause.
- **`activate_root`.** Permissionless after the delay, if the proposal still stands. It must
  happen before `end + claim_window`.
- **`claim(amount, proof)`.**
  - The claimant signs; a separate payer funds `ClaimStatus` `["claim", epoch, claimant]`.
  - Amounts are gross and capped by `max_total_claim`.
  - The fee is snapshotted at activation, rounded up, and paid into the program's own fee
    vault `["fee-vault"]`, so a claim depends on no outside account.
- **`withdraw_fees`.** Permissionless. Sweeps the fee vault to the configured (timelocked) fee
  account.
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
| Dispute delay | 24h | 1h–7 days, and at most half the claim window |
| Root quorum | 2, and never less than a majority of active oracles | 2..=registered oracles (max 7) |
| Fee | 1% | ≤ 1%, change timelocked 24h |

## Admin

- `initialize_pool`: upgrade authority only; VCoin mint pinned.
- Oracles: `register_oracle` / `remove_oracle`; `set_root_quorum`.
- `set_params`, `set_next_epoch_budget`.
- Fee: `propose_fee` / `apply_fee` / `cancel_fee`.
- `set_paused`, `set_guardian`.
- Two-step authority.

## Guardian

After launch the authority is a Realms governance PDA, and a DAO vote plus hold-up takes days.
The default dispute delay is 24 hours. Without a faster actor, a bad root could activate before
any veto lands.

The guardian (set by the authority with `set_guardian`; a council multisig is the intended
holder) can do two things, at once:

- `veto_root`;
- `set_paused(true)`.

It cannot unpause, move funds, or change oracles, parameters or fees. The authority can
unpause and replace it. A veto only sends the oracles back to voting: if they are the problem,
the guardian keeps vetoing or pauses until the DAO removes them. Budget that never gets a root
returns to the reserve after the root deadline.

If no guardian is set, keep `dispute_delay` longer than the DAO's voting period plus hold-up
(the maximum is 7 days, which needs a claim window of at least 14 days), or accept that roots
are effectively final once proposed.
