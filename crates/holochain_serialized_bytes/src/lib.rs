extern crate rmp_serde;
extern crate serde;
extern crate serde_json;

use serde::{Deserialize, Serialize};
use std::convert::Infallible;
use std::convert::TryFrom;

pub mod prelude;

#[cfg_attr(feature = "trace", tracing::instrument)]
pub fn encode<T: serde::Serialize + std::fmt::Debug>(
    val: &T,
) -> Result<Vec<u8>, SerializedBytesError> {
    let buf = Vec::with_capacity(128);
    let mut se = rmp_serde::encode::Serializer::new(buf).with_struct_map();
    val.serialize(&mut se).map_err(|err| {
        #[cfg(feature = "trace")]
        tracing::warn!("Failed to serialize input");
        SerializedBytesError::Serialize(err.to_string())
    })?;
    let ret = se.into_inner();
    #[cfg(feature = "trace")]
    tracing::trace!(
        "Serialized {} input into {:?}",
        std::any::type_name::<T>(),
        ret
    );
    Ok(ret)
}

#[cfg_attr(feature = "trace", tracing::instrument)]
pub fn decode<'a, R, T>(input: &'a R) -> Result<T, SerializedBytesError>
where
    R: AsRef<[u8]> + ?Sized + std::fmt::Debug,
    T: Deserialize<'a> + std::fmt::Debug,
{
    let ret = rmp_serde::from_slice(input.as_ref()).map_err(|err| {
        #[cfg(feature = "trace")]
        tracing::warn!(
            "Failed to deserialize input into: {}",
            std::any::type_name::<T>()
        );
        SerializedBytesError::Deserialize(err.to_string())
    })?;
    #[cfg(feature = "trace")]
    tracing::trace!("Deserialized input into: {:?}", ret);
    Ok(ret)
}

#[derive(
    Clone,
    Debug,
    PartialEq,
    Eq,
    Hash,
    PartialOrd,
    Ord,
    serde::Serialize,
    serde::Deserialize,
    thiserror::Error,
)]
pub enum SerializedBytesError {
    /// somehow failed to move to bytes
    /// most likely hit a messagepack limit https://github.com/msgpack/msgpack/blob/master/spec.md#limitation
    Serialize(String),
    /// somehow failed to restore bytes
    /// i mean, this could be anything, how do i know what's wrong with your bytes?
    Deserialize(String),
}

impl std::fmt::Display for SerializedBytesError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl From<SerializedBytesError> for String {
    fn from(sb: SerializedBytesError) -> Self {
        match sb {
            SerializedBytesError::Serialize(s) => s,
            SerializedBytesError::Deserialize(s) => s,
        }
    }
}

impl From<Infallible> for SerializedBytesError {
    fn from(_: Infallible) -> Self {
        unreachable!()
    }
}

#[derive(Clone, Debug)]
#[cfg_attr(
    feature = "fuzzing",
    derive(arbitrary::Arbitrary, proptest_derive::Arbitrary)
)]
/// UnsafeBytes the only way to implement a custom round trip through bytes for SerializedBytes
/// It is intended to be an internal implementation in TryFrom implementations
/// The assumption is that any code using UnsafeBytes is NOT valid messagepack data
/// This allows us to enforce that all data round-tripping through SerializedBytes is via TryFrom
/// and also allow for custom non-messagepack canonical representations of data types.
pub struct UnsafeBytes(Vec<u8>);

impl From<Vec<u8>> for UnsafeBytes {
    fn from(v: Vec<u8>) -> Self {
        Self(v)
    }
}

impl From<UnsafeBytes> for Vec<u8> {
    fn from(unsafe_bytes: UnsafeBytes) -> Vec<u8> {
        unsafe_bytes.0
    }
}

impl From<UnsafeBytes> for SerializedBytes {
    fn from(b: UnsafeBytes) -> Self {
        SerializedBytes(b.0)
    }
}

impl From<SerializedBytes> for UnsafeBytes {
    fn from(sb: SerializedBytes) -> Self {
        UnsafeBytes(sb.0)
    }
}

