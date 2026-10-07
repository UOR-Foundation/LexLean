//! External time (SPEC.md §33.4).
//!
//! A timestamp is an RFC 3161 `TimeStampResp` in DER. What makes it evidence is
//! that the signer is a key the inventor does not control: verification anchors
//! the token's certificate under a root carried in the entry and checks that the
//! root was valid at the token's own `genTime`. An internal clock, however
//! precise, cannot satisfy that, which is why the timestamp step is not
//! optional for a claim of existence.
//!
//! Verification here is single-anchor path validation: the leaf certificate
//! must verify directly under the pinned root, both validity windows must
//! contain `genTime`, and the token's signature must verify under the leaf. It
//! is not full RFC 5280 path building, because the entry pins exactly one root
//! and a chain with intermediates would need those too; the claim this
//! specification makes is the one it can check completely.

use crate::artifact::canonical_json::Json;

use super::signature::{base64_decode_strict, base64_encode};

/// The parsed `TSTInfo` of an RFC 3161 token (RFC 3161 §2.4.1).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TstInfo {
    /// The `messageImprint.hashAlgorithm` OID, e.g. SHA-256.
    pub hash_algorithm: String,
    /// The `messageImprint.hashedMessage`: the digest of the timestamped
    /// artifact.
    pub hashed_message: Vec<u8>,
    /// The `genTime` as written, an RFC 3339 UTC instant.
    pub gen_time: String,
    /// The token's serial number.
    pub serial: Vec<u8>,
}

/// The external time anchor an entry carries (SPEC.md §33.4).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Timestamp {
    /// The authority that issued the token.
    pub tsa: String,
    /// The token's `genTime`, duplicated for a reader who will not parse DER.
    pub gen_time: String,
    /// Which artifact the token covers: `signature` per §33.6.
    pub artifact: String,
    /// The DER `TimeStampResp`, base64-encoded.
    pub tsr_der: Vec<u8>,
    /// The DER certificate of the issuing TSA, base64-encoded.
    pub tsa_certificate: Vec<u8>,
    /// The DER certificate of the pinned root, base64-encoded.
    pub tsa_root: Vec<u8>,
}

impl Timestamp {
    /// The canonical JSON of §33.2.
    #[must_use]
    pub fn to_json(&self) -> Json {
        Json::object(vec![
            ("tsa", Json::Str(self.tsa.clone())),
            ("gen_time", Json::Str(self.gen_time.clone())),
            ("artifact", Json::Str(self.artifact.clone())),
            ("tsr_der", Json::Str(base64_encode(&self.tsr_der))),
            (
                "tsa_certificate",
                Json::Str(base64_encode(&self.tsa_certificate)),
            ),
            ("tsa_root", Json::Str(base64_encode(&self.tsa_root))),
        ])
    }

    /// Parse the token's `TSTInfo` out of the stored DER.
    ///
    /// # Errors
    /// Returns [`LLG1005`](crate::code) when the response is not a well-formed
    /// `TimeStampResp`, the token does not carry `id-ct-TSTInfo`, or `genTime`
    /// is not an RFC 3339 UTC instant.
    pub fn tst_info(&self) -> Result<TstInfo, crate::error::LexLeanError> {
        self.parsed().map(|parsed| parsed.info)
    }

    /// Parse the whole token response.
    ///
    /// # Errors
    /// Returns [`LLG1005`](crate::code) naming the first malformed structure.
    pub fn parsed(&self) -> Result<ParsedResponse, crate::error::LexLeanError> {
        parse_time_stamp_resp(&self.tsr_der).map_err(timestamp_error)
    }

    /// The six checks of §33.4, over the artifact this entry says was
    /// timestamped.
    ///
    /// # Errors
    /// Returns [`LLG1005`](crate::code) when the token's imprint names another
    /// artifact and [`LLG1006`](crate::code) when the token or its chain does
    /// not verify.
    pub fn verify(&self, artifact: &[u8]) -> Result<(), crate::error::LexLeanError> {
        let parsed = self.parsed()?;
        verify_token(&parsed, artifact, &self.tsa_certificate, &self.tsa_root)
            .map_err(TokenFailure::into_error)
    }

    /// Decode a timestamp field from an entry's canonical JSON.
    ///
    /// # Errors
    /// Returns [`LLG1005`](crate::code) when the field is absent, is not an
    /// object, or carries DER that is not valid base64.
    pub fn from_json(value: &Json) -> Result<Self, crate::error::LexLeanError> {
        let Json::Obj(object) = value else {
            return Err(timestamp_error(
                "a timestamp is not a JSON object".to_owned(),
            ));
        };
        let text = |key: &str| -> Result<String, crate::error::LexLeanError> {
            match object.get(key) {
                Some(Json::Str(text)) => Ok(text.clone()),
                _ => Err(timestamp_error(format!(
                    "timestamp `{key}` is absent or not a string"
                ))),
            }
        };
        let der = |key: &str| -> Result<Vec<u8>, crate::error::LexLeanError> {
            decode_field(&text(key)?)
                .ok_or_else(|| timestamp_error(format!("timestamp `{key}` is not valid base64")))
        };
        Ok(Self {
            tsa: text("tsa")?,
            gen_time: text("gen_time")?,
            artifact: text("artifact")?,
            tsr_der: der("tsr_der")?,
            tsa_certificate: der("tsa_certificate")?,
            tsa_root: der("tsa_root")?,
        })
    }
}

/// A single-diagnostic token failure.
fn timestamp_error(reason: impl Into<String>) -> crate::error::LexLeanError {
    crate::error::LexLeanError::from_diagnostic(crate::diagnostic::Diagnostic::new(
        crate::code!("LLG1005"),
        reason,
    ))
}

/// A single-diagnostic token failure about the token's own contents.
pub(crate) fn malformed_error(reason: impl Into<String>) -> crate::error::LexLeanError {
    timestamp_error(reason)
}

