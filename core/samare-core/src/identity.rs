// SPDX-License-Identifier: GPL-3.0-or-later

//! Local identity: the key pairs created on the device, and the safety number
//! two people compare by hand.
//!
//! An identity is two key pairs, both from the dalek crates:
//! - Ed25519 for signatures;
//! - X25519 for key agreement with a contact, before any Olm session exists
//!   (recipient tags, envelope keys).
//!
//! The Olm account used for end-to-end sessions is separate: vodozemac does
//! not expose its private Curve25519 key for a raw Diffie-Hellman.

use std::fmt;

use ed25519_dalek::{SigningKey, VerifyingKey};
use sha2::{Digest, Sha512};
use x25519_dalek::{PublicKey as AgreementPublicKey, StaticSecret};
use zeroize::Zeroizing;

/// Length of the serialized secret identity: Ed25519 seed, then X25519 secret.
pub const SECRET_LENGTH: usize = 64;

/// Length of a public key, Ed25519 or X25519.
pub const PUBLIC_KEY_LENGTH: usize = 32;

/// Errors raised while creating or loading an identity.
#[derive(Debug, thiserror::Error)]
pub enum IdentityError {
    /// The operating system random generator failed.
    #[error("the system random generator failed")]
    Random,
    /// The signing key is not a valid Ed25519 point, or is of small order.
    #[error("invalid signing key")]
    InvalidSigningKey,
}

/// The local identity, with its secret keys.
///
/// Secrets are zeroized on drop. `Debug` never prints key material.
#[derive(Clone)]
pub struct Identity {
    signing: SigningKey,
    agreement: StaticSecret,
}

impl Identity {
    /// Creates a new identity from the operating system random generator.
    pub fn generate() -> Result<Self, IdentityError> {
        let mut secret = Zeroizing::new([0u8; SECRET_LENGTH]);
        getrandom::fill(secret.as_mut()).map_err(|_| IdentityError::Random)?;
        Ok(Self::from_secret_bytes(&secret))
    }

    /// Restores an identity saved with [`Identity::to_secret_bytes`].
    pub fn from_secret_bytes(secret: &[u8; SECRET_LENGTH]) -> Self {
        let mut seed = Zeroizing::new([0u8; 32]);
        let mut agreement = Zeroizing::new([0u8; 32]);
        seed.copy_from_slice(&secret[..32]);
        agreement.copy_from_slice(&secret[32..]);
        Self {
            signing: SigningKey::from_bytes(&seed),
            agreement: StaticSecret::from(*agreement),
        }
    }

    /// Serializes the secret keys, for the encrypted local storage only.
    pub fn to_secret_bytes(&self) -> Zeroizing<[u8; SECRET_LENGTH]> {
        let mut secret = Zeroizing::new([0u8; SECRET_LENGTH]);
        secret[..32].copy_from_slice(self.signing.as_bytes());
        secret[32..].copy_from_slice(self.agreement.as_bytes());
        secret
    }

    /// The public half, shared with contacts.
    pub fn public(&self) -> PublicIdentity {
        PublicIdentity {
            signing: self.signing.verifying_key(),
            agreement: AgreementPublicKey::from(&self.agreement),
        }
    }
}

impl fmt::Debug for Identity {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("Identity(..)")
    }
}

/// The public keys of an identity. Two identities are the same person for
/// Samare only if both keys match.
///
/// `Debug` never prints the keys: they identify a person.
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct PublicIdentity {
    signing: VerifyingKey,
    agreement: AgreementPublicKey,
}

impl PublicIdentity {
    /// Builds a public identity from its two keys, as read from a contact card.
    ///
    /// Rejects a signing key that is not a valid point or is of small order.
    pub fn from_bytes(
        signing: &[u8; PUBLIC_KEY_LENGTH],
        agreement: &[u8; PUBLIC_KEY_LENGTH],
    ) -> Result<Self, IdentityError> {
        let signing =
            VerifyingKey::from_bytes(signing).map_err(|_| IdentityError::InvalidSigningKey)?;
        if signing.is_weak() {
            return Err(IdentityError::InvalidSigningKey);
        }
        Ok(Self {
            signing,
            agreement: AgreementPublicKey::from(*agreement),
        })
    }

    /// The Ed25519 public key.
    pub fn signing_key(&self) -> &[u8; PUBLIC_KEY_LENGTH] {
        self.signing.as_bytes()
    }

    /// The X25519 public key.
    pub fn agreement_key(&self) -> &[u8; PUBLIC_KEY_LENGTH] {
        self.agreement.as_bytes()
    }

    /// The 30-digit fingerprint of this identity, half of a safety number.
    pub fn fingerprint(&self) -> Fingerprint {
        Fingerprint::of(self)
    }
}

impl fmt::Debug for PublicIdentity {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("PublicIdentity(..)")
    }
}

/// Number of SHA-512 rounds in a fingerprint, as in Signal's safety numbers.
///
/// A fingerprint keeps about 100 bits of the hash; the rounds make each try
/// of a brute-force search for a matching fingerprint about 2^12 times dearer.
const FINGERPRINT_ROUNDS: usize = 5200;

/// Domain separation for fingerprint version 1.
const FINGERPRINT_DOMAIN: &[u8] = b"samare-fingerprint-v1";

/// Number of decimal digits in a fingerprint.
pub const FINGERPRINT_DIGITS: usize = 30;

/// The fingerprint of one identity: 30 decimal digits.
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Fingerprint([u8; FINGERPRINT_DIGITS]);

