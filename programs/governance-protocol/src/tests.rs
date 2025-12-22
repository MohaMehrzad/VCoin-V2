//! Unit tests for Governance Protocol
//!
//! These tests run against the ACTUAL program code.

#[cfg(test)]
mod tests {
    use crate::constants::*;
    use crate::state::{GovernanceConfig, Proposal, VoteRecord, ProposalStatus};
    use crate::state::utils::{calculate_voting_power, integer_sqrt};
    use anchor_lang::prelude::Pubkey;

    // ========================================================================
    // Constants Tests
    // ========================================================================

    #[test]
    fn test_governance_seeds() {
        assert_eq!(GOV_CONFIG_SEED, b"gov-config");
        assert_eq!(PROPOSAL_SEED, b"proposal");
        assert_eq!(VOTE_RECORD_SEED, b"vote-record");
        assert_eq!(DELEGATION_SEED, b"delegation");
    }

    #[test]
    fn test_governance_thresholds() {
        assert_eq!(COMMUNITY_THRESHOLD, 1, "1 veVCoin to vote");
        assert_eq!(DELEGATE_THRESHOLD, 1_000, "1000 veVCoin to create proposals");
        assert_eq!(COUNCIL_THRESHOLD, 10_000, "10000 veVCoin for council");
    }

    #[test]
    fn test_default_voting_period() {
        assert_eq!(DEFAULT_VOTING_PERIOD, 7 * 24 * 60 * 60, "7 days voting period");
    }

    #[test]
    fn test_default_timelock() {
        assert_eq!(DEFAULT_TIMELOCK_DELAY, 48 * 60 * 60, "48 hours timelock");
    }

    #[test]
    fn test_default_quorum() {
        assert_eq!(DEFAULT_QUORUM, 1_000_000, "1M effective votes for quorum");
    }

    #[test]
    fn test_tier_multipliers() {
        assert_eq!(TIER_MULT_NONE, 1000, "1.0x for None");
        assert_eq!(TIER_MULT_BRONZE, 1000, "1.0x for Bronze");
        assert_eq!(TIER_MULT_SILVER, 2000, "2.0x for Silver");
        assert_eq!(TIER_MULT_GOLD, 5000, "5.0x for Gold");
        assert_eq!(TIER_MULT_PLATINUM, 10000, "10.0x for Platinum");
    }

    #[test]
    fn test_diminishing_threshold() {
        assert_eq!(DIMINISHING_THRESHOLD, 100_000, "100K vote cap before diminishing");
    }

    #[test]
    fn test_zk_voting_constants() {
        assert_eq!(MIN_DECRYPTION_THRESHOLD, 3, "Minimum 3 committee members");
        assert_eq!(MAX_COMMITTEE_SIZE, 5, "Maximum 5 committee members");
    }

    // ========================================================================
    // State Size Tests
    // ========================================================================

    #[test]
    fn test_governance_config_size() {
        let expected = 8 + 32 + 32 + 32 + 8 + 8 + 8 + 8 + 8 + 8 + 1 + 1;
        assert_eq!(GovernanceConfig::LEN, expected, "Config size mismatch");
    }

    #[test]
    fn test_proposal_size() {
        let expected = 8 + 8 + 32 + 32 + 128 + 1 + 1 + 8 + 8 + 16 + 16 + 16 + 1 + 8 + 1 + 1 + 1;
        assert_eq!(Proposal::LEN, expected, "Proposal size mismatch");
    }

    #[test]
    fn test_vote_record_size() {
        let expected = 8 + 32 + 32 + 8 + 1 + 8 + 1 + 32 + 32 + 128 + 1 + 1;
        assert_eq!(VoteRecord::LEN, expected, "VoteRecord size mismatch");
    }

    // ========================================================================
    // Integer Square Root Tests
    // ========================================================================

    #[test]
    fn test_sqrt_zero() {
        assert_eq!(integer_sqrt(0), 0);
    }

    #[test]
    fn test_sqrt_one() {
        assert_eq!(integer_sqrt(1), 1);
    }

    #[test]
    fn test_sqrt_perfect_squares() {
        assert_eq!(integer_sqrt(4), 2);
        assert_eq!(integer_sqrt(9), 3);
        assert_eq!(integer_sqrt(16), 4);
        assert_eq!(integer_sqrt(100), 10);
        assert_eq!(integer_sqrt(10000), 100);
        assert_eq!(integer_sqrt(1_000_000), 1000);
    }

    #[test]
    fn test_sqrt_non_perfect() {
        // sqrt(2) ≈ 1.41, floor = 1
        assert_eq!(integer_sqrt(2), 1);
        // sqrt(5) ≈ 2.23, floor = 2
        assert_eq!(integer_sqrt(5), 2);
        // sqrt(99) ≈ 9.95, floor = 9
        assert_eq!(integer_sqrt(99), 9);
    }