/// A single-diagnostic chain failure, which §26 registers separately because a
/// well-formed token resting on an untrusted chain is a different fact from a
/// malformed one.
pub(crate) fn chain_error(reason: impl Into<String>) -> crate::error::LexLeanError {
    crate::error::LexLeanError::from_diagnostic(crate::diagnostic::Diagnostic::new(
        crate::code!("LLG1006"),
        reason,
    ))
}

/// A single-diagnostic provider failure: the question was never asked.
///
/// This is registered apart from [`chain_error`] because "the signature does
/// not verify" and "no verifier was available" are opposite findings. Reporting
/// a machine with no OpenSSL as an untrusted timestamp chain would make the
/// tool's own environment look like evidence against the inventor, which is the
/// one inference a provenance tool must never draw.
pub(crate) fn provider_error(reason: impl Into<String>) -> crate::error::LexLeanError {
    crate::error::LexLeanError::from_diagnostic(crate::diagnostic::Diagnostic::new(
        crate::code!("LLG1010"),
        reason,
    ))
}

/// Why one §33.4 verification produced no verdict.
///
/// [`Self::Invalid`] is a finding about the token: it is malformed, it names
/// another artifact, or it does not verify. [`Self::Provider`] is a finding
/// about this machine: the verifier was absent, would not start, or exceeded a
/// §25.5 limit, so nothing was established either way.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TokenFailure {
    /// The token is malformed or names another artifact. This is a fact about
    /// the token's own contents, which `LLG1005` registers and `LLG1006` does
    /// not, so the two are not folded together: a token whose imprint is over
    /// other bytes is not a token resting on an untrusted chain.
    Malformed(String),
    /// The token or its chain does not stand up.
    Invalid(String),
    /// The verification provider could not be reached.
    Provider(String),
}

impl TokenFailure {
    /// The registered error for this failure.
    #[must_use]
    pub fn into_error(self) -> crate::error::LexLeanError {
        match self {
            Self::Malformed(reason) => malformed_error(reason),
            Self::Invalid(reason) => chain_error(reason),
            Self::Provider(reason) => provider_error(reason),
        }
    }

    /// This finding with `what` prefixed to its reason, keeping its variant.
    ///
    /// Prefixing must not be allowed to blur the two cases: a reader told "the
    /// token's signature does not verify: no `openssl` executable is
    /// available" has been told two contradictory things, and the second is
    /// the true one.
    #[must_use]
    pub fn context(self, what: &str) -> Self {
        match self {
            Self::Malformed(reason) => Self::Malformed(format!("{what}: {reason}")),
            Self::Invalid(reason) => Self::Invalid(format!("{what}: {reason}")),
            Self::Provider(reason) => Self::Provider(format!("{what}: {reason}")),
        }
    }
}

impl From<super::tsa_crypto::Pkcs1Failure> for TokenFailure {
    fn from(failure: super::tsa_crypto::Pkcs1Failure) -> Self {
        match failure {
            super::tsa_crypto::Pkcs1Failure::Provider(reason) => Self::Provider(reason),
            super::tsa_crypto::Pkcs1Failure::Rejected(reason) => Self::Invalid(reason),
        }
    }
}

impl std::fmt::Display for TokenFailure {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Malformed(reason) | Self::Invalid(reason) | Self::Provider(reason) => {
                formatter.write_str(reason)
            }
        }
    }
}

/// A DER reader over one tag-length-value triple.
///
/// RFC 3161 and RFC 5280 are both DER, and the structures walked here are
/// fixed by those standards, so a length-prefixed reader with explicit bounds
/// checks is the whole requirement. Every read is checked against the
/// remaining length, so a truncated or over-long encoding is refused rather than
/// read past.
#[derive(Debug, Clone)]
struct Der<'a> {
    bytes: &'a [u8],
}

impl<'a> Der<'a> {
    fn new(bytes: &'a [u8]) -> Self {
        Self { bytes }
    }

    /// The next element: its tag, its content, and its total encoded length.
    fn element(&self) -> Result<(u8, &'a [u8], usize), String> {
        if self.bytes.len() < 2 {
            return Err("a DER element needs a tag and a length".to_owned());
        }
        let tag = self.bytes[0];
        let first = self.bytes[1];
        let (length, header) = if first < 0x80 {
            (usize::from(first), 2usize)
        } else {
            let count = usize::from(first & 0x7f);
            if count == 0 || count > 4 {
                return Err("a DER length is indefinite or absurdly large".to_owned());
            }
            if self.bytes.len() < 2 + count {
                return Err("a DER length runs past the buffer".to_owned());
            }
            let mut length = 0usize;
            for byte in &self.bytes[2..2 + count] {
                length = (length << 8) | usize::from(*byte);
            }
            if length < 0x80 {
                return Err("a DER long form length below 128 is not canonical".to_owned());
            }
            (length, 2 + count)
        };
        let end = header.checked_add(length).ok_or("a DER length overflows")?;
        if end > self.bytes.len() {
            return Err("a DER element runs past the buffer".to_owned());
        }
        Ok((tag, &self.bytes[header..end], end))
    }

