# Holochain Serialized Bytes

## Why

Holochain has specific requirements for serialization that need to be enforced
consistently in a "fool proof" way.

- Support arbitrary binary data
- Consistent bytes for _canonical representations_ of things for cryptographic use
- Support multiple languages
- Wide industry usage and tooling across many languages including Rust
- Reasonably fast and compact
- Human readable for debugging
- Able to preserve semantics for debugging
- Able to derive a sane serialization for the 99% of use cases
- Able to manually implement serialization for edge-cases (e.g. Merkle Trees)
- Able to be form a base infrastructure layer for both wasm and networking
- Able to leverage the Rust compiler as much as possible to avoid bugs and help refactors

There are some additional goals of this crate that I'd like to make explicit
that informed some of the code design choices.

- Canonical representations of bytes of data should be enforced as canonical by the compiler at the type level
- For two systems sharing bytes and a shared type crate on the same version, round tripping bytes should be guaranteed safe by the compiler
- The design of `SerializedBytes` should be such that end-user-happ-devs should not need to be aware of it for BAU tasks
- Internal dependencies (e.g. serde messagepack handling) of this crate should not bleed into downstream concerns due to macros or whatever
- Everything internal to serialization should be upgradeable and refactorable without any external interface changes for consumers

## How

The usage is very simple. Most of this README is dedicated to discussing design
decisions as the crate is fairly opinionated to minimise "foot guns" and maximise
how much the compiler can do for us, given that serialized data de facto erases
type information across systems (e.g. across wasm guest/host boundary).

A single `SerializedBytes` new type that includes a `Vec<u8>` of bytes.

These bytes are by default to be MessagePack serialized binary data.

https://msgpack.org/

There is constructor for it e.g. there is no `SerializedBytes::new()`.

Implement `TryFrom<SerializedBytes> for Foo` and `TryFrom<Foo> for SerializedBytes`.

There is one macro `holochain_serial!` that accepts a list of types.

Each type passed to `holochain_serial!` will get default `TryFrom` using rust messagepack.

https://github.com/3Hren/msgpack-rust

Definitely use `holochain_serial!` for all your types if you can.

You also need to derive or implement `Serialize` and `Deserialize` for your types.

It looks like this:

```rust
/// struct with a utf8 string in it
#[derive(Serialize, Deserialize)]
struct Foo {
    inner: String,
}

/// struct with raw bytes in it
#[derive(Serialize, Deserialize)]
struct Bar {
    whatever: Vec<u8>,
}

/// register our types for messagepack implementation
holochain_serial!(Foo, Bar);

let foo = Foo { inner: "foo".into() };

let serialized_bytes: SerializedBytes = foo.try_into().unwrap();

println!("{:?}", &serialized_bytes); // debugs to json: {"inner":"foo"}
println!("{:?}", serialized_bytes.bytes()); // messagepack bytes [129_u8, 165_u8, 105_u8, 110_u8, 110_u8, 101_u8, 114_u8, 163_u8, 102_u8, 111_u8, 111_u8,]

let deserialized_foo: Foo = serialized_bytes.try_into().unwrap();
```

## Debugging

For debugging, the internal messagepack serialized bytes are transcoded to JSON
using `serde-transcode`. This means that you will see JSON output from `"{:?}"`
which is much easier to read than binary from messagepack.

If you want a read only view of the actual messagepack bytes call the `.bytes()` method.

## Design limitations and choices

These design limitations all exist to keep things simple.

I acknowledge that some of these limitations are quite strict and may even feel quite
restrictive in a "local optimisation" kind of way, but in context of where and
how `SerializedBytes` is intended to be employed, it should all be for the greater good ;)

### Messagepack limits

All limitations from messagepack: https://github.com/msgpack/msgpack/blob/master/spec.md#limitation

Any/all bugs from the rust messagepack implementation https://github.com/3Hren/msgpack-rust

### Immutable

The `SerializedBytes` is intented to be immutable because it is a canonical
representation of something else.

### SerializedBytes TryFrom SerializedBytes is a no-op, nesting inside another struct double-serializes

Moving from `SerializedBytes` to `SerializedBytes` is a no-op.

