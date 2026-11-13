# 07 — identity-protocol (portable DID)

**Program:** `programs/identity-protocol` · ID `3egAds3pFR5oog6iQCN42KPvgih8HQz2FGybNjiVWixG`

**What it does:**
- Each wallet gets one `Identity` with a DID hash, a username and a verification level from 0 to 4.
- A trusted attester co-signs `link_sas_attestation` to raise that level.
- Users can buy 30-day USDC subscriptions.

**Verdict:** ❌ The admin verification path can never succeed, and levels can't be upgraded, revoked or downgraded. The "SAS integration" is a co-signature, with no SAS account checked.

---

### ID-01 · High · Confirmed · bug — `update_verification` always reverts, and users are stuck at their first attested level
- **Where:**
  - `programs/identity-protocol/src/instructions/admin/update_verification.rs:22,31`
  - `programs/identity-protocol/src/instructions/user/create_identity.rs:19`
  - `programs/identity-protocol/src/contexts/link_sas_attestation.rs:21-27`
- **Code:**
  - `require!(identity.verification_hash != [0u8; 32], IdentityError::SASAttestationRequired);`
  - `identity.verification_hash = [0u8; 32];`, set at creation.
  - The SAS link account is created with `init, … seeds = [SAS_ATTESTATION_SEED, user.key().as_ref()]`.
- **Fails when:**
  - **Admin path:** `verification_hash` is written only at create (all zeros) and at `update_verification.rs:31`, which runs after the guard. So `update_verification` always reverts.
  - **User path:** `link_sas_attestation` uses `init` on one PDA per user, so it works exactly once.
- **Consequence:**
  - A user whose first attestation was email + phone (Basic) can never move up to KYC, Full or Enhanced.
  - The admin can never set a level; the "C-AUDIT-17 fix" disabled the admin path entirely.
  - `verified_identities` stays 0 forever, because only `update_verification.rs:37` increments it. `VerificationUpdated` is never emitted.
  - The only test covers the rejection (`tests/identity-protocol.ts:169-188`).
- **Fix:**
  - In `update_verification`, require a linked, unexpired `UserSASAttestation` account passed in the context, instead of the hash check.
  - Make the SAS link upsertable (`init_if_needed` plus attester check), or key it by `[seed, user, attester]`.
  - Increment `verified_identities` when a level goes from 0 to above 0.

### ID-02 · Medium · Confirmed · security — Levels can never be revoked or downgraded; expiry and attester removal have no effect
*Downgraded from High by the verifier: the SAS link PDA stores `attester` and `expires_at` (`link_sas_attestation.rs:33,41`), so a careful consumer can check them. Nothing on-chain reads `verification_level` today.*
- **Where:**
  - `programs/identity-protocol/src/instructions/user/link_sas_attestation.rs:26,47-50`
  - `programs/identity-protocol/src/instructions/admin/update_verification.rs:25-28`
  - `programs/identity-protocol/src/instructions/admin/remove_trusted_attester.rs:13-27`
- **Code:** `if sas.verification_level > identity.verification_level { identity.verification_level = sas.verification_level; }` and `require!(new_level >= old_level, IdentityError::CannotDowngradeVerification);`
- **Fails when:**
  - `expires_at` is checked only at link time.
  - Removing a compromised attester leaves every level it granted in place.
  - No instruction can lower `verification_level`.
- **Consequence:** fraudulent or expired KYC and Enhanced levels persist forever, for every dApp that trusts this "portable identity".
- **Fix:** add an admin- or attester-callable `revoke_attestation` that lowers the level and closes the link. Store the attester and expiry on the identity so consumers can check them, and allow admin downgrades.

### ID-03 · Low · Confirmed · half-impl — The "SAS integration" never reads SAS; one attestation can be reused across wallets (each link still needs a trusted attester's co-signature)
- **Where:**
  - `programs/identity-protocol/src/instructions/user/link_sas_attestation.rs:30-42`
  - `programs/identity-protocol/src/state/config.rs:12`: `sas_program` is stored but never read.
- **Code:** `sas.sas_attestation_id = sas_attestation_id; sas.attestation_type = attestation_type; sas.portable_score = portable_score;`. All three values are caller-supplied.
- **Fails when:**
  - The same `sas_attestation_id` (for example a "UniqueHuman" proof) can be linked to any number of wallets.
  - `attestation_type` is not range-checked and is not tied to `verified_claims`.
  - `portable_score` is not bounded to 10000.
  - The doc claims "Checks attestation validity" (`docs/programs/identity-protocol.md:119`) and "Sybil resistance proof" (`:160`).
- **Fix:**
  - Deserialize the SAS attestation account: owner == `config.sas_program`, subject == user, issuer == attester, not expired.
  - Add a uniqueness PDA seeded by the attestation id.
  - Validate the type and score ranges.

### ID-04 · Low · Confirmed · bug — Email + phone + social is classified as "KYC" (reproduced by PoC `bug_id04_…`; looks deliberate in code but contradicts the level's documented meaning)
- **Where:** `programs/identity-protocol/src/utils.rs:24`
- **Code:** `} else if has_kyc || (has_email && has_phone && has_social) {` → `VerificationLevel::KYC`
- **Fails when:** an attestation with no identity documents is given level 2. Level 2 is documented as "Identity documents" (`state/verification_level.rs:9`, `docs/programs/identity-protocol.md:25`). Any dApp gating on level ≥ 2 accepts undocumented users. This function has no test.
- **Fix:** return KYC only when `has_kyc`, and map email + phone + social to Basic.