    /// The children of a constructed element.
    ///
    /// Each child is pushed as its whole encoding rather than its content,
    /// because every accessor here --- `oid`, `octets`, `integer`, `context`,
    /// `encoded` --- begins by reading a tag and a length, and a content slice
    /// has neither.
    fn children(&self) -> Result<Vec<Der<'a>>, String> {
        let (_, content, _) = self.element()?;
        let mut out = Vec::new();
        let mut rest = content;
        while !rest.is_empty() {
            let (_, _, total) = Der::new(rest).element()?;
            out.push(Der::new(&rest[..total]));
            rest = &rest[total..];
        }
        Ok(out)
    }

    /// This element's whole encoding, tag and length included.
    fn encoded(&self) -> Result<&'a [u8], String> {
        let (_, _, total) = self.element()?;
        Ok(&self.bytes[..total])
    }

    /// A context-specific constructed element's content, `[n] EXPLICIT`.
    /// A context-specific constructed element, `[n] EXPLICIT`.
    ///
    /// The result wraps the whole `[n]` encoding rather than its payload. Every
    /// `Der` in this module wraps a whole encoding --- that is what `element`,
    /// `encoded`, and every accessor read --- so returning the payload here
    /// would hand back a slice with no tag of its own, and the caller's next
    /// `children` would descend past the element it asked about.
    fn context(&self, tag: u8) -> Result<Der<'a>, String> {
        let (actual, _, _) = self.element()?;
        if actual != tag {
            return Err(format!("expected DER tag {tag:#04x}, found {actual:#04x}"));
        }
        Ok(Der::new(self.encoded()?))
    }

    /// A context-specific constructed element's payload, `[n] IMPLICIT`.
    fn implicit(&self, tag: u8) -> Result<Der<'a>, String> {
        self.context(tag)
    }

    /// The OID of an OBJECT IDENTIFIER element, dotted decimal.
    fn oid(&self) -> Result<String, String> {
        let (tag, content, _) = self.element()?;
        if tag != 0x06 {
            return Err(format!(
                "expected an OBJECT IDENTIFIER, found tag {tag:#04x}"
            ));
        }
        if content.is_empty() {
            return Err("an empty OBJECT IDENTIFIER".to_owned());
        }
        let mut parts = Vec::new();
        let first = content[0];
        parts.push(u64::from(first / 40));
        parts.push(u64::from(first % 40));
        let mut value = 0u64;
        for byte in &content[1..] {
            value = (value << 7) | u64::from(byte & 0x7f);
            if byte & 0x80 == 0 {
                parts.push(value);
                value = 0;
            }
        }
        Ok(parts
            .iter()
            .map(u64::to_string)
            .collect::<Vec<String>>()
            .join("."))
    }

    /// The content octets of an OCTET STRING.
    fn octets(&self) -> Result<Vec<u8>, String> {
        let (tag, content, _) = self.element()?;
        if tag != 0x04 {
            return Err(format!("expected an OCTET STRING, found tag {tag:#04x}"));
        }
        Ok(content.to_vec())
    }

    /// The content of an INTEGER, as big-endian magnitude with no sign bit
    /// handling: RFC 3161 serial numbers are non-negative.
    fn integer(&self) -> Result<Vec<u8>, String> {
        let (tag, content, _) = self.element()?;
        if tag != 0x02 {
            return Err(format!("expected an INTEGER, found tag {tag:#04x}"));
        }
        if content.is_empty() {
            return Err("an empty INTEGER".to_owned());
        }
        Ok(content
            .iter()
            .copied()
            .skip_while(|byte| *byte == 0)
            .collect())
    }
}

/// `id-ct-TSTInfo`, the content type a timestamp token must carry (RFC 3161 §2.4.1).
const ID_CT_TST_INFO: &str = "1.2.840.113549.1.9.16.1.4";
/// The `messageDigest` signed attribute, PKCS#9.
const ID_MESSAGE_DIGEST: &str = "1.2.840.113549.1.9.4";

/// `id-signedData`, the CMS content type of the token (RFC 5652 §3).
const ID_SIGNED_DATA: &str = "1.2.840.113549.1.7.2";

/// The parsed parts of a `TimeStampResp` that §33.4 checks.
#[derive(Debug, Clone)]
pub struct ParsedResponse {
    /// The `TSTInfo` of the token.
    pub info: TstInfo,
    /// The `SignedData.signerInfos` element.
    pub signer_info_der: Vec<u8>,
    /// The `encapContentInfo` content octets, that is the `TSTInfo` DER. This
    /// is the content the token carries, and the thing the message imprint is
    /// over, whether or not the signature covers it directly.
    pub signed_content: Vec<u8>,
    /// The exact bytes the `SignerInfo` signature is computed over: the
    /// retagged `signedAttrs` when the `SignerInfo` carries them, and
    /// [`Self::signed_content`] when it does not.
    pub signed_message: Vec<u8>,
    /// The digest algorithm named by the `SignerInfo`, dotted decimal.
    pub signature_digest_algorithm: String,
    /// The signature algorithm named by the `SignerInfo`, dotted decimal.
    pub signature_algorithm: String,
    /// The retagged `SET OF` `signedAttrs` the signature covers, when the
    /// `SignerInfo` carries any.
    pub signed_attributes: Option<Vec<u8>>,
    /// The signature octets.
    pub signature: Vec<u8>,
    /// The `Certificate` elements carried in `SignedData.certificates`.
    pub certificates: Vec<Vec<u8>>,
}

/// The owned pieces of a `SignedData` this module reads.
struct TokenParts {
    /// The exact bytes the `SignerInfo` signature covers: the `TSTInfo` DER, or
    /// the `SET OF` `signedAttrs` when the `SignerInfo` carries any.
    signed_content: Vec<u8>,
    /// The bytes the signature is actually computed over, which are the
    /// retagged `signedAttrs` when present and `signed_content` otherwise.
    signed_message: Vec<u8>,
    /// The `SignerInfo` DER, kept so a caller can report what it checked.
    signer_info_der: Vec<u8>,
    /// The digest algorithm the `SignerInfo` names, dotted decimal.
    signature_digest_algorithm: String,
    /// The signature algorithm the `SignerInfo` names, dotted decimal. This,
    /// not the digest algorithm, is what says whether the signature is RSA or
    /// ECDSA and under which digest.
    signature_algorithm: String,
    /// The retagged `SET OF` `signedAttrs` the signature covers, when present.
    signed_attributes: Option<Vec<u8>>,
    /// The signature octets.
    signature: Vec<u8>,
    /// The `Certificate` elements carried in `SignedData.certificates`.
    certificates: Vec<Vec<u8>>,
}

