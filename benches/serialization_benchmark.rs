use bincode::{serialize, deserialize};
use bcs::{to_bytes, from_bytes};
use protobuf::Message;
use serde::{Serialize, Deserialize};
use serde_json::{to_string, from_str};
use borsh::{BorshSerialize, BorshDeserialize};
use criterion::{black_box, criterion_group, criterion_main, Criterion};
mod message;
mod trallocator;
use message as proto;
use std::alloc::System;

#[global_allocator]
static GLOBAL: trallocator::Trallocator<System> 
    = trallocator::Trallocator::new(System);

// Custom memory tracker
#[derive(Default)]
struct MemoryTracker {
    initial: u64,
    after_serialize: u64,
    after_deserialize: u64,
}

impl MemoryTracker {
    fn new() -> Self {
        MemoryTracker {
            initial: 0,
            after_serialize: 0,
            after_deserialize: 0,
        }
    }

    fn log_initial(&mut self) {
        GLOBAL.reset();
        self.initial = GLOBAL.get();
    }

    fn log_after_serialize(&mut self) {
        self.after_serialize = GLOBAL.get();
        GLOBAL.reset();
    }

    fn log_after_deserialize(&mut self) {
        self.after_deserialize = GLOBAL.get();
        GLOBAL.reset();
    }

    fn print_summary(&self, operation: &str) {
        println!("--- {} ---", operation);
        println!("Memory before: {} bytes", self.initial);
        println!("Memory after serialize: {} bytes", self.after_serialize);
        println!("Memory after deserialize: {} bytes", self.after_deserialize);
        println!("Memory used during {}: {} bytes", operation, self.after_deserialize + self.after_serialize - self.initial);
        println!("---------------------\n");
    }
}

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
    let mut tracker = MemoryTracker::new();

    tracker.log_initial();
    c.bench_function("bincode serialize", |b| {
        b.iter(|| serialize(&black_box(&test_data)).unwrap())
    });
    tracker.log_after_serialize();

    c.bench_function("bincode deserialize", |b| {
        b.iter(|| deserialize::<TestData>(&black_box(&serialize(&test_data).unwrap())).unwrap())
    });
    tracker.log_after_deserialize();

    tracker.print_summary("Bincode");
}

// BCS
fn benchmark_bcs(c: &mut Criterion) {
    let test_data = TestData::new();
    let mut tracker = MemoryTracker::new();

    tracker.log_initial();
    c.bench_function("bcs serialize", |b| {
        b.iter(|| to_bytes(&black_box(&test_data)).unwrap())
    });
    tracker.log_after_serialize();

    c.bench_function("bcs deserialize", |b| {
        b.iter(|| from_bytes::<TestData>(&black_box(&to_bytes(&test_data).unwrap())).unwrap())
    });
    tracker.log_after_deserialize();

    tracker.print_summary("BCS");
}

// Protobuf
fn benchmark_protobuf(c: &mut Criterion) {
    let test_data = proto::TestData {
        id: 1,
        name: "Rust".to_string(),
        active: true,
        ..Default::default()
    };
    let mut tracker = MemoryTracker::new();

    tracker.log_initial();
    let serialized_data = test_data.write_to_bytes().unwrap();
    c.bench_function("protobuf serialize", |b| {
        b.iter(|| black_box(test_data.write_to_bytes().unwrap()))
    });
    tracker.log_after_serialize();

    c.bench_function("protobuf deserialize", |b| {
        b.iter(|| black_box(proto::TestData::parse_from_bytes(&serialized_data).unwrap()))
    });
    tracker.log_after_deserialize();

    tracker.print_summary("Protobuf");
}

// Serde JSON
fn benchmark_serde_json(c: &mut Criterion) {
    let test_data = TestData::new();
    let mut tracker = MemoryTracker::new();

    tracker.log_initial();
    c.bench_function("serde_json serialize", |b| {
        b.iter(|| to_string(&black_box(&test_data)).unwrap())
    });
    tracker.log_after_serialize();

    c.bench_function("serde_json deserialize", |b| {
        let serialized_data = to_string(&test_data).unwrap();
        b.iter(|| from_str::<TestData>(&black_box(&serialized_data)).unwrap())
    });
    tracker.log_after_deserialize();

    tracker.print_summary("Serde JSON");
}

// Borsh
fn benchmark_borsh(c: &mut Criterion) {
    let test_data = TestData::new();
    let mut tracker = MemoryTracker::new();

    tracker.log_initial();
    c.bench_function("borsh serialize", |b| {
        b.iter(|| borsh::to_vec(&test_data).unwrap())
    });
    tracker.log_after_serialize();

    c.bench_function("borsh deserialize", |b| {
        let serialized_data = borsh::to_vec(&test_data).unwrap();
        b.iter(|| TestData::try_from_slice(&black_box(&serialized_data)).unwrap())
    });
    tracker.log_after_deserialize();

    tracker.print_summary("Borsh");
}

// Group all benchmarks
fn criterion_benchmark(c: &mut Criterion) {
    GLOBAL.reset();
    benchmark_bincode(c);
    benchmark_bcs(c);
    benchmark_protobuf(c);
    benchmark_serde_json(c);
    benchmark_borsh(c);
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
