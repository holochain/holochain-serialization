extern crate serde;
#[allow(unused_imports)]
#[macro_use]
extern crate serde_derive;
extern crate rmp_serde;

#[derive(Debug)]
pub enum SerializedBytesError {
    /// somehow failed to move to bytes
    /// most likely hit a messagepack limit https://github.com/msgpack/msgpack/blob/master/spec.md#limitation
    ToBytes(String),
    /// somehow failed to restore bytes
    /// i mean, this could be anything, how do i know what's wrong with your bytes?
    FromBytes(String),
}

pub struct SerializedBytes(Vec<u8>);

#[macro_export]
macro_rules! holochain_serial {
    ( $( $t:ty ),* ) => {{

        $(
            impl std::convert::TryFrom<$t> for $crate::SerializedBytes {
                type Error = $crate::SerializedBytesError;
                fn try_from(t: $t) -> Result<$crate::SerializedBytes, $crate::SerializedBytesError> {
                    match $crate::rmp_serde::to_vec(&t) {
                        Ok(v) => Ok($crate::SerializedBytes(v)),
                        Err(e) => Err($crate::SerializedBytesError::ToBytes(e.to_string())),
                    }
                }
            }

            impl std::convert::TryFrom<$crate::SerializedBytes> for $t {
                type Error = $crate::SerializedBytesError;
                fn try_from(sb: $crate::SerializedBytes) -> Result<$t, $crate::SerializedBytesError> {
                    match $crate::rmp_serde::from_read_ref(&sb.0) {
                        Ok(v) => Ok(v),
                        Err(e) => Err($crate::SerializedBytesError::FromBytes(e.to_string())),
                    }
                }
            }
        )*

    }};
}

#[cfg(test)]
pub mod tests {

    use super::*;
    use std::convert::TryInto;

    #[test]
    fn round_trip() {
        /// struct with a utf8 string in it
        #[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
        struct Foo {
            inner: String,
        }

        /// struct with raw bytes in it
        #[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
        struct Bar {
            whatever: Vec<u8>,
        }

        holochain_serial!(Foo, Bar);

        let foo = Foo {
            inner: "foo".into(),
        };

        let sb_foo: SerializedBytes = foo.clone().try_into().unwrap();

        assert_eq!(
            &vec![145_u8, 163_u8, 102_u8, 111_u8, 111_u8],
            &sb_foo.0,
        );

        let returned_foo: Foo = sb_foo.try_into().unwrap();

        assert_eq!(
            returned_foo,
            foo,
        );

        let bar = Bar {
            whatever: vec![1_u8, 2_u8, 3_u8],
        };

        let sb_bar: SerializedBytes = bar.clone().try_into().unwrap();

        assert_eq!(
            &vec![145, 147, 1, 2, 3],
            &sb_bar.0,
        );

        let returned_bar: Bar = sb_bar.try_into().unwrap();

        assert_eq!(
            returned_bar,
            bar,
        );
    }
}