/// Parse a DER `TimeStampResp` down to the parts §33.4 verifies.
///
/// # Errors
/// Returns the reason when the response is not a `SEQUENCE`, the status is not
/// granted, the token is not `id-signedData`, its content is not
/// `id-ct-TSTInfo`, or `genTime` is not an RFC 3339 UTC instant.
pub fn parse_time_stamp_resp(der: &[u8]) -> Result<ParsedResponse, String> {
    let response = Der::new(der);
    let (tag, _, _) = response.element()?;
    if tag != 0x30 {
        return Err("a TimeStampResp is a SEQUENCE".to_owned());
    }
    let parts = response.children()?;
    let status = parts
        .first()
        .ok_or("a TimeStampResp has a status")?
        .children()?
        .first()
        .ok_or("a PKIStatusInfo has a status value")?
        .integer()?;
    // PKIStatus granted(0) encodes as `02 01 00`; the magnitude above drops the
    // leading zero byte, so an empty magnitude is the granted status and any
    // other magnitude is a refusal the verifier must not paper over.
    if !status.is_empty() {
        return Err(format!(
            "the time stamp authority refused to grant the token (status {})",
            status
                .iter()
                .map(u8::to_string)
                .collect::<Vec<String>>()
                .join(".")
        ));
    }
    let content_info = parts
        .get(1)
        .ok_or("a granted TimeStampResp carries a token")?;
    let token = parse_signed_data(content_info)?;
    let info = parse_tst_info(&token.signed_content)?;
    if !is_rfc3339_utc(&info.gen_time) {
        return Err(format!(
            "`{}` is not an RFC 3339 UTC instant",
            info.gen_time
        ));
    }
    Ok(ParsedResponse {
        info,
        signer_info_der: token.signer_info_der,
        signed_content: token.signed_content,
        signed_message: token.signed_message,
        signature_digest_algorithm: token.signature_digest_algorithm,
        signature_algorithm: token.signature_algorithm,
        signed_attributes: token.signed_attributes,
        signature: token.signature,
        certificates: token.certificates,
    })
}

/// Walk a CMS `SignedData` as RFC 3161 §2.4.1 lays it out.
fn parse_signed_data(content_info: &Der<'_>) -> Result<TokenParts, String> {
    let (tag, _, _) = content_info.element()?;
    if tag != 0x30 {
        return Err("a ContentInfo is a SEQUENCE".to_owned());
    }
    let parts = content_info.children()?;
    if parts.first().map(Der::oid).transpose()? != Some(ID_SIGNED_DATA.to_owned()) {
        return Err("a time stamp token is a CMS SignedData".to_owned());
    }
    let explicit = parts
        .get(1)
        .ok_or("a ContentInfo carries content")?
        .context(0xa0)?;
    let signed_data = explicit
        .children()?
        .into_iter()
        .next()
        .ok_or("the explicit content is a SignedData")?;
    let fields = signed_data.children()?;

    let encap = fields
        .get(2)
        .ok_or("a SignedData has an encapContentInfo")?;
    let encap_parts = encap.children()?;
    let econtent_type = encap_parts
        .first()
        .ok_or("an encapContentInfo names its content type")?
        .oid()?;
    if econtent_type != ID_CT_TST_INFO {
        return Err(format!(
            "a time stamp token's content is {ID_CT_TST_INFO}, found {econtent_type}"
        ));
    }
    let signed_content = encap_parts
        .get(1)
        .ok_or("an encapContentInfo carries eContent")?
        .context(0xa0)?
        .children()?
        .into_iter()
        .next()
        .ok_or("the eContent is an OCTET STRING")?
        .octets()?;

    // `SignedData` is `version, digestAlgorithms, encapContentInfo,
    // certificates [0] IMPLICIT OPTIONAL, crls [1] IMPLICIT OPTIONAL,
    // signerInfos SET` (RFC 5652 §5.1). The two optional fields move the
    // position of `signerInfos`: a token that carries no certificate at all ---
    // which is what a TSA returns when the request does not ask for one --- has
    // `signerInfos` at index 3 rather than 4, and reading index 4 there would
    // consume the next element of a shorter list. The fields are therefore
    // selected by their tags, which is the only reading that survives both
    // shapes, and `certificates` is absent rather than empty when no field
    // carries it.
    let certificates = match fields
        .iter()
        .skip(3)
        .find(|field| field.element().is_ok_and(|(tag, _, _)| tag == 0xa0))
    {
        Some(implicit) => implicit
            .implicit(0xa0)
            .and_then(|inner| inner.children())?
            .into_iter()
            .filter_map(|certificate| certificate.encoded().ok().map(<[u8]>::to_vec))
            .collect(),
        None => Vec::new(),
    };

    let signer_info = fields
        .iter()
        .skip(3)
        .find(|field| field.element().is_ok_and(|(tag, _, _)| tag == 0x31))
        .ok_or("a SignedData has signerInfos")?
        .children()?
        .into_iter()
        .next()
        .ok_or("a time stamp token has exactly one SignerInfo")?;
    let signer_fields = signer_info.children()?;
    // CMS `SignerInfo` is `version, sid, digestAlgorithm, signedAttrs?,
    // signatureAlgorithm, signature, unsignedAttrs?`, so the digest algorithm
    // is the third field and not the first: the first is an INTEGER, and
    // reading children of an INTEGER is not a parse but a read past it. The
    // signature is the last field unless `unsignedAttrs` follows it.
    let signature_digest_algorithm = signer_fields
        .get(2)
        .ok_or("a SignerInfo names its digest algorithm")?
        .children()?
        .first()
        .ok_or("an AlgorithmIdentifier names an algorithm")?
        .oid()?;
    let signature_index = if signer_fields.len() >= 2
        && signer_fields[signer_fields.len() - 1]
            .element()
            .is_ok_and(|(tag, _, _)| tag == 0xa1)
    {
        signer_fields.len() - 2
    } else {
        signer_fields.len().saturating_sub(1)
    };
    let signature_algorithm = signer_fields
        .get(
            signature_index
                .checked_sub(1)
                .ok_or("a SignerInfo names a signature algorithm")?,
        )
        .ok_or("a SignerInfo names a signature algorithm")?
        .children()?
        .first()
        .ok_or("an AlgorithmIdentifier names an algorithm")?
        .oid()?;
    let signature = signer_fields
        .get(signature_index)
        .ok_or("a SignerInfo carries a signature")?
        .octets()?;
    // When `signedAttrs` is present it, not the content, is what the signature
    // covers: CMS signs the DER `SET OF Attribute`, while the token carries the
    // attributes context-tagged `[0] IMPLICIT`. Verifying over the content in
    // that case would report a genuine token as forged, and skipping the
    // retagging would report it as forged in the other direction.
    let signed_attributes = signer_fields
        .get(3)
        .filter(|field| field.element().is_ok_and(|(tag, _, _)| tag == 0xa0))
        .map(|field| {
            let mut der = field.encoded()?.to_vec();
            der[0] = 0x31;
            Ok::<Vec<u8>, String>(der)
        })
        .transpose()?;
    let signed_message = signed_attributes
        .clone()
        .unwrap_or_else(|| signed_content.clone());
    Ok(TokenParts {
        signed_content,
        signed_message,
        signer_info_der: signer_info.encoded()?.to_vec(),
        signature_digest_algorithm,
        signature_algorithm,
        signed_attributes,
        signature,
        certificates,
    })
}

