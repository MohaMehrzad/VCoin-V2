//! Pedersen DKG (every member deals a Feldman-verifiable sharing; the election key is the
//! sum of the constant-term commitments).
//!
//! Protocol for an `n`-member committee with threshold `t`
//! (`MIN_THRESHOLD <= t <= n <= MAX_COMMITTEE`), members indexed `1..=n`:
//! 1. Each member `j` runs [`Dealer::new`], publishes [`Dealer::public_dealing`] and sends
//!    [`Dealer::share_for`]`(i)` to member `i` **over an authenticated, encrypted channel**.
//! 2. Each member `i` calls [`finish`] with all `n` public dealings and the `n` shares
//!    addressed to it. Every share is checked against its dealer's commitments with
//!    `zk_poll_core::verify_feldman`; a bad dealer is named in the error so the committee can
//!    exclude it and rerun.
//! 3. Anyone computes the public [`GroupKey`] with [`aggregate_dealings`] and registers
//!    `election_pk`, `commitments` and `member_pks` on-chain, where
//!    `zk_poll_core::verify_committee` re-checks them.
//!
//! The joint secret `Σ_j f_j(0)` is never computed by any function here.

use core::fmt;

use curve25519_dalek::{ristretto::RistrettoPoint, scalar::Scalar};
use rand::{CryptoRng, RngCore};
use zeroize::{Zeroize, ZeroizeOnDrop};
use zk_poll_core::{check_committee_shape, verify_committee, verify_feldman};

use crate::error::{SdkError, SdkResult};
use crate::points::{bytes, decompress, G};

/// One member's dealing polynomial `f_j(x) = Σ_k a_k x^k`. Secret.
#[derive(Zeroize, ZeroizeOnDrop)]
pub struct Dealer {
    #[zeroize(skip)]
    index: u8,
    #[zeroize(skip)]
    size: u8,
    coefficients: Vec<Scalar>,
}

impl fmt::Debug for Dealer {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Dealer")
            .field("index", &self.index)
            .finish_non_exhaustive()
    }
}

/// The public half of a dealing: `A_{j,k} = a_k·G`.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PublicDealing {
    pub dealer: u8,
    pub commitments: Vec<[u8; 32]>,
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
    pub fn election_pk(&self) -> [u8; 32] {
        self.commitments[0]
    }

    pub fn size(&self) -> u8 {
        self.member_pks.len() as u8
    }

    pub fn member_pk(&self, index: u8) -> SdkResult<[u8; 32]> {
        if index == 0 || index > self.size() {
            return Err(SdkError::InvalidIndex(index));
        }
        Ok(self.member_pks[index as usize - 1])
    }

    /// Same check the program runs when the committee is registered.
    pub fn verify(&self) -> SdkResult<()> {
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
    pub fn new<R: RngCore + CryptoRng>(
        index: u8,
        threshold: u8,
        size: u8,
        rng: &mut R,
    ) -> SdkResult<Self> {
        check_committee_shape(threshold, size)?;
        check_index(index, size)?;
        let coefficients = (0..threshold).map(|_| Scalar::random(rng)).collect();
        Ok(Self {
            index,
            size,
            coefficients,
        })
    }

    pub fn public_dealing(&self) -> PublicDealing {
        PublicDealing {
            dealer: self.index,
            commitments: self.coefficients.iter().map(|a| bytes(&(a * G))).collect(),
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

/// Sums all dealings into the public group key (anyone can run this).
pub fn aggregate_dealings(threshold: u8, dealings: &[PublicDealing]) -> SdkResult<GroupKey> {
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

/// Member `index` verifies every share it received against its dealer's commitments and
/// derives its key share. Requires one dealing and one share from every member.
pub fn finish(
    index: u8,
    threshold: u8,
    dealings: &[PublicDealing],
    shares: &[ShareMessage],
) -> SdkResult<KeyShare> {
    let group = aggregate_dealings(threshold, dealings)?;
    let size = group.size();
    check_index(index, size)?;
    if shares.len() != size as usize {
        return Err(SdkError::IncompleteDealings { expected: size });
    }
    let mut secret = Scalar::ZERO;
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
        secret += share.value;
    }
    let public = bytes(&(secret * G));
    if public != group.member_pk(index)? {
        // Unreachable when every share passed its Feldman check.
        return Err(SdkError::Zk(zk_poll_core::ZkError::FeldmanMismatch));
    }
    Ok(KeyShare {
        index,
        secret,
        public,
        group,
    })
}
