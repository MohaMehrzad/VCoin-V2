//! Pedersen DKG (every member deals a Feldman-verifiable sharing; the election key is the
//! sum of the constant-term commitments).
//!
//! Protocol for an `n`-member committee with threshold `t`
//! (`MIN_THRESHOLD <= t <= n <= MAX_COMMITTEE`), members indexed `1..=n`:
//! 0. The members agree on a fresh 32-byte `session` id for this run (for example a hash
//!    of the member list, the threshold and a nonce).
//! 1. Each member `j` runs [`Dealer::new`], publishes [`Dealer::public_dealing`] and sends
//!    [`Dealer::share_for`]`(i)` to member `i` **over an authenticated, encrypted channel**.
//!    A public dealing carries a Schnorr proof that the dealer knows its constant term,
//!    bound to the session and the dealer's index.
//! 2. Each member `i` calls [`finish`] with all `n` public dealings and the `n` shares
//!    addressed to it. Every proof of knowledge is checked, and every share is checked
//!    against its dealer's commitments with `zk_poll_core::verify_feldman`; a bad dealer is
//!    named in the error so the committee can exclude it and rerun.
//! 3. Anyone computes the public [`GroupKey`] with [`aggregate_dealings`] (which checks
//!    the proofs again) and registers `election_pk`, `commitments` and `member_pks`
//!    on-chain, where `zk_poll_core::verify_committee` re-checks them. Publish the
//!    dealings, so anyone can confirm the registered key is their sum.
//!
//! Why the proof of knowledge: without it, the member who publishes last could choose its
//! constant term as `x·G − Σ(others)`, making the election key `x·G` for an `x` it knows,
//! and decrypt every ballot alone (a rogue-key attack). A dealer that must prove it knows
//! the discrete log of its own term cannot do that.
//!
//! Complaints: if member `i` says dealer `j` sent a bad share, `j` publishes that share
//! and anyone runs [`check_revealed_share`]. It tells who lied; exclude that member and
//! rerun with a new session.
//!
//! The joint secret `Σ_j f_j(0)` is never computed by any function here.

use core::fmt;

use curve25519_dalek::{ristretto::RistrettoPoint, scalar::Scalar};
use rand::{CryptoRng, RngCore};
use zeroize::{Zeroize, ZeroizeOnDrop, Zeroizing};
use zk_poll_core::{
    check_committee_shape, scalar::hs, verify_committee, verify_feldman, ZkError, IDENTITY,
};

use crate::error::{SdkError, SdkResult};
use crate::points::{bytes, challenge, decompress, G};

/// Domain tag of the dealer's proof of knowledge.
pub const DOMAIN_DKG_POK: &[u8] = b"VCoin-PrivatePoll-DKG-PoK-v1";
/// `R ‖ z` of the Schnorr proof.
pub const DKG_POK_LEN: usize = 64;

/// Fiat–Shamir challenge of a dealer's proof of knowledge. Binds the run (`session`), the
/// committee shape and the dealer's index, so a proof cannot be replayed by another dealer
/// or in another run.
fn pok_challenge(
    session: &[u8; 32],
    dealer: u8,
    threshold: u8,
    size: u8,
    constant_term: &[u8; 32],
    nonce_point: &[u8; 32],
) -> Scalar {
    challenge(hs(&[
        DOMAIN_DKG_POK,
        session,
        &[dealer, threshold, size],
        constant_term,
        nonce_point,
    ]))
}

/// One member's dealing polynomial `f_j(x) = Σ_k a_k x^k`. Secret.
#[derive(Zeroize, ZeroizeOnDrop)]
pub struct Dealer {
    #[zeroize(skip)]
    index: u8,
    #[zeroize(skip)]
    size: u8,
    coefficients: Vec<Scalar>,
    /// Public proof of knowledge of `coefficients[0]`.
    #[zeroize(skip)]
    proof: [u8; DKG_POK_LEN],
}

impl fmt::Debug for Dealer {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Dealer")
            .field("index", &self.index)
            .finish_non_exhaustive()
    }
}

/// The public half of a dealing: `A_{j,k} = a_k·G`, and a Schnorr proof (`R ‖ z`) that the
/// dealer knows `a_0`.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PublicDealing {
    pub dealer: u8,
    pub commitments: Vec<[u8; 32]>,
    pub proof: [u8; DKG_POK_LEN],
}

impl PublicDealing {
    /// Checks the proof of knowledge for a committee of `size` with `threshold`, in the
    /// run identified by `session`.
    pub fn verify_proof(&self, session: &[u8; 32], threshold: u8, size: u8) -> SdkResult<()> {
        let bad = SdkError::BadDealing {
            dealer: self.dealer,
        };
        let constant_term = match self.commitments.first() {
            Some(a0) if *a0 != IDENTITY => a0,
            _ => return Err(bad),
        };
        let nonce_point: [u8; 32] = self.proof[..32].try_into().expect("32 bytes");
        let z_bytes: [u8; 32] = self.proof[32..].try_into().expect("32 bytes");
        let (Ok(a0), Ok(r), Some(z)) = (
            decompress(constant_term),
            decompress(&nonce_point),
            Option::<Scalar>::from(Scalar::from_canonical_bytes(z_bytes)),
        ) else {
            return Err(bad);
        };
        let c = pok_challenge(
            session,
            self.dealer,
            threshold,
            size,
            constant_term,
            &nonce_point,
        );
        if z * G == r + c * a0 {
            Ok(())
        } else {
            Err(bad)
        }
    }
}

