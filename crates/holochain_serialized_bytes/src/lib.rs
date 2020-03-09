extern crate serde;
#[allow(unused_imports)]
#[macro_use]
extern crate serde_derive;
extern crate rmp_serde;
extern crate serde_json;

#[derive(Deserialize)]
/// @TODO this is hacky
/// i filed an upstream issue
/// https://github.com/3Hren/msgpack-rust/issues/244
enum FakeResult<T, E> {
    Ok(T),
    Err(E),
}

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

            impl std::convert::TryFrom<Option<$t>> for $crate::SerializedBytes {
                type Error = $crate::SerializedBytesError;
                fn try_from(t: Option<$t>) -> Result<$crate::SerializedBytes, $crate::SerializedBytesError> {
                    match $crate::rmp_serde::to_vec_named(&t) {
                        Ok(v) => Ok($crate::SerializedBytes(v)),
                        Err(e) => Err($crate::SerializedBytesError::ToBytes(e.to_string())),
                    }
                }
            }

            impl<S: $crate::serde::Serialize> std::convert::TryFrom<Result<$t, S>> for $crate::SerializedBytes {
                type Error = $crate::SerializedBytesError;
                fn try_from(r: Result<$t, S>) -> Result<$crate::SerializedBytes, $crate::SerializedBytesError> {
                    match $crate::rmp_serde::to_vec_named(&r) {
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

            impl std::convert::TryFrom<$crate::SerializedBytes> for Option<$t> {
                type Error = $crate::SerializedBytesError;
                fn try_from(sb: $crate::SerializedBytes) -> Result<Option<$t>, $crate::SerializedBytesError> {
                    match $crate::rmp_serde::from_read_ref(&sb.0) {
                        Ok(v) => Ok(v),
                        Err(e) => Err($crate::SerializedBytesError::FromBytes(e.to_string())),
                    }
                }
            }

            impl<D: $crate::serde::de::DeserializeOwned> std::convert::TryFrom<$crate::SerializedBytes> for Result<$t, D> {
                type Error = $crate::SerializedBytesError;
                fn try_from(sb: $crate::SerializedBytes) -> Result<Result<$t, D>, $crate::SerializedBytesError> {
                    match $crate::rmp_serde::from_read_ref(&sb.0) {
                        Ok(FakeResult::Ok(v)) => Ok(Ok(v)),
                        Ok(FakeResult::Err(e)) => Ok(Err(e)),
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

    /// struct with raw bytes in it
    #[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
    struct Baz {
        wow: Option<Option<Result<Vec<u8>, Result<String, String>>>>,
    }

    holochain_serial!(Foo, Bar, Baz);

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
            Result<Foo, Bar>,
            Ok(fixture_foo()),
            vec![129, 0, 129, 165, 105, 110, 110, 101, 114, 163, 102, 111, 111]
        );

        do_test!(
            Result<Foo, Bar>,
            Err(fixture_bar()),
            vec![129, 1, 129, 168, 119, 104, 97, 116, 101, 118, 101, 114, 147, 1, 2, 3]
        );

        do_test!(
            Result<Bar, Foo>,
            Ok(fixture_bar()),
            vec![129, 0, 129, 168, 119, 104, 97, 116, 101, 118, 101, 114, 147, 1, 2, 3]
        );

        do_test!(
            Result<Bar, Foo>,
            Err(fixture_foo()),
            vec![129, 1, 129, 165, 105, 110, 110, 101, 114, 163, 102, 111, 111]
        );

        do_test!(
            Option<Foo>,
            Some(fixture_foo()),
            vec![129, 165, 105, 110, 110, 101, 114, 163, 102, 111, 111]
        );

        do_test!(
            Option<Foo>,
            None,
            vec![192]
        );

        do_test!(
            Option<Baz>,
            Some(Baz{ wow: Some(Some(Ok(Err("foo".into())))) }),
            vec![192]
        );
    }

}
