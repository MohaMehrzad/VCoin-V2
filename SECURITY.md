# Security policy

## Status

v3 has **not** been externally audited (see [`AUDIT_STATUS.md`](AUDIT_STATUS.md)). The internal
review and its remediation are documented in `docs/audit/2026-11/` and `FIX-TRACKER.md`.

## Reporting a vulnerability

Email **security@viwoapp.com**. Include a description, reproduction steps (a failing test is
ideal), and the impact. Please do not open public issues for vulnerabilities.

We acknowledge reports within 72 hours and share a fix timeline after triage.

## Trust model

The programs remove single-key control wherever they can. The remaining trust is explicit:

| Role | Can do | Cannot do | Check |
|---|---|---|---|
| Upgrade authority | Upgrade programs; run each program's one-time `initialize` | Anything else once authority is handed over | Move it to the DAO when hotfixes are no longer needed |
| Program authority (the Realms governance PDA after launch) | Pause and unpause; set parameters within on-chain bounds; manage oracles and attesters; propose slashes; set the guardian | Act without a passed vote plus hold-up; skip the 24h fee, treasury, oracle and authority timelocks | Realms thresholds, hold-up and council veto |
| Guardian (optional per program; a council multisig) | Pause at once; veto a proposed SSCRE root | Unpause; move funds; change any parameter, oracle, fee or key | The authority unpauses and can replace it; pauses never block stake withdrawals and extend claim and reveal deadlines |
| 5A oracles | Set reputation scores; evaluate vouches | Apply a score alone or as a minority (needs `required_consensus` ≥ 2 and a majority of active oracles) | Removal by the DAO; 24h activation of new oracles |
| SSCRE oracles | Propose an epoch's merkle root | Pay more than the epoch's escrowed budget; activate without a quorum (≥ 2 and a majority of active oracles); skip the dispute delay; keep a proposal alive across an oracle-set change | `veto_root` by the DAO or the guardian; per-epoch vaults; clawback |
| Decryption committee | Decrypt poll totals (t-of-n) | Decrypt a single ballot unless t members collude; forge a tally (it is verified on-chain); bias the election key toward one member (every dealer proves knowledge of its contribution) | Feldman-verified DKG keys registered by the DAO; DKG dealings published for anyone to re-check |
| Identity attesters | Grant verification levels | Grant levels after removal (`effective_level` drops to 0); reuse one attestation across wallets | `revoke_attestation`; expiry |
| Engagement oracle | Report engagement (energy refunds only) | Move tokens | 24h-timelocked rotation |
| Relayer (off-chain) | Pay fees and rent; charge VCoin up to the user-signed `max_fee` | Use sessions for transfers, tips, claims, staking or votes | Session scope bits, ≤ 24h validity |

## Invariants the tests enforce

- **Staking:** voting weight is linear in locked VCoin and lock time, never multiplied by
  reputation. Withdrawals are blocked while Realms votes are unrelinquished or a slash is
  pending. Receipts stay 1:1 with principal.
- **SSCRE:** an epoch can pay at most its approved `max_total_claim`, which is at most its
  escrowed budget. Each claimant claims once per epoch.
- **Private polls:** a ballot is valid only for its program, poll, voter and exact weight.
  Finalize is one-shot and needs at least t verified shares. A pass can be consumed once, by its
  named consumer.
- **All programs:** init is upgrade-authority gated. Authority transfer is two-step with 24h.
  Withdrawals are never paused.

See `docs/programs/` for the per-program invariants.
