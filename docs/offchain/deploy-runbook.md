# Deploy runbook (v3)

Every step changes live state. Run each on devnet first, then on mainnet only after the external
audit (see `AUDIT_STATUS.md`).

The upgrade authority (`E2UGn…`) must sign the deploys and every `initialize`: each program's init
is gated to its upgrade authority.

## 0. Build and verify

```bash
anchor build                         # IDL on; CI fails on any "Stack offset" warning
cargo test --workspace               # unit + LiteSVM
node scripts/sync-idl.mjs && git diff --exit-code packages/viwoapp-sdk/src/idl
(cd packages/viwoapp-sdk && npm ci && npm test)
sha256sum target/deploy/*.so         # record the hashes in the release notes
```

## 1. Upgrade programs at their existing addresses

The program keypairs are not in the repo, and should not be. Deploy each binary to its existing
address:

```bash
solana program extend <PROGRAM_ID> <EXTRA_BYTES> -u <cluster>      # only if the new .so is larger
solana program deploy target/deploy/<name>.so \
  --program-id <PROGRAM_ID> --upgrade-authority <AUTHORITY_KEYPAIR> -u <cluster>
```

- Never use `anchor deploy` or `anchor keys sync`. They target the local keypairs in
  `target/deploy/` and would deploy to new addresses (audit TOOL-9).
- `governance-protocol` is upgraded in place as `private-poll` (same id `3R256k…`).
- `transfer-hook` (`9K14FcDRrBeHKD9FPNYeVJaEqJQTac2xspJyb1mM6m48`) is retired. Close it with
  `solana program close` only after confirming that no mint references it.

Devnet state from v2 can be ignored. The new seeds (`vcoin-config-v2` and others) avoid the stale
accounts.

## 2. Tokens and staking

1. **`vcoin_token::initialize(name, symbol, uri)`:** creates the metadata-only VCoin mint PDA.
   Then mint the genesis allocations with `mint_tokens`, including the SSCRE reserve.
2. **`vevcoin_token::initialize`.**
3. **`staking_protocol::create_registrar`.**

## 3. Realms DAO

1. **Create the realm** with `@solana/spl-governance` on `GovER5Lthms3bLBqWub97yVrMmEogzX7xNjdXpPPCVZw`:
   - community mint = VCoin (Token-2022), community token type **Dormant**;
   - voter-weight addin and max-voter-weight addin = `staking_protocol`;
   - a council mint, for the veto.
2. **`staking_protocol::configure_governance(GovER5…, min_vote_score, min_propose_score)`:**
   - set `slash_destination` to the DAO treasury's VCoin token account;
   - this creates the MaxVoterWeightRecord.
3. **Create governances** with these suggested settings:
   - community threshold: % of max voter weight. The max is about 2× the locked VCoin in
     deposits that have not been deactivated; run `deactivate_expired` for expired deposits
     so abandoned stake does not inflate it;
   - `min_community_weight_to_create_proposal` ≥ 1,000 VCoin of weight;
   - hold-up ≥ 48h, and 7 days for program upgrades and treasury;
   - council veto enabled, with a cool-off period.
4. **Create the native treasury** for each governance that needs one.

## 4. Remaining programs

| Program | Initialize | Then |
|---|---|---|
| private-poll | `initialize_config(params)` | DKG ceremony among 3–5 committee members using `zk-voting-sdk::dkg` over authenticated channels (agree on a fresh session id; every member runs `finish`, which checks each dealer's proof of knowledge), publish the dealings, then `set_committee`; every member then confirms the registered committee equals its own `KeyShare.group` |
| five-a | `initialize` | `register_oracle` × ≥ 2 (active after 24h); `set_required_consensus` (a majority of active oracles is always required too) |
| sscre | `initialize_pool(params, first_epoch_budget, fee_bps ≤ 100)` | fund the reserve vault; `register_oracle` × ≥ 2 (epochs cannot start with fewer than the quorum); quorum ≥ 2; `set_guardian` |
| gasless | `initialize(vcoin_fee_per_tx)` | point the Kora relayer at it (see `relayer-kora.md`) |
| identity | `initialize` with the USDC mint (6 decimals) and the treasury USDC account | `add_attester` |
| content-registry | `initialize(engagement_oracle)` | — |
| vilink | `initialize(fee_bps)` with the treasury VCoin account | — |

## 5. Set the guardians, then hand every authority to the DAO

First, for each program that has one (`set_guardian`: vcoin-token, staking, private-poll,
sscre, five-a, gasless, identity, content-registry, vilink), set the guardian to the council
multisig. It can pause and veto a rewards root at once; it cannot unpause or change anything
(see ARCHITECTURE.md, "Emergency guardian"). This matters most for sscre: without a guardian a
24h dispute delay is shorter than any DAO vote, so set one or raise `dispute_delay`.

For each program, `propose_authority(governance_pda)`. After 24h, a DAO proposal executes
`accept_authority`, signed by the governance PDA through `ExecuteTransaction`. From then on:

- every unpause, parameter, oracle, fee and slashing change is a DAO vote plus its hold-up
  (the guardian can still pause at once);
- the deployer key keeps only the upgrade authority.

Move the upgrade authority to the DAO as well, with
`solana program set-upgrade-authority <PROGRAM_ID> --new-upgrade-authority <governance_pda>`,
once the team is ready to give up hotfixes.

## 6. Verify

- Read each config. Its `admin.authority` must be the governance PDA, with no pending transfer,
  and `admin.guardian` must be the council multisig.
- Before anything else, check that nobody initialized a program ahead of you: every
  `initialize` is restricted to the upgrade authority, and vevcoin-token's works even if its
  mint address was sent lamports in advance.
- Run a small staking deposit and a Realms vote with the plugin.
- Run a test private poll end to end on devnet.
- Run an SSCRE epoch with a tiny budget and two oracles.
