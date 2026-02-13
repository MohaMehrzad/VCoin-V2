# VCoin Protocol Stack - Remaining Audit Items

**Date:** February 13, 2026
**Status:** Items requiring verification or fixing
**Previous Items Fixed:** 53 of 68 total findings (78%)
**Remaining Items:** 15 issues requiring attention

---

## Executive Summary

This document tracks the **15 remaining audit findings** that require verification or fixes. The VCoin Protocol Stack has successfully remediated **53 out of 68 issues (78%)** from the comprehensive security audit. This document focuses solely on outstanding items.

### Remaining Issues Breakdown

| Severity | Count | Priority |
|----------|-------|----------|
| 🔴 **CRITICAL** | 2 | Immediate |
| 🟠 **HIGH** | 3 | High |
| 🟡 **MEDIUM** | 10 | Medium |
| 🔵 **LOW** | 0 | All verified fixed |
| **TOTAL** | **15** | - |

---

## 🔴 CRITICAL SEVERITY (2 Items)

### C-REMAINING-01: ViLink Fee Calculation Precision Loss

**Program:** vilink-protocol
**Location:** TBD - requires code review
**Status:** ⚠️ NEEDS VERIFICATION

**Description:**
Similar to SSCRE fee calculation, ViLink may have precision loss in tip fee calculations due to integer division before multiplication.

**Expected Fix:**
Should use ceiling division like SSCRE:
```rust
let fee = ((amount as u128 * PLATFORM_FEE_BPS as u128 + 9999) / 10000) as u64;
```

**Impact:** Small amounts may have fees rounded to zero, or rounding losses accumulate.

**Verification Steps:**
1. Check `programs/vilink-protocol/src/lib.rs` tip execution functions
2. Search for fee calculation patterns
3. Verify ceiling division is used for all fee calculations

---

### C-REMAINING-02: Content Energy Calculation Overflow

**Program:** content-registry
**File:** `programs/content-registry/src/utils.rs` (lines 6-26)
**Status:** ⚠️ NEEDS VERIFICATION

**Description:**
Energy regeneration calculation may overflow if not properly bounded:
```rust
let hours_elapsed = (now - user_energy.last_update_at) / 3600;
let regenerated = hours_elapsed * user_energy.regen_rate;
```

**Expected Fix:**
Should use checked or saturating arithmetic:
```rust
let regenerated = hours_elapsed.saturating_mul(user_energy.regen_rate as i64);
user_energy.current_energy = user_energy.current_energy
    .saturating_add(regenerated)
    .min(user_energy.max_energy);
```

**Impact:** Energy system overflow could lead to DoS or incorrect state.

**Verification Steps:**
1. Read `programs/content-registry/src/utils.rs`
2. Check `regenerate_energy()` function
3. Verify saturating arithmetic is used
4. Confirm bounds checking exists

---

## 🟠 HIGH SEVERITY (3 Items)

### H-REMAINING-01: Session Key Cryptographic Signature Verification

**Program:** gasless-protocol
**File:** `programs/gasless-protocol/src/instructions/session/execute_action.rs`
**Status:** ⚠️ NEEDS VERIFICATION

**Description:**
Session keys should have cryptographic signature verification to prevent forgery. The session key should sign the action parameters, and the signature should be verified on-chain.

**Expected Fix:**
```rust
// Verify session key signature
let message = create_session_message(&action_data);
require!(
    verify_signature(&session_key.pubkey, &message, &signature),
    GaslessError::InvalidSessionSignature
);
```

**Impact:** Without signature verification, session keys could be forged or manipulated.

**Verification Steps:**
1. Read gasless-protocol session creation and execution
2. Check for signature verification calls
3. Verify cryptographic validation (ed25519_verify or similar)
4. Test with invalid signatures

---

### H-REMAINING-02: Oracle Score Consensus Expiry Race Condition

**Program:** five-a-protocol
**File:** `programs/five-a-protocol/src/instructions/oracle/submit_score.rs` (lines 100-137)
**Status:** ⚠️ NEEDS VERIFICATION

**Description:**
When consensus is reached (line 100), the code applies the score immediately. However, if consensus is reached exactly at the expiry timestamp, the expired score could be applied.

**Expected Fix:**
Add expiry re-check at application time:
```rust
if pending_update.reached_consensus() {
    let clock = Clock::get()?;
    require!(
        clock.unix_timestamp < pending_update.expires_at,
        FiveAError::UpdateExpired
    );
    // Apply score update
}
```

**Impact:** Stale scores could be applied if consensus arrives at expiry boundary.

**Verification Steps:**
1. Read five-a-protocol score submission logic
2. Check consensus application code
3. Verify expiry is re-checked before applying score
4. Test with edge-case timestamps

---

### H-REMAINING-03: Delegation Amount Validation

