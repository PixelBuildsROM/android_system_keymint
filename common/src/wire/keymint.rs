//! Local types that are equivalent to those generated for KeyMint HAL interfaces
//!
//! - Enums are encoded as exhaustive Rust enums backed by `i32`, using Rust naming
//!   conventions (CamelCase values).
//! - Structs have all fields `pub`, using Rust naming conventions (snake_case fields).
//! - Both enums and structs get a `[derive(AsCborValue)]`
//!
//! Special cases:
//! - The `BeginResult` type of the HAL interface is omitted here, as it includes a
//!   Binder reference.
//! - `Tag` is private to this module, because....
//! - `KeyParam` is a Rust `enum` that is used in place of the `KeyParameter` struct, meaning...
//! - `KeyParameterValue` is not included here.

use crate::{cbor, cbor_type_error, crypto, AsCborValue, CborError};
use alloc::format;
use alloc::string::{String, ToString};
use alloc::{vec, vec::Vec};
use core::mem::size_of;
use enumn::N;
use kmr_derive::AsCborValue;

/// Representation of a date/time.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct DateTime {
    pub ms_since_epoch: i64,
}

impl AsCborValue for DateTime {
    fn from_cbor_value(value: cbor::value::Value) -> Result<Self, CborError> {
        let val = <i64>::from_cbor_value(value)?;
        Ok(Self { ms_since_epoch: val })
    }

    fn to_cbor_value(self) -> Result<cbor::value::Value, CborError> {
        self.ms_since_epoch.to_cbor_value()
    }

    fn cddl_typename() -> Option<String> {
        Some("int".to_string())
    }
}

/// Macro that emits an implementation of `TryFrom<i32>` for an enum type that has
/// `[derive(N)]` attached to it.
#[macro_export]
macro_rules! try_from_n {
    { $ename:ident } => {
        impl core::convert::TryFrom<i32> for $ename {
            type Error = super::ValueNotRecognized;
            fn try_from(value: i32) -> Result<Self, Self::Error> {
                Self::n(value).ok_or(super::ValueNotRecognized)
            }
        }
    };
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, AsCborValue, N)]
#[repr(i32)]
pub enum Algorithm {
    Rsa = 1,
    Ec = 3,
    Aes = 32,
    TripleDes = 33,
    Hmac = 128,
}
try_from_n!(Algorithm);

#[derive(Clone, Debug, Eq, PartialEq, AsCborValue)]
pub struct AttestationKey {
    pub key_blob: Vec<u8>,
    pub attest_key_params: Vec<KeyParam>,
    pub issuer_subject_name: Vec<u8>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, AsCborValue, N)]
#[repr(i32)]
pub enum BlockMode {
    Ecb = 1,
    Cbc = 2,
    Ctr = 3,
    Gcm = 32,
}
try_from_n!(BlockMode);

#[derive(Clone, Debug, Eq, PartialEq, AsCborValue)]
pub struct Certificate {
    pub encoded_certificate: Vec<u8>,
}

#[derive(Clone, Debug, Eq, PartialEq, AsCborValue)]
pub struct DeviceInfo {
    pub device_info: Vec<u8>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, AsCborValue, N)]
