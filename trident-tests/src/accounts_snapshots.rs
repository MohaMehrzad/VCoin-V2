//! Account snapshots for fuzz testing
//!
//! Captures account state before and after operations for invariant checking.

use anchor_lang::prelude::*;
use std::collections::HashMap;

/// Snapshot of account state
#[derive(Debug, Clone)]
pub struct AccountSnapshot {
    pub pubkey: Pubkey,
    pub lamports: u64,
    pub data: Vec<u8>,
    pub owner: Pubkey,
}

/// Snapshot of the entire test state
#[derive(Debug, Clone, Default)]
pub struct StateSnapshot {
    pub accounts: HashMap<Pubkey, AccountSnapshot>,
    pub slot: u64,
    pub timestamp: i64,
}

impl StateSnapshot {
    pub fn new() -> Self {
        Self::default()
    }
    
    /// Add an account to the snapshot
    pub fn add_account(&mut self, pubkey: Pubkey, lamports: u64, data: Vec<u8>, owner: Pubkey) {
        self.accounts.insert(pubkey, AccountSnapshot {
            pubkey,
            lamports,
            data,
            owner,
        });
    }
    
    /// Get total lamports across all accounts
    pub fn total_lamports(&self) -> u64 {
        self.accounts.values().map(|a| a.lamports).sum()
    }
    
    /// Compare with another snapshot
    pub fn compare(&self, other: &StateSnapshot) -> Vec<String> {
        let mut differences = Vec::new();
        
        // Check for added accounts
        for key in other.accounts.keys() {
            if !self.accounts.contains_key(key) {
                differences.push(format!("Account added: {}", key));
            }
        }
        
        // Check for removed accounts
        for key in self.accounts.keys() {
            if !other.accounts.contains_key(key) {
                differences.push(format!("Account removed: {}", key));
            }
        }
        
        // Check for modified accounts
        for (key, account) in &self.accounts {
            if let Some(other_account) = other.accounts.get(key) {
                if account.lamports != other_account.lamports {
                    differences.push(format!(
                        "Account {} lamports changed: {} -> {}",
                        key, account.lamports, other_account.lamports
                    ));
                }
                if account.data != other_account.data {
                    differences.push(format!("Account {} data changed", key));
                }
            }
        }
        
        differences
    }
}

/// Token balance snapshot
#[derive(Debug, Clone, Default)]
pub struct TokenBalanceSnapshot {
    pub balances: HashMap<Pubkey, u64>,
    pub total_supply: u64,
}

impl TokenBalanceSnapshot {
    pub fn new() -> Self {
        Self::default()
    }
    
    pub fn add_balance(&mut self, account: Pubkey, balance: u64) {
        self.balances.insert(account, balance);
    }
    
    pub fn total(&self) -> u64 {
        self.balances.values().sum()
    }
    
    /// Verify conservation of tokens
    pub fn verify_conservation(&self, other: &TokenBalanceSnapshot) -> bool {
        // In a closed system, total should be conserved
        // (unless minting/burning occurred)
        self.total() == other.total()
    }
}

/// Staking state snapshot
#[derive(Debug, Clone, Default)]
pub struct StakingSnapshot {
    pub total_staked: u64,
    pub total_stakers: u64,
    pub vault_balance: u64,
    pub user_stakes: HashMap<Pubkey, u64>,
}

impl StakingSnapshot {
    /// Verify staking invariants
    pub fn verify_invariants(&self) -> Result<(), String> {
        // Vault balance should >= total staked
        if self.vault_balance < self.total_staked {
            return Err(format!(
                "Vault {} < total staked {}",
                self.vault_balance, self.total_staked
            ));
        }
        
        // Sum of user stakes should == total staked
        let sum: u64 = self.user_stakes.values().sum();
        if sum != self.total_staked {
            return Err(format!(
                "Sum of stakes {} != total staked {}",
                sum, self.total_staked
            ));
        }
        
        // Staker count should match
        let actual_stakers = self.user_stakes.values().filter(|&&s| s > 0).count() as u64;
        if actual_stakers != self.total_stakers {
            return Err(format!(
                "Actual stakers {} != total stakers {}",
                actual_stakers, self.total_stakers
            ));
        }
        
        Ok(())
    }
}

/// Governance state snapshot
#[derive(Debug, Clone, Default)]
pub struct GovernanceSnapshot {
    pub proposal_count: u64,
    pub total_delegated: u64,
    pub active_proposals: Vec<u64>,
}

impl GovernanceSnapshot {
    /// Verify governance invariants
    pub fn verify_invariants(&self) -> Result<(), String> {
        // Active proposals should be subset of all proposals
        for &id in &self.active_proposals {
            if id > self.proposal_count {
                return Err(format!(
                    "Active proposal {} > total count {}",
                    id, self.proposal_count
                ));
            }
        }
        
        Ok(())
    }
}

