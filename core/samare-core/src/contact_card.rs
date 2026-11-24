// SPDX-License-Identifier: GPL-3.0-or-later

//! Contact card: what a QR code carries to add someone as a contact.
//!
//! The card is a link, `https://samare.app/c#` followed by the card bytes in
//! unpadded base64url. Scanned with the camera app by someone without Samare,
//! the link opens the site; the card stays in the fragment, which a browser
//! never sends to the server.
//!
//! Card bytes, version 1:
//!
//! | Field          | Bytes   |
//! |----------------|---------|
//! | version (1)    | 1       |
//! | Ed25519 key    | 32      |
//! | X25519 key     | 32      |
//! | nickname length| 1       |
//! | nickname UTF-8 | 1 to 64 |
//!
//! The card is not signed: whoever can replace a QR code can replace all of
//! it. A contact is verified by scanning in person or by comparing the safety
//! number, which covers both keys.

use std::fmt;

use base64ct::{Base64UrlUnpadded, Encoding};

use crate::identity::{IdentityError, PUBLIC_KEY_LENGTH, PublicIdentity};

/// Prefix of every contact card link.
pub const CARD_PREFIX: &str = "https://samare.app/c#";

/// Current card format version.
pub const CARD_VERSION: u8 = 1;

/// Maximum length of a nickname, in UTF-8 bytes.
pub const NICKNAME_MAX_BYTES: usize = 64;

/// Length of the fixed part of a card: version, two keys, nickname length.
const FIXED_LENGTH: usize = 1 + 2 * PUBLIC_KEY_LENGTH + 1;

/// Errors raised by an invalid nickname.
#[derive(Debug, PartialEq, Eq, thiserror::Error)]
pub enum NicknameError {
    /// Nothing left once surrounding spaces are removed.
    #[error("the nickname is empty")]
    Empty,
    /// Longer than [`NICKNAME_MAX_BYTES`] in UTF-8.
    #[error("the nickname is longer than {NICKNAME_MAX_BYTES} bytes")]
    TooLong,
    /// Contains a control character or an invisible formatting character.
    #[error("the nickname contains a forbidden character")]
    ForbiddenCharacter,
}

