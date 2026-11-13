//! SDK prover -> zk-poll-core verifier (the code the program runs), end to end, plus the
//! attacks from audit ZK-01/02/03/06/10. Audit T-ZK-1.

use curve25519_dalek::{ristretto::RistrettoPoint, scalar::Scalar};
use rand::{rngs::OsRng, Rng};
use zk_voting_sdk::*;

const PROGRAM: [u8; 32] = [1; 32];
const POLL: [u8; 32] = [2; 32];
const THRESHOLD: u8 = 3;
const SIZE: u8 = 5;
const G: RistrettoPoint = curve25519_dalek::constants::RISTRETTO_BASEPOINT_POINT;

fn decompress(b: &[u8; 32]) -> RistrettoPoint {
    curve25519_dalek::ristretto::CompressedRistretto(*b)
        .decompress()
        .unwrap()
}

/// Identifies the DKG run in these tests.
const SESSION: [u8; 32] = [9u8; 32];

/// Full Pedersen DKG among `SIZE` members.
fn committee() -> Vec<KeyShare> {
    let dealers: Vec<Dealer> = (1..=SIZE)
        .map(|i| Dealer::new(i, THRESHOLD, SIZE, &SESSION, &mut OsRng).unwrap())
        .collect();
    let dealings: Vec<PublicDealing> = dealers.iter().map(Dealer::public_dealing).collect();
    (1..=SIZE)
        .map(|i| {
            let shares: Vec<ShareMessage> =
                dealers.iter().map(|d| d.share_for(i).unwrap()).collect();
            finish(i, THRESHOLD, &SESSION, &dealings, &shares).unwrap()
        })
        .collect()
}

fn ctx(election_pk: [u8; 32], voter: u8, weight: u64) -> BallotContext {
    BallotContext {
        program_id: PROGRAM,
        poll: POLL,
        voter: [voter; 32],
        election_pk,
        weight,
    }
}

fn aggregate(ballots: &[EncryptedBallot]) -> ([[u8; 32]; 3], [[u8; 32]; 3]) {
    let mut r = [RistrettoPoint::default(); 3];
    let mut c = [RistrettoPoint::default(); 3];
    for b in ballots {
        for k in 0..3 {
            r[k] += decompress(&b.ballot[k].r);
            c[k] += decompress(&b.ballot[k].c);
        }
    }
    (
        r.map(|p| p.compress().to_bytes()),
        c.map(|p| p.compress().to_bytes()),
    )
}

#[test]
fn dkg_gives_every_member_the_same_verified_group_key() {
    let keys = committee();
    let group = &keys[0].group;
    assert!(keys.iter().all(|k| k.group == *group));
    group.verify().unwrap();
    assert_eq!(group.commitments.len(), THRESHOLD as usize);
    for k in &keys {
        assert_eq!(group.member_pk(k.index).unwrap(), k.public);
    }
    // Any t member keys interpolate to the election key (no secret needed to check it).
    let idx = [1u8, 3, 5];
    let lambdas = lagrange_at_zero(&idx).unwrap();
    let pk: RistrettoPoint = idx
        .iter()
        .zip(&lambdas)
        .map(|(i, l)| l * decompress(&keys[*i as usize - 1].public))
        .sum();
    assert_eq!(pk.compress().to_bytes(), group.election_pk());
}

#[test]
fn two_hundred_honest_ballots_tally_correctly_with_any_quorum() {
    let keys = committee();
    let pk = keys[0].group.election_pk();
    let mut expected = [0u64; 3];
    let mut total = 0u64;
    let ballots: Vec<EncryptedBallot> = (0..200u32)
        .map(|i| {
            let weight = OsRng.gen_range(1..=1_000u64);
            let choice = Choice::try_from((i % 3) as u8).unwrap();
            expected[choice as usize] += weight;
            total += weight;
            let c = ctx(pk, (i % 251) as u8, weight);
            let b = encrypt_ballot(&c, choice, &mut OsRng).unwrap();
            verify_ballot(&c, &b.ballot, &b.proof).unwrap();
            b
        })
        .collect();
    let (agg_r, agg_c) = aggregate(&ballots);

    for quorum in [[2u8, 4, 5], [1, 2, 3]] {
        let partials: Vec<PartialDecryption> = quorum
            .iter()
            .map(|&i| {
                let key = &keys[i as usize - 1];
                let p = partial_decrypt(key, PROGRAM, POLL, &agg_r, &mut OsRng).unwrap();
                let dctx = DleqContext {
                    program_id: PROGRAM,
                    poll: POLL,
                    member_index: i,
                    member_pk: key.public,
                };
                verify_dleq(&dctx, &agg_r, &p.shares, &p.proof).unwrap();
                p
            })
            .collect();
        let result = combine(THRESHOLD, &agg_c, &partials, total).unwrap();
        assert_eq!(result.tally, expected, "quorum {quorum:?}");
        let pairs: Vec<_> = result
            .lagrange
            .iter()
            .zip(&partials)
            .map(|(l, p)| (*l, p.shares))
            .collect();
        verify_tally(&agg_c, &pairs, &expected).unwrap();
        let mut wrong = expected;
        wrong[0] += 1;
        assert_eq!(
            verify_tally(&agg_c, &pairs, &wrong),
            Err(ZkError::TallyMismatch)
        );
    }
}

