# 18 — Tests to add (P0/P1 regression tests)

## Two kinds of tests

**1. Pure-logic tests (runnable today).** These call the programs' own functions natively, in the style of `programs/*/src/tests.rs`.
- The runnable proof-of-concept crate is in `poc/`: `cd poc && cargo test -- --nocapture`. It has 8 tests, all passing, and each one **asserts the current buggy behaviour**.
- After each fix, flip the PoC assertion into the regression form shown below and move it into the program's `src/tests.rs`.

**2. Transaction-level tests (need a new harness).**
- The existing `programs/*/tests` harness doesn't compile (TOOL-2). The bankrun suite never loads a program (TOOL-4).
- We recommend a `litesvm` harness that loads `target/deploy/*.so` and builds instructions from Anchor's generated `program::instruction::*` and `program::accounts::*` types, so account lists can't drift.
- The sketches below follow that pattern but **were not run**. The exact litesvm API calls must be checked against the version pinned when the harness is added.

---

### T-GOV-1 — Voting power must not be flat in base units (GOV-02)
**File:** `programs/governance-protocol/src/tests.rs`
```rust
#[test]
fn whale_outweighs_three_ten_vcoin_wallets() {
    use crate::state::utils::calculate_voting_power;
    const ONE: u64 = 1_000_000_000;
    // ve as produced by staking for 10 VCoin @ 4y, tier None
    let small = calculate_voting_power(10 * ONE, 0, 0);
    // ve for 1B VCoin @ 4y Platinum (1.4x)
    let whale = calculate_voting_power(1_400_000_000 * ONE, 10_000, 4);
    assert!(whale > 3 * small, "small={small} whale={whale}");
    assert!(10 * small < crate::constants::DEFAULT_QUORUM, "10 dust wallets must not reach quorum");
}
```
Today `small = 100_000` and `whale = 253_831` (PoC), so both assertions fail.

### T-ZK-1 — An honest SDK vote verifies on-chain; tampered votes don't (ZK-01, ZK-02, ZK-06)
**File:** `programs/governance-protocol/tests/zk_roundtrip.rs` (host-side). Add `zk-voting-sdk = { path = "../../packages/zk-voting-sdk" }` to `[dev-dependencies]`. The conversion helpers are copied from `poc/tests/poc.rs`.
```rust
#[test]
fn honest_vote_verifies_and_tampered_vote_fails() {
    let kp = zk_voting_sdk::generate_elgamal_keypair();
    let w = 1_000u64;
    let (f, a, ab, proof) = zk_voting_sdk::encrypt_and_prove(&kp.public, zk_voting_sdk::VoteChoice::For, w);
    let pk = kp.public.compress().to_bytes();
    let onchain = to_onchain_proof(&proof);
    // must PASS after the fix (fails 20/20 today)
    verify_vote_validity_proof(&pk, &ct(&f), &ct(&a), &ct(&ab), w, &onchain).unwrap();
    // wrong weight must FAIL
    assert!(verify_vote_validity_proof(&pk, &ct(&f), &ct(&a), &ct(&ab), w + 1, &onchain).is_err());
    // swapped categories must FAIL
    assert!(verify_vote_validity_proof(&pk, &ct(&a), &ct(&f), &ct(&ab), w, &onchain).is_err());
}
```
After ZK-06 is fixed, add a case: the same proof with a different `voter` or `proposal` in the transcript context must fail.

**DLEQ:** add a matching round trip, `zk_voting_sdk::generate_partial_decryption` → `crate::crypto::verify_dleq::*`. Repeat it 64 times to catch the 1-in-16 raw-hash comparison (ZK-02).

### T-STK-1 — A top-up must never lower recorded ve without burning (STK-03)
**File:** litesvm harness, `programs/staking-protocol/tests/stake_flow.rs` (sketch, not run).
1. Create the Token-2022 VCoin and veVCoin mints. Set the veVCoin mint's PermanentDelegate and mint authority to the vevcoin config PDA, and NonTransferable.
2. Initialize vevcoin with `staking_protocol = PDA(["staking-pool"], staking_id)`.
3. Stake `100_000e9` for `MAX_LOCK_DURATION`.
4. Warp the clock to `lock_end - MIN_LOCK_DURATION + 1`.
5. Stake `1` for `MIN_LOCK_DURATION`.
6. **Assert:** `token_balance(user_vevcoin_ata) == UserStake.ve_vcoin_amount == UserVeVCoin.balance`.
7. Unstake everything after `lock_end`. **Assert:** veVCoin balance == 0 and VCoin returned in full.

### T-STK-2 — Unstake survives a user self-burn (VEV-01)
Same setup as T-STK-1. Stake, then the **user** calls Token-2022 `burn` on their whole veVCoin balance. Warp past `lock_end` and unstake.

**Assert:** VCoin is returned, and the transaction does not revert. Today it reverts and the funds are locked.

