# 13 — Cross-cutting security and privilege (all 11 programs)

**Source:** an attacker's-table pass over every `#[derive(Accounts)]` context and its handler.

**Already covered elsewhere:**
- Program-specific issues are in sections 01–10.
- Voting-power Sybil capture (SEC-1) is **GOV-02**.
- Claim-window forfeiture (SEC-3) is **SSC-05**.
- The uncapped default gasless fee (SEC-2) is **GAS-16**.
- The global freeze via hook pause (SEC-4) is **TH-09**.
- Governance pause gaps (SEC-5) are **GOV-11**.
- Vouch griefing (SEC-7) is **5A-05**.

---

### X-1 · High · Confirmed · security — Every program's `initialize` can be front-run
All 11 programs create a singleton config PDA from any signer. Whoever calls first becomes authority, and usually also fixes mints, treasuries and program IDs that have no setter.

| Program | Where | Worst outcome if front-run |
|---|---|---|
| vcoin-token | `contexts/initialize_mint.rs:7-24` | Attacker mints up to the full 1B supply (TOK-04) |
| vevcoin-token | `contexts/initialize_mint.rs:7-24` | Attacker controls veVCoin issuance |
| staking-protocol | `contexts/initialize_pool.rs:9-38` | Pool and vault PDAs taken; redeploy needed |
| governance | `contexts/initialize.rs:7-17` | Attacker chooses which "staking" and "5A" programs voting power is read from (GOV-10) |
| five-a | `contexts/initialize.rs:17-21` | Attacker registers oracles, which sets governance weight |
| identity | `contexts/initialize.rs:17-21` | Attacker picks the USDC mint and treasury |
| sscre | `lib.rs:467-477` | Attacker owns the 350M reward pool's oracle list |
| content-registry | `contexts/initialize.rs:6-19`, `initialize_energy.rs:6-19` | Attacker controls refunds and tiers |
| transfer-hook | `contexts/initialize.rs:9-22` | Attacker binds the wrong mint and holds the global pause |
| gasless | `contexts/initialize.rs:9-33` | Attacker binds a fake mint |
| vilink | `lib.rs:574-582` | Attacker fixes mint and treasury forever |

- **Fails when:** on mainnet, deploy and initialize happen in separate transactions, which is what this repo's own test flow does (`tests/vcoin-token.ts:52-91`).
- **Fix:**
  - In every initializer, require the signer to be the program's upgrade authority, using a ProgramData check:
    `#[account(constraint = program_data.upgrade_authority_address == Some(authority.key()))] pub program_data: Account<'info, ProgramData>`.
  - Alternatively, deploy and initialize atomically through a script.

### X-2 · Medium · Confirmed · security — 7 of 11 programs have no authority-transfer timelock, contrary to the "all 11" claim
- **Where:** `accept_authority` in:
  - `programs/vilink-protocol/src/lib.rs:512-523`
  - `programs/sscre-protocol/src/lib.rs:395-412`
  - `programs/{five-a-protocol,gasless-protocol,identity-protocol,content-registry,transfer-hook}/src/instructions/admin/accept_authority.rs:6-13`
- **Code:** `config.authority = new_authority;`, with no `pending_authority_activated_at` check.
- **Fails when:** a stolen admin key proposes and accepts in one transaction. `SECURITY.md:71,94,144`, `README.md:258` and `CHANGELOG.md:64,168-169` claim a 24h timelock everywhere. AUDIT_COMPLETE item H-NEW-01 marks it fixed.
- **Fix:** reuse the vcoin/vevcoin/staking/governance pattern (`AUTHORITY_TRANSFER_TIMELOCK`).

### X-3 · Medium · Confirmed · design — Admin powers that take effect instantly, with no timelock
| Program | Instruction | Effect | Blast radius |
|---|---|---|---|
| vcoin-token | `mint_tokens` | Mint the unminted remainder to any account | Dilution, up to the 1B cap |
| vcoin-token | `update_permanent_delegate` + `approve_slash` | One key slashes anyone (TOK-02) | Any holder |
| vevcoin-token | `update_staking_protocol` | Freeze all unstakes or brick chosen users (VEV-02) | All staked VCoin |
| governance | `update_config` | Quorum, voting period and timelock read live at finalize (GOV-04) | Every open proposal |
| five-a | `register_oracle` | New oracle sets any score instantly (consensus = 1) | Governance weight ×2, vouch rights |
| identity | `add_trusted_attester` | Admin attests itself and grants any verification level | Fake KYC for Sybils |
| content-registry | `update_engagement`, `update_user_tier` | Controls refunds and energy for everyone | Anti-spam model |
| transfer-hook | `set_paused`, `update_config(block_wash_trading)` | Freeze every VCoin transfer once the hook is live | All holders |
| gasless | `update_fee_config`, `update_daily_budget` | Unbounded default fee (GAS-16), budget set to 0 | Every default-path payer |
| sscre | `register_oracle`, `set_paused`, `trigger_circuit_breaker` | Drain via oracle root (SSC-02); forfeit claims (SSC-05) | 350M reserve |
| vilink | `set_platform_fee` (bounded 0.1–10%), `set_enabled_actions` | Take up to 10% of each tip | Tips |
| 7 programs | `accept_authority` | Takeover effective immediately (X-2) | Everything |