/// A Canonical Serialized Bytes representation for data
/// If you have a data structure that needs a canonical byte representation use this
/// Always round-trip through SerializedBytes via. a single TryFrom implementation.
/// This ensures that the internal bytes of SerializedBytes are indeed canonical.
/// The corrolary is that if bytes are NOT wrapped in SerializedBytes we can assume they are NOT
/// canonical.
/// Typically we need a canonical serialization when data is to be handled at the byte level by
/// independently implemented and maintained systems.
///
/// Examples of times we need a canonical set of bytes to represent data:
/// - cryptographic operations
/// - moving across the host/guest wasm boundary
/// - putting data on the network
///
/// Examples of times where we may not need a canonical representation and so may not need this:
/// - round tripping data through a database that has its own serialization preferences
/// - debug output or logging of data that is to be human readible
/// - moving between data types within a single system that has no external facing representation
///
/// uses #[repr(transparent)] to maximise compatibility with ffi
/// @see https://doc.rust-lang.org/1.26.2/unstable-book/language-features/repr-transparent.html#enter-reprtransparent
///
/// uses serde_bytes for efficient serialization and deserialization
/// without this __every byte will be individually round tripped through serde__
/// @see https://crates.io/crates/serde_bytes
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize, Hash)]
#[cfg_attr(
    feature = "fuzzing",
    derive(arbitrary::Arbitrary, proptest_derive::Arbitrary)
)]
#[repr(transparent)]
pub struct SerializedBytes(#[serde(with = "serde_bytes")] Vec<u8>);

impl SerializedBytes {
    pub fn bytes(&self) -> &Vec<u8> {
        &self.0
    }
}

/// A bit of magic to convert the internal messagepack bytes into roughly equivalent JSON output
/// for the purposes of debugging.
/// 90% of the time you probably want this if you are a dev, to see something that "looks like" a
/// data structure when you do {:?} in a formatted string, rather than a vector of bytes
/// in the remaining 10% of situations where you want to debug the real messagepack bytes, call the
/// .bytes() method on SerializedBytes and debug that.
impl std::fmt::Debug for SerializedBytes {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // NB: there is a crate::decode function which currently simply uses
        // from_read_ref. If that ever changes, this may become inconsistent
        // and should be changed also.
        let mut deserializer = rmp_serde::Deserializer::from_read_ref(&self.0);
        let writer = Vec::new();
        let mut serializer = serde_json::ser::Serializer::new(writer);
        if serde_transcode::transcode(&mut deserializer, &mut serializer).is_err() {
            write!(f, "<invalid msgpack>")
        } else {
            let s = unsafe { String::from_utf8_unchecked(serializer.into_inner()) };
            write!(f, "{}", s)
        }
    }
}

