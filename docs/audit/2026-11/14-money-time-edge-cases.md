# 14 — Edge cases: money, arithmetic and time

**Scope:** every value-moving and accounting path:
- stake / extend / unstake
- veVCoin mint and burn
- vcoin mint and slash
- SSCRE epochs and claims
- ViLink tips
- gasless fees and budgets
- identity subscriptions
- content energy and refunds
- governance power and tallies
- 5A composite scores

**Format:** edge-case-finder priority (P0 = Likely × Catastrophic; anything an attacker can trigger counts as Likely). Scenarios already written up as findings are linked by ID, not repeated.

**Assumptions the code makes and nothing enforces:**
1. Voting-power numbers are whole tokens. They are actually 9-decimal base units (GOV-02).
2. `UserVeVCoin.balance` equals the user's real token balance. A self-burn breaks this (VEV-01).
3. A recomputed ve is never lower than the stored ve (STK-03).
4. Oracles post the Merkle root early, because the claim window counts from `start_epoch` (SSC-05).
5. Every Merkle leaf is between 1 and 100k VCoin (SSC-12).
6. An approved slash executes soon after approval (TOK-03).
7. The externally created VCoin mint has zero supply, no freeze authority and no extensions (TOK-04).
8. Only subsidized sessions need fee handling (GAS-01).
9. The "USDC" mint has 6 decimals (MON-14 below).
10. No program depends on VCoin transfers while the hook is paused (TH-09).

## P0 / P1

| # | Scenario | Where | Status | What happens | Finding |
|---|---|---|---|---|---|
| 1 | 10 wallets × 10 VCoin vote | `programs/governance-protocol/src/state/utils.rs:10,30` | Not handled · **Reproduced (PoC)** | Quorum reached; whole supply outvoted | GOV-02 |
| 2 | Oracle posts root with N × 100k leaves | `programs/sscre-protocol/src/lib.rs:227,292` | Not handled · Verified | Vault drained | SSC-02 |
| 3 | User burns own veVCoin, then unstakes | `programs/vevcoin-token/src/instructions/token/burn.rs:22,37-48` | Not handled · Verified | VCoin locked forever | VEV-01 |
| 4 | Lock expired, vote, unstake, restake elsewhere, vote again | `programs/governance-protocol/src/instructions/vote/cast.rs:71-85` | Not handled · Verified | Same tokens counted twice | GOV-03 |
| 5 | No subsidized action for 3 days | `programs/gasless-protocol/src/instructions/session/execute_action.rs:43-47` | Not handled · Verified | Subsidies dead forever | GAS-02 |
| 6 | Root posted late, or a pause overlaps the claim window | `programs/sscre-protocol/src/lib.rs:158,235` | Not handled · Verified | Rewards forfeited | SSC-05 |
| 7 | `deduct_vcoin_fee(0)` after an admin fee change | `programs/gasless-protocol/src/instructions/fee/deduct_vcoin.rs:26-29` | Not handled · Verified | Whole balance taken into a vault with no withdraw path | GAS-16, GAS-03 |
| 8 | Slash targets the staking `pool-vault` | `programs/vcoin-token/src/contexts/propose_slash.rs:31-34` | Not handled · Verified | Staking insolvent; last unstakers locked | TOK-03 (MON-7) |
| 9 | Top-up with a short lock | `programs/staking-protocol/src/instructions/user/stake.rs:53,116` | Not handled · **Reproduced (PoC)** | 139,440 veVCoin orphaned | STK-03 |

**Repro steps** for each row are in the linked finding. Runnable reproductions for rows 1 and 9 are in `poc/tests/poc.rs`.

## P2 / P3 — not covered elsewhere

### MON-4 · P2 · Verified — Late root strands the epoch (part of SSC-05)
`claim_expiry` is fixed at `start_epoch` (`programs/sscre-protocol/src/lib.rs:158`), and `update_merkle_root` has no deadline. A root posted after day 120 can never be claimed.

### MON-5 · P2 · Hypothetical — Leaves outside [1, 100k] VCoin are unclaimable (SSC-12)
An early epoch with few users can give each leaf more than 100k, for example 10M VCoin shared among 50 users. None of those users can claim, and there are no partial claims.