/// The digest the `messageDigest` signed attribute commits to.
///
/// An `Attribute` is `SEQUENCE { attrType OID, attrValues SET OF ANY }`, so the
/// value is the first element of the second field. The attributes are searched
/// by OID rather than by position because CMS orders them by DER, not by any
/// order a signer finds convenient.
fn message_digest(signed_attributes: &[u8]) -> Option<Vec<u8>> {
    for attribute in Der::new(signed_attributes).children().ok()? {
        let fields = attribute.children().ok()?;
        if fields
            .first()
            .and_then(|field| field.oid().ok())
            .is_none_or(|oid| oid != ID_MESSAGE_DIGEST)
        {
            continue;
        }
        return fields
            .get(1)?
            .children()
            .ok()?
            .into_iter()
            .next()?
            .octets()
            .ok();
    }
    None
}

/// Parse a `TSTInfo` (RFC 3161 §2.4.1).
fn parse_tst_info(der: &[u8]) -> Result<TstInfo, String> {
    let (tag, _, _) = Der::new(der).element()?;
    if tag != 0x30 {
        return Err("a TSTInfo is a SEQUENCE".to_owned());
    }
    let fields = Der::new(der).children()?;
    let mut index = 0usize;
    // version
    index += 1;
    let policy = fields.get(index).ok_or("a TSTInfo names a policy")?.oid()?;
    if policy.is_empty() {
        return Err("a TSTInfo has an empty policy".to_owned());
    }
    index += 1;
    let imprint_parts = fields
        .get(index)
        .ok_or("a TSTInfo carries a messageImprint")?
        .children()?;
    let hash_algorithm = imprint_parts
        .first()
        .ok_or("a messageImprint names its algorithm")?
        .children()?
        .first()
        .ok_or("an AlgorithmIdentifier names an algorithm")?
        .oid()?;
    let hashed_message = imprint_parts
        .get(1)
        .ok_or("a messageImprint carries a hashed message")?
        .octets()?;
    index += 1;
    let serial = fields
        .get(index)
        .ok_or("a TSTInfo carries a serial number")?
        .integer()?;
    index += 1;
    let gen_time = fields
        .get(index)
        .ok_or("a TSTInfo carries a genTime")?
        .generalized_time()?;
    Ok(TstInfo {
        hash_algorithm,
        hashed_message,
        gen_time,
        serial,
    })
}

impl Der<'_> {
    /// The text of a `GeneralizedTime`, which RFC 3161 requires to be UTC with
    /// a `Z` suffix and no offset.
    fn generalized_time(&self) -> Result<String, String> {
        let (tag, content, _) = self.element()?;
        if tag != 0x18 {
            return Err(format!(
                "a genTime is a GeneralizedTime, found tag {tag:#04x}"
            ));
        }
        let text = std::str::from_utf8(content).map_err(|error| error.to_string())?;
        if !text.ends_with('Z') {
            return Err(format!("a genTime must be UTC with a Z suffix: {text}"));
        }
        Ok(text.to_owned())
    }
}

/// Whether a string is an RFC 3339 UTC instant of the form RFC 3161 requires:
/// `YYYYMMDDHHMMSS[.fraction]Z`.
///
/// The shape is checked rather than the calendar: the calendar of a token's
/// time is the authority's claim, and the only thing a verifier can check
/// without a clock is that the claim is well formed and inside the root's
/// validity window.
#[must_use]
pub fn is_rfc3339_utc(text: &str) -> bool {
    let Some(body) = text.strip_suffix('Z') else {
        return false;
    };
    let (whole, fraction) = match body.split_once('.') {
        Some((whole, fraction)) => (whole, Some(fraction)),
        None => (body, None),
    };
    if whole.len() != 14 || !whole.bytes().all(|byte| byte.is_ascii_digit()) {
        return false;
    }
    if let Some(fraction) = fraction {
        if fraction.is_empty() || !fraction.bytes().all(|byte| byte.is_ascii_digit()) {
            return false;
        }
    }
    let number = |slice: &str| slice.parse::<u32>().unwrap_or(0);
    let month = number(&whole[4..6]);
    let day = number(&whole[6..8]);
    let hour = number(&whole[8..10]);
    let minute = number(&whole[10..12]);
    let second = number(&whole[12..14]);
    (1..=12).contains(&month) && (1..=31).contains(&day) && hour < 24 && minute < 60 && second < 60
}

