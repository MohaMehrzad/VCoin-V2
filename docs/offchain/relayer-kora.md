# Fee sponsorship: off-chain relayer (Kora)

The v2 `gasless-protocol` claimed to be a paymaster but moved no SOL: its "subsidy budget" was
an on-chain counter that sponsored nothing and bricked itself after 3 idle days (audit
GAS-01/02/10). In v3:

- **SOL fees and rent** are paid by an off-chain relayer that co-signs as fee payer. We recommend
  [Kora](https://github.com/solana-foundation/kora) (Solana Foundation; audited), configured with:
  - a Jupiter/Pyth price source, never a fixed constant;
  - a margin;
  - a per-transaction lamport cap and per-user rate limits;
  - an instruction allowlist covering only VCoin programs.
- **Session keys** (`gasless_protocol::create_session`) let a short-lived key (≤24h, one target
  program, scope bits for content / follow / vouch only) act for a wallet. The relayer may pay
  the session's rent, so users need no SOL. Anyone can close an expired session; its rent
  returns to whoever paid it.
- **VCoin fees** (`pay_vcoin_fee(max_fee)`) let a relayer charge in VCoin. The user always signs
  the maximum, fee changes are bounded and take effect after 24h, and fees are withdrawable only
  to the configured treasury.

Sessions never authorize transfers, tips, claims, staking, withdrawals or votes; those always need
the main wallet.
