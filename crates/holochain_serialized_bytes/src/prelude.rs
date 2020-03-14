pub use serde_derive::Deserialize;
/// allow downstream consumers to not manage their own deps on serde
pub use serde_derive::Serialize;

/// allow downstream consumers to forget to pull in basic try_from/try_into methods
pub use std::convert::TryFrom;
pub use std::convert::TryInto;

/// 90% of the time these are the only two things downstream consumers need from this crate
pub use crate::holochain_serial;
pub use crate::SerializedBytes;