**Program:** governance-protocol
**File:** `programs/governance-protocol/src/instructions/delegation/delegate.rs`
**Status:** ⚠️ NEEDS VERIFICATION

**Description:**
When delegating voting power, the delegated amount should be validated against the delegator's actual veVCoin balance to prevent claiming more voting power than possessed.

**Expected Fix:**
```rust
let user_vevcoin_balance = get_vevcoin_balance(&ctx.accounts.delegator_vevcoin)?;
require!(
    amount <= user_vevcoin_balance,
    GovernanceError::ExceedsDelegatedAmount
);
```

**Impact:** Users could claim more voting power than they possess through delegation.

**Verification Steps:**
1. Read delegation creation code
2. Check if veVCoin balance is queried
3. Verify amount <= balance validation
4. Test with excess amounts

---

## 🟡 MEDIUM SEVERITY (10 Items)

### M-REMAINING-01: Voting Power Calculation Precision Loss

**Program:** governance-protocol
**File:** `programs/governance-protocol/src/state/utils.rs` (lines 1-34)
**Status:** ⚠️ NEEDS FIX

**Description:**
```rust
let five_a_boost = 1000 + (five_a_score as u64 / 10); // 1000-2000
```

If `five_a_score = 5`, then `5 / 10 = 0` (integer division), losing the boost entirely. Small 5A scores (1-9) produce zero boost.

**Recommended Fix:**
```rust
let five_a_boost = 1000 + ((five_a_score as u64 * 1000) / 10000); // Scale before divide
```

**Impact:** Voters with small but non-zero 5A scores receive no boost.

**Verification:** Check `calculate_voting_power()` function in governance utils.

---

### M-REMAINING-02: Proposal Threshold Hardcoded Memory Offsets

**Program:** governance-protocol
**File:** `programs/governance-protocol/src/instructions/vote/cast.rs` (lines 63-96)
**Status:** ⚠️ DESIGN LIMITATION

**Description:**
Cross-program data reading uses hardcoded byte offsets (`stake_data[73..81]` for veVCoin amount). If the staking protocol changes its struct layout, governance breaks silently.

**Impact:** Silent data corruption if external protocol structs are modified.

**Recommendation:**
- Document the dependency clearly
- Consider using CPI queries instead of data slicing
- Add struct layout version checks
- Implement integration tests that fail on layout changes

**Note:** This is marked as a design limitation in C-NEW-01. May be acceptable with proper documentation and testing.

---

### M-REMAINING-03: Lock Duration Manipulation Confusion

**Program:** staking-protocol
**File:** `programs/staking-protocol/src/instructions/user/extend_lock.rs` (lines 12-26)
**Status:** ⚠️ DOCUMENTATION NEEDED

**Description:**
`extend_lock` naming is confusing. It calculates `new_lock_end = now + new_lock_duration` and checks `new_lock_end > user_stake.lock_end`. This prevents shortening locks but the function name suggests extension from now.

**Impact:** No security impact - the check prevents shortening. However, naming is confusing.

**Recommendation:**
- Rename to `extend_lock_period` or similar
- Add clear documentation explaining the calculation
- Add examples in comments

---

### M-REMAINING-04: Tier Update Spam Prevention

**Program:** staking-protocol
**File:** `programs/staking-protocol/src/instructions/user/update_tier.rs` (lines 12-31)
**Status:** ⚠️ NEEDS FIX

**Description:**
Users can call `update_tier` repeatedly even when their tier hasn't changed. Each call triggers CPI to veVCoin program and emits events.

**Recommended Fix:**
```rust
let old_tier = user_stake.tier;
let new_tier = calculate_tier(user_stake.amount);
require!(new_tier != old_tier, StakingError::TierUnchanged);
```

**Impact:** Compute waste, event spam, unnecessary CPI calls.

**Verification:** Check if tier comparison exists before CPI calls.

---

### M-REMAINING-05: Batch Nonce vs Batch ID Semantic Gap

**Program:** vilink-protocol
**File:** `programs/vilink-protocol/src/lib.rs` (lines 390-423, 647)
**Status:** ⚠️ NEEDS CLARIFICATION

**Description:**
`batch_id` is generated from timestamp but PDA uses `batch_nonce`. Querying by `batch_id` doesn't match PDA derivation.

**Impact:** Client-side confusion. Off-chain indexers may not correctly map batch IDs to PDAs.

**Recommendation:** Unify batch identification to use nonce consistently.

**Verification:**
- Check how batch_id is generated vs how PDA is derived
- Verify SDK uses correct field for PDA derivation

---

### M-REMAINING-06: No 5A Score Bounds Validation in SSCRE

**Program:** sscre-protocol
**File:** `programs/sscre-protocol/src/lib.rs` (line 170)
**Status:** ⚠️ NEEDS FIX

