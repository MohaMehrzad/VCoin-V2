# 17 — QA: user flows traced end to end (SDK → program → state)

**Method:** each flow is traced from the SDK call a dApp would make, through the instruction and its account validation, to the state change. At every step we asked: what if it succeeds, fails, is interrupted, happens twice, or happens out of order?

The "app" here is any client built on `@viwoapp/sdk` or on direct Anchor calls. The repo has no UI.

## Flow map

```
Wallet ─┬─ Stake ── stake ─CPI─► vevcoin.mint ── UserStake/ve
        ├─ Unstake ── unstake ─CPI─► vevcoin.burn ── VCoin back
        ├─ Propose ── create_proposal ── Vote ── cast_vote ── finalize ── execute
        ├─ Private vote ── enable_private_voting ── cast_private_vote ── initiate_reveal ── submit_share ×t ── aggregate
        ├─ Claim rewards ── (oracle: start_epoch, update_merkle_root) ── claim_rewards
        ├─ Tip link ── create_action ── execute_tip_action
        ├─ Session key ── create_session_key ── execute_session_action (subsidized) ── deduct_vcoin_fee
        ├─ Content ── initialize_user_energy ── create_content ── edit ── update_engagement (admin) ── claim_energy_refund
        ├─ Identity ── create_identity ── link_sas_attestation (attester co-sign) ── subscribe
        └─ Reputation ── (oracle) submit_score ── vouch_for_user ── evaluate_vouch
```

## CRITICAL: core flows that cannot complete

### F1 · Stake → unstake
1. **SDK `buildStakeTransaction`** returns an empty transaction (SDK-01). The wallet signs, a fee is paid, a success signature comes back, and **nothing is staked**.
2. **Direct call to `stake`** works if `vevcoin.config.staking_protocol` is set to the staking-pool **PDA**. The docs say "staking program", which fails `Unauthorized` (VEV-03).
3. **Unstake after the lock ends** fails unless the veVCoin mint's PermanentDelegate is the vevcoin config PDA (VEV-01). Nothing in the repo sets that. When it fails, **VCoin is stuck in the vault** and there is no recovery instruction.
4. **"Burn spam" tools in the user's wallet** that burn the soulbound veVCoin lock the user's VCoin forever (VEV-01, case 2).
5. **Twice / out of order:**
   - A second stake with a shorter lock silently lowers the recorded ve and orphans tokens (STK-03, PoC).
   - Partial unstake keeps the higher tier's boost (STK-04).
   - The SDK's `canUnstake` throws for about 255 in 256 users (SDK-04).
6. **Interrupted:** if the admin calls `update_staking_protocol` (instant), every unstake fails (VEV-02).

### F2 · Private vote
1. `enable_private_voting`: the proposer supplies all keys, and none are checked (ZK-03).
2. **`cast_private_vote` always fails** with `InvalidOrProof` (ZK-01; PoC 20/20).
3. `submit_decryption_share` fails about 94% of the time for honest members (ZK-02).
4. **Out of order:** `finalize` before the reveal leaves the proposal Rejected with 0 votes (GOV-08).
5. **Result:** no private proposal can ever pass.

### F3 · Propose → vote → finalize → execute
1. **SDK `getActiveProposals`** always returns `[]`: wrong config seed (SDK-08) and status 0 is Pending (SDK-09). The UI shows no proposals.
2. **The user votes "Against" through the SDK enum**, and the chain records **Abstain** (SDK-09).
3. **Out of order / twice:** anyone can call `finalize` repeatedly, so execution never becomes possible and an Executed proposal flips back to Passed (GOV-01).
4. **Hostile user:** 10 wallets of 10 VCoin decide any vote (GOV-02, PoC).
5. **Success path:** `execute` sets a flag and changes nothing (GOV-06).

### F4 · Claim SSCRE rewards
1. **SDK `getUnclaimedEpochs`** throws on `.toNumber()` (wrong offsets), or returns `[]` because it computes expiry from 1970 (SDK-06).
2. **SDK `verifyMerkleProof`** rejects every valid proof (SDK-07).
3. **On-chain `claim_rewards`** has a stack frame over the SBF limit, which may access-violate on every call (SSC-01).
4. **If the oracle followed the docs' leaf recipe,** every claim fails `InvalidMerkleProof`, and the root can't be fixed (SSC-03, SSC-04, PoC).
5. **Interrupted:** a pause or circuit-breaker trip overlapping the claim deadline forfeits the rewards (SSC-05).
6. **Twice:** blocked correctly by the epoch bitmap. From epoch 1024 on, every claim fails (SSC-10, PoC).

