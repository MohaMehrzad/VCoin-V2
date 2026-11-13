# gasless-protocol

Program id `FcXJAjzJs8eVY2WTRFXynQBpC7WZUqKZppyp9xS6PaB3`. [API](../API.md#gasless-protocol).
Relayer setup: [relayer-kora.md](../offchain/relayer-kora.md).

## Sessions

`SessionToken` `["session", target_program, session_signer, authority]` lets an ephemeral key act
for a wallet:

- in one target program only;
- for the scope bits CONTENT (content-registry), VOUCH and FOLLOW (vilink);
- for at most 24h.

Consuming programs load the token with `seeds::program` and call `verify(authority, signer,
target, action, now)`, which requires exactly one action bit. Sessions never authorize transfers,
tips, claims, staking, withdrawals or votes.

- **`create_session(target_program, scope, valid_until)`.** The authority and the session key
  sign; any payer funds the rent.
- **`revoke_session`.** The authority can revoke at any time; anyone can once the session has
  expired. Rent goes back to the payer.

## Fees

- **`pay_vcoin_fee(max_fee)`.** Charges the configured VCoin fee, up to the user-signed maximum,
  into the fee vault `["fee-vault"]`.
- **`withdraw_fees`.** Permissionless, and only to the configured treasury.
- **Fee and treasury changes.** `schedule_*` then `apply_*` after 24h. Fee changes are bounded.

## Admin

- `initialize`: upgrade authority only.
- `set_paused`: new sessions and fees.
- Two-step authority.