**Description:**
`avg_five_a_score: u16` in `update_merkle_root` accepted without bounds check. Valid range should be 0-10000.

**Recommended Fix:**
```rust
require!(avg_five_a_score <= 10000, SSCREError::InvalidScore);
```

**Impact:** Invalid data stored if oracle is compromised.

**Verification:** Check `update_merkle_root` or similar functions for score validation.

---

### M-REMAINING-07: No Minimum Epoch Allocation in SSCRE

**Program:** sscre-protocol
**File:** `programs/sscre-protocol/src/lib.rs` (line 132)
**Status:** ⚠️ NEEDS FIX

**Description:**
`start_epoch` allows `total_allocation = 0`, creating empty epochs that waste PDA slots.

**Recommended Fix:**
```rust
require!(total_allocation > 0, SSCREError::ZeroAllocation);
```

**Impact:** Empty epochs waste resources.

**Verification:** Check epoch initialization for zero-allocation check.

---

### M-REMAINING-08: Zero Amount Fee Deduction

**Program:** gasless-protocol
**File:** `programs/gasless-protocol/src/instructions/fee/deduct_vcoin.rs`
**Status:** ⚠️ NEEDS FIX

**Description:**
Zero amount bypasses slippage check and records a fee deduction event with no actual fee.

**Recommended Fix:**
```rust
require!(amount > 0, GaslessError::ZeroAmount);
```

**Impact:** Fake fee accounting entries.

**Verification:** Check fee deduction for minimum amount validation.

---

### M-REMAINING-09: Proposal ID Saturation

**Program:** governance-protocol
**File:** `programs/governance-protocol/src/instructions/proposal/create.rs` (lines 69-73)
**Status:** ⚠️ NEEDS FIX

**Description:**
`saturating_add(1)` at u64::MAX stays at MAX. Subsequent proposals all get ID = u64::MAX, causing PDA collisions.

**Recommended Fix:**
```rust
let new_id = config.proposal_count.checked_add(1)
    .ok_or(GovernanceError::ProposalCountOverflow)?;
config.proposal_count = new_id;
```

**Impact:** Theoretical DoS after 2^64 proposals. `saturating_add` silently fails.

**Verification:** Check proposal ID assignment for checked arithmetic.

---

### M-REMAINING-10: Energy Regeneration Uses Floating Point

**Program:** content-registry
**File:** `programs/content-registry/src/utils.rs` (lines 6-26)
**Status:** ⚠️ NEEDS FIX

**Description:**
```rust
(hours_elapsed as f64 * energy.regen_rate as f64) as u16
```
Uses floating-point arithmetic which is non-deterministic across platforms.

**Recommended Fix:**
```rust
let regenerated = (hours_elapsed * energy.regen_rate as i64) as u16;
```

**Impact:** Slight energy over/under-accumulation due to floating point rounding.

**Verification:** Search for f64 usage in energy calculations and replace with integer math.

---

## 🔵 LOW SEVERITY

**Status:** ✅ ALL LOW SEVERITY ITEMS VERIFIED FIXED

All 8 low severity items from the original audit have been verified as fixed or are documentation issues that have been addressed.

---

## Verification Checklist

### For Each Item Above:

- [ ] **Read the specific file and line numbers**
- [ ] **Verify the issue exists or has been fixed**
- [ ] **Document the current state** (fixed/unfixed/partial)
- [ ] **Add test cases** for regression prevention
- [ ] **Update this document** with findings

### Testing Requirements:

- [ ] Create unit tests for each fixed item
- [ ] Create integration tests for cross-program issues
- [ ] Add fuzzing tests for arithmetic operations
- [ ] Test edge cases (zero values, max values, boundary conditions)

---

## Priority Action Plan

### Week 1 - Critical Items (2 issues)
1. Verify ViLink fee calculation precision
2. Verify content energy overflow protection

### Week 2 - High Priority (3 issues)
3. Implement/verify session key signature verification
4. Add oracle consensus expiry re-check
5. Implement delegation amount validation

### Week 3-4 - Medium Priority (10 issues)
6. Fix voting power precision loss
7. Document hardcoded offset dependencies
8. Add tier update deduplication
9. Fix remaining medium severity items

### Week 5 - External Audit Prep
10. Complete all verifications
11. Update test coverage to 100%
12. Document all fixes
13. Prepare for external security audit

---

## Success Criteria

✅ **All 15 items verified/fixed**
✅ **Test coverage > 95%**
✅ **Integration tests passing**
✅ **External audit scheduled**
✅ **Documentation updated**

---

**Next Review:** TBD
**Responsible:** Development Team
**External Audit:** Recommended before mainnet

---

*This document tracks remaining work from the comprehensive security audit. Previous audit archived as `audit-archive-YYYYMMDD.md`.*