Even though it is binary data that messagepack could represent as a nested binary
message, it won't because Rust won't trigger the serialization logic.

If you nest `SerializedBytes` inside another struct it WILL be double serialized.

E.g. this (JSON representation):

```
// Foo { inner: String }
// foo = Foo { inner: "foo".into() }
{"inner":"foo"}
```

becomes this (JSON representation) when converted into `SerializedBytes` then nested in another struct:

```
// Bar { inner: SerializedBytes }
// bar = Bar { inner: foo.into() }
{"inner":[129,165,105,110,110,101,114,163,102,111,111]}
```

### Semantic and shared types only

We intentionally do not support moving from Rust primitives to/from `SerializedBytes`.

The only exception is `()` which maps to `nil` in messagepack.

I.e. `SerializedBytes::try_from(())` is valid.

This is because everything other than nothing (nil) has ambiguous meaning when used across systems.

For example, consider `Ok(())` vs. `ValidationResult::Ok`.

The former quickly becomes confusing when shared e.g. across a wasm host/guest system boundary.

It gets nastier when these things start to nest like `Ok(Err("Some string"))`
where the different levels of nesting originate from different systems.

It gets nastier when dealing with systems (like wasm) that reference linear memory directly
and we're dealing with lengths/offsets to other data as integers.

So maybe some integer is a reference to memory of some data that is an `Ok(Err("Some string"))`,
and maybe _that_ integer is serialized somehow to be sent somewhere else, like across
the wasm host/guest boundary.

It gets nastier again when serialization is represented as strings (e.g. JSON)
and we're trying to accept data serialized by other systems in a nested way that
can cross multiple nested function calls that can hit other arbitrary systems,
that all represent their data like the above...

Very quickly we end up with something the compiler can't help with because it is
all essentially "stringly typed" full of backslashes to escape it all.

https://www.xkcd.com/1638/

Things like `Option` and `Result` aren't representation of domain specific data
anyway, they are conceits of compiler type systems. They don't need to be
serialized because the type information needs to be given to the compiler by the
developer for rust to be able to deserialize anyway.

https://www.youtube.com/watch?v=YR5WdGrpoug&feature=youtu.be

To put it another way, `Result` tells the compiler something about the runtime
behaviour of a function, it doesn't represent "something" in the real world or
domain. `Option` also tells the compiler to allow the absence of something at
runtime, which doesn't need to be serialized, it can simply _be absent_ in
serialized data.

Of course, we may need to represent a closed set of possible result types, like
`ValidationResult` or `CallbackResult` and this is something that we can use an
enum for. In Holochain world a `ValidationResult` is something in the real world,
that is worth serializing, it feeds into cryptographically signed claims about
the validity of other things according to a set of rules, so the serialized
representation feeds into a cryptographic proof, which can't be said about a
generic Rust `Result` that simply means "some function may fail".

The closest we get to "needing" a `Result` is to represent the return value of
imported/exported functions between a wasm host/guest, but even this would work
and could even benefit from `WasmHostResult` and `WasmGuestResult` enums to
track the provenence of any errors.

Other primitives like strings and integers are also NOT supported to directly
move between `SerializedBytes` and this is by design.

A single serialized integer or string floating around outside of the compilation
context that produced it is mostly useless in a different compilation context.
This is especially true for strings when serialized data is also a string,
forgetting to serialize or double serializing strings is a huge problem in lots
of contexts, including security sensitive ones.

Numbers have problems where the serialization format doesn't map 1:1 with the
compiler types, e.g. when "1" exists in some serialized format it could be signed
or unsigned of any size, whereas rust treats `u8` and `i8` and `u32` as completely
different things. This is more or less of an issue depending on where you sit on
the scale between serialzation formats that are tightly coupled to the language
you are currently using vs. general purpose formats that can't assume anything
about language support.

In addition to these issues on the philosophical/domain-modelling side of things
it is also _really_ messy to "correctly" handle primitives the way we want.
Rust hasn't implemented "type specialization" yet, which makes handling things
like `Result<Result<SerializedBytes, String>, String>` lead to hundreds of lines
of (still buggy) code. See the legacy `JsonString` implementation and issues for
examples of how edge-cases in the type system can  introduce subtle bugs that
break serialization round-trips.