### MON-6 · P2 · Verified — Approved slashes never expire, and target ownership isn't re-checked (TOK-03)
- **Where:** `programs/vcoin-token/src/state/slash_request.rs:66-68`, `programs/vcoin-token/src/contexts/execute_slash.rs:36-40`.
- **What happens:** the target account is bound by key, not by owner. A non-ATA account whose owner was reassigned is slashed under its new, innocent owner.

### MON-8 · P2 · Hypothetical — The supply cap counts mints only through this program
- **Where:** `programs/vcoin-token/src/instructions/admin/initialize.rs:25`, `programs/vcoin-token/src/instructions/token/mint.rs:28-31`.
- **What happens:** the cap uses `config.total_minted`, not the mint's actual supply. Anything minted before the authority moved isn't counted (TOK-04).

### MON-12 · P3 · Verified — Refund daily counter is never enforced
`energy_refunded_today` is incremented (`programs/content-registry/src/instructions/energy/claim_refund.rs:45`) but never read, so viral 150% refunds have no daily cap. Early-claim forfeiture is CR-06.

### MON-14 · P3 · Verified — Subscriptions ignore pause and never check mint decimals
- `programs/identity-protocol/src/instructions/user/subscribe.rs:11-30` has no `paused` check (ID-08).
- `programs/identity-protocol/src/contexts/subscribe.rs:29-32` never asserts `usdc_mint.decimals == 6`, while prices are 6-decimal constants (`constants.rs:11-13`). A mis-set 9-decimal mint makes subscriptions 1000× cheaper.
- **Fix:** `require!(usdc_mint.decimals == 6)`.

### MON-15 · P3 · Verified — Staking can mint veVCoin while the pool is paused
`programs/staking-protocol/src/instructions/user/extend_lock.rs:12-28` has no `paused` check (TOK-06).

### MON-16 · P3 · Verified — Refund is applied to stale energy
`programs/content-registry/src/instructions/energy/claim_refund.rs:42-44` adds the refund without regenerating energy first, so the max-energy cap is applied to a stale value.

## Invariants that hold (with the guard)

| Invariant | Guard |
|---|---|
| `pool.total_staked` equals the sum of `staked_amount`; the same amount is moved and booked with checked math (holds only if VCoin has no transfer fee, STK-01) | `programs/staking-protocol/src/instructions/user/stake.rs:67-79,125`, `unstake.rs:90-103,122` |
| Unstake needs `now ≥ lock_end` and `amount ≤ staked`; partial burns round up (protocol-favourable) | `unstake.rs:24,30,45-49` |
| `calculate_vevcoin` u128→u64 cast is safe under the 1B cap (max ≈ 1.4e18) | `programs/staking-protocol/src/utils.rs:17-25` |
| No double claim per epoch for epochs < 1024; the 62-byte leaf can't collide with a 64-byte node | `programs/sscre-protocol/src/lib.rs:236,282`, `state/utils.rs:24-35` |
| Fees round up and are capped at the amount | `programs/sscre-protocol/src/lib.rs:243-245`, `programs/vilink-protocol/src/lib.rs:208-210` |
| Only the staking pool PDA can mint or burn veVCoin | `programs/vevcoin-token/src/instructions/token/mint.rs:17-20`, `burn.rs:16-19` |
| One vote per (proposal, voter); voting window and finalize boundaries are consistent | `programs/governance-protocol/src/contexts/cast_vote.rs:21-27`, `cast.rs:32-35`, `finalize.rs:11-14` |
| 5A components ≤ 10000, so the voting-power product can't overflow | `programs/five-a-protocol/src/instructions/oracle/submit_score.rs:27-31` |
| A slash executes at most once | `programs/vcoin-token/src/contexts/execute_slash.rs:24` |
| Expiry comparisons are consistently inclusive or exclusive | `vilink lib.rs:190`, `gasless state/session_key.rs:60`, `sscre lib.rs:235`, `vcoin state/slash_request.rs:67`, `content claim_refund.rs:15-18` |

## Not covered
- **The deployed VCoin mint's extensions:** TransferFee would make vaults insolvent, and TransferHook would break every CPI transfer. This needs the on-chain mint account.
- **Off-chain systems:** the oracle tree builder and the relayer.
- **CPI Guard:** how CPI Guard on a user's veVCoin account interacts with a permanent-delegate burn.
