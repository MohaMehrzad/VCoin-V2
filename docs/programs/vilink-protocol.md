# vilink-protocol

Program id `CFGXTS2MueQwTYTMMTBQbRWzJtSTC2p4ZRuKPpLDmrv7`. [API](../API.md#vilink-protocol).

## Links

**`create_action(type, amount, target, metadata_hash, expiry_secs, max_executions, nonce)`.**

- Creates `ViLinkAction` `["action", creator, nonce]`, where `nonce` is the creator's
  `actions_created`.
- Types:
  - **tip:** 0.1–10,000 VCoin;
  - **vouch:** no amount;
  - **follow:** no amount.
- Expiry is 1h–7 days; `max_executions` is 1..=10,000.
- The fee in force is stored on the link (audit VL-16).
- `action_id` includes the nonce (VL-11).

## Execution

Each executor executes a link at most once: `ExecutionReceipt` `["exec", action, executor]`
(VL-3). The creator and the target are excluded (VL-9), and a disabled type stops executing at
once (VL-2).

- **`execute_tip`.** The executor's wallet signs. The target receives the amount minus the
  link's fee (rounded up, capped at the amount); the fee goes to the treasury. There is no
  delegate check, so CPI Guard and approved delegates work (VL-10).
- **`execute_vouch` / `execute_follow`.** Update stats only. The executor or a gasless session
  key for the executor (scope VOUCH / FOLLOW) signs, and a separate payer funds the receipt.
- **`close_action`.** The creator, once the link has expired or been fully executed.
- **`close_receipt`.** Anyone, once the action is closed or no longer executable.

## Admin

- `initialize(fee_bps)`: upgrade authority only; VCoin mint pinned; fee 0.1–10%.
- Fee and treasury: `propose_config` / `apply_config` / `cancel_config` (24h).
- `set_enabled_actions`: immediate kill switch.
- `set_paused`.
- Two-step authority.

Removed: action types 3–7 (never executable), batches, and the unenforced dApp registry
(VL-6/7/8).
