/**
 * @viwoapp/sdk - TypeScript SDK for VCoin Protocol Integration
 * 
 * This SDK provides easy-to-use interfaces for interacting with
 * all ViWoApp smart contracts on Solana.
 * 
 * Modules:
 * - @viwoapp/core - Connection, wallet adapters
 * - @viwoapp/staking - Staking operations
 * - @viwoapp/governance - Proposal and voting
 * - @viwoapp/rewards - SSCRE claim helpers
 * - @viwoapp/vilink - Action link generation
 * - @viwoapp/gasless - Session key management
 */

// Core exports
export * from './core';
export * from './types';
export * from './constants';

// Module exports
export * from './staking';
export * from './governance';
export * from './rewards';
export * from './vilink';
export * from './gasless';
export * from './identity';
export * from './fivea';
export * from './content';

// Re-export main client
export { ViWoClient } from './client';

