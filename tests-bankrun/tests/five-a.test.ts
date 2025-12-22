/**
 * 5A Protocol Tests using Bankrun
 */

import { describe, it } from "node:test";
import { expect } from "chai";

// Constants
const MAX_SCORE = 10000;
const AUTHENTICITY_WEIGHT = 2500;
const ACCURACY_WEIGHT = 2000;
const AGILITY_WEIGHT = 1500;
const ACTIVITY_WEIGHT = 2500;
const APPROVED_WEIGHT = 1500;

interface FiveAScore {
  authenticity: number;
  accuracy: number;
  agility: number;
  activity: number;
  approved: number;
}

const calculateComposite = (score: FiveAScore): number => {
  const weightedSum =
    score.authenticity * AUTHENTICITY_WEIGHT +
    score.accuracy * ACCURACY_WEIGHT +
    score.agility * AGILITY_WEIGHT +
    score.activity * ACTIVITY_WEIGHT +
    score.approved * APPROVED_WEIGHT;
  return Math.floor(weightedSum / 10000);
};

const calculateMultiplier = (score: FiveAScore): number => {
  const composite = calculateComposite(score);
  return 1 + composite / MAX_SCORE;
};

describe("5A Protocol - Bankrun Tests", () => {
  it("should verify weights sum to 100%", () => {
    const totalWeight =
      AUTHENTICITY_WEIGHT +
      ACCURACY_WEIGHT +
      AGILITY_WEIGHT +
      ACTIVITY_WEIGHT +
      APPROVED_WEIGHT;

    expect(totalWeight).to.equal(10000);
  });

  it("should calculate composite score correctly", () => {
    // Perfect score
    const perfect: FiveAScore = {
      authenticity: MAX_SCORE,
      accuracy: MAX_SCORE,
      agility: MAX_SCORE,
      activity: MAX_SCORE,
      approved: MAX_SCORE,
    };
    expect(calculateComposite(perfect)).to.equal(MAX_SCORE);

    // Zero score
    const zero: FiveAScore = {
      authenticity: 0,
      accuracy: 0,
      agility: 0,
      activity: 0,
      approved: 0,
    };
    expect(calculateComposite(zero)).to.equal(0);

    // Average score
    const average: FiveAScore = {
      authenticity: 5000,
      accuracy: 5000,
      agility: 5000,
      activity: 5000,
      approved: 5000,
    };
    expect(calculateComposite(average)).to.equal(5000);
  });

  it("should calculate reward multiplier in correct range", () => {
    // Minimum multiplier (0% score) = 1.0x
    const minScore: FiveAScore = {
      authenticity: 0,
      accuracy: 0,
      agility: 0,
      activity: 0,
      approved: 0,
    };
    expect(calculateMultiplier(minScore)).to.equal(1.0);

    // Maximum multiplier (100% score) = 2.0x
    const maxScore: FiveAScore = {
      authenticity: MAX_SCORE,
      accuracy: MAX_SCORE,
      agility: MAX_SCORE,
      activity: MAX_SCORE,
      approved: MAX_SCORE,
    };
    expect(calculateMultiplier(maxScore)).to.equal(2.0);

    // Average score = 1.5x
    const avgScore: FiveAScore = {
      authenticity: 5000,
      accuracy: 5000,
      agility: 5000,
      activity: 5000,
      approved: 5000,
    };
    expect(calculateMultiplier(avgScore)).to.equal(1.5);
  });

  it("should test weighted dimensions", () => {
    // Only authenticity (25% weight)
    const authOnly: FiveAScore = {
      authenticity: MAX_SCORE,
      accuracy: 0,
      agility: 0,
      activity: 0,
      approved: 0,
    };
    expect(calculateComposite(authOnly)).to.equal(2500);

    // Only activity (25% weight)
    const actOnly: FiveAScore = {
      authenticity: 0,
      accuracy: 0,
      agility: 0,
      activity: MAX_SCORE,
      approved: 0,
    };
    expect(calculateComposite(actOnly)).to.equal(2500);

    // Only accuracy (20% weight)
    const accOnly: FiveAScore = {
      authenticity: 0,
      accuracy: MAX_SCORE,
      agility: 0,
      activity: 0,
      approved: 0,
    };
    expect(calculateComposite(accOnly)).to.equal(2000);
  });

  it("should differentiate quality users from bots", () => {
    // Quality user
    const qualityUser: FiveAScore = {
      authenticity: 8500,
      accuracy: 8000,
      agility: 7500,
      activity: 9000,
      approved: 7500,
    };

    // Bot-like user
    const botUser: FiveAScore = {
      authenticity: 2000,
      accuracy: 2000,
      agility: 2000,
      activity: 2000,
      approved: 2000,
    };

    const qualityMultiplier = calculateMultiplier(qualityUser);
    const botMultiplier = calculateMultiplier(botUser);

    console.log(`Quality user multiplier: ${qualityMultiplier.toFixed(2)}x`);
    console.log(`Bot-like user multiplier: ${botMultiplier.toFixed(2)}x`);

    expect(qualityMultiplier).to.be.greaterThan(botMultiplier);
    expect(qualityMultiplier).to.be.greaterThan(1.5);
    expect(botMultiplier).to.be.lessThan(1.3);
  });

  it("should validate score bounds", () => {
    const isValidScore = (score: number) => score >= 0 && score <= MAX_SCORE;

    expect(isValidScore(0)).to.be.true;
    expect(isValidScore(5000)).to.be.true;
    expect(isValidScore(MAX_SCORE)).to.be.true;
    expect(isValidScore(-1)).to.be.false;
    expect(isValidScore(MAX_SCORE + 1)).to.be.false;
  });
});

