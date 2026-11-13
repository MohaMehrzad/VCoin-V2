# Off-chain activity indexer (replaces the transfer-hook program)

The `transfer-hook` program was retired in v3 (audit TH-01…TH-11). This note records why
and what replaces it.

## Why the hook was removed

- **It never worked.** Token-2022 could not invoke it: wrong instruction discriminator,
  wrong account order, an ExtraAccountMetaList PDA that could not be created, a payer that
  could never sign. Anyone could also call it directly to forge activity data.
- **It cost every holder.** A hook adds accounts and compute to every VCoin transfer, and
  many DEX/aggregator routes don't support hooked mints.
- **A hook pause is a global freeze.** One key could have halted every transfer,
  unstake, claim and fee payment.
- **The "auto-update 5A" claim was false.** The hook never called the 5A program.

The new VCoin mint is created with **metadata only**: no transfer hook, no permanent
delegate, no freeze authority. `vcoin-common::validate_vcoin_mint` enforces this
wherever a program accepts the VCoin mint.

## Replacement: indexer → 5A oracle

Activity scoring belongs off-chain, where it is cheap, can look at full history, and
can't block anyone's transfer:

1. **Ingest.** An indexer subscribes to VCoin `transfer_checked` instructions via
   Geyser/websocket or an RPC provider's webhooks and stores
   `(slot, signature, source_owner, destination_owner, amount)`.
2. **Score.** Per wallet, per day, it computes:
   - counterparty diversity;
   - transfer velocity;
   - round-trip / circular-flow detection (A→B→A and A→B→C→A within a window),
     which the old hook never implemented;
   - minimum-amount filtering.
3. **Publish.** Each 5A oracle independently runs the scorer and submits the
   *Activity* dimension through `five_a_protocol::submit_score`. The 5A program applies
   a score only after `required_consensus` independent oracles agree (≥2; audit 5A-01).
4. **Transparency.** The scoring code and parameters are published, and each oracle
   posts its inputs (a daily Merkle root of the indexed transfers), so anyone can
   recompute a score.

Wash-trading signals affect reputation only. They never block transfers.