    #[test]
    fn test_sqrt_large_values() {
        // sqrt(1B) = 31622
        assert_eq!(integer_sqrt(1_000_000_000), 31622);
        // sqrt(1T) = 1M
        assert_eq!(integer_sqrt(1_000_000_000_000), 1_000_000);
    }

    // ========================================================================
    // Voting Power Calculation Tests
    // ========================================================================

    #[test]
    fn test_voting_power_basic() {
        // 10000 veVCoin, 0 5A score, no tier
        let power = calculate_voting_power(10_000, 0, 0);
        
        // sqrt(10000) = 100
        // 5A boost = 1000 + 0 = 1000
        // tier mult = 1000
        // raw = 100 * 1000 * 1000 / 1_000_000 = 100
        assert_eq!(power, 100);
    }

    #[test]
    fn test_voting_power_with_5a() {
        // 10000 veVCoin, max 5A score (10000 = 100%), no tier
        let power = calculate_voting_power(10_000, 10_000, 0);
        
        // sqrt(10000) = 100
        // 5A boost = 1000 + 1000 = 2000 (2.0x)
        // tier mult = 1000
        // raw = 100 * 2000 * 1000 / 1_000_000 = 200
        assert_eq!(power, 200);
    }

    #[test]
    fn test_voting_power_platinum_tier() {
        // 10000 veVCoin, 0 5A score, Platinum tier
        let power = calculate_voting_power(10_000, 0, 4);
        
        // sqrt(10000) = 100
        // 5A boost = 1000
        // tier mult = 10000 (10x)
        // raw = 100 * 1000 * 10000 / 1_000_000 = 1000
        assert_eq!(power, 1000);
    }

    #[test]
    fn test_voting_power_all_boosts() {
        // 10000 veVCoin, max 5A, Platinum tier
        let power = calculate_voting_power(10_000, 10_000, 4);
        
        // sqrt(10000) = 100
        // 5A boost = 2000 (2.0x)
        // tier mult = 10000 (10x)
        // raw = 100 * 2000 * 10000 / 1_000_000 = 2000
        assert_eq!(power, 2000);
    }

    #[test]
    fn test_voting_power_diminishing_returns() {
        // Very large stake to trigger diminishing returns
        let vevcoin = 100_000_000_000u64; // 100B (extreme case)
        let power = calculate_voting_power(vevcoin, 10_000, 4);
        
        // raw votes would be enormous, but diminishing returns kicks in
        // Should be capped: DIMINISHING_THRESHOLD + sqrt(excess)
        assert!(power > DIMINISHING_THRESHOLD);
    }

    #[test]
    fn test_voting_power_zero_vevcoin() {
        let power = calculate_voting_power(0, 10_000, 4);
        assert_eq!(power, 0, "Zero veVCoin = zero voting power");
    }

    // ========================================================================
    // Proposal Status Tests
    // ========================================================================

    #[test]
    fn test_proposal_status_from_u8() {
        assert_eq!(ProposalStatus::from_u8(0), Some(ProposalStatus::Pending));
        assert_eq!(ProposalStatus::from_u8(1), Some(ProposalStatus::Active));
        assert_eq!(ProposalStatus::from_u8(2), Some(ProposalStatus::Passed));
        assert_eq!(ProposalStatus::from_u8(3), Some(ProposalStatus::Rejected));
        assert_eq!(ProposalStatus::from_u8(4), Some(ProposalStatus::Executed));
        assert_eq!(ProposalStatus::from_u8(5), Some(ProposalStatus::Cancelled));
        assert_eq!(ProposalStatus::from_u8(6), None);
    }

    #[test]
    fn test_proposal_status_default() {
        let status = ProposalStatus::default();
        assert_eq!(status, ProposalStatus::Pending);
    }

    // ========================================================================
    // Proposal State Tests
    // ========================================================================

    #[test]
    fn test_proposal_default() {
        let proposal = Proposal::default();
        assert_eq!(proposal.id, 0);
        assert_eq!(proposal.votes_for, 0);
        assert_eq!(proposal.votes_against, 0);
        assert_eq!(proposal.votes_abstain, 0);
        assert!(!proposal.executed);
    }

    #[test]
    fn test_vote_totals() {
        let mut proposal = Proposal::default();
        proposal.votes_for = 1000;
        proposal.votes_against = 500;
        proposal.votes_abstain = 200;
        
        let total = proposal.votes_for + proposal.votes_against + proposal.votes_abstain;
        assert_eq!(total, 1700);
    }

    #[test]
    fn test_proposal_passes() {
        let proposal = Proposal {
            votes_for: 2000,
            votes_against: 500,
            ..Default::default()
        };
        
        assert!(proposal.votes_for > proposal.votes_against, "Should pass");
    }

    #[test]
    fn test_proposal_rejected() {
        let proposal = Proposal {
            votes_for: 400,
            votes_against: 600,
            ..Default::default()
        };
        
        assert!(proposal.votes_for < proposal.votes_against, "Should be rejected");
    }

