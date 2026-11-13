# vcoin-token

Program id `Gg1dtrjAfGYi6NLC31WaJjZNBoucvD98rK2h1u9qrUjn`. Instruction details: [API](../API.md#vcoin-token).

## What it is

The VCoin mint and its supply control.

- **Mint.** PDA `["vcoin-mint"]`, Token-2022, 9 decimals.
  - Its only extensions are MetadataPointer (pointing at itself) and TokenMetadata.
  - No transfer hook, permanent delegate, transfer fee or freeze authority. Every other program
    accepts it only after `validate_vcoin_mint` confirms this.
  - The mint authority is the config PDA `["vcoin-config-v2"]`.
- **Supply cap.** `mint_tokens` checks the mint's real `supply + amount ≤ 1,000,000,000 VCoin`.

## Instructions

| Who | Instruction |
|---|---|
| Upgrade authority, once | `initialize(name, symbol, uri)`: creates the config, the mint and its metadata |
| Authority | `mint_tokens(amount)`, `update_metadata(field, value)`, `set_paused`, `propose_authority` / `cancel_authority_transfer` |
| New authority, after 24h | `accept_authority` |

Pausing stops minting only. Holders' transfers are plain Token-2022 transfers that no program
can block.

## Removed in v3

- Admin slashing of any holder's balance, the permanent delegate and its updater (audit TOK-01..03).
- The front-runnable `initialize_mint` (TOK-04).