fn honest(weight: u64, choice: Choice) -> (BallotContext, EncryptedBallot) {
    let keys = committee();
    let c = ctx(keys[0].group.election_pk(), 9, weight);
    let b = encrypt_ballot(&c, choice, &mut OsRng).unwrap();
    (c, b)
}

fn rejected(c: &BallotContext, ballot: &Ballot, proof: &[u8; BALLOT_PROOF_LEN]) -> ZkError {
    verify_ballot(c, ballot, proof).unwrap_err()
}

#[test]
fn ballots_bind_weight_voter_poll_and_program() {
    let (c, b) = honest(40, Choice::Against);
    for other in [
        BallotContext { weight: 41, ..c },
        BallotContext { weight: 20, ..c },
        BallotContext {
            voter: [10; 32],
            ..c
        },
        BallotContext { poll: [3; 32], ..c },
        BallotContext {
            program_id: [4; 32],
            ..c
        },
    ] {
        assert_eq!(
            rejected(&other, &b.ballot, &b.proof),
            ZkError::InvalidBallotProof,
            "{other:?}"
        );
    }
}

#[test]
fn tampered_ballots_are_rejected() {
    let (c, b) = honest(40, Choice::For);
    let wh = Scalar::from(40u64) * decompress(&zk_poll_core::GENERATOR_H);
    let shift = |p: &[u8; 32], by: RistrettoPoint| (decompress(p) + by).compress().to_bytes();

    // Encrypting 2w for the chosen option.
    let mut two_w = b.ballot;
    two_w[0].c = shift(&two_w[0].c, wh);
    assert_eq!(rejected(&c, &two_w, &b.proof), ZkError::InvalidBallotProof);

    // For = w + K, Against = −K (sum still w): each OR proof fails.
    let k = Scalar::from(1_000u64) * decompress(&zk_poll_core::GENERATOR_H);
    let mut skew = b.ballot;
    skew[0].c = shift(&skew[0].c, k);
    skew[1].c = shift(&skew[1].c, -k);
    assert_eq!(rejected(&c, &skew, &b.proof), ZkError::InvalidBallotProof);

    // Swapping two choices together with their proofs: the choice index is in the transcript.
    let mut swapped = b.ballot;
    swapped.swap(0, 1);
    let mut proof = b.proof;
    let (first, second) = proof.split_at_mut(OR_LEN);
    first.swap_with_slice(&mut second[..OR_LEN]);
    assert_eq!(rejected(&c, &swapped, &proof), ZkError::InvalidBallotProof);

    // Non-canonical response (s0 + ℓ encodes the same residue).
    let mut proof = b.proof;
    let s0 = Scalar::from_canonical_bytes(proof[64..96].try_into().unwrap()).unwrap();
    let plus_l = add_l(&s0.to_bytes());
    proof[64..96].copy_from_slice(&plus_l);
    assert_eq!(rejected(&c, &b.ballot, &proof), ZkError::NonCanonicalScalar);

    // Every single flipped proof byte is caught.
    for i in (0..BALLOT_PROOF_LEN).step_by(37) {
        let mut proof = b.proof;
        proof[i] ^= 0x01;
        assert!(verify_ballot(&c, &b.ballot, &proof).is_err(), "byte {i}");
    }
}

const OR_LEN: usize = 128;