/// The digest of `bytes` under the OID `algorithm`, or `None` when the OID is
/// not a digest this specification admits.
#[must_use]
pub fn digest_for_algorithm(algorithm: &str, bytes: &[u8]) -> Option<Vec<u8>> {
    use sha2::Digest;
    match algorithm {
        "1.3.14.3.2.26" => Some(sha1::Sha1::digest(bytes).to_vec()),
        "2.16.840.1.101.3.4.2.1" => Some(sha2::Sha256::digest(bytes).to_vec()),
        "2.16.840.1.101.3.4.2.2" => Some(sha2::Sha384::digest(bytes).to_vec()),
        "2.16.840.1.101.3.4.2.3" => Some(sha2::Sha512::digest(bytes).to_vec()),
        _ => None,
    }
}

/// The OID of the digest this specification names for a new request.
#[must_use]
pub const fn preferred_digest_oid() -> &'static str {
    "2.16.840.1.101.3.4.2.1"
}

/// Build a DER `TimeStampReq` for a timestamp request over `digest`.
///
/// A request is produced rather than issued: §33.4 is verified offline against
/// a token the authority returns, and the request is the artifact a third party
/// hashes to check that a token answers the question that was asked.
///
/// # Errors
/// Returns the reason when the digest length does not match the algorithm.
pub fn build_timestamp_request(digest: &[u8], algorithm: &str) -> Result<Vec<u8>, String> {
    let expected = match algorithm {
        "1.3.14.3.2.26" => 20,
        "2.16.840.1.101.3.4.2.1" => 32,
        "2.16.840.1.101.3.4.2.2" => 48,
        "2.16.840.1.101.3.4.2.3" => 64,
        other => return Err(format!("{other} is not an admitted digest algorithm")),
    };
    if digest.len() != expected {
        return Err(format!(
            "a {algorithm} imprint is {expected} bytes, found {}",
            digest.len()
        ));
    }
    let algorithm_identifier = sequence(&[&oid(algorithm)?]);
    let imprint = sequence(&[&algorithm_identifier, &octet_string(digest)]);
    // `TimeStampReq ::= SEQUENCE { version, messageImprint, ... }` (RFC 3161
    // §2.4.1). The request is that sequence and nothing else: an earlier version
    // additionally wrapped it in a context tag `[0]`, which is not the type the
    // authority parses, and FreeTSA rejected every such request as a malformed
    // one rather than answering it. The length-bearing `wrap` is used so a
    // request longer than 127 bytes is encoded with a long-form length.
    Ok(sequence(&[&version_one(), &imprint]))
}

/// A DER `INTEGER 1`.
fn version_one() -> Vec<u8> {
    vec![0x02, 0x01, 0x01]
}

/// A DER `SEQUENCE` over already-encoded elements.
fn sequence(parts: &[&[u8]]) -> Vec<u8> {
    let mut content = Vec::new();
    for part in parts {
        content.extend_from_slice(part);
    }
    wrap(0x30, &content)
}

/// A DER `OBJECT IDENTIFIER` from its dotted-decimal form.
fn oid(dotted: &str) -> Result<Vec<u8>, String> {
    let parts: Vec<u64> = dotted
        .split('.')
        .map(|part| part.parse::<u64>().map_err(|error| error.to_string()))
        .collect::<Result<Vec<u64>, _>>()?;
    if parts.len() < 2 || parts[0] > 2 || parts[1] > 39 && parts[0] < 2 {
        return Err(format!("{dotted} is not a valid OID"));
    }
    let mut out = vec![u8::try_from(parts[0] * 40 + parts[1]).unwrap_or(0)];
    for arc in &parts[2..] {
        let mut stack = Vec::new();
        let mut value = *arc;
        loop {
            stack.push(u8::try_from(value & 0x7f).unwrap_or(0));
            value >>= 7;
            if value == 0 {
                break;
            }
        }
        for (index, byte) in stack.iter().rev().enumerate() {
            let last = index + 1 == stack.len();
            out.push(if last { *byte } else { byte | 0x80 });
        }
    }
    Ok(wrap(0x06, &out))
}

/// A DER `OCTET STRING`.
fn octet_string(bytes: &[u8]) -> Vec<u8> {
    wrap(0x04, bytes)
}

/// A DER tag-length-value wrapper.
fn wrap(tag: u8, content: &[u8]) -> Vec<u8> {
    let mut out = vec![tag];
    let length = content.len();
    if length < 0x80 {
        out.push(u8::try_from(length).unwrap_or(0));
    } else if length < 0x100 {
        out.push(0x81);
        out.push(u8::try_from(length).unwrap_or(0));
    } else if length < 0x1_0000 {
        out.push(0x82);
        out.extend_from_slice(&(u16::try_from(length).unwrap_or(0)).to_be_bytes());
    } else {
        out.push(0x83);
        out.extend_from_slice(&(u32::try_from(length).unwrap_or(0)).to_be_bytes()[1..]);
    }
    out.extend_from_slice(content);
    out
}

/// Decode a base64 field of an entry, refusing anything outside the alphabet.
#[must_use]
pub fn decode_field(text: &str) -> Option<Vec<u8>> {
    base64_decode_strict(text)
}
/// The public key and validity window of one certificate (RFC 5280 §4.1).
#[derive(Debug, Clone)]
pub struct CertificateFacts {
    /// The `SubjectPublicKeyInfo` DER, from which the verification key is taken.
    pub subject_public_key_info: Vec<u8>,
    /// The `notBefore` instant, as RFC 5280 writes it.
    pub not_before: String,
    /// The `notAfter` instant.
    pub not_after: String,
    /// The signature algorithm OID of the certificate's own signature.
    pub signature_algorithm: String,
    /// The certificate's signature bits.
    pub signature: Vec<u8>,
    /// The DER of the TBS certificate the signature covers.
    pub tbs_der: Vec<u8>,
}

