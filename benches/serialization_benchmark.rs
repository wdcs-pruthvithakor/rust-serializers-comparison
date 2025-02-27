extern crate jemalloc_sys;

use bincode::{serialize, deserialize};
use bcs::{to_bytes, from_bytes};
use protobuf::Message;
use serde::{Serialize, Deserialize};
use serde_json::{to_string, from_str};
use borsh::{BorshSerialize, BorshDeserialize};
use criterion::{black_box, criterion_group, criterion_main, Criterion};
mod message;
use message as proto;

// Assuming the generated Protobuf code is in the `src/proto` directory

// Define a sample struct for serialization
#[derive(Serialize, Deserialize, BorshSerialize, BorshDeserialize, Debug, Clone)]
struct TestData {
    id: u32,
    name: String,
    active: bool,
}

impl TestData {
    fn new() -> Self {
        TestData {

            id: 1,
            name: "Rust".to_string(),
            active: true,
        }
    }
}

// Serialization and Deserialization functions for different formats

// Bincode
fn benchmark_bincode(c: &mut Criterion) {
    let test_data = TestData::new();
    c.bench_function("bincode serialize", |b| {
        b.iter(|| serialize(&black_box(&test_data)).unwrap())
    });
    c.bench_function("bincode deserialize", |b| {
        let serialized_data = serialize(&test_data).unwrap();
        b.iter(|| deserialize::<TestData>(&black_box(&serialized_data)).unwrap())
    });
}

// BCS
fn benchmark_bcs(c: &mut Criterion) {
    let test_data = TestData::new();
    c.bench_function("bcs serialize", |b| {
        b.iter(|| to_bytes(&black_box(&test_data)).unwrap())
    });
    c.bench_function("bcs deserialize", |b| {
        let serialized_data = to_bytes(&test_data).unwrap();
        b.iter(|| from_bytes::<TestData>(&black_box(&serialized_data)).unwrap())
    });
}

use crate::proto::TestData as ProtoTestData;
// Protobuf
fn benchmark_protobuf(c: &mut Criterion) {
    // Creating a new TestData instance with mock data
    
    let test_data = ProtoTestData {
        id: 1,
        name: "Rust".to_string(),
        active: true,
        ..Default::default()  // Default values for other fields
    };

    // Serialize the `test_data` to protobuf byte format
    let serialized_data = test_data.write_to_bytes().unwrap();

    // Benchmarking the serialization process
    c.bench_function("protobuf serialize", |b| {
        b.iter(|| black_box(test_data.write_to_bytes().unwrap()))
    });

    // Benchmarking the deserialization process
    c.bench_function("protobuf deserialize", |b| {
        b.iter(|| black_box(ProtoTestData::parse_from_bytes(&serialized_data).unwrap()))
    });
}

// Serde JSON
fn benchmark_serde_json(c: &mut Criterion) {
    let test_data = TestData::new();
    c.bench_function("serde_json serialize", |b| {
        b.iter(|| to_string(&black_box(&test_data)).unwrap())
    });
    c.bench_function("serde_json deserialize", |b| {
        let serialized_data = to_string(&test_data).unwrap();
        b.iter(|| from_str::<TestData>(&black_box(&serialized_data)).unwrap())
    });
}

// Borsh
fn benchmark_borsh(c: &mut Criterion) {
    let test_data = TestData::new();
    c.bench_function("borsh serialize", |b| {
        b.iter(|| borsh::to_vec(&test_data).unwrap())
    });
    c.bench_function("borsh deserialize", |b| {
        let serialized_data = borsh::to_vec(&test_data).unwrap();
        b.iter(|| TestData::try_from_slice(&black_box(&serialized_data)).unwrap())
    });
}

// Group all benchmarks
fn criterion_benchmark(c: &mut Criterion) {
    benchmark_bincode(c);
    benchmark_bcs(c);
    benchmark_protobuf(c);
    benchmark_serde_json(c);
    benchmark_borsh(c);
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