/// `s + ℓ` as 32 bytes (fits: s < ℓ < 2^253).
fn add_l(s: &[u8; 32]) -> [u8; 32] {
    const L: [u8; 32] = [
        0xed, 0xd3, 0xf5, 0x5c, 0x1a, 0x63, 0x12, 0x58, 0xd6, 0x9c, 0xf7, 0xa2, 0xde, 0xf9, 0xde,
        0x14, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0x10,
    ];
    let mut out = [0u8; 32];
    let mut carry = 0u16;
    for i in 0..32 {
        let v = s[i] as u16 + L[i] as u16 + carry;
        out[i] = v as u8;
        carry = v >> 8;
    }
    out
}

#[test]
fn identity_and_zero_weight_are_refused() {
    let c = ctx(zk_poll_core::IDENTITY, 1, 5);
    assert_eq!(
        encrypt_ballot(&c, Choice::For, &mut OsRng),
        Err(SdkError::Zk(ZkError::InvalidPoint))
    );
    let keys = committee();
    let c = ctx(keys[0].group.election_pk(), 1, 0);
    assert_eq!(
        encrypt_ballot(&c, Choice::For, &mut OsRng),
        Err(SdkError::ZeroWeight)
    );
}

#[test]
fn partial_decryptions_are_bound_to_member_poll_and_program() {
    let keys = committee();
    let pk = keys[0].group.election_pk();
    let b = encrypt_ballot(&ctx(pk, 1, 7), Choice::Abstain, &mut OsRng).unwrap();
    let (agg_r, _) = aggregate(std::slice::from_ref(&b));
    let key = &keys[1];
    let p = partial_decrypt(key, PROGRAM, POLL, &agg_r, &mut OsRng).unwrap();
    let good = DleqContext {
        program_id: PROGRAM,
        poll: POLL,
        member_index: 2,
        member_pk: key.public,
    };
    verify_dleq(&good, &agg_r, &p.shares, &p.proof).unwrap();

    for bad in [
        DleqContext {
            member_index: 3,
            ..good
        },
        DleqContext {
            member_pk: keys[2].public,
            ..good
        },
        DleqContext {
            poll: [9; 32],
            ..good
        },
        DleqContext {
            program_id: [9; 32],
            ..good
        },
    ] {
        assert!(
            verify_dleq(&bad, &agg_r, &p.shares, &p.proof).is_err(),
            "{bad:?}"
        );
    }
    let mut forged = p.shares;
    forged[0] = keys[0].public; // any other point
    assert_eq!(
        verify_dleq(&good, &agg_r, &forged, &p.proof),
        Err(ZkError::InvalidDleqProof)
    );
}

#[test]
fn committee_registration_checks() {
    let keys = committee();
    let g = keys[0].group.clone();

    let mut wrong_member = g.clone();
    wrong_member.member_pks[1] = wrong_member.member_pks[2];
    assert_eq!(
        wrong_member.verify(),
        Err(SdkError::Zk(ZkError::InvalidCommittee)),
        "duplicate member key"
    );

    let mut wrong_key = g.clone();
    wrong_key.member_pks[4] = zk_poll_core::GENERATOR_H;
    assert_eq!(
        wrong_key.verify(),
        Err(SdkError::Zk(ZkError::FeldmanMismatch))
    );

    let mut identity = g.clone();
    identity.commitments[0] = zk_poll_core::IDENTITY;
    assert!(identity.verify().is_err());

    let mut small = g.clone();
    small.threshold = 2;
    small.commitments.truncate(2);
    assert_eq!(
        small.verify(),
        Err(SdkError::Zk(ZkError::InvalidCommittee)),
        "t < 3"
    );

    assert!(Dealer::new(1, 6, 5, &SESSION, &mut OsRng).is_err());
    assert!(Dealer::new(6, 3, 5, &SESSION, &mut OsRng).is_err());
}

#[test]
fn a_bad_dealer_is_named() {
    let dealers: Vec<Dealer> = (1..=3)
        .map(|i| Dealer::new(i, 3, 3, &SESSION, &mut OsRng).unwrap())
        .collect();
    let dealings: Vec<PublicDealing> = dealers.iter().map(Dealer::public_dealing).collect();
    let mut shares: Vec<ShareMessage> = dealers.iter().map(|d| d.share_for(1).unwrap()).collect();
    let bogus = Scalar::random(&mut OsRng).to_bytes();
    shares[1] = ShareMessage::from_bytes(2, 1, &bogus).unwrap();
    assert_eq!(
        finish(1, 3, &SESSION, &dealings, &shares).unwrap_err(),
        SdkError::BadDealing { dealer: 2 }
    );

    // Missing or duplicated shares.
    assert!(finish(1, 3, &SESSION, &dealings, &shares[..2]).is_err());
    let mut dup = dealings.clone();
    dup[2].dealer = 1;
    assert_eq!(
        aggregate_dealings(3, &SESSION, &dup).unwrap_err(),
        SdkError::DuplicateIndex(1)
    );
}

