//! The dual digest of PrismPM ADR-015 (SPEC.md §22.13): a `sha256` and a
//! `blake3` of one byte string, both lowercase hexadecimal of 64 characters.
//!
//! The pair exists because neither side can be dropped: consumers of an
//! in-toto DigestSet (cosign, Kyverno) check `sha256` and none of them checks
//! `blake3`, and the PrismPM side names its blobs by `blake3`. ADR-015 makes a
//! verifier require both keys and check both, which is what the reader here
//! does.

use crate::artifact::canonical_json::Json;
use crate::artifact::content_id::Sha256Digest;

/// A BLAKE3 digest (default hash mode, 256-bit output) with lowercase-hex
/// display.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Blake3Digest(pub [u8; 32]);

impl Blake3Digest {
    /// The 64-character lowercase hexadecimal form.
    #[must_use]
    pub fn to_hex(&self) -> String {
        hex(&self.0)
    }

    /// Parse 64 lowercase hex digits.
    pub fn from_hex(text: &str) -> Result<Self, String> {
        parse_hex(text).map(Self)
    }
}

fn hex(bytes: &[u8; 32]) -> String {
    let mut out = String::with_capacity(64);
    for byte in bytes {
        out.push(char::from_digit(u32::from(byte >> 4), 16).expect("nibble"));
        out.push(char::from_digit(u32::from(byte & 0xf), 16).expect("nibble"));
    }
    out
}

fn parse_hex(text: &str) -> Result<[u8; 32], String> {
    let bytes = text.as_bytes();
    if bytes.len() != 64 {
        return Err(format!("expected 64 hex digits, found {}", bytes.len()));
    }
    let nibble = |b: u8| -> Result<u8, String> {
        match b {
            b'0'..=b'9' => Ok(b - b'0'),
            b'a'..=b'f' => Ok(b - b'a' + 10),
            _ => Err("lowercase hexadecimal is mandatory".to_owned()),
        }
    };
    let mut out = [0u8; 32];
    for (index, pair) in bytes.chunks_exact(2).enumerate() {
        out[index] = (nibble(pair[0])? << 4) | nibble(pair[1])?;
    }
    Ok(out)
}

/// `{"blake3": H, "sha256": S}`, both digests over the same bytes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct DigestSet {
    /// SHA-256 of the bytes.
    pub sha256: Sha256Digest,
    /// BLAKE3 of the same bytes.
    pub blake3: Blake3Digest,
}

impl DigestSet {
    /// Both digests of a byte string.
    #[must_use]
    pub fn of(bytes: &[u8]) -> Self {
        let mut writer = DigestSetWriter::new();
        writer.update(bytes);
        writer.finish()
    }

    /// The label `blake3:<64 lowercase hex digits>`: for a content blob the
    /// `blake3` value is its label without the prefix (ADR-015, decision 6).
    #[must_use]
    pub fn label(&self) -> String {
        format!("blake3:{}", self.blake3.to_hex())
    }

    /// The canonical JSON object, members `blake3` and `sha256`.
    #[must_use]
    pub fn to_json(&self) -> Json {
        Json::object(vec![
            ("blake3", Json::Str(self.blake3.to_hex())),
            ("sha256", Json::Str(self.sha256.to_hex())),
        ])
    }

    /// Read a DigestSet. ADR-015 forbids any other member and any other
    /// spelling of a key (cosign accepted `BLAKE3`; a verifier must not), and
    /// requires both digests, so each of those is an error here.
    pub fn from_json(value: &Json) -> Result<Self, String> {
        let Json::Obj(members) = value else {
            return Err("a DigestSet is an object".to_owned());
        };
        if let Some(extra) = members
            .keys()
            .find(|key| !matches!(key.as_str(), "blake3" | "sha256"))
        {
            return Err(format!("a DigestSet has no member `{extra}`"));
        }
        let member = |name: &str| -> Result<&str, String> {
            match members.get(name) {
                Some(Json::Str(text)) => Ok(text),
                Some(_) => Err(format!("DigestSet member `{name}` is not a string")),
                None => Err(format!("DigestSet member `{name}` is missing")),
            }
        };
        Ok(Self {
            sha256: Sha256Digest::from_hex(member("sha256")?)?,
            blake3: Blake3Digest::from_hex(member("blake3")?)?,
        })
    }
}