#[repr(i32)]
pub enum Digest {
    None = 0,
    Md5 = 1,
    Sha1 = 2,
    Sha224 = 3,
    Sha256 = 4,
    Sha384 = 5,
    Sha512 = 6,
}
try_from_n!(Digest);

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, AsCborValue, N)]
#[repr(i32)]
pub enum EcCurve {
    P224 = 0,
    P256 = 1,
    P384 = 2,
    P521 = 3,
    Curve25519 = 4,
}
try_from_n!(EcCurve);

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, AsCborValue, N)]
#[repr(i32)]
pub enum ErrorCode {
    Ok = 0,
    RootOfTrustAlreadySet = -1,
    UnsupportedPurpose = -2,
    IncompatiblePurpose = -3,
    UnsupportedAlgorithm = -4,
    IncompatibleAlgorithm = -5,
    UnsupportedKeySize = -6,
    UnsupportedBlockMode = -7,
    IncompatibleBlockMode = -8,
    UnsupportedMacLength = -9,
    UnsupportedPaddingMode = -10,
    IncompatiblePaddingMode = -11,
    UnsupportedDigest = -12,
    IncompatibleDigest = -13,
    InvalidExpirationTime = -14,
    InvalidUserId = -15,
    InvalidAuthorizationTimeout = -16,
    UnsupportedKeyFormat = -17,
    IncompatibleKeyFormat = -18,
    UnsupportedKeyEncryptionAlgorithm = -19,
    UnsupportedKeyVerificationAlgorithm = -20,
    InvalidInputLength = -21,
    KeyExportOptionsInvalid = -22,
    DelegationNotAllowed = -23,
    KeyNotYetValid = -24,
    KeyExpired = -25,
    KeyUserNotAuthenticated = -26,
    OutputParameterNull = -27,
    InvalidOperationHandle = -28,
    InsufficientBufferSpace = -29,
    VerificationFailed = -30,
    TooManyOperations = -31,
    UnexpectedNullPointer = -32,
    InvalidKeyBlob = -33,
    ImportedKeyNotEncrypted = -34,
    ImportedKeyDecryptionFailed = -35,
    ImportedKeyNotSigned = -36,
    ImportedKeyVerificationFailed = -37,
    InvalidArgument = -38,
    UnsupportedTag = -39,
    InvalidTag = -40,
    MemoryAllocationFailed = -41,
    ImportParameterMismatch = -44,
    SecureHwAccessDenied = -45,
    OperationCancelled = -46,
    ConcurrentAccessConflict = -47,
    SecureHwBusy = -48,
    SecureHwCommunicationFailed = -49,
    UnsupportedEcField = -50,
    MissingNonce = -51,
    InvalidNonce = -52,
    MissingMacLength = -53,
    KeyRateLimitExceeded = -54,
    CallerNonceProhibited = -55,
    KeyMaxOpsExceeded = -56,
    InvalidMacLength = -57,
    MissingMinMacLength = -58,
    UnsupportedMinMacLength = -59,
    UnsupportedKdf = -60,
    UnsupportedEcCurve = -61,
    KeyRequiresUpgrade = -62,
    AttestationChallengeMissing = -63,
    KeymintNotConfigured = -64,
    AttestationApplicationIdMissing = -65,
    CannotAttestIds = -66,
    RollbackResistanceUnavailable = -67,
    HardwareTypeUnavailable = -68,
    ProofOfPresenceRequired = -69,
    ConcurrentProofOfPresenceRequested = -70,
    NoUserConfirmation = -71,
    DeviceLocked = -72,
    EarlyBootEnded = -73,
    AttestationKeysNotProvisioned = -74,
    AttestationIdsNotProvisioned = -75,
    InvalidOperation = -76,
    StorageKeyUnsupported = -77,
    IncompatibleMgfDigest = -78,
    UnsupportedMgfDigest = -79,
    MissingNotBefore = -80,
    MissingNotAfter = -81,
    MissingIssuerSubject = -82,
    InvalidIssuerSubject = -83,
    BootLevelExceeded = -84,
    HardwareNotYetAvailable = -85,
    Unimplemented = -100,
    VersionMismatch = -101,
    UnknownError = -1000,
}
try_from_n!(ErrorCode);

#[derive(Clone, Debug, Eq, PartialEq, AsCborValue)]
pub struct HardwareAuthToken {
    pub challenge: i64,
    pub user_id: i64,
    pub authenticator_id: i64,
    pub authenticator_type: HardwareAuthenticatorType,
    pub timestamp: super::secureclock::Timestamp,
    pub mac: Vec<u8>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, AsCborValue, N)]
#[repr(i32)]
pub enum HardwareAuthenticatorType {
    None = 0,
    Password = 1,
    Fingerprint = 2,
    Any = -1,
}
try_from_n!(HardwareAuthenticatorType);

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
#[repr(i32)]
pub enum RpcErrorCode {
    Ok = 0, // not in HAL, assumed
    Failed = 1,
    InvalidMac = 2,
    ProductionKeyInTestRequest = 3,
    TestKeyInProductionRequest = 4,
    InvalidEek = 5,
}

#[derive(Clone, Debug, Eq, PartialEq, AsCborValue)]
pub struct KeyCharacteristics {
    pub security_level: SecurityLevel,
    pub authorizations: Vec<KeyParam>,
}

#[derive(Clone, Debug, Eq, PartialEq, AsCborValue)]
pub struct KeyCreationResult {
    pub key_blob: Vec<u8>,
    pub key_characteristics: Vec<KeyCharacteristics>,
    pub certificate_chain: Vec<Certificate>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, AsCborValue, N)]
#[repr(i32)]
pub enum KeyFormat {
    X509 = 0,
    Pkcs8 = 1,
    Raw = 3,
}
try_from_n!(KeyFormat);

#[derive(Clone, Debug, Eq, PartialEq, AsCborValue)]
pub struct KeyMintHardwareInfo {
    pub version_number: i32,
    pub security_level: SecurityLevel,
    pub key_mint_name: String,
    pub key_mint_author_name: String,
    pub timestamp_token_required: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, AsCborValue, N)]
#[repr(i32)]
pub enum KeyOrigin {
    Generated = 0,
    Derived = 1,
    Imported = 2,
    Reserved = 3,
    SecurelyImported = 4,
}
try_from_n!(KeyOrigin);