#[test]
fn combine_rejects_duplicates_too_few_shares_and_small_bounds() {
    assert_eq!(
        lagrange_at_zero(&[1, 2, 2]).unwrap_err(),
        SdkError::DuplicateIndex(2)
    );
    assert_eq!(
        lagrange_at_zero(&[0, 1]).unwrap_err(),
        SdkError::InvalidIndex(0)
    );

    let keys = committee();
    let pk = keys[0].group.election_pk();
    let ballots: Vec<_> = (0..3)
        .map(|i| encrypt_ballot(&ctx(pk, i, 50), Choice::For, &mut OsRng).unwrap())
        .collect();
    let (agg_r, agg_c) = aggregate(&ballots);
    let partials: Vec<_> = keys
        .iter()
        .take(3)
        .map(|k| partial_decrypt(k, PROGRAM, POLL, &agg_r, &mut OsRng).unwrap())
        .collect();

    assert_eq!(
        combine(THRESHOLD, &agg_c, &partials[..2], 150).unwrap_err(),
        SdkError::NotEnoughShares { needed: 3, got: 2 }
    );
    let dup = vec![
        partials[0].clone(),
        partials[1].clone(),
        partials[0].clone(),
    ];
    assert_eq!(
        combine(THRESHOLD, &agg_c, &dup, 150).unwrap_err(),
        SdkError::DuplicateIndex(1)
    );
    assert_eq!(
        combine(THRESHOLD, &agg_c, &partials, 149).unwrap_err(),
        SdkError::TallyOutOfRange(149)
    );
    assert_eq!(
        combine(THRESHOLD, &agg_c, &partials, 150).unwrap().tally,
        [150, 0, 0]
    );
}

#[test]
fn secrets_never_appear_in_debug_output() {
    let dealer = Dealer::new(1, 3, 3, &SESSION, &mut OsRng).unwrap();
    let share = dealer.share_for(2).unwrap();
    let secret_hex: String = share
        .to_bytes()
        .iter()
        .map(|b| format!("{b:02x}"))
        .collect();
    let secret_dec = format!("{:?}", *share.to_bytes());
    for out in [format!("{share:?}"), format!("{dealer:?}")] {
        assert!(
            !out.contains(&secret_hex) && !out.contains(&secret_dec[1..20]),
            "{out}"
        );
    }
    let keys = committee();
    assert!(format!("{:?}", keys[0]).starts_with("KeyShare { index: 1"));
}

#[test]
fn poll_weight_counts_whole_vcoin() {
    assert_eq!(poll_weight(2_999_999_999), 2);
    assert_eq!(poll_weight(WEIGHT_UNIT - 1), 0);
}

