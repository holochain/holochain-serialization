use criterion::BenchmarkId;
use criterion::Throughput;
use criterion::{criterion_group, criterion_main, Criterion};
use holochain_serialized_bytes::prelude::*;

#[derive(serde::Serialize, serde::Deserialize, SerializedBytes)]
struct StringNewType(String);

pub fn round_trip_string(c: &mut Criterion) {
    let mut group = c.benchmark_group("round_trip_string");

    for n in vec![0, 1, 1_000, 1_000_000] {
        group.throughput(Throughput::Bytes(n as _));
        group.bench_with_input(BenchmarkId::from_parameter(n), &n, |b, &n| {
            b.iter_batched(
                || ".".repeat(n).to_string(),
                |s| {
                    StringNewType::try_from(SerializedBytes::try_from(StringNewType(s)).unwrap())
                        .unwrap();
                },
                criterion::BatchSize::PerIteration,
            );
        });
    }

    group.finish();
}

#[derive(serde::Serialize, serde::Deserialize, SerializedBytes)]
struct GenericBytesNewType(Vec<u8>);
#[derive(serde::Serialize, serde::Deserialize, SerializedBytes)]
struct SpecializedBytesNewType(#[serde(with = "serde_bytes")] Vec<u8>);

pub fn round_trip_bytes(c: &mut Criterion) {
    let mut group = c.benchmark_group("round_trip_bytes");

    macro_rules! do_it {
        ( $newtype:tt ) => {
            for n in vec![0, 1, 1_000, 1_000_000] {
                group.throughput(Throughput::Bytes(n as _));
                group.sample_size(10);
                group.bench_with_input(BenchmarkId::new(stringify!($newtype), n), &n, |b, &n| {
                    b.iter_batched(
                        || vec![0_u8; n],
                        |s| {
                            <$newtype>::try_from(SerializedBytes::try_from($newtype(s)).unwrap())
                                .unwrap();
                        },
                        criterion::BatchSize::PerIteration,
                    );
                });
            }
        };
    };

    do_it!(GenericBytesNewType);
    do_it!(SpecializedBytesNewType);

    group.finish();
}

#[derive(serde::Serialize, serde::Deserialize, SerializedBytes)]
struct SerializedBytesNewType(SerializedBytes);

pub fn round_nested(c: &mut Criterion) {
    let mut group = c.benchmark_group("round_nested");

    macro_rules! do_it {
        ( $newtype:tt ) => {
            for n in vec![0, 1, 1_000, 1_000_000] {
                group.throughput(Throughput::Bytes(n as _));
                group.sample_size(10);
                group.bench_with_input(BenchmarkId::new(stringify!($newtype), n), &n, |b, &n| {
                    b.iter_batched(
                        || vec![0_u8; n],
                        |s| {
                            let inner = SerializedBytes::try_from($newtype(s)).unwrap();
                            <$newtype>::try_from(SerializedBytes::try_from(inner).unwrap())
                                .unwrap();
                        },
                        criterion::BatchSize::PerIteration,
                    );
                });
            }
        };
    };

    do_it!(GenericBytesNewType);
    do_it!(SpecializedBytesNewType);

    group.finish();
}

criterion_group!(bench, round_trip_string, round_trip_bytes, round_nested);

criterion_main!(bench);
