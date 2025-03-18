#[cfg(test)]
pub mod tests {

    use serde_json::Value;

    use holochain_serialized_bytes::prelude::*;
    use std::convert::TryInto;

    #[test]
    fn conductor_api() {
        use rmp_serde::Deserializer;

        #[derive(Serialize, Deserialize, Debug)]
        #[serde(rename_all = "snake_case", tag = "type", content = "data")]
        enum ConductorApi {
            Request { param: i32 },
        }

        let request = ConductorApi::Request { param: 100 };
        let request_encoded = encode(&request).unwrap();
        assert_eq!(
            request_encoded,
            [
                130, 164, 116, 121, 112, 101, 167, 114, 101, 113, 117, 101, 115, 116, 164, 100, 97,
                116, 97, 129, 165, 112, 97, 114, 97, 109, 100
            ]
        );

        let mut deserializer = Deserializer::new(&*request_encoded);
        let value: Value = Deserialize::deserialize(&mut deserializer).unwrap();
        let json_string = serde_json::to_string(&value).unwrap();
        assert_eq!(r#"{"type":"request","data":{"param":100}}"#, json_string);
    }

    /// struct with a utf8 string in it
    #[derive(Serialize, Deserialize, PartialEq, Debug, Clone, SerializedBytes)]
    struct Foo {
        inner: String,
    }

    /// struct with raw bytes in it
    #[derive(Serialize, Deserialize, PartialEq, Debug, Clone, SerializedBytes)]
    struct Bar {
        whatever: Vec<u8>,
    }

    #[derive(Serialize, Deserialize, PartialEq, Debug, Clone, SerializedBytes)]
    enum BazResult {
        Ok(Vec<u8>),
        Err(String),
    }

    /// struct with raw bytes in it
    #[derive(Serialize, Deserialize, PartialEq, Debug, Clone, SerializedBytes)]
    struct Baz {
        wow: Option<BazResult>,
    }

    #[derive(Serialize, Deserialize, PartialEq, Debug, Clone, SerializedBytes)]
    struct Tiny(u8);

    #[derive(Serialize, Deserialize, PartialEq, Debug, Clone, SerializedBytes)]
    struct SomeBytes(Vec<u8>);

    #[derive(Serialize, Deserialize, PartialEq, Debug, Clone, SerializedBytes)]
    struct IncludesSerializedBytes {
        inner: SerializedBytes,
    }

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

    #[cfg(feature = "trace")]
    #[test]
    fn test_trace() {
        let collector = tracing_subscriber::fmt()
            .with_max_level(tracing::Level::TRACE)
            .finish();

        #[derive(Debug)]
        struct BadSerialize;

        impl serde::Serialize for BadSerialize {
            fn serialize<S>(&self, _serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                Err(serde::ser::Error::custom("Cannot serialize!"))
            }
        }

        tracing::subscriber::with_default(collector, || {
            let bad_bytes = vec![1, 2, 3];
            let encode_error: Result<Vec<u8>, SerializedBytesError> = encode(&BadSerialize);
            assert_eq!(
                encode_error,
                Err(SerializedBytesError::Serialize("Cannot serialize!".into()))
            );

            let decode_error: Result<String, SerializedBytesError> = decode(&bad_bytes);
            assert_eq!(
                decode_error,
                Err(SerializedBytesError::Deserialize(
                    "invalid type: integer `1`, expected a string".into()
                ))
            );

            let encode: Result<Vec<u8>, SerializedBytesError> = encode(&());
            assert_eq!(encode.unwrap(), vec![192],);

            let decode: Result<(), SerializedBytesError> = decode(&vec![192]);
            assert_eq!(decode.unwrap(), ());
        });
    }

    #[test]
    fn round_trip() {
        macro_rules! do_test {
            ( $t:ty, $i:expr, $o:expr ) => {{
                let i = $i;
                let sb: SerializedBytes = i.clone().try_into().unwrap();
                // this isn't for testing it just shows how the debug output looks
                println!("{:?}", &sb);

                assert_eq!(&$o, sb.bytes(),);

                let returned: $t = sb.try_into().unwrap();

                assert_eq!(returned, i);

                // as ref
                let sb2 = SerializedBytes::try_from(&i).unwrap();

                assert_eq!(&$o, sb2.bytes());
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
            vec![129, 163, 119, 111, 119, 129, 162, 79, 107, 147, 2, 5, 6]
        );

        do_test!(Tiny, Tiny(5), vec![5]);

        do_test!(
            SomeBytes,
            SomeBytes(vec![1_u8, 90_u8, 155_u8]),
            vec![147, 1, 90, 204, 155]
        );

        do_test!((), (), vec![192]);

        do_test!(
            IncludesSerializedBytes,
            IncludesSerializedBytes {
                inner: fixture_foo().try_into().unwrap()
            },
            vec![
                129, 165, 105, 110, 110, 101, 114, 196, 11, 129, 165, 105, 110, 110, 101, 114, 163,
                102, 111, 111
            ]
        );
    }

    #[test_fuzz::test_fuzz]
    fn round_any_string(inner: String) {
        let foo = Foo { inner };
        let _: Foo = decode(&encode(&foo).unwrap()).unwrap();
    }

    #[test]
    fn round_a_string() {
        round_any_string("foo".into());
    }

    #[test_fuzz::test_fuzz]
    fn provide_own_bytes(bytes: Vec<u8>) {
        // let bytes = vec![1_u8, 90_u8, 155_u8];
        let own_bytes = UnsafeBytes::from(bytes.clone());
        let sb: SerializedBytes = own_bytes.clone().into();

        assert_eq!(sb.bytes(), &bytes);

        let own_bytes_restored: UnsafeBytes = sb.into();

        let own_bytes_vec: Vec<u8> = own_bytes.into();
        assert_eq!(bytes, own_bytes_vec);
        let own_bytes_restored_vec: Vec<u8> = own_bytes_restored.into();
        assert_eq!(bytes, own_bytes_restored_vec);
    }

    #[derive(Deserialize, Debug)]
    pub struct UnlikelyToDeserialize {
        pub foo: u32,
        pub bar: String,
        #[serde(with = "serde_bytes")]
        pub baz: Vec<u8>,
    }

    #[test_fuzz::test_fuzz]
    fn things_that_probably_wont_deserialize(bytes: Vec<u8>) {
        match decode::<_, UnlikelyToDeserialize>(&bytes) {
            Ok(_) => { /* unlikely! */ }
            Err(SerializedBytesError::Deserialize(_)) => { /* likely! */ }
            _ => unreachable!(),
        }
    }

    #[test]
    fn thing_that_wont_deserialize() {
        things_that_probably_wont_deserialize(vec![1, 2, 3]);
    }

    #[test]
    fn default_test() {
        assert_eq!(&vec![192_u8], SerializedBytes::default().bytes());
    }
}