impl Fingerprint {
    fn of(identity: &PublicIdentity) -> Self {
        let keys = [identity.signing_key(), identity.agreement_key()];
        let mut hash = Sha512::new()
            .chain_update(FINGERPRINT_DOMAIN)
            .chain_update(keys[0])
            .chain_update(keys[1])
            .finalize();
        for _ in 0..FINGERPRINT_ROUNDS {
            hash = Sha512::new()
                .chain_update(hash)
                .chain_update(keys[0])
                .chain_update(keys[1])
                .finalize();
        }

        // Six chunks of 5 bytes, each reduced to 5 digits.
        let mut digits = [0u8; FINGERPRINT_DIGITS];
        let (chunks, _) = hash[..30].as_chunks::<5>();
        let (outs, _) = digits.as_chunks_mut::<5>();
        for (chunk, out) in chunks.iter().zip(outs) {
            let mut value = chunk.iter().fold(0u64, |acc, b| (acc << 8) | u64::from(*b)) % 100_000;
            for digit in out.iter_mut().rev() {
                // `value % 10` is below 10: the cast cannot truncate.
                *digit = b'0' + (value % 10) as u8;
                value /= 10;
            }
        }
        Self(digits)
    }

    /// The digits, without separators.
    pub fn as_str(&self) -> &str {
        // Only ASCII digits are ever written into the array.
        std::str::from_utf8(&self.0).unwrap_or_default()
    }
}

impl fmt::Display for Fingerprint {
    /// Six groups of five digits.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write_groups(f, self.as_str())
    }
}

impl fmt::Debug for Fingerprint {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("Fingerprint(..)")
    }
}

/// The safety number two contacts compare by hand: both fingerprints, the
/// smaller first, so that both phones show the same 60 digits.
#[derive(Clone, PartialEq, Eq)]
pub struct SafetyNumber(String);

impl SafetyNumber {
    /// The safety number between two identities. The order of the arguments
    /// does not matter.
    pub fn new(a: &PublicIdentity, b: &PublicIdentity) -> Self {
        let (a, b) = (a.fingerprint(), b.fingerprint());
        let (low, high) = if a <= b { (a, b) } else { (b, a) };
        Self(format!("{}{}", low.as_str(), high.as_str()))
    }

    /// The 60 digits, without separators.
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for SafetyNumber {
    /// Twelve groups of five digits.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write_groups(f, &self.0)
    }
}

impl fmt::Debug for SafetyNumber {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("SafetyNumber(..)")
    }
}

/// Writes ASCII digits in groups of five, separated by spaces.
fn write_groups(f: &mut fmt::Formatter<'_>, digits: &str) -> fmt::Result {
    for (i, group) in digits.as_bytes().chunks(5).enumerate() {
        if i > 0 {
            f.write_str(" ")?;
        }
        f.write_str(std::str::from_utf8(group).map_err(|_| fmt::Error)?)?;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn identity(byte: u8) -> Identity {
        Identity::from_secret_bytes(&[byte; SECRET_LENGTH])
    }

    #[test]
    fn generated_identities_differ() {
        let a = Identity::generate().unwrap();
        let b = Identity::generate().unwrap();
        assert_ne!(a.public(), b.public());
    }

    #[test]
    fn secret_bytes_round_trip() {
        let a = Identity::generate().unwrap();
        let b = Identity::from_secret_bytes(&a.to_secret_bytes());
        assert_eq!(a.public(), b.public());
    }

    #[test]
    fn public_identity_round_trip() {
        let public = identity(1).public();
        let read =
            PublicIdentity::from_bytes(public.signing_key(), public.agreement_key()).unwrap();
        assert_eq!(public, read);
    }

    #[test]
    fn small_order_signing_key_is_rejected() {
        // The Ed25519 identity point, of order 1.
        let mut identity_point = [0u8; 32];
        identity_point[0] = 1;
        assert!(matches!(
            PublicIdentity::from_bytes(&identity_point, &[9; 32]),
            Err(IdentityError::InvalidSigningKey)
        ));
    }

    #[test]
    fn debug_hides_keys() {
        let a = identity(1);
        assert_eq!(format!("{a:?}"), "Identity(..)");
        assert_eq!(format!("{:?}", a.public()), "PublicIdentity(..)");
        assert_eq!(format!("{:?}", a.public().fingerprint()), "Fingerprint(..)");
    }

    #[test]
    fn fingerprint_is_30_digits_in_6_groups() {
        let fingerprint = identity(1).public().fingerprint();
        assert_eq!(fingerprint.as_str().len(), FINGERPRINT_DIGITS);
        assert!(fingerprint.as_str().bytes().all(|b| b.is_ascii_digit()));
        let shown = fingerprint.to_string();
        assert_eq!(shown.split(' ').count(), 6);
        assert!(shown.split(' ').all(|group| group.len() == 5));
    }

    #[test]
    fn fingerprint_depends_on_both_keys() {
        let a = identity(1).public();
        let b = identity(2).public();
        let mixed = PublicIdentity::from_bytes(a.signing_key(), b.agreement_key()).unwrap();
        assert_ne!(a.fingerprint(), mixed.fingerprint());
        assert_ne!(b.fingerprint(), mixed.fingerprint());
    }

    #[test]
    fn safety_number_is_symmetric() {
        let a = identity(1).public();
        let b = identity(2).public();
        let number = SafetyNumber::new(&a, &b);
        assert_eq!(number, SafetyNumber::new(&b, &a));
        assert_eq!(number.as_str().len(), 2 * FINGERPRINT_DIGITS);
        assert_eq!(number.to_string().split(' ').count(), 12);
    }
}