/// Rust exhaustive enum for all key parameters.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum KeyParam {
    Purpose(KeyPurpose),
    Algorithm(Algorithm),
    KeySize(crypto::KeySizeInBits),
    BlockMode(BlockMode),
    Digest(Digest),
    Padding(PaddingMode),
    CallerNonce,
    MinMacLength(u32),
    EcCurve(EcCurve),
    RsaPublicExponent(crypto::rsa::Exponent),
    IncludeUniqueId,
    RsaOaepMgfDigest(Digest),
    BootloaderOnly,
    RollbackResistance,
    EarlyBootOnly,
    ActiveDatetime(DateTime),
    OriginationExpireDatetime(DateTime),
    UsageExpireDatetime(DateTime),
    MaxUsesPerBoot(u32),
    UsageCountLimit(u32),
    UserId(u32),
    UserSecureId(u64),
    NoAuthRequired,
    UserAuthType(u32),
    AuthTimeout(u32),
    AllowWhileOnBody,
    TrustedUserPresenceRequired,
    TrustedConfirmationRequired,
    UnlockedDeviceRequired,
    ApplicationId(Vec<u8>),
    ApplicationData(Vec<u8>),
    CreationDatetime(DateTime),
    Origin(KeyOrigin),
    RootOfTrust(Vec<u8>),
    OsVersion(u32),
    OsPatchlevel(u32),
    AttestationChallenge(Vec<u8>),
    AttestationApplicationId(Vec<u8>),
    AttestationIdBrand(Vec<u8>),
    AttestationIdDevice(Vec<u8>),
    AttestationIdProduct(Vec<u8>),
    AttestationIdSerial(Vec<u8>),
    AttestationIdImei(Vec<u8>),
    AttestationIdMeid(Vec<u8>),
    AttestationIdManufacturer(Vec<u8>),
    AttestationIdModel(Vec<u8>),
    VendorPatchlevel(u32),
    BootPatchlevel(u32),
    DeviceUniqueAttestation,
    StorageKey,
    Nonce(Vec<u8>),
    MacLength(u32),
    ResetSinceIdRotation,
    CertificateSerial(Vec<u8>),
    CertificateSubject(Vec<u8>),
    CertificateNotBefore(DateTime),
    CertificateNotAfter(DateTime),
    MaxBootLevel(u32),
}

impl KeyParam {
    pub fn tag(&self) -> Tag {
        match self {
            KeyParam::Algorithm(_) => Tag::Algorithm,
            KeyParam::BlockMode(_) => Tag::BlockMode,
            KeyParam::Padding(_) => Tag::Padding,
            KeyParam::Digest(_) => Tag::Digest,
            KeyParam::EcCurve(_) => Tag::EcCurve,
            KeyParam::Origin(_) => Tag::Origin,
            KeyParam::Purpose(_) => Tag::Purpose,
            KeyParam::KeySize(_) => Tag::KeySize,
            KeyParam::CallerNonce => Tag::CallerNonce,
            KeyParam::MinMacLength(_) => Tag::MinMacLength,
            KeyParam::RsaPublicExponent(_) => Tag::RsaPublicExponent,
            KeyParam::IncludeUniqueId => Tag::IncludeUniqueId,
            KeyParam::RsaOaepMgfDigest(_) => Tag::RsaOaepMgfDigest,
            KeyParam::BootloaderOnly => Tag::BootloaderOnly,
            KeyParam::RollbackResistance => Tag::RollbackResistance,
            KeyParam::EarlyBootOnly => Tag::EarlyBootOnly,
            KeyParam::ActiveDatetime(_) => Tag::ActiveDatetime,
            KeyParam::OriginationExpireDatetime(_) => Tag::OriginationExpireDatetime,
            KeyParam::UsageExpireDatetime(_) => Tag::UsageExpireDatetime,
            KeyParam::MaxUsesPerBoot(_) => Tag::MaxUsesPerBoot,
            KeyParam::UsageCountLimit(_) => Tag::UsageCountLimit,
            KeyParam::UserId(_) => Tag::UserId,
            KeyParam::UserSecureId(_) => Tag::UserSecureId,
            KeyParam::NoAuthRequired => Tag::NoAuthRequired,
            KeyParam::UserAuthType(_) => Tag::UserAuthType,
            KeyParam::AuthTimeout(_) => Tag::AuthTimeout,
            KeyParam::AllowWhileOnBody => Tag::AllowWhileOnBody,
            KeyParam::TrustedUserPresenceRequired => Tag::TrustedUserPresenceRequired,
            KeyParam::TrustedConfirmationRequired => Tag::TrustedConfirmationRequired,
            KeyParam::UnlockedDeviceRequired => Tag::UnlockedDeviceRequired,
            KeyParam::ApplicationId(_) => Tag::ApplicationId,
            KeyParam::ApplicationData(_) => Tag::ApplicationData,
            KeyParam::CreationDatetime(_) => Tag::CreationDatetime,
            KeyParam::RootOfTrust(_) => Tag::RootOfTrust,
            KeyParam::OsVersion(_) => Tag::OsVersion,
            KeyParam::OsPatchlevel(_) => Tag::OsPatchlevel,
            KeyParam::AttestationChallenge(_) => Tag::AttestationChallenge,
            KeyParam::AttestationApplicationId(_) => Tag::AttestationApplicationId,
            KeyParam::AttestationIdBrand(_) => Tag::AttestationIdBrand,
            KeyParam::AttestationIdDevice(_) => Tag::AttestationIdDevice,
            KeyParam::AttestationIdProduct(_) => Tag::AttestationIdProduct,
            KeyParam::AttestationIdSerial(_) => Tag::AttestationIdSerial,
            KeyParam::AttestationIdImei(_) => Tag::AttestationIdImei,
            KeyParam::AttestationIdMeid(_) => Tag::AttestationIdMeid,
            KeyParam::AttestationIdManufacturer(_) => Tag::AttestationIdManufacturer,
            KeyParam::AttestationIdModel(_) => Tag::AttestationIdModel,
            KeyParam::VendorPatchlevel(_) => Tag::VendorPatchlevel,
            KeyParam::BootPatchlevel(_) => Tag::BootPatchlevel,
            KeyParam::DeviceUniqueAttestation => Tag::DeviceUniqueAttestation,
            KeyParam::StorageKey => Tag::StorageKey,
            KeyParam::Nonce(_) => Tag::Nonce,
            KeyParam::MacLength(_) => Tag::MacLength,
            KeyParam::ResetSinceIdRotation => Tag::ResetSinceIdRotation,
            KeyParam::CertificateSerial(_) => Tag::CertificateSerial,
            KeyParam::CertificateSubject(_) => Tag::CertificateSubject,
            KeyParam::CertificateNotBefore(_) => Tag::CertificateNotBefore,
            KeyParam::CertificateNotAfter(_) => Tag::CertificateNotAfter,
            KeyParam::MaxBootLevel(_) => Tag::MaxBootLevel,
        }
    }
}