/// Read the facts §33.4 needs from a DER certificate.
///
/// # Errors
/// Returns the reason when the certificate is not an X.509 `Certificate`, or
/// when it uses a validity encoding this reader does not accept.
pub fn parse_certificate(der: &[u8]) -> Result<CertificateFacts, String> {
    let certificate = Der::new(der);
    let (tag, _, _) = certificate.element()?;
    if tag != 0x30 {
        return Err("an X.509 Certificate is a SEQUENCE".to_owned());
    }
    let fields = certificate.children()?;
    let tbs = fields.first().ok_or("a Certificate has a tbsCertificate")?;
    let mut tbs_fields = tbs.children()?;
    // `tbsCertificate` opens with an optional EXPLICIT `[0] version`. A v3
    // certificate has it and a v1 certificate does not, so the fields after it
    // shift by one. Indexing without dropping it reads `issuer` as `validity`
    // and `subject` as `subjectPublicKeyInfo`, and every date read off the
    // result is a field that is not a date.
    if tbs_fields
        .first()
        .is_some_and(|field| field.element().is_ok_and(|(tag, _, _)| tag == 0xa0))
    {
        tbs_fields.remove(0);
    }
    // serialNumber, signature, issuer, validity, subject, subjectPublicKeyInfo
    let validity = tbs_fields
        .get(3)
        .ok_or("a tbsCertificate has a validity")?
        .children()?;
    let not_before = validity.first().ok_or("a validity has notBefore")?.time()?;
    let not_after = validity.get(1).ok_or("a validity has notAfter")?.time()?;
    let spki = tbs_fields
        .get(5)
        .ok_or("a tbsCertificate has a subjectPublicKeyInfo")?
        .encoded()?
        .to_vec();
    let signature_algorithm = fields
        .get(1)
        .ok_or("a Certificate names its signature algorithm")?
        .children()?
        .first()
        .ok_or("an AlgorithmIdentifier names an algorithm")?
        .oid()?;
    let signature = fields
        .get(2)
        .ok_or("a Certificate carries a signature")?
        .bit_string()?;
    Ok(CertificateFacts {
        subject_public_key_info: spki,
        not_before,
        not_after,
        signature_algorithm,
        signature,
        tbs_der: tbs.encoded()?.to_vec(),
    })
}

impl Der<'_> {
    /// The text of a `UTCTime` or `GeneralizedTime`, whichever the certificate
    /// uses; both are normalized to the fourteen-digit UTC form RFC 3161 writes
    /// so that a comparison against `genTime` is a string comparison of one
    /// shape.
    fn time(&self) -> Result<String, String> {
        let (tag, content, _) = self.element()?;
        let text = std::str::from_utf8(content).map_err(|error| error.to_string())?;
        let normalized = match tag {
            0x17 => {
                // UTCTime: YYMMDDHHMMSSZ, with the century resolved by RFC 5280.
                if text.len() != 13 {
                    return Err(format!("a UTCTime is 13 characters: {text}"));
                }
                let year: u32 = text[0..2]
                    .parse::<u32>()
                    .map_err(|error: std::num::ParseIntError| error.to_string())?;
                let century = if year >= 50 { 1900 } else { 2000 };
                format!("{}{}", century + year, &text[2..])
            }
            0x18 => text.to_owned(),
            other => {
                return Err(format!(
                    "a certificate time is a UTCTime or GeneralizedTime, found tag {other:#04x}"
                ))
            }
        };
        // Both encodings end in `Z`, and the UTCTime branch has just prefixed a
        // century, so the terminator is stripped once here rather than being
        // expected to be absent from one branch and present in the other.
        let digits = normalized.trim_end_matches('Z');
        if digits.len() != 14 || !digits.bytes().all(|byte| byte.is_ascii_digit()) {
            return Err(format!("`{normalized}` is not a certificate time"));
        }
        Ok(format!("{digits}Z"))
    }

    /// The payload bits of a `BIT STRING`, with the unused-bit count removed.
    fn bit_string(&self) -> Result<Vec<u8>, String> {
        let (tag, content, _) = self.element()?;
        if tag != 0x03 {
            return Err(format!("expected a BIT STRING, found tag {tag:#04x}"));
        }
        let Some((unused, rest)) = content.split_first() else {
            return Err("an empty BIT STRING".to_owned());
        };
        if *unused > 7 {
            return Err("a BIT STRING declares more than seven unused bits".to_owned());
        }
        Ok(rest[..rest.len() - usize::from(*unused) / 8].to_vec())
    }
}

/// Whether `instant` falls inside the inclusive window `[not_before, not_after]`.
///
/// Both bounds are compared as the normalized fourteen-digit form, which is why
/// [`CertificateFacts::not_before`] is produced by [`Der::time`] and not read
/// raw.
#[must_use]
pub fn within_validity(facts: &CertificateFacts, instant: &str) -> bool {
    let Some(normalized) = instant.strip_suffix('Z').map(|body| body.get(..14)) else {
        return false;
    };
    match normalized {
        Some(body) => body >= facts.not_before.as_str() && body <= facts.not_after.as_str(),
        None => false,
    }
}

/// Verify an RSA PKCS#1 v1.5 signature under a DER `SubjectPublicKeyInfo`.
///
/// The verification itself is [`super::tsa_crypto::verify_pkcs1v15`], which
/// delegates to OpenSSL; see that module for why the crate carries no RSA
/// dependency of its own. This function owns only the choice of what to accept,
/// namely a certificate whose key is RSA and whose signature algorithm is a
/// PKCS#1 v1.5 SHA-2 or SHA-1 digest. Ed25519-issued timestamps, which no
/// public TSA produces, are refused rather than approximated.
///
/// # Errors
/// Returns the reason when the key is not RSA, the algorithm is not one this
/// specification admits, or the signature does not verify. A provider that
/// could not be reached is reported apart from a signature that does not
/// verify: the first leaves the timestamp unestablished, the second disproves
/// it, and a verifier that cannot tell them apart would let a machine missing
/// OpenSSL read as evidence of a forged token.
pub fn verify_rsa_signature(
    subject_public_key_info: &[u8],
    algorithm: &str,
    message: &[u8],
    signature: &[u8],
) -> Result<(), super::tsa_crypto::Pkcs1Failure> {
    super::tsa_crypto::verify_signature(subject_public_key_info, algorithm, message, signature)
}