/// Re-review C1 (High): the rogue-key attack. Member 3 publishes last and sets its constant
/// term to `x·G − A_1 − A_2`, so the election key becomes `x·G` for an `x` it knows. It can
/// make its dealing pass every honest member's Feldman check, but it cannot prove knowledge
/// of that constant term, so the dealing is rejected.
#[test]
fn a_rogue_key_dealing_is_rejected() {
    let honest: Vec<Dealer> = (1..=2)
        .map(|i| Dealer::new(i, 3, 3, &SESSION, &mut OsRng).unwrap())
        .collect();
    let mut dealings: Vec<PublicDealing> = honest.iter().map(Dealer::public_dealing).collect();

    let x = Scalar::random(&mut OsRng);
    let others: RistrettoPoint = dealings.iter().map(|d| decompress(&d.commitments[0])).sum();
    let a0 = x * G - others;
    // Shares the attacker hands to members 1 and 2, and commitments that fit them:
    // A_0 + i·A_1 + i²·A_2 = s_i·G for i = 1, 2.
    let (s1, s2) = (Scalar::random(&mut OsRng), Scalar::random(&mut OsRng));
    let (p1, p2) = (s1 * G - a0, s2 * G - a0);
    let two = Scalar::from(2u64);
    let a2 = (p2 - two * p1) * two.invert();
    let a1 = p1 - a2;
    let commitments = vec![
        a0.compress().to_bytes(),
        a1.compress().to_bytes(),
        a2.compress().to_bytes(),
    ];
    // The attack is real: both honest Feldman checks pass and the key would be x·G.
    for (i, s) in [(1u8, s1), (2u8, s2)] {
        verify_feldman(&commitments, i, &(s * G).compress().to_bytes()).unwrap();
    }
    assert_eq!(others + a0, x * G);

    // It has no valid proof for a0. Replaying an honest dealer's proof, or its own proof
    // for a term it does know, does not help.
    let own = Dealer::new(3, 3, 3, &SESSION, &mut OsRng)
        .unwrap()
        .public_dealing();
    for proof in [dealings[0].proof, own.proof, [0u8; DKG_POK_LEN]] {
        let mut all = dealings.clone();
        all.push(PublicDealing {
            dealer: 3,
            commitments: commitments.clone(),
            proof,
        });
        assert_eq!(
            aggregate_dealings(3, &SESSION, &all).unwrap_err(),
            SdkError::BadDealing { dealer: 3 }
        );
        let shares: Vec<ShareMessage> = honest
            .iter()
            .map(|d| d.share_for(1).unwrap())
            .chain([ShareMessage::from_bytes(3, 1, &s1.to_bytes()).unwrap()])
            .collect();
        assert_eq!(
            finish(1, 3, &SESSION, &all, &shares).unwrap_err(),
            SdkError::BadDealing { dealer: 3 }
        );
    }

    // An honest third dealing is accepted, but only in its own session and under its index.
    dealings.push(own.clone());
    aggregate_dealings(3, &SESSION, &dealings).unwrap();
    assert_eq!(
        aggregate_dealings(3, &[1u8; 32], &dealings).unwrap_err(),
        SdkError::BadDealing { dealer: 1 }
    );
    assert!(own.verify_proof(&SESSION, 3, 3).is_ok());
    assert!(
        own.verify_proof(&SESSION, 3, 4).is_err(),
        "bound to the size"
    );
    let mut renumbered = own;
    renumbered.dealer = 2;
    assert!(renumbered.verify_proof(&SESSION, 3, 3).is_err());
}

/// A complaint is settled in public. A valid revealed share clears nobody and blames
/// nobody (the complainant just uses it); an invalid one convicts the dealer.
#[test]
fn a_revealed_share_settles_a_complaint() {
    let dealer = Dealer::new(2, 3, 3, &SESSION, &mut OsRng).unwrap();
    let dealing = dealer.public_dealing();

    // The dealer answers member 1's complaint with the valid share: the run continues.
    let sent = dealer.share_for(1).unwrap();
    check_revealed_share(&dealing, 1, 3, &sent).expect("valid share: nobody is excluded");

    // A share that does not fit the commitments convicts the dealer.
    let bogus = ShareMessage::from_bytes(2, 1, &Scalar::random(&mut OsRng).to_bytes()).unwrap();
    assert_eq!(
        check_revealed_share(&dealing, 1, 3, &bogus).unwrap_err(),
        SdkError::BadDealing { dealer: 2 }
    );
    // Neither a relabelled share nor a correctly labelled share for someone else answers
    // member 1's complaint.
    let other = dealer.share_for(3).unwrap();
    let relabelled = ShareMessage::from_bytes(2, 1, &other.to_bytes()).unwrap();
    assert!(check_revealed_share(&dealing, 1, 3, &relabelled).is_err());
    assert!(check_revealed_share(&dealing, 1, 3, &other).is_err());
    // The complainant must be a member of this committee.
    assert_eq!(
        check_revealed_share(&dealing, 4, 3, &sent).unwrap_err(),
        SdkError::InvalidIndex(4)
    );
}

/// Re-review C9/C10: malformed public input returns an error instead of panicking.
#[test]
fn malformed_group_keys_and_commitment_lists_are_errors_not_panics() {
    let empty = GroupKey {
        threshold: 3,
        commitments: vec![],
        member_pks: vec![],
    };
    assert_eq!(empty.election_pk(), [0u8; 32]);
    assert_eq!(empty.verify(), Err(SdkError::Zk(ZkError::InvalidCommittee)));

    let g = G.compress().to_bytes();
    assert_eq!(
        verify_feldman(&vec![g; 64], 5, &g),
        Err(ZkError::InvalidCommittee)
    );
}