/// Manual implementation of [`crate::AsCborValue`] for the [`KeyParam`] enum that
/// matches the serialization of the HAL `Tag` / `KeyParameterValue` types.
impl crate::AsCborValue for KeyParam {
    fn from_cbor_value(value: cbor::value::Value) -> Result<Self, crate::CborError> {
        let mut a = match value {
            cbor::value::Value::Array(a) => a,
            _ => return crate::cbor_type_error(&value, "arr"),
        };
        if a.len() != 2 {
            return Err(crate::CborError::UnexpectedItem("arr", "arr len 2"));
        }

        // Need to know the tag value to completely parse the value.
        let raw = a.remove(1);
        let tag = <Tag>::from_cbor_value(a.remove(0))?;

        Ok(match tag {
            Tag::Algorithm => KeyParam::Algorithm(<Algorithm>::from_cbor_value(raw)?),
            Tag::BlockMode => KeyParam::BlockMode(<BlockMode>::from_cbor_value(raw)?),
            Tag::Padding => KeyParam::Padding(<PaddingMode>::from_cbor_value(raw)?),
            Tag::Digest => KeyParam::Digest(<Digest>::from_cbor_value(raw)?),
            Tag::EcCurve => KeyParam::EcCurve(<EcCurve>::from_cbor_value(raw)?),
            Tag::Origin => KeyParam::Origin(<KeyOrigin>::from_cbor_value(raw)?),
            Tag::Purpose => KeyParam::Purpose(<KeyPurpose>::from_cbor_value(raw)?),
            Tag::KeySize => KeyParam::KeySize(crypto::KeySizeInBits(<u32>::from_cbor_value(raw)?)),
            Tag::CallerNonce => KeyParam::CallerNonce,
            Tag::MinMacLength => KeyParam::MinMacLength(<u32>::from_cbor_value(raw)?),
            Tag::RsaPublicExponent => {
                KeyParam::RsaPublicExponent(crypto::rsa::Exponent(<u64>::from_cbor_value(raw)?))
            }
            Tag::IncludeUniqueId => KeyParam::IncludeUniqueId,
            Tag::RsaOaepMgfDigest => KeyParam::RsaOaepMgfDigest(<Digest>::from_cbor_value(raw)?),
            Tag::BootloaderOnly => KeyParam::BootloaderOnly,
            Tag::RollbackResistance => KeyParam::RollbackResistance,
            Tag::EarlyBootOnly => KeyParam::EarlyBootOnly,
            Tag::ActiveDatetime => KeyParam::ActiveDatetime(<DateTime>::from_cbor_value(raw)?),
            Tag::OriginationExpireDatetime => {
                KeyParam::OriginationExpireDatetime(<DateTime>::from_cbor_value(raw)?)
            }
            Tag::UsageExpireDatetime => {
                KeyParam::UsageExpireDatetime(<DateTime>::from_cbor_value(raw)?)
            }
            Tag::MaxUsesPerBoot => KeyParam::MaxUsesPerBoot(<u32>::from_cbor_value(raw)?),
            Tag::UsageCountLimit => KeyParam::UsageCountLimit(<u32>::from_cbor_value(raw)?),
            Tag::UserId => KeyParam::UserId(<u32>::from_cbor_value(raw)?),
            Tag::UserSecureId => KeyParam::UserSecureId(<u64>::from_cbor_value(raw)?),
            Tag::NoAuthRequired => KeyParam::NoAuthRequired,
            Tag::UserAuthType => KeyParam::UserAuthType(<u32>::from_cbor_value(raw)?),
            Tag::AuthTimeout => KeyParam::AuthTimeout(<u32>::from_cbor_value(raw)?),
            Tag::AllowWhileOnBody => KeyParam::AllowWhileOnBody,
            Tag::TrustedUserPresenceRequired => KeyParam::TrustedUserPresenceRequired,
            Tag::TrustedConfirmationRequired => KeyParam::TrustedConfirmationRequired,
            Tag::UnlockedDeviceRequired => KeyParam::UnlockedDeviceRequired,
            Tag::ApplicationId => KeyParam::ApplicationId(<Vec<u8>>::from_cbor_value(raw)?),
            Tag::ApplicationData => KeyParam::ApplicationData(<Vec<u8>>::from_cbor_value(raw)?),
            Tag::CreationDatetime => KeyParam::CreationDatetime(<DateTime>::from_cbor_value(raw)?),
            Tag::RootOfTrust => KeyParam::RootOfTrust(<Vec<u8>>::from_cbor_value(raw)?),
            Tag::OsVersion => KeyParam::OsVersion(<u32>::from_cbor_value(raw)?),
            Tag::OsPatchlevel => KeyParam::OsPatchlevel(<u32>::from_cbor_value(raw)?),
            Tag::AttestationChallenge => {
                KeyParam::AttestationChallenge(<Vec<u8>>::from_cbor_value(raw)?)
            }
            Tag::AttestationApplicationId => {
                KeyParam::AttestationApplicationId(<Vec<u8>>::from_cbor_value(raw)?)
            }
            Tag::AttestationIdBrand => {
                KeyParam::AttestationIdBrand(<Vec<u8>>::from_cbor_value(raw)?)
            }
            Tag::AttestationIdDevice => {
                KeyParam::AttestationIdDevice(<Vec<u8>>::from_cbor_value(raw)?)
            }
            Tag::AttestationIdProduct => {
                KeyParam::AttestationIdProduct(<Vec<u8>>::from_cbor_value(raw)?)
            }
            Tag::AttestationIdSerial => {
                KeyParam::AttestationIdSerial(<Vec<u8>>::from_cbor_value(raw)?)
            }
            Tag::AttestationIdImei => KeyParam::AttestationIdImei(<Vec<u8>>::from_cbor_value(raw)?),
            Tag::AttestationIdMeid => KeyParam::AttestationIdMeid(<Vec<u8>>::from_cbor_value(raw)?),
            Tag::AttestationIdManufacturer => {
                KeyParam::AttestationIdManufacturer(<Vec<u8>>::from_cbor_value(raw)?)
            }
            Tag::AttestationIdModel => {
                KeyParam::AttestationIdModel(<Vec<u8>>::from_cbor_value(raw)?)
            }
            Tag::VendorPatchlevel => KeyParam::VendorPatchlevel(<u32>::from_cbor_value(raw)?),
            Tag::BootPatchlevel => KeyParam::BootPatchlevel(<u32>::from_cbor_value(raw)?),
            Tag::DeviceUniqueAttestation => KeyParam::DeviceUniqueAttestation,
            Tag::StorageKey => KeyParam::StorageKey,
            Tag::Nonce => KeyParam::Nonce(<Vec<u8>>::from_cbor_value(raw)?),
            Tag::MacLength => KeyParam::MacLength(<u32>::from_cbor_value(raw)?),
            Tag::ResetSinceIdRotation => KeyParam::ResetSinceIdRotation,
            Tag::CertificateSerial => KeyParam::CertificateSerial(<Vec<u8>>::from_cbor_value(raw)?),
            Tag::CertificateSubject => {
                KeyParam::CertificateSubject(<Vec<u8>>::from_cbor_value(raw)?)
            }
            Tag::CertificateNotBefore => {
                KeyParam::CertificateNotBefore(<DateTime>::from_cbor_value(raw)?)
            }
            Tag::CertificateNotAfter => {
                KeyParam::CertificateNotAfter(<DateTime>::from_cbor_value(raw)?)
            }
            Tag::MaxBootLevel => KeyParam::MaxBootLevel(<u32>::from_cbor_value(raw)?),

            _ => return Err(crate::CborError::UnexpectedItem("tag", "known tag")),
        })
    }
    fn to_cbor_value(self) -> Result<cbor::value::Value, crate::CborError> {
        let (tag, val) = match self {
            KeyParam::Algorithm(v) => (Tag::Algorithm, v.to_cbor_value()?),
            KeyParam::BlockMode(v) => (Tag::BlockMode, v.to_cbor_value()?),
            KeyParam::Padding(v) => (Tag::Padding, v.to_cbor_value()?),
            KeyParam::Digest(v) => (Tag::Digest, v.to_cbor_value()?),
            KeyParam::EcCurve(v) => (Tag::EcCurve, v.to_cbor_value()?),
            KeyParam::Origin(v) => (Tag::Origin, v.to_cbor_value()?),
            KeyParam::Purpose(v) => (Tag::Purpose, v.to_cbor_value()?),
            KeyParam::KeySize(v) => (Tag::KeySize, v.0.to_cbor_value()?),
            KeyParam::CallerNonce => (Tag::CallerNonce, true.to_cbor_value()?),
            KeyParam::MinMacLength(v) => (Tag::MinMacLength, v.to_cbor_value()?),
            KeyParam::RsaPublicExponent(v) => (Tag::RsaPublicExponent, v.0.to_cbor_value()?),
            KeyParam::IncludeUniqueId => (Tag::IncludeUniqueId, true.to_cbor_value()?),
            KeyParam::RsaOaepMgfDigest(v) => (Tag::RsaOaepMgfDigest, v.to_cbor_value()?),
            KeyParam::BootloaderOnly => (Tag::BootloaderOnly, true.to_cbor_value()?),
            KeyParam::RollbackResistance => (Tag::RollbackResistance, true.to_cbor_value()?),
            KeyParam::EarlyBootOnly => (Tag::EarlyBootOnly, true.to_cbor_value()?),
            KeyParam::ActiveDatetime(v) => (Tag::ActiveDatetime, v.to_cbor_value()?),
            KeyParam::OriginationExpireDatetime(v) => {
                (Tag::OriginationExpireDatetime, v.to_cbor_value()?)
            }
            KeyParam::UsageExpireDatetime(v) => (Tag::UsageExpireDatetime, v.to_cbor_value()?),
            KeyParam::MaxUsesPerBoot(v) => (Tag::MaxUsesPerBoot, v.to_cbor_value()?),
            KeyParam::UsageCountLimit(v) => (Tag::UsageCountLimit, v.to_cbor_value()?),
            KeyParam::UserId(v) => (Tag::UserId, v.to_cbor_value()?),
            KeyParam::UserSecureId(v) => (Tag::UserSecureId, v.to_cbor_value()?),
            KeyParam::NoAuthRequired => (Tag::NoAuthRequired, true.to_cbor_value()?),
            KeyParam::UserAuthType(v) => (Tag::UserAuthType, v.to_cbor_value()?),
            KeyParam::AuthTimeout(v) => (Tag::AuthTimeout, v.to_cbor_value()?),
            KeyParam::AllowWhileOnBody => (Tag::AllowWhileOnBody, true.to_cbor_value()?),
            KeyParam::TrustedUserPresenceRequired => {
                (Tag::TrustedUserPresenceRequired, true.to_cbor_value()?)
            }
            KeyParam::TrustedConfirmationRequired => {
                (Tag::TrustedConfirmationRequired, true.to_cbor_value()?)
            }
            KeyParam::UnlockedDeviceRequired => {
                (Tag::UnlockedDeviceRequired, true.to_cbor_value()?)
            }
            KeyParam::ApplicationId(v) => (Tag::ApplicationId, v.to_cbor_value()?),
            KeyParam::ApplicationData(v) => (Tag::ApplicationData, v.to_cbor_value()?),
            KeyParam::CreationDatetime(v) => (Tag::CreationDatetime, v.to_cbor_value()?),
            KeyParam::RootOfTrust(v) => (Tag::RootOfTrust, v.to_cbor_value()?),
            KeyParam::OsVersion(v) => (Tag::OsVersion, v.to_cbor_value()?),
            KeyParam::OsPatchlevel(v) => (Tag::OsPatchlevel, v.to_cbor_value()?),
            KeyParam::AttestationChallenge(v) => (Tag::AttestationChallenge, v.to_cbor_value()?),
            KeyParam::AttestationApplicationId(v) => {
                (Tag::AttestationApplicationId, v.to_cbor_value()?)
            }
            KeyParam::AttestationIdBrand(v) => (Tag::AttestationIdBrand, v.to_cbor_value()?),
            KeyParam::AttestationIdDevice(v) => (Tag::AttestationIdDevice, v.to_cbor_value()?),
            KeyParam::AttestationIdProduct(v) => (Tag::AttestationIdProduct, v.to_cbor_value()?),
            KeyParam::AttestationIdSerial(v) => (Tag::AttestationIdSerial, v.to_cbor_value()?),
            KeyParam::AttestationIdImei(v) => (Tag::AttestationIdImei, v.to_cbor_value()?),
            KeyParam::AttestationIdMeid(v) => (Tag::AttestationIdMeid, v.to_cbor_value()?),
            KeyParam::AttestationIdManufacturer(v) => {
                (Tag::AttestationIdManufacturer, v.to_cbor_value()?)
            }
            KeyParam::AttestationIdModel(v) => (Tag::AttestationIdModel, v.to_cbor_value()?),
            KeyParam::VendorPatchlevel(v) => (Tag::VendorPatchlevel, v.to_cbor_value()?),
            KeyParam::BootPatchlevel(v) => (Tag::BootPatchlevel, v.to_cbor_value()?),
            KeyParam::DeviceUniqueAttestation => {
                (Tag::DeviceUniqueAttestation, true.to_cbor_value()?)
            }
            KeyParam::StorageKey => (Tag::StorageKey, true.to_cbor_value()?),
            KeyParam::Nonce(v) => (Tag::Nonce, v.to_cbor_value()?),
            KeyParam::MacLength(v) => (Tag::MacLength, v.to_cbor_value()?),
            KeyParam::ResetSinceIdRotation => (Tag::ResetSinceIdRotation, true.to_cbor_value()?),
            KeyParam::CertificateSerial(v) => (Tag::CertificateSerial, v.to_cbor_value()?),
            KeyParam::CertificateSubject(v) => (Tag::CertificateSubject, v.to_cbor_value()?),
            KeyParam::CertificateNotBefore(v) => (Tag::CertificateNotBefore, v.to_cbor_value()?),
            KeyParam::CertificateNotAfter(v) => (Tag::CertificateNotAfter, v.to_cbor_value()?),
            KeyParam::MaxBootLevel(v) => (Tag::MaxBootLevel, v.to_cbor_value()?),
        };
        Ok(cbor::value::Value::Array(vec![tag.to_cbor_value()?, val]))
    }
    fn cddl_typename() -> Option<String> {
        Some("KeyParameter".to_string())
    }
    fn cddl_schema() -> Option<String> {
        Some(format!(
            r#"[
    tag: {},
    ; Choice for value is determined by tag value
    value: &(
        Invalid: {},
        Algorithm: {},
        BlockMode: {},
        PaddingMode: {},
        Digest: {},
        EcCurve: {},
        Origin: {},
        KeyPurpose: {},
        HardwareAuthenticatorType: {},
        SecurityLevel: {},
        BoolValue: {},
        Integer: {},
        LongInteger: {},
        DateTime: {},
        Blob: {},
    )
]"#,
            <Tag>::cddl_ref(),
            <i32>::cddl_ref(),
            <Algorithm>::cddl_ref(),
            <BlockMode>::cddl_ref(),
            <PaddingMode>::cddl_ref(),
            <Digest>::cddl_ref(),
            <EcCurve>::cddl_ref(),
            <KeyOrigin>::cddl_ref(),
            <KeyPurpose>::cddl_ref(),
            <HardwareAuthenticatorType>::cddl_ref(),
            <SecurityLevel>::cddl_ref(),
            <bool>::cddl_ref(),
            <i32>::cddl_ref(),
            <i64>::cddl_ref(),
            <i64>::cddl_ref(),
            <Vec<u8>>::cddl_ref(),
        ))
    }
}