/// Both digests of a byte string given in pieces.
pub struct DigestSetWriter {
    sha256: sha2::Sha256,
    blake3: blake3::Hasher,
}

impl Default for DigestSetWriter {
    fn default() -> Self {
        Self::new()
    }
}

impl DigestSetWriter {
    /// Start an empty byte string.
    #[must_use]
    pub fn new() -> Self {
        use sha2::Digest as _;
        Self {
            sha256: sha2::Sha256::new(),
            blake3: blake3::Hasher::new(),
        }
    }

    /// Append bytes.
    pub fn update(&mut self, bytes: &[u8]) {
        use sha2::Digest as _;
        self.sha256.update(bytes);
        self.blake3.update(bytes);
    }

    /// The digests of everything appended.
    #[must_use]
    pub fn finish(self) -> DigestSet {
        use sha2::Digest as _;
        DigestSet {
            sha256: Sha256Digest(self.sha256.finalize().into()),
            blake3: Blake3Digest(*self.blake3.finalize().as_bytes()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // BLAKE3 and SHA-256 of the empty string and of "abc": the known answers
    // ADR-015 cites for the PrismPM implementation.
    const EMPTY_BLAKE3: &str = "af1349b9f5f9a1a6a0404dea36dcc9499bcb25c9adc112b7cc9a93cae41f3262";
    const EMPTY_SHA256: &str = "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855";
    const ABC_BLAKE3: &str = "6437b3ac38465133ffb63b75273a8db548c558465d79db03fd359c6cd5bd9d85";
    const ABC_SHA256: &str = "ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad";

    #[test]
    fn known_answers_for_both_digests() {
        let empty = DigestSet::of(b"");
        assert_eq!(empty.blake3.to_hex(), EMPTY_BLAKE3);
        assert_eq!(empty.sha256.to_hex(), EMPTY_SHA256);
        let abc = DigestSet::of(b"abc");
        assert_eq!(abc.blake3.to_hex(), ABC_BLAKE3);
        assert_eq!(abc.sha256.to_hex(), ABC_SHA256);
        assert_eq!(abc.label(), format!("blake3:{ABC_BLAKE3}"));
    }

    #[test]
    fn pieces_equal_the_whole() {
        let mut writer = DigestSetWriter::new();
        writer.update(b"a");
        writer.update(b"bc");
        assert_eq!(writer.finish(), DigestSet::of(b"abc"));
    }

    #[test]
    fn json_round_trip_and_strict_reader() {
        let set = DigestSet::of(b"abc");
        assert_eq!(DigestSet::from_json(&set.to_json()), Ok(set));
        let text = set.to_json().to_canonical_string();
        assert!(text.starts_with("{\"blake3\":"), "{text}");

        let parse = |text: &str| DigestSet::from_json(&Json::parse(text.as_bytes()).unwrap());
        let both = format!("\"blake3\":\"{ABC_BLAKE3}\",\"sha256\":\"{ABC_SHA256}\"");
        assert!(parse(&format!("{{{both}}}")).is_ok());
        assert!(parse(&format!(
            "{{\"BLAKE3\":\"{ABC_BLAKE3}\",\"sha256\":\"{ABC_SHA256}\"}}"
        ))
        .is_err());
        assert!(parse(&format!("{{{both},\"sha512\":\"00\"}}")).is_err());
        assert!(parse(&format!("{{\"sha256\":\"{ABC_SHA256}\"}}")).is_err());
        assert!(parse(&format!("{{\"blake3\":\"{ABC_BLAKE3}\"}}")).is_err());
        let upper = ABC_BLAKE3.to_uppercase();
        assert!(parse(&format!(
            "{{\"blake3\":\"{upper}\",\"sha256\":\"{ABC_SHA256}\"}}"
        ))
        .is_err());
        assert!(parse(&format!(
            "{{\"blake3\":\"abc\",\"sha256\":\"{ABC_SHA256}\"}}"
        ))
        .is_err());
    }
}
