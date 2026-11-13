//! Scalars mod ℓ = 2²⁵² + 27742317777372353535851937790883648493, as 32-byte little-endian.
//!
//! Only what verification needs: a canonical hash-to-scalar, a canonicity check, addition and
//! negation. Inputs to `add` / `neg` must be canonical.

use solana_sha256_hasher::hashv;

/// ℓ as little-endian u64 limbs.
const L: [u64; 4] = [
    0x5812631a5cf5d3ed,
    0x14def9dea2f79cd6,
    0,
    0x1000000000000000,
];

fn limbs(b: &[u8; 32]) -> [u64; 4] {
    let mut out = [0u64; 4];
    for (i, limb) in out.iter_mut().enumerate() {
        *limb = u64::from_le_bytes(b[i * 8..i * 8 + 8].try_into().unwrap());
    }
    out
}

fn bytes(l: &[u64; 4]) -> [u8; 32] {
    let mut out = [0u8; 32];
    for (i, limb) in l.iter().enumerate() {
        out[i * 8..i * 8 + 8].copy_from_slice(&limb.to_le_bytes());
    }
    out
}

/// a >= b on limbs.
fn ge(a: &[u64; 4], b: &[u64; 4]) -> bool {
    for i in (0..4).rev() {
        if a[i] != b[i] {
            return a[i] > b[i];
        }
    }
    true
}

fn sub(a: &[u64; 4], b: &[u64; 4]) -> [u64; 4] {
    let mut out = [0u64; 4];
    let mut borrow = 0u64;
    for i in 0..4 {
        let (d1, b1) = a[i].overflowing_sub(b[i]);
        let (d2, b2) = d1.overflowing_sub(borrow);
        out[i] = d2;
        borrow = (b1 | b2) as u64;
    }
    out
}

/// True iff `s < ℓ`.
pub fn is_canonical(s: &[u8; 32]) -> bool {
    !ge(&limbs(s), &L)
}

/// `(a + b) mod ℓ` for canonical `a`, `b`.
pub fn add(a: &[u8; 32], b: &[u8; 32]) -> [u8; 32] {
    let (a, b) = (limbs(a), limbs(b));
    let mut sum = [0u64; 4];
    let mut carry = 0u64;
    for i in 0..4 {
        let (s1, c1) = a[i].overflowing_add(b[i]);
        let (s2, c2) = s1.overflowing_add(carry);
        sum[i] = s2;
        carry = (c1 | c2) as u64;
    }
    // a, b < ℓ < 2^253, so the sum < 2^254 and never carries out of 256 bits.
    if ge(&sum, &L) {
        sum = sub(&sum, &L);
    }
    bytes(&sum)
}

/// `(-a) mod ℓ` for canonical `a`.
pub fn neg(a: &[u8; 32]) -> [u8; 32] {
    if *a == [0u8; 32] {
        return *a;
    }
    bytes(&sub(&L, &limbs(a)))
}

/// Canonical scalar from a small integer.
pub fn from_u64(v: u64) -> [u8; 32] {
    let mut out = [0u8; 32];
    out[..8].copy_from_slice(&v.to_le_bytes());
    out
}

/// Hash-to-scalar shared by prover and verifier: SHA-256 with the top four bits cleared.
/// The result is below 2²⁵² < ℓ, so it is always canonical, and its distribution is within
/// 2⁻¹²⁸ of uniform mod ℓ (audit ZK-02: one definition everywhere).
pub fn hs(parts: &[&[u8]]) -> [u8; 32] {
    let mut out = hashv(parts).to_bytes();
    out[31] &= 0x0f;
    out
}