/// Determine the tag type for a tag, based on the top 4 bits of the tag number.
pub fn tag_type(tag: Tag) -> TagType {
    match ((tag as u32) & 0xf0000000u32) as i32 {
        x if x == TagType::Enum as i32 => TagType::Enum,
        x if x == TagType::EnumRep as i32 => TagType::EnumRep,
        x if x == TagType::Uint as i32 => TagType::Uint,
        x if x == TagType::UintRep as i32 => TagType::UintRep,
        x if x == TagType::Ulong as i32 => TagType::Ulong,
        x if x == TagType::Date as i32 => TagType::Date,
        x if x == TagType::Bool as i32 => TagType::Bool,
        x if x == TagType::Bignum as i32 => TagType::Bignum,
        x if x == TagType::Bytes as i32 => TagType::Bytes,
        x if x == TagType::UlongRep as i32 => TagType::UlongRep,
        _ => TagType::Invalid,
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, AsCborValue, N)]
#[repr(i32)]
pub enum KeyPurpose {
    Encrypt = 0,
    Decrypt = 1,
    Sign = 2,
    Verify = 3,
    WrapKey = 5,
    AgreeKey = 6,
    AttestKey = 7,
}
try_from_n!(KeyPurpose);

#[derive(Clone, Debug, Eq, PartialEq, AsCborValue)]
pub struct MacedPublicKey {
    pub maced_key: Vec<u8>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, AsCborValue, N)]