- **Fix:**
  - Put a multisig behind every authority.
  - Timelock parameter changes and emit events for them.
  - Where governance is meant to control parameters (as `constants.rs:25-26` says), wire governance execution (GOV-06) to these setters.

### X-4 · Low · Confirmed · design — Instructions anyone can call that change other people's state
| Program | Instruction | What a stranger can change |
|---|---|---|
| governance | `finalize_proposal`, `execute_proposal`, `initiate_reveal`, `aggregate_revealed_votes` | Status, execution time, reveal, tally (GOV-01/07) |
| governance | `delegate_votes` | Inflate any delegate's `unique_delegators` and `total_delegated_vevcoin` (`delegate.rs:41-47`) |
| five-a | `evaluate_vouch`, `vouch_for_user` | Timing of vouch outcomes; fill a target's vouch slots without consent (5A-05) |
| five-a | `submit_score` (oracle) | `user` is unchecked, so oracles can create `UserScore` for PDAs and program IDs and inflate `total_users` (`contexts/submit_score.rs:45`) |
| transfer-hook | `execute`, `initialize_extra_account_meta_list` | Forge activity and pair data (TH-03); create program-owned accounts at arbitrary addresses (TH-02) |
| vilink | `create_action`, `execute_vouch_action`, `execute_follow_action` | Target any pubkey with arbitrary metadata (phishing links); unlimited executions (VL-3) |
| identity | `create_identity` | Take any username or DID hash; no uniqueness, and any UTF-8 including homoglyphs and RTL override is accepted (`create_identity.rs:11-24`) |

### X-5 · Low · Confirmed · design — Raw cross-program reads depend on layout (no spoofing is possible, since only the owning program can allocate the PDA; this is a maintenance risk)
- **Where:** `programs/governance-protocol/src/instructions/vote/cast.rs:79-83,101-103`
- **What happens:** it reads `UserStake` and `UserScore` at hard-coded byte offsets after re-deriving the PDA, with no owner or discriminator check. The offsets are correct today.
- **Risk:** any field added to the staking or 5A structs silently corrupts voting power. The SDK already shows how easily this drifts (SDK-04/05).
- **Fix:** depend on the program crates with `features = ["cpi"]` and use `Account<'info, staking_protocol::UserStake>` with `seeds::program`.

### X-6 · Medium · Confirmed · design — Almost no account in the system can be closed
- **What happens:** the only `close =` in `programs/` is `programs/governance-protocol/src/contexts/revoke_delegation.rs:9` (`close = delegator`). Every vote record, proposal, session key, ViLink action, content record, pending score, claim record, slash request and decryption share keeps its rent forever.
- **Scale:** roughly 0.0015–0.003 SOL per account. At 1M users that is thousands of SOL.
- **Fix:** add `close = <payer>` paths for terminal states.

### X-7 · Medium · Likely · perf — Global write locks on hot paths
- **Where:** content `create`/`delete` (CR-05), every gasless action and fee (GAS-09), every ViLink create/execute (VL-13), every 5A score submission (5A-L1).
- **What happens:** each of these write-locks its program's singleton config just to increment counters. That serialises all of that program's traffic within a block.
- **Fix:** derive the counters from events or an indexer, or shard them.

### Guards that hold (verified)
- **Staking:**
  - `UserStake` is bound by seeds and owner, and the vault by seeds (`staking contexts/unstake.rs:21-27`, `stake.rs:51-57`).
  - The staking authority has no path to vault funds, and `unstake` is **not** pausable, which is good.
- **veVCoin:** mint and burn require the configured staking signer (`vevcoin mint.rs:17-20`, `burn.rs:16-19`).
- **SSCRE:** leaves are built from the signer's key with a domain prefix. Each epoch's bitmap blocks double claims, and proofs are capped at 32 nodes.
- **Governance:**
  - The VoteRecord PDA is shared by public and private votes, so there is one vote per wallet.
  - `aggregate` checks the owner, PDA, proposal and member of each share account.
- **Gasless:** the session signer and user are bound to the session PDA.
- **5A:** the reverse-vouch PDA is pinned with `seeds::program`.
- **ViLink:** nonce and fee bounds are enforced.
- **Content-registry:** edit, delete and refund use `has_one = author`.
- **All programs:** `accept_authority` requires `pending != default` and `signer == pending`.