### ID-05 · Medium · Confirmed · bug (money) — Subscription renewal throws away paid time; the free tier overwrites paid
- **Where:** `programs/identity-protocol/src/instructions/user/subscribe.rs:36-40`
- **Code:** `subscription.tier = tier; subscription.started_at = clock.unix_timestamp; subscription.expires_at = clock.unix_timestamp + SUBSCRIPTION_DURATION;`
- **Fails when:**
  - Renewing on day 29 loses the paid day that was left.
  - Switching from Enterprise ($59) to Verified ($4) mid-period loses the Enterprise time, with no proration.
  - `subscribe(0)` (free, no transfer) replaces a paid tier.
  - The free tier still requires a USDC mint and both token accounts.
  - Nothing on-chain reads `Subscription`, and `SubscriptionExpired` and `InsufficientPayment` are never raised.
- **Fix:**
  - For the same tier, set `expires_at = max(now, expires_at) + DURATION`.
  - Reject a downgrade while the current tier is active, or prorate.
  - Reject tier 0 while a paid tier is active.

### ID-06 · Medium · Confirmed · security — Front-runnable `initialize`; a mis-set treasury permanently bricks subscriptions
- **Where:**
  - `programs/identity-protocol/src/contexts/initialize.rs:17-21`
  - `programs/identity-protocol/src/instructions/admin/initialize.rs:10-12`
  - `programs/identity-protocol/src/contexts/subscribe.rs:43`
- **Code:** `/// CHECK: Treasury account for subscription payments  pub treasury: UncheckedAccount<'info>,` and `constraint = treasury_token_account.owner == identity_config.treasury`
- **Fails when:**
  - Anyone can initialize first and become authority.
  - `usdc_mint`, `treasury` and `sas_program` are unvalidated and have no setter.
  - If the deployer passes the treasury's token account instead of its wallet, payments go to whatever token account someone creates for that owner address. That account may be unspendable. Without a setter, this can only be fixed by redeploying.
- **Fix:**
  - Check the upgrade authority in `initialize`.
  - Validate `usdc_mint` as a Mint and `treasury` as a system-owned wallet.
  - Add authority-gated setters.

### ID-07 · Medium · Confirmed · security / doc-mismatch — No 24h timelock on authority transfer
- **Where:** `programs/identity-protocol/src/instructions/admin/accept_authority.rs:12`
- **Code:** `config.authority = new_authority;`, with no time check
- **Fails when:** propose and accept can happen in the same transaction, contrary to `SECURITY.md:71,75`.
- **Fix:** add `pending_authority_activated_at` and a `now >= set_at + 86_400` check in accept.

### ID-08 · Low · Confirmed · reliability — Pause covers only `create_identity`
- **Where:** `programs/identity-protocol/src/instructions/user/create_identity.rs:10` is the only `paused` check in the program.
- **Fails when:** during an incident the admin pauses, but `subscribe` keeps taking USDC and `link_sas_attestation` keeps granting levels. `update_did_hash` and `update_verification` also ignore pause.
- **Fix:** add the pause check to every mutating user instruction.

### ID-09 · Medium · Confirmed · test-gap — No executed test of verification, attestation, or subscription renewal/downgrade (only one paid `subscribe(1)` runs, in `tests/identity-protocol.ts:229-241`)
- **Where:**
  - `programs/identity-protocol/src/tests.rs:207-221`, which are tautologies such as `assert!(current_time < expires_at)` on literals.
  - `programs/identity-protocol/tests/common/mod.rs:144-151`, where subscribe passes the identity PDA and only 3 of its 8 accounts.
  - `programs/identity-protocol/src/tests.rs:41`, a stale size test.
- **Fails when:** every behavioural test fails or tests nothing, which is how ID-01 shipped.
- **Fix:** run real transactions for `link_sas_attestation`, `update_verification`, and subscribe/renew/downgrade.

### ID-10 · Low · Confirmed · doc-mismatch — Docs describe a different API
- **Where:** `docs/programs/identity-protocol.md:31-36,52-53,61-63,82,84-86,92-114,120-121`
- **Mismatches:**
  - The docs list Creator/Business tiers; the code has Verified/Enterprise (`state/subscription_tier.rs:10-12`).
  - `create_identity(metadata_uri)` is documented, but the code takes `did_hash, username`.
  - `update_verification(user, level)` is documented, but the code takes `new_level, verification_hash`.
  - `subscribe(…, duration_months)` and `update_profile` don't exist.
  - The documented account layouts don't exist.
  - "Cannot skip verification levels" is false: 0→4 is allowed.
  - "Tier features gated" is false: nothing on-chain reads `Subscription`.

### Low findings
- **ID-L1.** Wrong error codes.
  - `create_identity.rs:11` returns `InvalidVerificationLevel` for an over-long username.
  - `update_did_hash.rs:12` returns `IdentityNotFound` for an unverified user, so level-0 users can never fix a wrong DID.
  - `add_trusted_attester.rs:10` returns `Overflow` for a full list.
- **ID-L2.** Usernames are not unique or validated (`create_identity.rs:22-24`). A trusted attester may attest itself (attester == user is allowed).
- **ID-L3.** `get_identity` only calls `msg!`, with no `set_return_data`, so CPI callers can't use it. No account can be closed.

### Dead code and write-only state
- **Identity:** `is_active` is never set to false.
- **Subscription:** `auto_renew` is always false.
- **IdentityConfig:** `sas_program` is never read, and `verified_identities` is always 0.
- **SAS link:** `portable_score` and `attestation_type` are never read.
- **Errors never raised:** `IdentityAlreadyExists`, `SubscriptionExpired`, `InsufficientPayment`.