/// Verify one timestamp token against the certificate and root the entry
/// carries, the six checks of §33.4.
///
/// # Errors
/// Returns the reason a check failed, naming which one, so a reader of the
/// verdict can see whether the token is malformed, names another artifact, or
/// rests on a chain they do not trust.
pub fn verify_token(
    parsed: &ParsedResponse,
    artifact: &[u8],
    leaf_certificate: &[u8],
    root_certificate: &[u8],
) -> Result<(), TokenFailure> {
    let expected =
        digest_for_algorithm(&parsed.info.hash_algorithm, artifact).ok_or_else(|| {
            TokenFailure::Invalid(format!(
                "`{}` is not an admitted digest",
                parsed.info.hash_algorithm
            ))
        })?;
    if expected != parsed.info.hashed_message {
        return Err(TokenFailure::Malformed(format!(
            "the message imprint is over {} bytes, not the {} bytes of the {} artifact",
            parsed.info.hashed_message.len(),
            artifact.len(),
            super::TIMESTAMP_ARTIFACT
        )));
    }

    // When the `SignerInfo` signs its attributes rather than the content, the
    // attributes are what carry the binding to this `TSTInfo`, and that binding
    // has to be checked here. Without it the `signedAttrs` of any genuine token
    // would verify under the TSA key while the `TSTInfo` beside it was free to
    // be any `TSTInfo` at all, so the signature would prove nothing about the
    // instant being reported.
    if let Some(attributes) = &parsed.signed_attributes {
        let bound =
            digest_for_algorithm(&parsed.signature_digest_algorithm, &parsed.signed_content)
                .ok_or_else(|| {
                    TokenFailure::Invalid(format!(
                        "`{}` is not an admitted digest",
                        parsed.signature_digest_algorithm
                    ))
                })?;
        if message_digest(attributes).ok_or_else(|| {
            TokenFailure::Malformed("the signed attributes carry no messageDigest".to_owned())
        })? != bound
        {
            return Err(TokenFailure::Malformed(
                "the signed attributes bind a digest of other content than this TSTInfo".to_owned(),
            ));
        }
    }

    let leaf = parse_certificate(leaf_certificate).map_err(TokenFailure::Invalid)?;
    if !within_validity(&leaf, &parsed.info.gen_time) {
        return Err(TokenFailure::Invalid(format!(
            "the TSA certificate is valid from {} to {}, which does not contain {}",
            leaf.not_before, leaf.not_after, parsed.info.gen_time
        )));
    }
    let root = parse_certificate(root_certificate).map_err(TokenFailure::Invalid)?;
    if !within_validity(&root, &parsed.info.gen_time) {
        return Err(TokenFailure::Invalid(format!(
            "the pinned root is valid from {} to {}, which does not contain {}",
            root.not_before, root.not_after, parsed.info.gen_time
        )));
    }
    verify_rsa_signature(
        &root.subject_public_key_info,
        &root.signature_algorithm,
        &root.tbs_der,
        &root.signature,
    )
    .map_err(|failure| TokenFailure::from(failure).context("the root is not self-consistent"))?;
    verify_rsa_signature(
        &root.subject_public_key_info,
        &leaf.signature_algorithm,
        &leaf.tbs_der,
        &leaf.signature,
    )
    .map_err(|failure| {
        TokenFailure::from(failure)
            .context("the TSA certificate does not verify under the pinned root")
    })?;
    verify_rsa_signature(
        &leaf.subject_public_key_info,
        &parsed.signature_algorithm,
        &parsed.signed_message,
        &parsed.signature,
    )
    .map_err(|failure| {
        TokenFailure::from(failure).context("the token's signature does not verify")
    })?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::{build_timestamp_request, preferred_digest_oid, Der};

    /// A 32-byte digest, so the SHA-256 algorithm identifier is the admitted one.
    fn digest() -> Vec<u8> {
        (0u8..32)
            .map(|byte| byte.wrapping_mul(7).wrapping_add(3))
            .collect()
    }

    /// The request is the `TimeStampReq` `SEQUENCE` itself.
    ///
    /// An authority reads the top-level tag to decide what it was handed, and
    /// `TimeStampReq ::= SEQUENCE { ... }` (RFC 3161 §2.4.1) is a plain
    /// `SEQUENCE`. The regression this pins is real rather than hypothetical: an
    /// earlier encoding wrapped the sequence in a context tag `[0]`, and FreeTSA
    /// answered it with a `PKIStatusInfo` of "Bad request format" instead of a
    /// token, so no request the tool built could ever be granted.
    #[test]
    fn the_request_is_a_time_stamp_req_sequence() {
        let bytes =
            build_timestamp_request(&digest(), preferred_digest_oid()).expect("sha256 is admitted");
        assert_eq!(bytes[0], 0x30, "a TimeStampReq is a SEQUENCE");

        let request = Der::new(&bytes);
        let fields = request.children().expect("the request parses");
        assert_eq!(fields.len(), 2, "version and messageImprint");
        assert_eq!(
            fields[0].integer().expect("version"),
            vec![1u8],
            "version is 1"
        );
        let imprint = fields[1].children().expect("the imprint parses");
        assert_eq!(
            imprint[0].children().expect("an algorithm identifier")[0]
                .oid()
                .expect("an OID"),
            preferred_digest_oid()
        );
        assert_eq!(
            imprint[1].octets().expect("an octet string"),
            digest(),
            "the imprint names the digest that was asked about"
        );
    }

    /// A digest of the wrong width, and an algorithm that is not admitted, are
    /// refused rather than encoded.
    #[test]
    fn a_malformed_request_is_refused() {
        assert!(build_timestamp_request(&[0u8; 31], preferred_digest_oid()).is_err());
        assert!(build_timestamp_request(&digest(), "1.2.840.113549.1.1.11").is_err());
    }
}