/// `f_dealer(recipient)`. Secret: send only to `recipient`, encrypted and authenticated.
#[derive(Clone, Zeroize, ZeroizeOnDrop)]
pub struct ShareMessage {
    #[zeroize(skip)]
    pub dealer: u8,
    #[zeroize(skip)]
    pub recipient: u8,
    value: Scalar,
}

impl fmt::Debug for ShareMessage {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ShareMessage")
            .field("dealer", &self.dealer)
            .field("recipient", &self.recipient)
            .finish_non_exhaustive()
    }
}

impl ShareMessage {
    /// Serialized share for transport (wrap the channel, not this value, in encryption).
    pub fn to_bytes(&self) -> zeroize::Zeroizing<[u8; 32]> {
        zeroize::Zeroizing::new(self.value.to_bytes())
    }

    pub fn from_bytes(dealer: u8, recipient: u8, bytes: &[u8; 32]) -> SdkResult<Self> {
        let value = Option::from(Scalar::from_canonical_bytes(*bytes))
            .ok_or(SdkError::BadDealing { dealer })?;
        Ok(Self {
            dealer,
            recipient,
            value,
        })
    }
}

/// Public committee key material, identical for every member and registered on-chain.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct GroupKey {
    pub threshold: u8,
    /// `A_k = Σ_j A_{j,k}`; `A_0` is the election key.
    pub commitments: Vec<[u8; 32]>,
    /// `member_pks[i - 1] = Σ_k i^k·A_k`, the public key of member `i`'s share.
    pub member_pks: Vec<[u8; 32]>,
}

impl GroupKey {
    /// `A_0`. The identity (never a valid key) if the commitments are empty.
    pub fn election_pk(&self) -> [u8; 32] {
        self.commitments.first().copied().unwrap_or(IDENTITY)
    }

    /// Number of members, saturating (anything above `MAX_COMMITTEE` fails `verify`).
    pub fn size(&self) -> u8 {
        u8::try_from(self.member_pks.len()).unwrap_or(u8::MAX)
    }

    pub fn member_pk(&self, index: u8) -> SdkResult<[u8; 32]> {
        if index == 0 || index > self.size() {
            return Err(SdkError::InvalidIndex(index));
        }
        Ok(self.member_pks[index as usize - 1])
    }

    /// Same check the program runs when the committee is registered. Never panics on
    /// malformed input.
    pub fn verify(&self) -> SdkResult<()> {
        if self.commitments.is_empty() {
            return Err(ZkError::InvalidCommittee.into());
        }
        verify_committee(
            self.threshold,
            &self.election_pk(),
            &self.commitments,
            &self.member_pks,
        )?;
        Ok(())
    }
}

/// A member's final key share. Secret.
#[derive(Zeroize, ZeroizeOnDrop)]
pub struct KeyShare {
    #[zeroize(skip)]
    pub index: u8,
    secret: Scalar,
    #[zeroize(skip)]
    pub public: [u8; 32],
    #[zeroize(skip)]
    pub group: GroupKey,
}

impl fmt::Debug for KeyShare {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("KeyShare")
            .field("index", &self.index)
            .field("public", &self.public)
            .finish_non_exhaustive()
    }
}

impl KeyShare {
    pub(crate) fn secret(&self) -> &Scalar {
        &self.secret
    }
}

fn check_index(index: u8, size: u8) -> SdkResult<()> {
    if index == 0 || index > size {
        return Err(SdkError::InvalidIndex(index));
    }
    Ok(())
}

impl Dealer {
    /// `session` identifies this DKG run; every member must use the same value.
    pub fn new<R: RngCore + CryptoRng>(
        index: u8,
        threshold: u8,
        size: u8,
        session: &[u8; 32],
        rng: &mut R,
    ) -> SdkResult<Self> {
        check_committee_shape(threshold, size)?;
        check_index(index, size)?;
        let coefficients: Vec<Scalar> = (0..threshold).map(|_| Scalar::random(rng)).collect();

        // Schnorr proof of knowledge of the constant term: R = k·G, z = k + c·a_0.
        let nonce = Zeroizing::new(Scalar::random(rng));
        let nonce_point = bytes(&(*nonce * G));
        let constant_term = bytes(&(coefficients[0] * G));
        let c = pok_challenge(
            session,
            index,
            threshold,
            size,
            &constant_term,
            &nonce_point,
        );
        let z = Zeroizing::new(*nonce + c * coefficients[0]);
        let mut proof = [0u8; DKG_POK_LEN];
        proof[..32].copy_from_slice(&nonce_point);
        proof[32..].copy_from_slice(z.as_bytes());

        Ok(Self {
            index,
            size,
            coefficients,
            proof,
        })
    }

