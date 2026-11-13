# staking-protocol

Program id `6EFcistyr2E81adLUcuBJRr8W2xzpt3D3dFYEcMewpWu`. [API](../API.md#staking-protocol).

## Accounts

- **`Registrar`** `["registrar"]`:
  - the admin;
  - the Realms wiring (program, realm, community mint);
  - mints and vault;
  - `slash_destination`;
  - `WeightParams`;
  - 5A gates;
  - `total_locked` (all principal in the vault) and `active_locked` (principal in deposits
    not yet deactivated after their lock ended);
  - the slash nonce.
- **Vault** `["vault"]`: VCoin held for all deposits.
- **`Voter`** `["voter", owner]`: up to 8 deposits `{amount, lock_start, lock_end, is_active}`
  and `pending_slashes`.
- **Realms records:**
  - `VoterWeightRecord` `["voter-weight-record", owner]`;
  - `MaxVoterWeightRecord` `["max-voter-weight-record"]`.

  Their layouts and discriminators match `spl-governance-addin-api`.
- **`SlashRequest`** `["slash", voter, nonce]`.

## Weight

At time `t`, each deposit is worth:

```
amount·baseline + amount·max_extra·min(lock_end − t, saturation)/saturation   (factors / 1e9)
```

It is worth 0 if `lock_end − t < min_lock_remaining_to_vote`.

| Parameter | Default |
|---|---|
| `baseline_factor` | 1.0× |
| `max_extra_factor` | +1.0× |
| `saturation` | 4 years |
| `min_lock_remaining_to_vote` | 4 days |

Weight is linear, so splitting tokens across wallets gains nothing (audit GOV-02).

The max record is `active_locked × (baseline + max_extra)`, an upper bound on the weight that can
be cast. Set Realms percentage thresholds with that in mind.

A deposit whose lock has ended has no voting weight. It leaves `active_locked` when its owner
withdraws from it, or when anyone calls `deactivate_expired(index)` on it, and it comes back if
the owner relocks it with `extend_lock`. Without this, stake that is expired but never
withdrawn (lost keys, inactive holders) would inflate the max forever and could make percentage
thresholds unreachable. Run the crank for expired deposits before an important vote; the SDK
has `deactivateExpiredIx`.

## Flows

- **Deposit.**
  - `deposit(index, amount, lock_secs)`: lock for 1 week to 4 years.
  - It credits exactly what the vault received and mints the receipt into the owner's veVCoin
    associated token account.
  - Adding to a live slot keeps the later lock end.
- **`extend_lock`.** Moves a lock end later only. Relocking an expired deposit reactivates it.
- **`deactivate_expired`.** Permissionless; see [Weight](#weight).
- **`withdraw`.**
  - Only after `lock_end`.
  - Never paused.
  - Refused while the owner's Realms TokenOwnerRecord has unrelinquished votes or open
    proposals (GOV-03), or while a slash is pending.
  - Burns `min(amount, balance)` of the receipt.
- **Realms plugin.**
  - `create_voter_weight_record`.
  - `update_voter_weight_record(action, target)`: valid for the current slot only. The 5A gate
    applies here.
  - `update_max_voter_weight_record`.
- **Slashing.**
  - The authority (the DAO) calls `propose_slash(amount, reason_hash)`; withdrawals freeze.
  - After 48h anyone calls `execute_slash`, which moves `min(amount, deposits)` from the vault
    to `slash_destination` and burns receipts.
  - The authority can `cancel_slash` at any time.
  - After a further 7 days anyone can `expire_slash`.

## Admin (Realms governance PDA after launch)

- `configure_governance`: one-shot; validates the realm.
- `set_weight_params`: bounded.
- `set_gate_params`: scores of at most 10000.
- `set_slash_destination`.
- `set_paused`: deposits and extensions only.
- `set_guardian`: an emergency key that may pause but never unpause.
- Two-step authority.

Initialization (`create_registrar`) is restricted to the upgrade authority and checks that the
VCoin mint is hook/fee/freeze-free.