#[repr(i32)]
pub enum PaddingMode {
    None = 1,
    RsaOaep = 2,
    RsaPss = 3,
    RsaPkcs115Encrypt = 4,
    RsaPkcs115Sign = 5,
    Pkcs7 = 64,
}
try_from_n!(PaddingMode);

#[derive(Clone, Debug, Eq, PartialEq, AsCborValue)]
pub struct ProtectedData {
    pub protected_data: Vec<u8>,
}

#[derive(Clone, Debug, Eq, PartialEq, AsCborValue)]
pub struct RpcHardwareInfo {
    pub version_number: i32,
    pub rpc_author_name: String,
    pub supported_eek_curve: RpcEekCurve,
    pub unique_id: Option<String>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, AsCborValue, N)]
#[repr(i32)]
pub enum RpcEekCurve {
    None = 0,
    P256 = 1,
    Curve25519 = 2,
}
try_from_n!(RpcEekCurve);

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, AsCborValue, N)]
#[repr(i32)]
pub enum SecurityLevel {
    Software = 0,
    TrustedEnvironment = 1,
    Strongbox = 2,
    Keystore = 100,
}
try_from_n!(SecurityLevel);

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, AsCborValue, N)]
#[repr(i32)]
pub enum Tag {
    Invalid = 0,
    Purpose = 536870913,
    Algorithm = 268435458,
    KeySize = 805306371,
    BlockMode = 536870916,
    Digest = 536870917,
    Padding = 536870918,
    CallerNonce = 1879048199,
    MinMacLength = 805306376,
    EcCurve = 268435466,
    RsaPublicExponent = 1342177480,
    IncludeUniqueId = 1879048394,
    RsaOaepMgfDigest = 536871115,
    BootloaderOnly = 1879048494,
    RollbackResistance = 1879048495,
    HardwareType = 268435760,
    EarlyBootOnly = 1879048497,
    ActiveDatetime = 1610613136,
    OriginationExpireDatetime = 1610613137,
    UsageExpireDatetime = 1610613138,
    MinSecondsBetweenOps = 805306771,
    MaxUsesPerBoot = 805306772,
    UsageCountLimit = 805306773,
    UserId = 805306869,
    UserSecureId = -1610612234,
    NoAuthRequired = 1879048695,
    UserAuthType = 268435960,
    AuthTimeout = 805306873,
    AllowWhileOnBody = 1879048698,
    TrustedUserPresenceRequired = 1879048699,
    TrustedConfirmationRequired = 1879048700,
    UnlockedDeviceRequired = 1879048701,
    ApplicationId = -1879047591,
    ApplicationData = -1879047492,
    CreationDatetime = 1610613437,
    Origin = 268436158,
    RootOfTrust = -1879047488,
    OsVersion = 805307073,
    OsPatchlevel = 805307074,
    UniqueId = -1879047485,
    AttestationChallenge = -1879047484,
    AttestationApplicationId = -1879047483,
    AttestationIdBrand = -1879047482,
    AttestationIdDevice = -1879047481,
    AttestationIdProduct = -1879047480,
    AttestationIdSerial = -1879047479,
    AttestationIdImei = -1879047478,
    AttestationIdMeid = -1879047477,
    AttestationIdManufacturer = -1879047476,
    AttestationIdModel = -1879047475,
    VendorPatchlevel = 805307086,
    BootPatchlevel = 805307087,
    DeviceUniqueAttestation = 1879048912,
    IdentityCredentialKey = 1879048913,
    StorageKey = 1879048914,
    AssociatedData = -1879047192,
    Nonce = -1879047191,
    MacLength = 805307371,
    ResetSinceIdRotation = 1879049196,
    ConfirmationToken = -1879047187,
    CertificateSerial = -2147482642,
    CertificateSubject = -1879047185,
    CertificateNotBefore = 1610613744,
    CertificateNotAfter = 1610613745,
    MaxBootLevel = 805307378,
}
try_from_n!(Tag);

