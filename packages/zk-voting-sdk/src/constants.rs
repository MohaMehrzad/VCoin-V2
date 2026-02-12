/// Domain separator for generating the second generator H
pub const GENERATOR_H_DOMAIN: &[u8] = b"VCoin-Governance-ZK-Vote-GeneratorH-v1";

/// Domain separator for vote validity proofs
pub const DOMAIN_VOTE_PROOF: &[u8] = b"VCoin-ZK-VoteValidityProof-v1";

/// Domain separator for DLEQ proofs
pub const DOMAIN_DLEQ_PROOF: &[u8] = b"VCoin-ZK-DLEQProof-v1";