    pub fn public_dealing(&self) -> PublicDealing {
        PublicDealing {
            dealer: self.index,
            commitments: self.coefficients.iter().map(|a| bytes(&(a * G))).collect(),
            proof: self.proof,
        }
    }

    pub fn share_for(&self, recipient: u8) -> SdkResult<ShareMessage> {
        check_index(recipient, self.size)?;
        let x = Scalar::from(recipient as u64);
        let value = self
            .coefficients
            .iter()
            .rev()
            .fold(Scalar::ZERO, |acc, a| acc * x + a);
        Ok(ShareMessage {
            dealer: self.index,
            recipient,
            value,
        })
    }
}

/// Settles a complaint in public: the accused dealer reveals the share it sent to the
/// complaining recipient, and anyone checks it against the dealer's published
/// commitments. `Ok` means the share was valid (the complaint was unfounded);
/// `BadDealing` means the dealer cheated.
pub fn check_revealed_share(dealing: &PublicDealing, share: &ShareMessage) -> SdkResult<()> {
    let bad = SdkError::BadDealing {
        dealer: dealing.dealer,
    };
    if share.dealer != dealing.dealer {
        return Err(bad);
    }
    let share_pk = bytes(&(share.value * G));
    verify_feldman(&dealing.commitments, share.recipient, &share_pk).map_err(|_| bad)
}

/// Sums all dealings into the public group key (anyone can run this). Every dealing's
/// proof of knowledge is checked first, which is what rules out a rogue election key.
pub fn aggregate_dealings(
    threshold: u8,
    session: &[u8; 32],
    dealings: &[PublicDealing],
) -> SdkResult<GroupKey> {
    let size =
        u8::try_from(dealings.len()).map_err(|_| SdkError::IncompleteDealings { expected: 0 })?;
    check_committee_shape(threshold, size)?;
    let mut seen = vec![false; size as usize + 1];
    let mut sums = vec![RistrettoPoint::default(); threshold as usize];
    for d in dealings {
        check_index(d.dealer, size)?;
        if std::mem::replace(&mut seen[d.dealer as usize], true) {
            return Err(SdkError::DuplicateIndex(d.dealer));
        }
        if d.commitments.len() != threshold as usize {
            return Err(SdkError::BadDealing { dealer: d.dealer });
        }
        d.verify_proof(session, threshold, size)?;
        for (sum, c) in sums.iter_mut().zip(&d.commitments) {
            *sum += decompress(c).map_err(|_| SdkError::BadDealing { dealer: d.dealer })?;
        }
    }
    let member_pks = (1..=size)
        .map(|i| {
            let x = Scalar::from(i as u64);
            bytes(
                &sums
                    .iter()
                    .rev()
                    .fold(RistrettoPoint::default(), |acc, a| acc * x + a),
            )
        })
        .collect();
    let group = GroupKey {
        threshold,
        commitments: sums.iter().map(bytes).collect(),
        member_pks,
    };
    group.verify()?;
    Ok(group)
}

/// Member `index` verifies every dealer's proof of knowledge and every share it received
/// against its dealer's commitments, and derives its key share. Requires one dealing and
/// one share from every member.
pub fn finish(
    index: u8,
    threshold: u8,
    session: &[u8; 32],
    dealings: &[PublicDealing],
    shares: &[ShareMessage],
) -> SdkResult<KeyShare> {
    let group = aggregate_dealings(threshold, session, dealings)?;
    let size = group.size();
    check_index(index, size)?;
    if shares.len() != size as usize {
        return Err(SdkError::IncompleteDealings { expected: size });
    }
    // Wiped on every exit path, including an early error.
    let mut secret = Zeroizing::new(Scalar::ZERO);
    for dealing in dealings {
        let mut from_dealer = shares.iter().filter(|s| s.dealer == dealing.dealer);
        let share = match (from_dealer.next(), from_dealer.next()) {
            (Some(s), None) if s.recipient == index => s,
            _ => {
                return Err(SdkError::BadDealing {
                    dealer: dealing.dealer,
                })
            }
        };
        let share_pk = bytes(&(share.value * G));
        verify_feldman(&dealing.commitments, index, &share_pk).map_err(|_| {
            SdkError::BadDealing {
                dealer: dealing.dealer,
            }
        })?;
        *secret += share.value;
    }
    let public = bytes(&(*secret * G));
    if public != group.member_pk(index)? {
        // Unreachable when every share passed its Feldman check.
        return Err(SdkError::Zk(zk_poll_core::ZkError::FeldmanMismatch));
    }
    Ok(KeyShare {
        index,
        secret: *secret,
        public,
        group,
    })
}