### T-GOV-2 — Finalize is one-shot (GOV-01, GOV-08)
**File:** litesvm harness, `programs/governance-protocol/tests/lifecycle.rs`.
1. Create a proposal, have 1 vote meeting quorum, then warp past `end_time`.
2. Call `finalize` and record `execution_time`.
3. Warp 47h and call `finalize` again. **Assert:** it errors (e.g. `ProposalNotActive`).
4. Warp past `execution_time` and call `execute`. **Assert:** it succeeds.
5. Call `finalize` again. **Assert:** it errors and status is still Executed.
6. For a private proposal, `finalize` before `aggregate_revealed_votes` must error.

### T-SSC-1 — Claims are capped by the epoch allocation (SSC-02)
**File:** litesvm harness, `programs/sscre-protocol/tests/claims.rs`.
1. Run `start_epoch(allocation = 100e9)`.
2. Post a root over 3 leaves of `50e9` each, i.e. 150e9 > allocation.
3. Make the first two claims. **Assert:** both succeed.
4. Make the third claim. **Assert:** it errors (e.g. `EpochAllocationExceeded`).
5. **Also:** claim at `claim_expiry` passes and at `claim_expiry + 1` fails; proof length 33 fails; epoch 255→256 and 1023 succeed.

### T-SSC-2 — The documented leaf equals the on-chain leaf (SSC-03)
**File:** `programs/sscre-protocol/src/tests.rs`.
```rust
#[test]
fn reference_leaf_vector() {
    use crate::state::compute_leaf;
    let user = anchor_lang::prelude::Pubkey::new_from_array([7u8; 32]);
    let leaf = compute_leaf(&user, 5_000_000_000, 3);
    // Freeze this vector and assert the TS/off-chain builder produces the same 32 bytes.
    assert_eq!(hex::encode(leaf), "<fill in from first run>");
}
```
Mirror the same vector in `packages/viwoapp-sdk/src/__tests__/rewards.test.ts` once SDK-07 is fixed.

### T-GAS-1 — Subsidy works after 3+ idle days (GAS-02)
**File:** litesvm harness, `programs/gasless-protocol/tests/subsidy.rs`.
1. Initialize at day D.
2. Warp to D+5.
3. Create a PlatformSubsidized session and execute an action. **Assert:** it succeeds and `config.current_day == D+5`.

### T-ID-1 — The admin verification path is reachable, and level 2 requires KYC (ID-01, ID-04)
**File:** `programs/identity-protocol/src/tests.rs`.
```rust
#[test]
fn email_phone_social_is_not_kyc() {
    use crate::utils::derive_verification_level;
    use crate::state::VerificationLevel;
    assert_eq!(derive_verification_level(0x01 | 0x02 | 0x04), VerificationLevel::Basic as u8);
    assert_eq!(derive_verification_level(0x08), VerificationLevel::KYC as u8);
}
```
**Also, in the litesvm harness:** `create_identity`, then `link_sas_attestation(level 1)`, then `update_verification(2, hash)` signed by the authority must succeed.

### T-TH-1 — The hook runs under a real Token-2022 transfer (TH-01, TH-02, TH-03)
**File:** litesvm harness, `programs/transfer-hook/tests/token2022.rs`.
1. Create a Token-2022 mint with a TransferHook pointing at the hook program.
2. Call `initialize_extra_account_meta_list`.
3. Run `transfer_checked`, resolving extras with `spl_transfer_hook_interface::offchain::add_extra_account_metas_for_execute`. **Assert:** success, and `UserActivity` is updated for the **destination owner**.
4. A direct `execute` call by an EOA must fail, because `transferring == false`.

### T-SDK-1 — Decoders round-trip real Borsh data (SDK-03..14)
**File:** `packages/viwoapp-sdk/src/__tests__/decoders.test.ts`.
1. For each account type, encode a fixture with `BorshAccountsCoder` from the program IDL (requires TOOL-3).
2. Decode it with the SDK.
3. Assert every field.

**Seeds and enums:** assert `SEEDS.*` against the Rust `constants.rs` strings, and the enum values against `state/*.rs` (e.g. `VoteChoice.Against === 2`).

### T-SDK-2 — The builders produce instructions (SDK-01)
```ts
it("buildStakeTransaction adds a stake instruction", async () => {
  const tx = await client.staking.buildStakeTransaction({ amount: new BN(1e9), lockDuration: 604800 });
  expect(tx.instructions.length).toBeGreaterThan(0);
  expect(tx.instructions[0].programId.equals(PROGRAM_IDS.stakingProtocol)).toBe(true);
});
```
Check the exact key names against the SDK when writing this.

### Property-based test (recommended)
Use `proptest` over `calculate_vevcoin`, `calculate_voting_power` and the claim bitmap:
- `UserClaim::is_epoch_claimed(e)` after `mark_epoch_claimed(e)` holds for all e < 1024, and never affects other epochs.
- `calculate_voting_power` is monotonic in `vevcoin_balance`.
- `stake` followed by `unstake` conserves VCoin.
