# vevcoin-token

Program id `FB39ae9x53FxVL3pER9LqCPEx2TRnEnQP55i838Upnjx`. [API](../API.md#vevcoin-token).

## What it is

veVCoin is a **receipt**, not a voting token. It mirrors staked principal 1:1 and is never used
to compute weight.

- **Mint.** PDA `["vevcoin-mint"]`, Token-2022 with NonTransferable and PermanentDelegate set to
  the config PDA `["vevcoin-config"]`.
- **Minting and burning.** `mint_vevcoin` / `burn_vevcoin` accept only the staking registrar PDA
  (`["registrar"]` of staking-protocol) as signer. That address is fixed at initialization, with
  no setter (audit VEV-02).
- **Burn clamp.** A burn takes `min(amount, balance)`, so a holder who burned their own receipt
  can still withdraw or be slashed (VEV-01).

## Instructions

| Who | Instruction |
|---|---|
| Upgrade authority, once | `initialize` |
| Staking registrar (CPI) | `mint_vevcoin`, `burn_vevcoin` |
| Authority | `propose_authority`, `cancel_authority_transfer`; then `accept_authority` |
