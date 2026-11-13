# content-registry

Program id `MJn1A4MPCBPJGWWuZrtq7bHSo2G289sUwW3ej2wcmLV`. [API](../API.md#content-registry).

## Records

**`create_content(tracking_id, content_hash, uri, type)`.**

- Writes `ContentRecord` `["content-record", author, tracking_id]`. The author is part of the
  address, so nobody can squat another author's id (audit CR-02).
- Costs energy by type: post 10, article 20, media 50, NFT 20, thread 40.
- URIs are at most 128 bytes.

Other instructions:

- **`edit_content`.** Costs 5 energy, bumps the version and emits the previous and new hash.
- **`delete_content`.** Closes the record and returns its rent to whoever paid it: the author,
  or the relayer that sponsored the post (`ContentRecord.rent_payer`). Posting through a
  sponsor and deleting earns nothing.
- **Signing.** Posts and edits may be signed by a gasless session key for the author (scope
  CONTENT). Deletes and refund claims need the wallet.

## Energy

`UserEnergy` `["user-energy", user]` holds energy, the daily post count and refunds.

- **Tier.** Derived from VCoin locked in staking-protocol and refreshed on every post, edit
  and refund claim. Users cannot choose it (CR-01). The user's `Voter` address is a required,
  derived account in every instruction that uses the tier: it cannot be left out to keep an
  old tier, and a stranger opening someone's energy account (`init_user_energy` is
  permissionless) gets that user's real tier. A wallet that never staked has no account at
  that address and is tier 0.

  | Tier | Locked VCoin | Max energy | Regeneration per hour | Posts per day |
  |---|---|---|---|---|
  | 0 | < 1k | 200 | 20 | 50 |
  | 1 | 1k | 500 | 50 | 100 |
  | 2 | 5k | 800 | 80 | 200 |
  | 3 | 20k | 1,200 | 120 | 400 |
  | 4 | 100k | 2,000 | 200 | 1,000 |

- **Regeneration.** Keeps partial progress (CR-03). A tier change settles regeneration at the old
  rate first (CR-09).
- **Refunds.**
  - The engagement oracle (a separate key from the admin, changed only after a 24h timelock)
    reports engagement.
  - `claim_refund` returns 25%, 50% or 100% of the energy spent, at 10, 50 or 100+ engagements.
  - The window is 24h to 7 days after posting.
  - A claim that does not qualify is not consumed. Refunds are capped per day (CR-06).

## Admin

- `initialize(engagement_oracle)`: upgrade authority only.
- Oracle changes: `propose_oracle` / `apply_oracle` / `cancel_oracle`.
- `set_paused`, `set_guardian` (an emergency key that may pause but never unpause).
- Two-step authority.
- No global counters (CR-05).
