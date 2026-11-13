# Test fixtures

## spl_governance.so

The SPL Governance (Realms) program as deployed on Solana **mainnet-beta**, used by the
LiteSVM tests that exercise `staking-protocol` as a Realms voter-weight plugin.

- Program id: `GovER5Lthms3bLBqWub97yVrMmEogzX7xNjdXpPPCVZw`
- Obtained with: `solana program dump -u m GovER5Lthms3bLBqWub97yVrMmEogzX7xNjdXpPPCVZw spl_governance.so`
- Dumped: 2026-11-13; mainnet ProgramData last-deploy slot 468,575,556
- SHA-256: `d9b561223a3e5db739eb430348aa9a61be1418bf49bddc642aeefb85131f880d`
- License: Apache-2.0 (Solana Program Library / Mythic-Project `governance-program-library`)

The devnet build (slot 399,231,146) behaves identically for every flow tested. Refresh the
dump and update the hash when Realms is upgraded on mainnet.
