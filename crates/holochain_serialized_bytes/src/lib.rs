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

#[derive(Clone, PartialEq, Eq)]
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

holochain_serial!(());

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

    #[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
    enum BazResult {
        Ok(Vec<u8>),
        Err(String),
    }

    /// struct with raw bytes in it
    #[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
    struct Baz {
        wow: Option<BazResult>
    }

    #[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
    struct Tiny(u8);

    holochain_serial!(Foo, Bar, Baz, Tiny);

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
        macro_rules! do_test {
            ( $t:ty, $i:expr, $o:expr ) => {{
                let i = $i;
                let sb: SerializedBytes = i.clone().try_into().unwrap();
                // this isn't for testing it just shows how the debug output looks
                println!("{:?}", &sb);

                assert_eq!(&$o, &sb.0,);

                let returned: $t = sb.try_into().unwrap();

                assert_eq!(returned, i);
            }};
        }

        do_test!(
            Foo,
            fixture_foo(),
            vec![
                129_u8, 165_u8, 105_u8, 110_u8, 110_u8, 101_u8, 114_u8, 163_u8, 102_u8, 111_u8,
                111_u8,
            ]
        );

        do_test!(
            Bar,
            fixture_bar(),
            vec![
                129_u8, 168_u8, 119_u8, 104_u8, 97_u8, 116_u8, 101_u8, 118_u8, 101_u8, 114_u8,
                147_u8, 1_u8, 2_u8, 3_u8,
            ]
        );

        do_test!(
            Baz,
            Baz {
                wow: Some(BazResult::Ok(vec![2, 5, 6]))
            },
            vec![129, 163, 119, 111, 119, 129, 0, 147, 2, 5, 6]
        );

        do_test!(
            Tiny,
            Tiny(5),
            vec![5]
        );

        do_test!(
            (),
            (),
            vec![192]
        );
    }
}
