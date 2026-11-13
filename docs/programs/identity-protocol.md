# identity-protocol

Program id `3egAds3pFR5oog6iQCN42KPvgih8HQz2FGybNjiVWixG`. [API](../API.md#identity-protocol).

## Identity

- **`create_identity(did_hash, username)`.**
  - Creates `Identity` `["identity", owner]` and reserves the username `["username", name]`.
  - Usernames match `[a-z0-9_]{3,32}` and are unique.
- **`update_did_hash`.** Works at any level.
- **`close_identity`.** Frees the username.

## Verification

Levels are **attester-signed**: a trusted attester co-signs. The program does not read Solana
Attestation Service accounts.

**`link_attestation(attestation_id, claims, expires_at)`.**

- The owner and the attester sign; the attester can never be the owner.
- The identity takes the attestation's level, whether that is higher or lower than before
  (audit ID-01).
- `AttestationLink` `["attestation", id]` binds an attestation id to one wallet (ID-03).
- Only the attester that issued an id can refresh it. Another attester cannot take a link
  over and remove the issuer's right to revoke it.
- Expiry is at most 2 years.
- Levels by claim:

  | Level | Claims required |
  |---|---|
  | Basic | email + phone |
  | KYC | the KYC claim (ID-04) |
  | Full | KYC + biometric |
  | Enhanced | Full + unique human |

**`Identity::effective_level(now, config)`** returns 0 once the attestation expired or its
attester was removed. Consumers should use it, not the raw level (ID-02).

- **`revoke_attestation`.** The issuing attester or the authority clears the level and closes
  the link. The owner's identity account is a required, derived account, so a revocation can
  never close the link and leave the level in place. It also works after the identity was
  closed.
- **`admin_lower_level`.** Can only lower a level.

## Subscriptions

`subscribe(tier 1..=3, max_payment)` charges USDC (6 decimals, legacy SPL Token or Token-2022) to
the treasury account, for 30 days.

- **Same tier:** extends from the current expiry, so no paid time is lost.
- **Upgrade:** credits the unused time of the current tier against the new tier's price, for
  30 days. The credit is what that time actually cost (`Subscription.unit_price`), not today's
  list price, so a price change cannot be turned into free time. If the prepaid time is worth
  a full period of the new tier or more, the upgrade is free and the time carries over at
  that value: no paid value is lost and none is created.
- **Downgrade:** refused while the higher tier is active (ID-05).
- **Default prices:** $4 / $12 / $59.

## Admin

- `initialize`: upgrade authority only; USDC mint and treasury validated.
- Attesters: `add_attester` / `remove_attester`.
- Payment changes: `propose_payment` / `apply_payment` / `cancel_payment` (24h).
- `set_paused`, `set_guardian` (an emergency key that may pause but never unpause).
- Two-step authority.
