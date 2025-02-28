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
use std::collections::HashMap;
use std::sync::{Mutex, Arc};
use once_cell::sync::Lazy;

#[global_allocator]
static GLOBAL: trallocator::Trallocator<System> 
    = trallocator::Trallocator::new(System);

// Global storage for benchmark results with thread-safe access
static BENCHMARK_RESULTS: Lazy<Arc<Mutex<HashMap<String, BenchmarkResults>>>> = 
    Lazy::new(|| Arc::new(Mutex::new(HashMap::new())));

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

// Structure to store benchmark results
#[derive(Clone)]
struct BenchmarkResults {
    serialize_time_ns: f64,
    serialize_ops_per_sec: u64,
    deserialize_time_ns: f64,
    deserialize_ops_per_sec: u64,
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

// Store benchmark results
fn store_results(format: &str, serialize_ns: f64, deserialize_ns: f64) {
    let serialize_ops = (1_000_000_000.0 / serialize_ns) as u64;
    let deserialize_ops = (1_000_000_000.0 / deserialize_ns) as u64;
    
    let mut results = BENCHMARK_RESULTS.lock().unwrap();
    results.insert(format.to_string(), BenchmarkResults {
        serialize_time_ns: serialize_ns,
        serialize_ops_per_sec: serialize_ops,
        deserialize_time_ns: deserialize_ns,
        deserialize_ops_per_sec: deserialize_ops,
    });
}

// Print results as a formatted table
fn print_results_table() {
    println!("\n{:-^80}", " Serialization Benchmark Results ");
    
    // Print table header
    println!("{:<12} | {:<20} | {:<20} | {:<20} | {:<20}", 
             "Format", 
             "Serialization Time (ns)", 
             "Serialization Ops/sec",
             "Deserialization Time (ns)",
             "Deserialization Ops/sec");
    
    println!("{:-<12}-+-{:-<20}-+-{:-<20}-+-{:-<20}-+-{:-<20}", 
             "", "", "", "", "");
    
    // Print table rows
    let results = BENCHMARK_RESULTS.lock().unwrap();
    
    // Sort formats alphabetically for consistent output
    let mut formats: Vec<&String> = results.keys().collect();
    formats.sort();
    
    for format in formats {
        if let Some(result) = results.get(format) {
            println!("{:<12} | {:<20.2} | {:<20} | {:<20.2} | {:<20}", 
                     format,
                     result.serialize_time_ns,
                     format!("{} ops/sec", result.serialize_ops_per_sec),
                     result.deserialize_time_ns,
                     format!("{} ops/sec", result.deserialize_ops_per_sec));
        }
    }
    
    println!("{:-^80}", "");
}

// Bincode
fn benchmark_bincode(c: &mut Criterion) {
    let test_data = TestData::new();
    let mut tracker = MemoryTracker::new();

    tracker.log_initial();
    let mut group = c.benchmark_group("bincode");
    
    group.bench_function("serialize", |b| {
        b.iter(|| serialize(&black_box(&test_data)).unwrap())
    });
    tracker.log_after_serialize();
    
    let serialized_data = serialize(&test_data).unwrap();
    
    group.bench_function("deserialize", |b| {
        b.iter(|| deserialize::<TestData>(&black_box(&serialized_data)).unwrap())
    });
    tracker.log_after_deserialize();
    
    group.finish();
    
    tracker.print_summary("Bincode");
}

// BCS
fn benchmark_bcs(c: &mut Criterion) {
    let test_data = TestData::new();
    let mut tracker = MemoryTracker::new();

    tracker.log_initial();
    let mut group = c.benchmark_group("bcs");
    
    group.bench_function("serialize", |b| {
        b.iter(|| to_bytes(&black_box(&test_data)).unwrap())
    });
    tracker.log_after_serialize();
    
    let serialized_data = to_bytes(&test_data).unwrap();
    
    group.bench_function("deserialize", |b| {
        b.iter(|| from_bytes::<TestData>(&black_box(&serialized_data)).unwrap())
    });
    tracker.log_after_deserialize();
    
    group.finish();
    
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
    let mut group = c.benchmark_group("protobuf");
    
    group.bench_function("serialize", |b| {
        b.iter(|| test_data.write_to_bytes().unwrap())
    });
    
    tracker.log_after_serialize();
    let serialized_data = test_data.write_to_bytes().unwrap();
    
    group.bench_function("deserialize", |b| {
        b.iter(|| proto::TestData::parse_from_bytes(&black_box(&serialized_data)).unwrap())
    });
    tracker.log_after_deserialize();
    
    group.finish();
    
    tracker.print_summary("Protobuf");
}

// Serde JSON
fn benchmark_serde_json(c: &mut Criterion) {
    let test_data = TestData::new();
    let mut tracker = MemoryTracker::new();

    tracker.log_initial();
    let mut group = c.benchmark_group("serde_json");
    
    group.bench_function("serialize", |b| {
        b.iter(|| to_string(&black_box(&test_data)).unwrap())
    });
    tracker.log_after_serialize();
    
    let serialized_data = to_string(&test_data).unwrap();
    
    group.bench_function("deserialize" , |b| {
        b.iter(|| from_str::<TestData>(&black_box(&serialized_data)).unwrap())
    });
    tracker.log_after_deserialize();
    
    group.finish();
    
    tracker.print_summary("Serde JSON");
}

// Borsh
fn benchmark_borsh(c: &mut Criterion) {
    let test_data = TestData::new();
    let mut tracker = MemoryTracker::new();

    tracker.log_initial();
    let mut group = c.benchmark_group("borsh");
    
    group.bench_function("serialize", |b| {
        b.iter(|| borsh::to_vec(&black_box(&test_data)).unwrap())
    });
    
    tracker.log_after_serialize();
    let serialized_data = borsh::to_vec(&test_data).unwrap();
    
    group.bench_function("deserialize", |b| {
        b.iter(|| TestData::try_from_slice(&black_box(&serialized_data)).unwrap())
    });
    
    group.finish();
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

    // After criterion runs, we can parse the json files.
    analyze_criterion_results("bincode");
    analyze_criterion_results("bcs");
    analyze_criterion_results("protobuf");
    analyze_criterion_results("serde_json");
    analyze_criterion_results("borsh");

    // Print the formatted table after all benchmarks are run
    print_results_table();
}
use serde_json::Value;
use std::fs;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;

fn analyze_criterion_results(group_name: &str) {
    let group_path = format!("target/criterion/{}", group_name);
    let group_dir = Path::new(&group_path);

    if !group_dir.exists() || !group_dir.is_dir() {
        println!("Group directory not found.");
        return;
    }

    let mut total_serialize_ns = 0.0;
    let mut total_deserialize_ns = 0.0;

    if let Ok(entries) = fs::read_dir(group_dir) {
        for entry in entries.flatten() {
            let bench_path = entry.path();
            if bench_path.is_dir() {
                let estimates_path = bench_path.join("base/estimates.json");
                if estimates_path.exists() {
                    if let Ok(file) = File::open(estimates_path) {
                        let reader = BufReader::new(file);
                        if let Ok(json) = serde_json::from_reader::<_, Value>(reader) {
                            if bench_path.file_name().unwrap().to_str().unwrap() == "serialize" {
                                if let Some(slope) = json["slope"]["point_estimate"].as_f64() {
                                    total_serialize_ns += slope;
                                }
                            } else if bench_path.file_name().unwrap().to_str().unwrap() == "deserialize" {
                                if let Some(slope) = json["slope"]["point_estimate"].as_f64() {
                                    total_deserialize_ns += slope;
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    let total_serialize_ops = (1_000_000_000.0 / total_serialize_ns) as u64;
    let total_deserialize_ops = (1_000_000_000.0 / total_deserialize_ns) as u64;

    store_results(group_name, total_serialize_ns, total_deserialize_ns);
    println!("Total estimated serialize time of group '{}': {:.3} ns", group_name, total_serialize_ns);
    println!("Total estimated deserialize time of group '{}': {:.3} ns", group_name, total_deserialize_ns);
    println!("Total estimated serialize ops of group '{}': {:.3} ops/sec", group_name, total_serialize_ops);
    println!("Total estimated deserialize ops of group '{}': {:.3} ops/sec", group_name, total_deserialize_ops);
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);