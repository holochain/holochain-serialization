extern crate serde;
#[allow(unused_imports)]
#[macro_use]
extern crate serde_derive;
extern crate rmp_serde;
extern crate serde_json;

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

impl std::fmt::Debug for SerializedBytes {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut deserializer = rmp_serde::Deserializer::from_read_ref(&self.0);
        let writer = Vec::new();
        let mut serializer = serde_json::ser::Serializer::new(writer);
        serde_transcode::transcode(&mut deserializer, &mut serializer).unwrap();
        let s = unsafe { String::from_utf8_unchecked(serializer.into_inner()) };
        write!(f, "{}", s)
    }
}

#[macro_export]
macro_rules! holochain_serial {
    ( $( $t:ty ),* ) => {

        $(
            impl std::convert::TryFrom<$t> for $crate::SerializedBytes {
                type Error = $crate::SerializedBytesError;
                fn try_from(t: $t) -> Result<$crate::SerializedBytes, $crate::SerializedBytesError> {
                    match $crate::rmp_serde::to_vec_named(&t) {
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

    };
}

#[cfg(test)]
pub mod tests {

    use super::*;
    use std::convert::TryInto;

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

    fn fixture_foo() -> Foo {
        Foo {
            inner: "foo".into(),
        }
    }

    fn fixture_bar() -> Bar {
        Bar {
            whatever: vec![1_u8, 2_u8, 3_u8],
        }
    }

    #[test]
    fn round_trip() {
        let sb_foo: SerializedBytes = fixture_foo().try_into().unwrap();

        assert_eq!(
            &vec![129_u8, 165_u8, 105_u8, 110_u8, 110_u8, 101_u8, 114_u8, 163_u8, 102_u8, 111_u8, 111_u8],
            &sb_foo.0,
        );

        let returned_foo: Foo = sb_foo.try_into().unwrap();

        assert_eq!(
            returned_foo,
            fixture_foo(),
        );

        let sb_bar: SerializedBytes = fixture_bar().try_into().unwrap();

        assert_eq!(
            &vec![129_u8, 168_u8, 119_u8, 104_u8, 97_u8, 116_u8, 101_u8, 118_u8, 101_u8, 114_u8, 147_u8, 1_u8, 2_u8, 3_u8],
            &sb_bar.0,
        );

        let returned_bar: Bar = sb_bar.try_into().unwrap();

        assert_eq!(
            returned_bar,
            fixture_bar(),
        );
    }

    #[test]

    #[test]
    /// this isn't a test really, it just prints out what comes from debug from SerializedBytes
    /// it's just a visual thing for whoever is running the tests
    fn debug() {
        let sb_foo: SerializedBytes = fixture_foo().try_into().unwrap();
        println!("{:?}", sb_foo);

        let sb_bar: SerializedBytes = fixture_bar().try_into().unwrap();
        println!("{:?}", sb_bar);
    }
}