The current setup that avoids primatives does the heavy lifting in under 50 LOC.

- specialization RFC: https://github.com/rust-lang/rust/issues/31844
- example messy code: https://github.com/holochain/holochain-serialization/pull/15/files#diff-634e1fc4ddb3416a36776dac7cfaa965R187

**Important note:** all of these types, including `Result` and `Option` and even
`SerializedBytes`itself, all implement both `Serialize` and `Deserialize` which
means they can be used _within_ your custom struct/enum type, but please be mindful
of the above when representing domain data in a serialized format.

**Important note:** New types/tuples serialize to the same bytes as the primitive they
wrap in messagepack, so don't worry about bloating serialized data by creating
custom types. On the other hand, we are using the messagepack configuration that
keeps field names, so creating structs with long fields relative to the inner data
may add some overhead.

#### DOs

- Put types that are shared across systems in a shared crate that all systems can include as a direct dependency
- Make thoughtful use of the New Type idiom https://doc.rust-lang.org/rust-by-example/generics/new_types.html to serialize primitives
- Be mindful that serialized data should be domain specific "stuff" (what a thing is) not meta/logic level stuff

#### DON'Ts

- Put things in shared type crates that will bloat or break wasm code
- Do weird nested things with `Result` and `Option` to try and avoid the above
- Nest `SerializedBytes` in other things unless you really mean to serialize already-serialized data

### Still need to implement/derive Serialize and Deserialize

For any struct/enum you want to move into `SerializedBytes` you need to add the
two basic Serde traits. We can't magic this boilerplate away with macros (yet).

Rust guidelines state to impl `Serialize` and `Deserialize` anyway.

https://rust-lang.github.io/api-guidelines/interoperability.html#data-structures-implement-serdes-serialize-deserialize-c-serde

### Use of procedural macros instead of derive

I chose to implement `holochain_serial!(FooType, BarType, ...)` as a proc macro instead of a derive.

I think this is a little non-standard but has a few advantages around dependency management and overall boilerplate.

Derive macros require a separate `*_derive` crate, which means I can't do `$crate::SerializedBytes` which means I can't lock down fully qualified paths to things, which introduces room for mistakes and additional
boilerplate/maintenance of dependencies.

Even if we re-export the macro from the derive crate in the main crate, there would be
circular dependencies if the derive crate tried to depend on the main crate to
directly reference things in it.

Another advantage is that "macros by example" (proc macros) are simply more straightfoward to write and maintain.

https://doc.rust-lang.org/1.30.0/book/2018-edition/appendix-04-macros.html#declarative-macros-with-macro_rules-for-general-metaprogramming

Another advantage is that a proc macro gives us more future extensibility than a simple derive,
meaning potential for more deep integrations with e.g. the HDK toolkit.

### Impossible to construct SerializedBytes directly

There is no `Serialized::new()` or `SerializedBytes::from_bytes()` or whatever.

This is by design.

You MUST do this (or equivalent) every time:

```rust
let serialized_bytes: SerializedBytes = foo.try_into()?;
```

and

```rust
let foo: Foo = serialized_bytes.try_into()?;
```

Which of course means you need to define a `Foo` for everything that needs to be
formally serialized into bytes, and you need to share that `Foo` type in a crate
that everywhere that will round trip `Foo` data can use as a depenency.

In the previous iteration of serialization (which was JSON based) we allowed things like this:

```rust
// we expected you to create you're own json
let foo: String = json!({ ... });
let json_string = JsonString::from_json(foo);

// we also allowed a regular From to do the same thing
let foo: String = "{...}".into();
let json_string = JsonString::from(foo);

// RawString was a hack to "undo" the above by wrapping String in a newtype that
// is then serialized into json to allow json strings of json strings
let foo: RawString = String::from("bar").into();
let json_string = JsonString::from(foo); // internally as "\"bar\""
```

Which led to serious issues:

- Sometimes `foo` didn't contain valid JSON data at all, but `from_json()` still accepted it (or users would have to eat runtime CPU costs to validate and potentially error)
- Having `JsonString::from` and `JsonString::from_json` and `RawString` just muddied the waters with subtleties that maybe a half dozen people ever understood or cared about
- There was no guarantee that `foo` would always contain the same bytes even if the serialized data was equivalent (e.g. whitespace, field ordering, etc.) so this breaks cryptography
- It's not immediately obvious what the "correct" behaviour of nested `JsonString` structs should be (e.g. no-op vs. double serialize) as ad-hoc data never has structural boundaries in the first place
- There's no canonical way to round-trip data if it never had a canonical type/form in the first place
- Wherever the lack of canonical round-tripping (or potential for) touched the wasm host/guest boundary, the `JsonString` abstraction leaked out of the HDK into a happ level concern
- It's impossible to modify the serialization approach later (e.g. moving to messagepack) without heroic efforts (vs. simply updating the `holochain_serial!` macro in this crate)

I do understand that for some use-cases, (e.g. merkle trees), you may need exact bytes (i.e. not messagepack).

There is an escape hatch for directly importing `u8` bytes into `SerializedBytes`.

The above issues have already burned hundreds of development hours, with 105 outstanding uses of `from_json()` across 48 files, so please avoid it!

To move bytes into `SerializedBytes` use the `UnsafeBytes` struct.

`UnsafeBytes` _does_ implement `From<Vec<u8>>` and round trips through `SerializedBytes`.

The round trip between `UnsafeBytes` and `SerializedBytes` is zero-copy.

Importantly, the intent is that you __use `UnsafeBytes` as an implementation detail inside a TryFrom__.

E.g.

```rust
impl TryFrom<Foo> for SerializedBytes {
  type Error = SerializedBytesError;
  fn try_from(f: Foo) -> Result<SerializedBytes, SerializedBytesError> {
    let bytes: Vec<u8> = foo.calculate_bytes_for_foo();
    Ok(SerializedBytes::from(UnsafeBytes::from(bytes)))
  }
}
```

This allows us to maintain the rule that we always use `TryFrom` to round trip `Foo` through `SerializedBytes`.
Among other things this rule allows us to write proc macros that completely hide the
`SerializedBytes` struct from the end-user-happ-developer.
If our HDK can safely assume the existence of `TryFrom<SerializedBytes>` for _everything_
that crosses the wasm boundary we can achieve an "almost native"
(sans-primitive types, see above) working experience for typed zome functions.

__Important note:__ If you use `UnsafeBytes` the expectation is that you are NOT using messagepack any more.
Therefore, if we move away from messagepack (e.g. to BSON or something) then don't expect any compatibility with `UnsafeBytes` based code.
This is a key difference with the legacy `JsonString::from_json()` approach that assumed valid JSON, here we assume _invalid_ messagepack.

### Why not JSON?

We used JSON for a long time. It certainly has many benefits:

- Very good javascript support
- "lowest common denominator" in many ways
- Very human readable
- Arbitrary length data for all types (c.f. messagepack `i32` size limit on objects)
- Speed comparable to all other serialization formats
- Gzipped size comparable or better than other formats

Ultimately though, JSON is not a binary format and a lot of people want a binary format.

Forcing everything through verbose UTF-8 introduces messy base64 encoding, gzipping etc.
that leads to overhead and mistakes.

JSON format also suffers the need for complex escaping (backslashes) that is hard to debug by hand.

### Why not BSON or similar?

No particular reason. We had to pick something reasonable, BSON would probably be fine too.

There is a rust crate: https://github.com/mongodb/bson-rust

It didn't show up in benchmarks though: https://github.com/erickt/rust-serialization-benchmarks

And the crate is tied to mongodb c.f. messagepack being more broadly owned/starred/maintained.

If there is a strong pull for BSON we could swap out or augment `holochain_serial!` fairly easily.

### Why not some Rust-coupled format (like bincode)?

Using something tied to Rust has technical benefits:

- Compact byte representation
- Fast (sometimes crazy fast) round tripping of data

But there are deal-breaking tradeoffs for us:

- Incompatible with other languages, e.g. JavaScript over the network
- Risk of instability in byte representation across compiler versions that breaks cryptography
- Impossible to represent in a human debuggable way, can't even transcode because not self-describing
