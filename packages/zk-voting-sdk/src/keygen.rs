use curve25519_dalek::constants::RISTRETTO_BASEPOINT_POINT;
use curve25519_dalek::scalar::Scalar;
use rand::rngs::OsRng;

use crate::types::ElGamalKeypair;

/// Generate a single ElGamal keypair
pub fn generate_elgamal_keypair() -> ElGamalKeypair {
    let secret = Scalar::random(&mut OsRng);
    let public = secret * RISTRETTO_BASEPOINT_POINT;
    ElGamalKeypair { secret, public }
}

/// Generate threshold key shares using Shamir Secret Sharing
///
/// Returns (joint_pubkey, shares) where shares[i] = (index, secret_share)
/// The joint secret key is the sum of individual secrets.
/// threshold-of-n: any `threshold` shares can reconstruct.
pub fn threshold_keygen(n: usize, threshold: usize) -> (ElGamalKeypair, Vec<(u8, Scalar)>) {
    assert!(threshold <= n && threshold >= 1 && n <= 5);

    // Generate random polynomial of degree (threshold - 1)
    let mut coefficients = Vec::with_capacity(threshold);
    for _ in 0..threshold {
        coefficients.push(Scalar::random(&mut OsRng));
    }

    // The joint secret is coefficients[0]
    let joint_secret = coefficients[0];
    let joint_public = joint_secret * RISTRETTO_BASEPOINT_POINT;

    // Evaluate polynomial at points 1..=n
    let mut shares = Vec::with_capacity(n);
    for i in 1..=(n as u64) {
        let x = Scalar::from(i);
        let mut y = Scalar::ZERO;
        let mut x_pow = Scalar::ONE;

        for coeff in &coefficients {
            y += coeff * x_pow;
            x_pow *= x;
        }

        shares.push(((i - 1) as u8, y));
    }

    (ElGamalKeypair { secret: joint_secret, public: joint_public }, shares)
}

/// Compute the joint public key from individual committee member public keys
///
/// For additive key generation: joint_pk = pk_1 + pk_2 + ... + pk_n
pub fn compute_joint_pubkey(pubkeys: &[curve25519_dalek::ristretto::RistrettoPoint]) -> curve25519_dalek::ristretto::RistrettoPoint {
    pubkeys.iter().fold(
        curve25519_dalek::ristretto::RistrettoPoint::default(),
        |acc, pk| acc + pk,
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use curve25519_dalek::constants::RISTRETTO_BASEPOINT_POINT;

    #[test]
    fn test_keygen() {
        let keypair = generate_elgamal_keypair();
        assert_eq!(keypair.public, keypair.secret * RISTRETTO_BASEPOINT_POINT);
    }

    #[test]
    fn test_threshold_keygen_3_of_5() {
        let (joint, shares) = threshold_keygen(5, 3);
        assert_eq!(shares.len(), 5);
        assert_eq!(joint.public, joint.secret * RISTRETTO_BASEPOINT_POINT);

        // Verify that any 3 shares can reconstruct the secret
        let selected = &shares[0..3];
        let reconstructed = reconstruct_secret(selected);
        assert_eq!(reconstructed, joint.secret);
    }

    fn reconstruct_secret(shares: &[(u8, Scalar)]) -> Scalar {
        let mut secret = Scalar::ZERO;
        for (i, (xi, yi)) in shares.iter().enumerate() {
            let xi_scalar = Scalar::from((*xi as u64) + 1);
            let mut lagrange = Scalar::ONE;
            for (j, (xj, _)) in shares.iter().enumerate() {
                if i != j {
                    let xj_scalar = Scalar::from((*xj as u64) + 1);
                    lagrange *= xj_scalar * (xj_scalar - xi_scalar).invert();
                }
            }
            secret += yi * lagrange;
        }
        secret
    }
}