### F5 · Gasless session action
1. **Subsidized actions work only if one happens at least every 2 days** after `initialize`. After that they are permanently dead (GAS-02).
2. **`deduct_vcoin_fee(0)`** takes whatever fee the admin last set, with no cap (GAS-16). Fees can never be withdrawn (GAS-03).
3. **Power user:** the 6th daily session fails `MaxSessionsReached` until 5 old sessions are revoked with the main wallet (GAS-05).

## HIGH: works, but shows wrong data or does the wrong thing

| Flow | Step | What the user sees | Finding |
|---|---|---|---|
| Tip link | SDK `isActionValid` | "Action has expired" for every link | SDK-03 |
| Tip link | Admin raises fee after link creation | Target silently receives less | VL-16 |
| Tip link | Self-tip (executor = target) | Reverts the first time, then corrupts stats | VL-9 |
| Tip link | Wallet with a DEX approval or CPI Guard | Can't tip | VL-10 |
| Content | `initialize_user_energy(4)` | Any user gets Platinum limits | CR-01 |
| Content | Post every 2 min | Energy never regenerates | CR-03 |
| Content | Claim refund at 24h with no engagement yet | Refund burned forever | CR-06 |
| Content | SDK `getContent` | "Not found" for everything (wrong seed) | SDK-08 |
| Identity | Renew subscription on day 29 | Remaining day lost | ID-05 |
| Identity | Upgrade Basic → KYC | Impossible (attestation is `init` once; admin path dead) | ID-01 |
| Identity | SDK `getIdentity` | Created 1970; KYC labelled "Standard" | SDK-14 |
| Reputation | One oracle key | Sets any score instantly | 5A-01 |
| Staking stats | SDK `getStats` / `getPool` | Garbage TVL; random "paused" | SDK-05 |
| Balances | SDK `getVCoinBalance` with default config | 0 for everyone | SDK-15 |
| Any ESM app | `import { ViWoClient } from "@viwoapp/sdk"` | `ERR_MODULE_NOT_FOUND` | TOOL-10 |

## Wiring verification summary
- **SDK builders → instructions:** 23 builders checked, 0 wired (all STUB).
- **SDK reads → account layouts:**
  - **Correct:** GaslessConfig, SessionKey, UserGaslessStats, ViLinkConfig, UserActionStats, Delegation, the UserClaim prefix, and UserScore (except `isPrivate`).
  - **Wrong:** StakingPool, UserStake, Proposal, GovernanceConfig, VoteRecord, ViLinkAction, RewardsPoolConfig, EpochDistribution, ContentRecord, UserEnergy, Identity.
- **SDK PDAs → program seeds:** all correct except governance config, vote record and content record.
- **Program IDs:** all 11 SDK and `Anchor.toml` IDs match `declare_id!`.
- **Cross-program wiring:**
  - **Staking → vevcoin CPI:** wired. It depends on the pool-PDA config and the mint's permanent delegate.
  - **Governance → staking and 5A:** raw reads with correct offsets.
  - **All other cross-program links are stored IDs that are never used:** 5A ← hook, SSCRE ← 5A, content ← staking/identity, vilink ← 5A/staking/content/governance/gasless, gasless ← SSCRE.

## Missing protections

| Protection | Missing in |
|---|---|
| Idempotency / "twice" | `finalize_proposal` (GOV-01); Vouch/Follow `max_executions` (VL-3); refund at 0% (CR-06) |
| Pause coverage | governance (vote/finalize/execute), identity (subscribe/attest), content (all but create), 5A (evaluate/oracle), staking (extend), vcoin (slash) |
| Expiry / deadline | slash approvals (TOK-03), passed proposals, reveal phase, SSCRE root posting |
| Cancel / undo | slash requests, private-voting enable, wrong Merkle root, wrong treasury (VL-4) |
| Account cleanup | No `close` anywhere (X-6) |
| Empty-transaction guard | SDK `sendTransaction` (SDK-01) |

## Summary

| Metric | Count |
|---|---|
| Flows traced | 10 |
| Core flows that cannot complete as designed | 4 (stake → unstake without manual mint setup, private vote, governance execution, subsidized gasless after an idle period) |
| Core flows broken through the SDK | all 10 (SDK-01) |
| SDK public methods checked | ~90 (see `11-typescript-sdk.md` wiring table) |
