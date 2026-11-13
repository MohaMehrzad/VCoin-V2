# Audit remediation tracker

Source: `docs/audit/2026-11/` (internal audit, 2026-11-13). One row per headed finding;
grouped Low items live in each report section and are fixed in the same commit as their section.
Status: `open` → `fixed (<commit>, <test>)` or `removed (<reason>)`.

| ID | Severity | Finding | Section | Planned | Status |
|---|---|---|---|---|---|
| VEV-01 | High | Every unstake reverts unless the veVCoin mint's permanent delegate is the vevcoin config PDA, and a user who burns their own veVCoin is locked out forever | 01-tokens-and-staking.md | C5 | open |
| TOK-01 | Medium | The "permanent delegate" means two different things, which breaks slashing or hands one key unlimited burn power | 01-tokens-and-staking.md | C4 | open |
| TOK-02 | High | "Governance-approved" slashing is controlled by a single admin key | 01-tokens-and-staking.md | C4 | open |
| TOK-03 | Medium | Slashes are easy to dodge and can't be cancelled; approvals never expire; staked VCoin can't be slashed fairly | 01-tokens-and-staking.md | C4 | open |
| TOK-04 | High | `initialize_mint` is front-runnable and never validates the mint; supply-cap claims don't hold | 01-tokens-and-staking.md | C4 | open |
| VEV-02 | High | The veVCoin admin can freeze every unstake instantly, or brick chosen users | 01-tokens-and-staking.md | C5 | open |
| STK-01 | Medium | If VCoin ships the advertised transfer hook, stake and unstake revert; a transfer fee would make the vault insolvent | 01-tokens-and-staking.md | C6 | open |
| STK-02 | High | veVCoin never decays, so expired locks keep full voting power | 01-tokens-and-staking.md | C6 | open |
| STK-03 | Medium | Top-ups and extends can lower recorded ve without burning, leaving orphaned veVCoin that can be minted repeatedly | 01-tokens-and-staking.md | C6 | open |
| STK-04 | Medium | Partial unstake keeps the old tier's boost permanently | 01-tokens-and-staking.md | C6 | open |
| STK-05 | Low | `update_tier` can never succeed | 01-tokens-and-staking.md | C6 | open |
| TOK-05 | Low | `propose_slash` requires `request_id` to equal the exact on-chain second (admin-only and retryable, so a usability issue) | 01-tokens-and-staking.md | C4 | open |
| TOK-06 | Medium | Pause covers almost nothing | 01-tokens-and-staking.md | C4 | open |
| VEV-03 | Low | Docs say "authorized staking program"; the code needs the staking-pool PDA (fails on the first stake and the admin can fix it) | 01-tokens-and-staking.md | C5 | open |
| STK-06 | Low | The tier fee discounts don't exist anywhere | 01-tokens-and-staking.md | C6 | open |
| STK-07 / VEV-04 | Medium | Staking and veVCoin initializers are front-runnable | 01-tokens-and-staking.md | C5/C6 | open |
| GOV-02 | Critical | Voting power is computed on 9-decimal base units, so 10 wallets of 10 VCoin capture governance | 02-governance-protocol.md | C7 (Realms) / C10 | open |
| GOV-01 | High | `finalize_proposal` can be re-run forever: execution is blocked and outcomes can be flipped | 02-governance-protocol.md | C7 (Realms) / C10 | open |
| GOV-03 | High | No snapshot and no `lock_end` check, so the same tokens vote twice and unlocked stake votes | 02-governance-protocol.md | C7 (Realms) / C10 | open |
| GOV-04 | High | One admin key can rewrite any vote outcome after the fact, or brick governance | 02-governance-protocol.md | C7 (Realms) / C10 | open |
| GOV-05 | Medium | Delegation transfers no voting power (an advertised feature that does nothing; no power is gained or lost) | 02-governance-protocol.md | C7 (Realms) / C10 | open |
| GOV-06 | Medium | `execute_proposal` executes nothing | 02-governance-protocol.md | C7 (Realms) / C10 | open |
| GOV-07 | Medium | A proposer can wipe a public vote through the ZK reveal path; bad committee parameters panic or brick | 02-governance-protocol.md | C10 | open |
| GOV-08 | Medium | Private proposals can be finalized before the reveal and always end Rejected | 02-governance-protocol.md | C10 | open |
| GOV-09 | Medium | The proposal threshold is 0.000001 veVCoin, so anyone can spam proposals | 02-governance-protocol.md | C7 (Realms) / C10 | open |
| GOV-10 | Medium | Front-runnable `initialize` lets an attacker choose which programs voting power is read from | 02-governance-protocol.md | C7 (Realms) / C10 | open |
| GOV-11 | Medium | Pause stops only new proposals | 02-governance-protocol.md | C7 (Realms) / C10 | open |
| GOV-12 | Low | The governance docs are wrong in security-relevant ways | 02-governance-protocol.md | C18 | open |
| GOV-13 | Low | Tests use stale interfaces and lock in wrong values | 02-governance-protocol.md | C10 | open |
| ZK-01 | Critical | The OR-proof verifier can never accept a proof, so private voting is dead | 03-zk-private-voting.md | C9/C10 | open |
| ZK-02 | Medium | The SDK reduces challenges mod ℓ but the program compares raw digests, so honest proofs pass about 6% of the time | 03-zk-private-voting.md | C9/C10 | open |
| ZK-03 | Medium | Privacy rests entirely on keys chosen by the proposer, and none are checked | 03-zk-private-voting.md | C9/C10 | open |
| ZK-04 | Medium | A proposer can zero out a public vote through the reveal path | 03-zk-private-voting.md | C9/C10 | open |
| ZK-05 | Medium | Finalize ignores reveal state | 03-zk-private-voting.md | C9/C10 | open |
| ZK-06 | Low | Ballots aren't bound to the voter or the proposal, so they can be copied | 03-zk-private-voting.md | C9/C10 | open |
| ZK-07 | Low | "Production-ready" and "disabled by flag" are both false | 03-zk-private-voting.md | C9/C10 | open |
| ZK-08 | Low | No test anywhere verifies a proof | 03-zk-private-voting.md | C9/C10 | open |
| ZK-09 | Low | Hygiene problems in the reveal path | 03-zk-private-voting.md | C9/C10 | open |
| ZK-10 | Low | SDK key hygiene and tally bounds | 03-zk-private-voting.md | C9/C10 | open |
| SSC-01 | High | The `claim_rewards` account-validation stack frame is 4,544 bytes, over the 4,096-byte SBF limit | 04-sscre-protocol.md | C12 | fixed (C1: ClaimRewards accounts boxed; CI fails on any `Stack offset` warning) |
| SSC-02 | High | Claims are never capped by the epoch's allocation or the emission limits, so one root can drain the vault | 04-sscre-protocol.md | C12 | open |
| SSC-03 | Medium | Following the documented leaf recipe makes an epoch unclaimable | 04-sscre-protocol.md | C12 | open |
| SSC-04 | Medium | A wrong or malicious root can never be replaced, and roots bypass the circuit breaker | 04-sscre-protocol.md | C12 | open |
| SSC-05 | Low | Pauses, circuit-breaker trips and late roots forfeit users' rewards (the tokens stay in the vault) | 04-sscre-protocol.md | C12 | open |
| SSC-06 | Low | Claims would fail if VCoin were minted with its documented transfer hook (latent; the extension can only be set when a mint is created) | 04-sscre-protocol.md | C12 | open |
| SSC-07 | Low | "Gasless" claims still need SOL for rent (a relayer can work around it by funding the user earlier in the same transaction) | 04-sscre-protocol.md | C12 | open |
| SSC-08 | Medium | `initialize_pool` is front-runnable, with an arbitrary mint | 04-sscre-protocol.md | C12 | open |
| SSC-09 | Medium | No test ever executes `claim_rewards` | 04-sscre-protocol.md | C12 | open |
| SSC-10 | Low | Epochs from 1024 on can never be claimed, and that is reachable quickly | 04-sscre-protocol.md | C12 | open |
| SSC-11 | Low | Oracle and fee-recipient management is missing | 04-sscre-protocol.md | C12 | open |
| SSC-12 | Low | Out-of-range leaves are silently unclaimable | 04-sscre-protocol.md | C12 | open |
| SSC-13 | Low | Misleading error, and inconsistent accounting units | 04-sscre-protocol.md | C12 | open |
| CR-01 | Medium | Users choose their own tier, which bypasses the anti-spam model | 05-content-registry.md | C15 | open |
| CR-02 | Low | Content records can be squatted by front-running a predictable `tracking_id` (denial of one ID; the victim can retry with a random ID) | 05-content-registry.md | C15 | open |
| CR-03 | Low | Energy regeneration throws away partial progress (less than 1 energy lost per successful action) | 05-content-registry.md | C15 | open |
| CR-04 | Medium | Both config initializers are front-runnable | 05-content-registry.md | C15 | open |
| CR-05 | Low | Every post write-locks one global account, which caps throughput | 05-content-registry.md | C15 | open |
| CR-06 | Low | An early refund claim burns the refund forever; deleted content still earns refunds (impact limited to rate-limit energy) | 05-content-registry.md | C15 | open |
| CR-07 | Medium | The engagement "oracle" is the admin key, with no timelock on authority transfer | 05-content-registry.md | C15 | open |
| CR-08 | Low | Pause covers only `create_content` | 05-content-registry.md | C15 | open |
| CR-09 | Low | A tier change applies the new regeneration rate retroactively | 05-content-registry.md | C15 | open |
| CR-10 | Low | The edit rules aren't enforced | 05-content-registry.md | C15 | open |
| CR-11 | Low | Rent is locked forever | 05-content-registry.md | C15 | open |
| CR-12 | Low | Test helpers use the wrong account order and count | 05-content-registry.md | C15 | open |
| CR-13 | Low | The docs describe a different program | 05-content-registry.md | C15 | open |
| 5A-01 | High | "3-of-N consensus" is actually 1-of-N, and oracles can't be removed | 06-five-a-protocol.md | C13 | open |
| 5A-02 | Medium | Front-runnable `initialize`; unvalidated, immutable config addresses | 06-five-a-protocol.md | C13 | open |
| 5A-03 | Medium | No 24h timelock on authority transfer | 06-five-a-protocol.md | C13 | open |
| 5A-04 | Low | The consensus design lets the first oracle block honest updates | 06-five-a-protocol.md | C13 | open |
| 5A-05 | Low | Vouching has no stake, no slashing, and a gameable evaluation (no program reads the vouch stats, so the gaming has no on-chain effect today) | 06-five-a-protocol.md | C13 | open |
| 5A-06 | Medium | Vouch slots can be locked permanently | 06-five-a-protocol.md | C13 | open |
| 5A-07 | Low | Pause doesn't cover all mutating instructions | 06-five-a-protocol.md | C13 | open |
| 5A-08 | Medium | No test exercises consensus or vouching | 06-five-a-protocol.md | C13 | open |
| 5A-09 | Low | Docs describe a different program | 06-five-a-protocol.md | C13 | open |
| ID-01 | High | `update_verification` always reverts, and users are stuck at their first attested level | 07-identity-protocol.md | C14 | open |
| ID-02 | Medium | Levels can never be revoked or downgraded; expiry and attester removal have no effect | 07-identity-protocol.md | C14 | open |
| ID-03 | Low | The "SAS integration" never reads SAS; one attestation can be reused across wallets (each link still needs a trusted attester's co-signature) | 07-identity-protocol.md | C14 | open |
| ID-04 | Low | Email + phone + social is classified as "KYC" (reproduced by PoC `bug_id04_…`; looks deliberate in code but contradicts the level's documented meaning) | 07-identity-protocol.md | C14 | open |
| ID-05 | Medium | Subscription renewal throws away paid time; the free tier overwrites paid | 07-identity-protocol.md | C14 | open |
| ID-06 | Medium | Front-runnable `initialize`; a mis-set treasury permanently bricks subscriptions | 07-identity-protocol.md | C14 | open |
| ID-07 | Medium | No 24h timelock on authority transfer | 07-identity-protocol.md | C14 | open |
| ID-08 | Low | Pause covers only `create_identity` | 07-identity-protocol.md | C14 | open |
| ID-09 | Medium | No executed test of verification, attestation, or subscription renewal/downgrade (only one paid `subscribe(1)` runs, in `tests/identity-protocol.ts:229-241`) | 07-identity-protocol.md | C14 | open |
| ID-10 | Low | Docs describe a different API | 07-identity-protocol.md | C14 | open |
| TH-01 | High | Wrong instruction discriminator and account order, so Token-2022 can never invoke it | 08-transfer-hook.md | C3 | removed (C3: transfer-hook program retired; VCoin mint is metadata-only; activity scoring off-chain → 5A oracle, docs/offchain/activity-indexer.md) |
| TH-02 | High | The ExtraAccountMetaList can never be created, and its entries are wrong | 08-transfer-hook.md | C3 | removed (C3: transfer-hook program retired; VCoin mint is metadata-only; activity scoring off-chain → 5A oracle, docs/offchain/activity-indexer.md) |
| TH-03 | Medium | Anyone can call `execute` directly to forge activity, poison pair stats and block real transfers | 08-transfer-hook.md | C3 | removed (C3: transfer-hook program retired; VCoin mint is metadata-only; activity scoring off-chain → 5A oracle, docs/offchain/activity-indexer.md) |
| TH-04 | High | There is no 5A update, no tip recording, and no circular wash detection | 08-transfer-hook.md | C3 | removed (C3: transfer-hook program retired; VCoin mint is metadata-only; activity scoring off-chain → 5A oracle, docs/offchain/activity-indexer.md) |
| TH-05 | Medium | The wash-trading heuristic blocks normal payments | 08-transfer-hook.md | C3 | removed (C3: transfer-hook program retired; VCoin mint is metadata-only; activity scoring off-chain → 5A oracle, docs/offchain/activity-indexer.md) |
| TH-06 | Low | The query instructions always fail because `bump` is never stored (log-only views; the data can still be read over RPC) | 08-transfer-hook.md | C3 | removed (C3: transfer-hook program retired; VCoin mint is metadata-only; activity scoring off-chain → 5A oracle, docs/offchain/activity-indexer.md) |
| TH-07 | Low | The "daily" activity cap is lifetime, and dust earns full credit (nothing consumes the score) | 08-transfer-hook.md | C3 | removed (C3: transfer-hook program retired; VCoin mint is metadata-only; activity scoring off-chain → 5A oracle, docs/offchain/activity-indexer.md) |
| TH-08 | Medium | Self-transfers between your own token accounts fail or lose data | 08-transfer-hook.md | C3 | removed (C3: transfer-hook program retired; VCoin mint is metadata-only; activity scoring off-chain → 5A oracle, docs/offchain/activity-indexer.md) |
| TH-09 | Medium | Front-runnable `initialize`, and one key can freeze every VCoin transfer | 08-transfer-hook.md | C3 | removed (C3: transfer-hook program retired; VCoin mint is metadata-only; activity scoring off-chain → 5A oracle, docs/offchain/activity-indexer.md) |
| TH-10 | Low | Activity is recorded under token-account addresses, not wallets | 08-transfer-hook.md | C3 | removed (C3: transfer-hook program retired; VCoin mint is metadata-only; activity scoring off-chain → 5A oracle, docs/offchain/activity-indexer.md) |
| TH-11 | Medium | No test exercises `execute` or a Token-2022 transfer | 08-transfer-hook.md | C3 | removed (C3: transfer-hook program retired; VCoin mint is metadata-only; activity scoring off-chain → 5A oracle, docs/offchain/activity-indexer.md) |
| GAS-02 | High | Subsidized actions stop working permanently after 3 days without one | 09-gasless-protocol.md | C11 | open |
| GAS-03 | High | Collected VCoin fees are locked forever | 09-gasless-protocol.md | C11 | open |
| GAS-01 | High | Session keys authorize nothing, and "sponsoring" moves no SOL | 09-gasless-protocol.md | C11 | open |
| GAS-16 | Medium | The default fee path charges whatever the admin sets, with no cap (SEC-2) | 09-gasless-protocol.md | C11 | open |
| GAS-04 | Low | Scope checks can be bypassed (sessions authorize nothing today, so only the event field is wrong; GAS-01) | 09-gasless-protocol.md | C11 | open |
| GAS-05 | Low | Expired sessions count toward the 5-session cap, and rent is never returned | 09-gasless-protocol.md | C11 | open |
| GAS-06 | Low | Invalid session parameters are silently replaced with the maximums | 09-gasless-protocol.md | C11 | open |
| GAS-07 | Medium | Front-runnable `initialize` with an unvalidated mint | 09-gasless-protocol.md | C11 | open |
| GAS-08 | Medium | Fee deduction reverts if VCoin has a transfer hook (latent; no hook is configured anywhere) | 09-gasless-protocol.md | C11 | open |
| GAS-09 | Medium | Every action write-locks one global account | 09-gasless-protocol.md | C11 | open |
| GAS-10 | Low | Sybil wallets can exhaust the global subsidy budget | 09-gasless-protocol.md | C11 | open |
| GAS-15 | Medium | No test covers the rollover, expiry, scope or budget paths | 09-gasless-protocol.md | C11 | open |
| VL-1 | Medium | Front-runnable `initialize`; the treasury and mint are then permanent | 10-vilink-protocol.md | C16 | open |
| VL-2 | Low | Disabling an action type doesn't stop existing links (the global pause, which all three execute handlers check, still works as an emergency stop) | 10-vilink-protocol.md | C16 | open |
| VL-3 | Low | Vouch and Follow links ignore `max_executions` (inflates ViLink-only stats that no other program reads) | 10-vilink-protocol.md | C16 | open |
| VL-4 | Low | The treasury can never be changed, and the zero key is accepted | 10-vilink-protocol.md | C16 | open |
| VL-5 | Medium | Tips revert if VCoin has a transfer hook | 10-vilink-protocol.md | C16 | open |
| VL-6 | Medium | Five action types can be created but never executed, and no action can be closed | 10-vilink-protocol.md | C16 | open |
| VL-7 | Low | The dApp registry is never enforced | 10-vilink-protocol.md | C16 | open |
| VL-16 | Low | The fee rate is read at execution, not when the link is created (MON-9) | 10-vilink-protocol.md | C16 | open |
| SDK-01 | High | Every `build*Transaction` returns an empty transaction | 11-typescript-sdk.md | C17 | open |
| SDK-02 | High | Published ESM and subpath exports point to files that don't exist | 11-typescript-sdk.md | C17 | open |
| SDK-03 | High | The ViLinkAction decoder makes every action look expired | 11-typescript-sdk.md | C17 | open |
| SDK-04 | High | The UserStake decoder is wrong, and `canUnstake` throws for about 255 in 256 users | 11-typescript-sdk.md | C17 | open |
| SDK-05 | High | The StakingPool decoder is wrong | 11-typescript-sdk.md | C17 | open |
| SDK-06 | High | The rewards decoders and claim-status logic are wrong | 11-typescript-sdk.md | C17 | open |
| SDK-07 | Medium | The Merkle helpers can never match the chain (no SDK path calls them, and they are marked as placeholders) | 11-typescript-sdk.md | C17 | open |
| SDK-08 | High | Three PDA seeds are wrong | 11-typescript-sdk.md | C17 | open |
| SDK-09 | High | The VoteChoice and ProposalStatus enums are wrong | 11-typescript-sdk.md | C17 | open |
| SDK-10 | High | The Proposal decoder and proposal-ID loop are wrong | 11-typescript-sdk.md | C17 | open |
| SDK-11 | Medium | The GovernanceConfig and VoteRecord decoders are wrong | 11-typescript-sdk.md | C17 | open |
| SDK-12 | Medium | Voting power, threshold and quorum disagree with the program | 11-typescript-sdk.md | C17 | open |
| SDK-13 | Medium | The content client is wrong | 11-typescript-sdk.md | C17 | open |
| SDK-14 | Medium | The identity client is wrong | 11-typescript-sdk.md | C17 | open |
| SDK-15 | Medium | The default VCoin mint is the System Program | 11-typescript-sdk.md | C17 | open |
| SDK-16 | Medium | The SDK tests lock in the wrong values | 11-typescript-sdk.md | C17 | open |
| SDK-17 | Medium | The README examples don't work | 11-typescript-sdk.md | C17 | open |
| TOOL-1 | High | The main CI suite has been red on every push, including the release | 12-tests-ci-tooling.md | C1 | fixed (C1: CI rewritten; fmt/clippy -D warnings/anchor build+IDL/cargo test all green locally) |
| TOOL-2 | High | None of the 209 Rust integration tests in `programs/*/tests` compile | 12-tests-ci-tooling.md | C1 | fixed (C1: broken solana-program-test suites removed; LiteSVM harness `integration-tests/`) |
| TOOL-3 | High | IDL generation is broken, so the TypeScript tests and an IDL-based SDK are impossible | 12-tests-ci-tooling.md | C1 | fixed (C1: `anchor build` with IDL succeeds for all programs) |
| TOOL-4 | High | No test executes any program instruction end to end | 12-tests-ci-tooling.md | C1 | partial (C1 harness + smoke test; per-program transaction tests land with each program commit) |
| TOOL-5 | Medium | The fuzz workflow reports success while doing nothing | 12-tests-ci-tooling.md | C1 | fixed (C1: fuzz workflow and no-op trident crate removed) |
| TOOL-6 | Medium | The CI environment cannot match the project's toolchain | 12-tests-ci-tooling.md | C1 | partial (C1: toolchain file, Agave from release.anza.xyz, localnet; yarn/e2e in C17) |
| TOOL-7 | Medium | Root `package.json` scripts point at things that don't exist | 12-tests-ci-tooling.md | C1 | fixed (C1: root scripts point at real crates) |
| TOOL-8 | Medium | 14 unit tests fail; 13 of them are stale size assertions | 12-tests-ci-tooling.md | C1 | fixed (C1: size tests compare LEN to Borsh size; epoch test asserts bitmap semantics) |
| TOOL-9 | Medium | Program keypairs are not in the repo, so a fresh build deploys to the wrong addresses | 12-tests-ci-tooling.md | C1 | partial (C1: localnet only; deploy runbook in C18) |
| TOOL-10 | High | The published `@viwoapp/sdk@2.0.1` has broken ESM and subpath exports | 12-tests-ci-tooling.md | C17 | open |
| TOOL-11 | Medium | Clippy deny-level error: an always-true scope check | 12-tests-ci-tooling.md | C1 | fixed (C1: `scope & !VALID_SCOPE_MASK == 0`) |
| TOOL-12 | Low | Smaller tooling problems | 12-tests-ci-tooling.md | C1 | partial (C1: fmt + clippy clean; SDK eslint in C17) |
| X-1 | High | Every program's `initialize` can be front-run | 13-cross-cutting-security.md | C2+per-program | partial (C2: `require_upgrade_authority` + Accounts pattern; applied in each program commit) |
| X-2 | Medium | 7 of 11 programs have no authority-transfer timelock, contrary to the "all 11" claim | 13-cross-cutting-security.md | C2+per-program | partial (C2: `AdminState` 24h two-step transfer; applied in each program commit) |
| X-3 | Medium | Admin powers that take effect instantly, with no timelock | 13-cross-cutting-security.md | C2+per-program | open |
| X-4 | Low | Instructions anyone can call that change other people's state | 13-cross-cutting-security.md | C2+per-program | open |
| X-5 | Low | Raw cross-program reads depend on layout (no spoofing is possible, since only the owning program can allocate the PDA; this is a maintenance risk) | 13-cross-cutting-security.md | C2+per-program | open |
| X-6 | Medium | Almost no account in the system can be closed | 13-cross-cutting-security.md | C2+per-program | open |
| X-7 | Medium | Global write locks on hot paths | 13-cross-cutting-security.md | C2+per-program | open |
