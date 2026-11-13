# five-a-protocol

Program id `783PbtJw5cc7yatnr9fsvTGSnkKaV6iJe6E8VUPTYrT8`. [API](../API.md#five-a-protocol).

## Scores

`UserScore` `["user-score", user]` holds five dimensions, each 0..=10000:

| Dimension | Weight in the composite |
|---|---|
| authenticity | 25% |
| accuracy | 20% |
| agility | 15% |
| activity | 25% |
| approved | 15% |

staking-protocol (the Realms gate) and private-poll read `composite_score` as a **participation
gate only**.

## Consensus

- **Oracles.** Up to 7, each active 24h after `register_oracle`, and removable at once.
- **`submit_score(values)`.**
  - Each active oracle has one changeable submission in the user's round
    `["pending-score", user]`.
  - Values are applied when `required_consensus` (default 2, settable 2..=min(registered, 5))
    active oracles submit identical values.
  - Competing value sets are tallied separately, so a dishonest first submission cannot block
    honest ones (audit 5A-01/04).
  - Applied rounds close and refund their opener. Rounds expire after 1h and reset.
  - Applied updates are at least 1h apart.
- **`close_expired_round`.** Permissionless.

## Vouching

Vouching is reputation-only: no stake and no reward.

- **`vouch_for_user`.**
  - The voucher needs a score of at least 60%.
  - The vouchee must be below 50% or unscored.
  - No direct mutual vouching.
  - Slots depend on the voucher's score: 3 / 5 / 8 / 10.
  - A vouchee receives at most 3 vouches.
- **`evaluate_vouch`.** Oracle only, between day 90 and day 365. It succeeds if the vouchee
  reached 50%; a missing score counts as failed.
- **`expire_vouch`.** Anyone, after a year.

Every outcome closes the record and frees the slot (5A-06).

## Admin

- `initialize`: upgrade authority only.
- Oracles: `register_oracle` / `remove_oracle`; `set_required_consensus`.
- `set_paused`: scoring and vouching.
- Two-step authority.

Removed: private mode, `get_score`, unread snapshots, and stake/reward numbers that moved no
tokens (5A-05/09).