#[macro_export]
/// unidiomatic way to derive default trait implementations of TryFrom in/out of SerializedBytes
///
/// Two main reasons this was done rather than the normal Derive approach:
/// - Derive requires a separate crate
/// - Derive doesn't allow for use of $crate to set unambiguous fully qualified paths to things
///
/// Both of these limitations push dependency management downstream to consumer crates more than we
/// want to.
/// This implementation allows us to manage all dependencies explicitly right here in this crate.
///
/// There is a default implementation of SerializedBytes into and out of ()
/// this is the ONLY supported direct primitive round-trip, which maps to `nil` in messagepack
/// for all other primitives, wrap them in a new type or enum
///
/// e.g. do NOT do this:
/// `u32::try_from(serialized_bytes)?;`
///
/// instead do this:
/// ```
/// use holochain_serialized_bytes::prelude::*;
///
/// #[derive(Serialize, Deserialize, Debug)]
/// pub struct SomeType(u32);
/// holochain_serial!(SomeType);
/// let serialized_bytes = SerializedBytes::try_from(SomeType(50)).unwrap();
/// let some_type = SomeType::try_from(serialized_bytes).unwrap();
/// ```
///
/// put `SomeType` in a separate crate that can be shared by all producers and consumers of the
/// serialized bytes in a minimal way.
/// this is a bit more upfront work but it's the only way the compiler can check a type maps to
/// a specific serialization across different systems, crate versions, and refactors.
///
/// for example, say we changed `SomeType(u32)` to `SomeType(u64)` in the shared crate
/// with the new type the compiler can enforce roundtripping of bytes works everywhere `SomeType`
/// is used, provided all producers and consumers use the same version of the shared crate.
/// in the case where we have no `SomeType` and would use integers directly, there is no safety.
/// the system can't tell the difference between a type mismatch (e.g. you just wrote u32 in the
/// wrong spot in one of the systems) and a serialization mismatch (e.g. the serialized bytes
/// produced by some system were consumed by another system using a different version of the shared
/// crate or something).
///
/// Developers then have to manually mentally impose the meaning of primitives over the top of code
/// across different code-bases that are ostensibly separate projects.
/// This introduces the effect where you can't understand/refactor one component of the system
/// without understanding and refactoring all the other components in the same PR/QA step.
///
/// An explicit goal of SerializedBytes is to introduce stability of byte-level data interfaces
/// across systems, provided they share the same version of a shared types crate.
/// This means that that one component can implement a new version of the shared types and know
/// that it will be compatible with other components when they similarly upgrade AND other
/// components are safe to delay upgrading to the latest version of the shared crate until they are
/// ready to move. Essentially it allows for async development workflows around serialized data.
///
/// This is especially important for wasm as the wasm hosts and guests may not even be developed
/// by the same people/organisations, so there MUST be some compiler level guarantee that at least
/// the shared types within the same shared crate version have compatible serialization logic.
///
/// usually when working with primitives we are within a single system, i.e. a single compilation
/// context, a single set of dependencies, a single release/QA lifecycle
/// in this case, while we _could_ wrap every single primitive in a new type for maximum compile
/// time safety it is often 'overkill' and we can eat some ambiguity for the sake of ergonomics and
/// minimising the number of parallel types/trait implementations.
/// in the case of parallel, decoupled, independently maintained systems that rely on byte-level
/// canonical representation of things that will fail (e.g. cryptographically break or (de)allocate
/// memory incorrectly) if even one byte is wrong, the guide-rails provided by new types and enums
/// are worth the additional up-front effort of creating a few extra shared crates/types.
///
/// see the readme for more discussion around this
macro_rules! holochain_serial {
    ( $( $t:ty ),* ) => {

        $(
            impl std::convert::TryFrom<&$t> for $crate::SerializedBytes {
                type Error = $crate::SerializedBytesError;
                fn try_from(t: &$t) -> std::result::Result<$crate::SerializedBytes, $crate::SerializedBytesError> {
                    $crate::encode(t).map(|v|
                        $crate::SerializedBytes::from($crate::UnsafeBytes::from(v))
                    )
                }
            }

            impl std::convert::TryFrom<$t> for $crate::SerializedBytes {
                type Error = $crate::SerializedBytesError;
                fn try_from(t: $t) -> std::result::Result<$crate::SerializedBytes, $crate::SerializedBytesError> {
                    $crate::SerializedBytes::try_from(&t)
                }
            }

            impl std::convert::TryFrom<$crate::SerializedBytes> for $t {
                type Error = $crate::SerializedBytesError;
                fn try_from(sb: $crate::SerializedBytes) -> std::result::Result<$t, $crate::SerializedBytesError> {
                    $crate::decode(sb.bytes())
                }
            }
        )*

    };
}

holochain_serial!(());

impl Default for SerializedBytes {
    fn default() -> Self {
        SerializedBytes::try_from(()).unwrap()
    }
}

impl<'a> TryFrom<&'a SerializedBytes> for SerializedBytes {
    type Error = SerializedBytesError;
    fn try_from(s: &'a SerializedBytes) -> Result<Self, Self::Error> {
        Ok(s.clone())
    }
}