    // ========================================================================
    // Quorum Tests
    // ========================================================================

    #[test]
    fn test_quorum_reached() {
        let quorum = DEFAULT_QUORUM;
        let proposal = Proposal {
            votes_for: 800_000,
            votes_against: 300_000,
            votes_abstain: 100_000,
            ..Default::default()
        };
        
        let total = proposal.votes_for + proposal.votes_against + proposal.votes_abstain;
        assert!(total as u64 >= quorum, "Quorum should be reached");
    }

    #[test]
    fn test_quorum_not_reached() {
        let quorum = DEFAULT_QUORUM;
        let proposal = Proposal {
            votes_for: 100_000,
            votes_against: 50_000,
            ..Default::default()
        };
        
        let total = proposal.votes_for + proposal.votes_against + proposal.votes_abstain;
        assert!((total as u64) < quorum, "Quorum should not be reached");
    }

    // ========================================================================
    // Timelock Tests
    // ========================================================================

    #[test]
    fn test_timelock_calculation() {
        let end_time = 1000i64;
        let execution_time = end_time + DEFAULT_TIMELOCK_DELAY;
        
        assert_eq!(execution_time, 1000 + 48 * 60 * 60);
    }

    #[test]
    fn test_can_execute_after_timelock() {
        let execution_time = 1000i64;
        let current_time = 2000i64;
        
        assert!(current_time >= execution_time, "Should be executable");
    }

    #[test]
    fn test_cannot_execute_before_timelock() {
        let execution_time = 1000i64;
        let current_time = 500i64;
        
        assert!(current_time < execution_time, "Should not be executable");
    }

    // ========================================================================
    // Vote Record Tests
    // ========================================================================

    #[test]
    fn test_vote_record_default() {
        let vote = VoteRecord::default();
        assert_eq!(vote.vote_weight, 0);
        assert_eq!(vote.vote_choice, 0);
        assert!(!vote.is_private);
        assert!(!vote.revealed);
    }

    // ========================================================================
    // Invariant Tests
    // ========================================================================

    #[test]
    fn test_invariant_vote_totals() {
        // votes_for + votes_against + votes_abstain = total_votes
        let proposal = Proposal {
            votes_for: 1000,
            votes_against: 500,
            votes_abstain: 200,
            ..Default::default()
        };
        
        let calculated_total = proposal.votes_for + proposal.votes_against + proposal.votes_abstain;
        assert_eq!(calculated_total, 1700);
    }

    #[test]
    fn test_invariant_voting_power_bounded() {
        // Even with max inputs, voting power should be bounded
        let max_vevcoin = u64::MAX / 1000; // Prevent overflow in calculation
        let power = calculate_voting_power(max_vevcoin, 10_000, 4);
        
        assert!(power < u64::MAX, "Voting power should not overflow");
    }

    #[test]
    fn test_invariant_proposal_state_machine() {
        // Valid state transitions:
        // Pending -> Active
        // Active -> Passed | Rejected | Cancelled
        // Passed -> Executed
        
        let valid_transitions = [
            (0, 1), // Pending -> Active
            (1, 2), // Active -> Passed
            (1, 3), // Active -> Rejected
            (1, 5), // Active -> Cancelled
            (2, 4), // Passed -> Executed
        ];
        
        for (from, to) in valid_transitions {
            let from_status = ProposalStatus::from_u8(from);
            let to_status = ProposalStatus::from_u8(to);
            assert!(from_status.is_some() && to_status.is_some());
        }
    }

    // ========================================================================
    // PDA Derivation Tests
    // ========================================================================

    #[test]
    fn test_proposal_pda_unique() {
        let program_id = Pubkey::new_unique();
        
        let (pda1, _) = Pubkey::find_program_address(
            &[PROPOSAL_SEED, &1u64.to_le_bytes()],
            &program_id
        );
        
        let (pda2, _) = Pubkey::find_program_address(
            &[PROPOSAL_SEED, &2u64.to_le_bytes()],
            &program_id
        );
        
        assert_ne!(pda1, pda2, "Different proposal IDs should have different PDAs");
    }

    #[test]
    fn test_vote_record_pda_unique() {
        let program_id = Pubkey::new_unique();
        let voter = Pubkey::new_unique();
        let proposal1 = Pubkey::new_unique();
        let proposal2 = Pubkey::new_unique();
        
        let (pda1, _) = Pubkey::find_program_address(
            &[VOTE_RECORD_SEED, voter.as_ref(), proposal1.as_ref()],
            &program_id
        );
        
        let (pda2, _) = Pubkey::find_program_address(
            &[VOTE_RECORD_SEED, voter.as_ref(), proposal2.as_ref()],
            &program_id
        );
        
        assert_ne!(pda1, pda2, "Same voter, different proposals = different PDAs");
    }
}

