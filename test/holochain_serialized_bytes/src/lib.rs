//! at the moment this just tests roughly what it looks like to consume serialized_bytes downstream
//! which is ideally as little as possible of the underlying abstractions leaked
extern crate holochain_serialized_bytes;

#[cfg(test)]
pub mod tests {

    use holochain_serialized_bytes::prelude::*;

    // Serialize and Deserialize handled by prelude
    #[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
    struct Foo {
        inner: String,
    }

    enum FooError {
        Serialize,
    }

    // SerializedBytesError handled by prelude
    impl From<SerializedBytesError> for FooError {
        fn from(_: SerializedBytesError) -> Self {
            FooError::Serialize
        }
    }

    // holochain_serial! in prelude
    holochain_serial!(Foo);

    #[test]
    pub fn foo_test() {
        let foo = Foo {
            inner: "foo".into(),
        };

        // SerializedBytes and TryInto already handled by prelude
        let sb: SerializedBytes = foo.clone().try_into().unwrap();
        // TryFrom handled by prelude
        let other_foo = Foo::try_from(sb).unwrap();

        assert_eq!(foo, other_foo);
    }
}