#[derive(Clone, Copy, Debug, PartialEq, Eq, AsCborValue, N)]
#[repr(i32)]
pub enum TagType {
    Invalid = 0,
    Enum = 268435456,
    EnumRep = 536870912,
    Uint = 805306368,
    UintRep = 1073741824,
    Ulong = 1342177280,
    Date = 1610612736,
    Bool = 1879048192,
    Bignum = -2147483648,
    Bytes = -1879048192,
    UlongRep = -1610612736,
}
try_from_n!(TagType);

/// Build the HMAC input for a [`HardwareAuthToken`]
pub fn hardware_auth_token_mac_input(token: &HardwareAuthToken) -> Vec<u8> {
    let mut result = Vec::with_capacity(
        size_of::<u8>() + // version=0 (BE)
        size_of::<i64>() + // challenge (Host)
        size_of::<i64>() + // user_id (Host)
        size_of::<i64>() + // authenticator_id (Host)
        size_of::<i32>() + // authenticator_type (BE)
        size_of::<i64>(), // timestamp (BE)
    );
    result.extend_from_slice(&0u8.to_be_bytes()[..]);
    result.extend_from_slice(&token.challenge.to_ne_bytes()[..]);
    result.extend_from_slice(&token.user_id.to_ne_bytes()[..]);
    result.extend_from_slice(&token.authenticator_id.to_ne_bytes()[..]);
    result.extend_from_slice(&(token.authenticator_type as i32).to_be_bytes()[..]);
    result.extend_from_slice(&token.timestamp.milliseconds.to_be_bytes()[..]);
    result
}