/// Errors raised while reading a contact card.
#[derive(Debug, thiserror::Error)]
pub enum ContactCardError {
    /// The text is not a Samare contact card link.
    #[error("not a Samare contact card")]
    NotAContactCard,
    /// The card was made by a newer version of Samare.
    #[error("unsupported card version {0}")]
    UnsupportedVersion(u8),
    /// Bad base64url, wrong length or trailing bytes.
    #[error("malformed contact card")]
    Malformed,
    /// A key in the card is not valid.
    #[error("invalid key in contact card")]
    InvalidKey(#[from] IdentityError),
    /// The nickname in the card is not valid.
    #[error("invalid nickname in contact card")]
    InvalidNickname(#[from] NicknameError),
}

/// A nickname, chosen by its owner and shown to contacts.
///
/// Trimmed, 1 to [`NICKNAME_MAX_BYTES`] bytes, without control characters or
/// invisible formatting characters (zero-width spaces, direction overrides)
/// that could make two nicknames look alike. `Debug` never prints it.
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct Nickname(String);

impl Nickname {
    /// Validates a nickname typed by the user. Surrounding spaces are removed.
    pub fn new(nickname: &str) -> Result<Self, NicknameError> {
        let nickname = nickname.trim();
        if nickname.is_empty() {
            return Err(NicknameError::Empty);
        }
        if nickname.len() > NICKNAME_MAX_BYTES {
            return Err(NicknameError::TooLong);
        }
        if nickname.chars().any(is_forbidden) {
            return Err(NicknameError::ForbiddenCharacter);
        }
        Ok(Self(nickname.to_owned()))
    }

    /// The nickname, for display.
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Debug for Nickname {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("Nickname(..)")
    }
}

/// Control characters, and invisible characters that change how text is laid
/// out or joined.
fn is_forbidden(c: char) -> bool {
    c.is_control()
        || matches!(
            c,
            '\u{00AD}'                  // soft hyphen
            | '\u{061C}'                // Arabic letter mark
            | '\u{180E}'                // Mongolian vowel separator
            | '\u{200B}'..='\u{200F}'   // zero-width spaces and joiners, LRM, RLM
            | '\u{2028}'..='\u{202E}'   // line and paragraph separators, embeddings, overrides
            | '\u{2060}'..='\u{206F}'   // word joiner, invisible operators, isolates
            | '\u{FEFF}'                // zero-width no-break space
            | '\u{FFF9}'..='\u{FFFB}' // interlinear annotations
        )
}

/// What a contact QR code carries: public keys and nickname.
#[derive(Clone, PartialEq, Eq)]
pub struct ContactCard {
    identity: PublicIdentity,
    nickname: Nickname,
}

impl ContactCard {
    /// Builds the card to show in one's own QR code.
    pub fn new(identity: PublicIdentity, nickname: Nickname) -> Self {
        Self { identity, nickname }
    }

    /// The public keys of the card's owner.
    pub fn identity(&self) -> &PublicIdentity {
        &self.identity
    }

    /// The nickname of the card's owner.
    pub fn nickname(&self) -> &Nickname {
        &self.nickname
    }

    /// The card bytes, version [`CARD_VERSION`].
    pub fn to_bytes(&self) -> Vec<u8> {
        let nickname = self.nickname.as_str().as_bytes();
        let mut bytes = Vec::with_capacity(FIXED_LENGTH + nickname.len());
        bytes.push(CARD_VERSION);
        bytes.extend_from_slice(self.identity.signing_key());
        bytes.extend_from_slice(self.identity.agreement_key());
        // A valid nickname is at most 64 bytes long: the cast cannot truncate.
        bytes.push(nickname.len() as u8);
        bytes.extend_from_slice(nickname);
        bytes
    }

    /// Reads card bytes. Rejects anything that [`ContactCard::to_bytes`]
    /// would not produce, trailing bytes included.
    pub fn from_bytes(bytes: &[u8]) -> Result<Self, ContactCardError> {
        let (&version, rest) = bytes.split_first().ok_or(ContactCardError::Malformed)?;
        if version != CARD_VERSION {
            return Err(ContactCardError::UnsupportedVersion(version));
        }
        let (signing, rest) = rest
            .split_first_chunk::<PUBLIC_KEY_LENGTH>()
            .ok_or(ContactCardError::Malformed)?;
        let (agreement, rest) = rest
            .split_first_chunk::<PUBLIC_KEY_LENGTH>()
            .ok_or(ContactCardError::Malformed)?;
        let (&length, nickname) = rest.split_first().ok_or(ContactCardError::Malformed)?;
        if nickname.len() != usize::from(length) {
            return Err(ContactCardError::Malformed);
        }

        let nickname = std::str::from_utf8(nickname).map_err(|_| ContactCardError::Malformed)?;
        let checked = Nickname::new(nickname)?;
        // A card holds the nickname as its owner validated it: already trimmed.
        if checked.as_str() != nickname {
            return Err(ContactCardError::InvalidNickname(
                NicknameError::ForbiddenCharacter,
            ));
        }

        Ok(Self {
            identity: PublicIdentity::from_bytes(signing, agreement)?,
            nickname: checked,
        })
    }

    /// The link to encode in a QR code.
    pub fn to_link(&self) -> String {
        format!(
            "{CARD_PREFIX}{}",
            Base64UrlUnpadded::encode_string(&self.to_bytes())
        )
    }

    /// Reads the text of a scanned QR code.
    pub fn from_link(link: &str) -> Result<Self, ContactCardError> {
        let encoded = link
            .strip_prefix(CARD_PREFIX)
            .ok_or(ContactCardError::NotAContactCard)?;
        let bytes =
            Base64UrlUnpadded::decode_vec(encoded).map_err(|_| ContactCardError::Malformed)?;
        Self::from_bytes(&bytes)
    }
}

impl fmt::Debug for ContactCard {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("ContactCard(..)")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::identity::{Identity, SECRET_LENGTH, SafetyNumber};

    fn card(byte: u8, nickname: &str) -> ContactCard {
        let identity = Identity::from_secret_bytes(&[byte; SECRET_LENGTH]);
        ContactCard::new(identity.public(), Nickname::new(nickname).unwrap())
    }

    #[test]
    fn nickname_is_trimmed() {
        assert_eq!(Nickname::new("  Lou \n").unwrap().as_str(), "Lou");
    }

    #[test]
    fn nickname_limits() {
        assert_eq!(Nickname::new(" \t "), Err(NicknameError::Empty));
        assert!(Nickname::new(&"a".repeat(NICKNAME_MAX_BYTES)).is_ok());
        assert_eq!(
            Nickname::new(&"a".repeat(NICKNAME_MAX_BYTES + 1)),
            Err(NicknameError::TooLong)
        );
        // 22 characters of 3 bytes each: 66 bytes.
        assert_eq!(Nickname::new(&"€".repeat(22)), Err(NicknameError::TooLong));
    }

    #[test]
    fn nickname_accepts_other_scripts() {
        for nickname in ["Zoé", "Ёжик", "Αλκηστις", "小明", "ليلى", "Lou 🌱"] {
            assert!(Nickname::new(nickname).is_ok(), "{nickname}");
        }
    }

    #[test]
    fn nickname_rejects_invisible_characters() {
        for nickname in [
            "Lo\u{200B}u",
            "\u{202E}uoL",
            "Lou\u{0007}",
            "L\nou",
            "Lo\u{FEFF}u",
        ] {
            assert_eq!(
                Nickname::new(nickname),
                Err(NicknameError::ForbiddenCharacter)
            );
        }
    }

    #[test]
    fn debug_hides_nickname() {
        let card = card(1, "Lou");
        assert_eq!(format!("{:?}", card.nickname()), "Nickname(..)");
        assert_eq!(format!("{card:?}"), "ContactCard(..)");
    }

    #[test]
    fn link_round_trip() {
        let card = card(1, "Zoé 🌱");
        let link = card.to_link();
        assert!(link.starts_with(CARD_PREFIX));
        assert_eq!(ContactCard::from_link(&link).unwrap(), card);
    }

    #[test]
    fn link_has_no_reserved_characters() {
        let link = card(1, "Lou").to_link();
        let fragment = &link[CARD_PREFIX.len()..];
        assert!(
            fragment
                .bytes()
                .all(|b| b.is_ascii_alphanumeric() || b == b'-' || b == b'_')
        );
    }

    #[test]
    fn other_links_are_rejected() {
        for link in [
            "",
            "https://samare.app/",
            "https://samare.app/g#AQ",
            "http://samare.app/c#AQ",
            "https://example.org/c#AQ",
        ] {
            assert!(matches!(
                ContactCard::from_link(link),
                Err(ContactCardError::NotAContactCard)
            ));
        }
    }

    #[test]
    fn newer_version_is_reported() {
        let mut bytes = card(1, "Lou").to_bytes();
        bytes[0] = 2;
        assert!(matches!(
            ContactCard::from_bytes(&bytes),
            Err(ContactCardError::UnsupportedVersion(2))
        ));
    }

    #[test]
    fn truncated_or_padded_cards_are_rejected() {
        let bytes = card(1, "Lou").to_bytes();
        for length in 0..bytes.len() {
            assert!(
                ContactCard::from_bytes(&bytes[..length]).is_err(),
                "length {length}"
            );
        }
        let mut longer = bytes.clone();
        longer.push(b'x');
        assert!(matches!(
            ContactCard::from_bytes(&longer),
            Err(ContactCardError::Malformed)
        ));
    }

    #[test]
    fn untrimmed_or_invalid_nickname_in_card_is_rejected() {
        for nickname in [&b" Lou"[..], b"Lou\0", b"\xFF\xFE"] {
            let mut bytes = card(1, "x").to_bytes();
            bytes.truncate(FIXED_LENGTH - 1);
            bytes.push(nickname.len() as u8);
            bytes.extend_from_slice(nickname);
            assert!(ContactCard::from_bytes(&bytes).is_err());
        }
    }

    #[test]
    fn bad_base64_is_rejected() {
        for fragment in ["", "!!!", "AQ=", "A"] {
            let link = format!("{CARD_PREFIX}{fragment}");
            assert!(ContactCard::from_link(&link).is_err(), "{fragment}");
        }
    }

    /// Fixed vector, checked against a separate Python implementation (the
    /// `cryptography` package and `hashlib`). Identity A has secret bytes all
    /// 0x01 and the nickname "Lou"; identity B has secret bytes all 0x02.
    const VECTOR_V1_LINK: &str = "https://samare.app/c#AYqI4910CfGV_VLbLTy6XXLKZwm_HZQSG_N0iAG0D29cpOCSkrZRwni5dyxWn1-puxPZBrRqtoyd-dwrRAn4ogkDTG91";
    const VECTOR_V1_SAFETY_NUMBER: &str =
        "811443699658359284883635253756878998060383714155338266869182";

    #[test]
    fn vector_v1() {
        let card = card(1, "Lou");
        assert_eq!(card.to_link(), VECTOR_V1_LINK);
        assert_eq!(ContactCard::from_link(VECTOR_V1_LINK).unwrap(), card);

        let other = Identity::from_secret_bytes(&[2; SECRET_LENGTH]).public();
        assert_eq!(
            SafetyNumber::new(card.identity(), &other).as_str(),
            VECTOR_V1_SAFETY_NUMBER
        );
    }
}
