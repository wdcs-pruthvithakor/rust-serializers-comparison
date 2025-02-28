# Rust Serializers Comparison

A benchmarking project that compares the performance and memory usage of various Rust serialization libraries.

## Overview

This project provides benchmarks for comparing different serialization formats in Rust, including:

- [Bincode](https://github.com/bincode-org/bincode) - A binary serialization format designed for Rust
- [BCS](https://github.com/diem/bcs) (Binary Canonical Serialization) - A serialization format created for the Diem blockchain
- [Protocol Buffers](https://github.com/stepancheg/rust-protobuf) - Google's language-neutral, platform-neutral extensible mechanism
- [Serde JSON](https://github.com/serde-rs/json) - A JSON serialization format using Serde
- [Borsh](https://github.com/near/borsh-rs) - Binary Object Representation Serializer for Hashing

The benchmarks measure both performance (speed) and memory usage for serialization and deserialization operations.

## Features

- Benchmark serialization and deserialization speed using Criterion
- Track memory allocation during serialization and deserialization
- Compare binary and text-based serialization formats
- Detailed performance reports

## Project Structure

```
├── benches
│   ├── message.rs           # Generated Protocol Buffers code
│   ├── mod.rs               # Module definitions
│   ├── serialization_benchmark.rs  # Main benchmark code
│   └── trallocator.rs       # Memory tracking allocator
├── Cargo.lock
├── Cargo.toml
└── src
    ├── main.rs              # Main application entry point
    └── proto
        └── message.proto    # Protocol Buffers definition file
```

## Prerequisites

- Rust (stable channel)
- Protocol Buffers compiler (`protoc`)
- Cargo and Rust's package manager

## Setup

1. **Clone the repository**:
   ```bash
   git clone https://github.com/wdcs-pruthvithakor/rust-serializers-comparison.git
   cd rust-serializers-comparison
   ```

2. **Generate Protocol Buffers Rust code**:
   To generate the `message.rs` file from the `message.proto` file, use the following command:
   ```bash
   protoc --rs_out=benches/ --proto-path=src/proto src/proto/message.proto
   ```

## Running the Benchmarks

Run all benchmarks with:

```bash
cargo bench
```

The benchmarks will output results to the console, showing both performance metrics and memory usage statistics for each serialization format.

## Memory Tracking and Benchmarking

The project uses a custom memory allocator (`Trallocator`) to track memory usage during the serialization and deserialization operations. The `MemoryTracker` struct logs the memory usage at three stages:
1. Before serialization
2. After serialization
3. After deserialization

This information is printed out to give insights into the memory overhead introduced by different serialization formats.

## Benchmark Operations

Each of the following operations is benchmarked:

1. **Bincode**
   - Bincode is a highly efficient binary serialization format.
   - Measures the memory used during serialization and deserialization of a `TestData` instance.

2. **BCS (Binary Canonical Serialization)**
   - Used by the Move language, BCS is a binary serialization format.
   - Measures the memory used during serialization and deserialization.

3. **Protobuf**
   - Protocol Buffers, a widely used language-neutral, platform-neutral format.
   - Protobuf's performance is measured by serializing and deserializing a `TestData` message.

4. **Serde JSON**
   - JSON format via the `serde` framework.
   - Benchmarks the time and memory used to serialize and deserialize the `TestData` struct to and from JSON.

5. **Borsh**
   - Borsh is a binary serialization format optimized for Rust.
   - Measures the performance of serializing and deserializing the `TestData` struct.


## Understanding the Results

The benchmark results include:

- **Speed**: Time taken for serialization and deserialization operations
- **Memory Usage**: How much memory is allocated during serialization and deserialization
- **Comparison**: Relative performance between different serialization libraries

Example output:

```
--- Bincode ---
Memory before: 0 bytes
Memory after serialize: X bytes
Memory after deserialize: Y bytes
Memory used during Bincode: Z bytes
---------------------
```
## Result Comparison

| Format      | Serialization Time (ns) | Serialization Ops/sec | Deserialization Time (ns) | Deserialization Ops/sec |
|-------------|-------------------------|-----------------------|---------------------------|-------------------------|
| bcs         | 71.09                   | 14,066,415 ops/sec    | 61.64                     | 16,222,547 ops/sec      |
| bincode     | 31.90                   | 31,345,959 ops/sec    | 44.01                     | 22,720,093 ops/sec      |
| borsh       | 31.62                   | 31,621,394 ops/sec    | 69.42                     | 14,404,395 ops/sec      |
| protobuf    | 67.10                   | 14,903,386 ops/sec    | 79.60                     | 12,563,200 ops/sec      |
| serde_json  | 70.88                   | 14,108,133 ops/sec    | 118.76                    | 8,420,194 ops/sec       |


## Results

```bash
     Running benches/serialization_benchmark.rs (target/release/deps/serialization_benchmark-e92b3fe8221ea804)
bincode/serialize       time:   [31.758 ns 31.902 ns 32.083 ns]
                        change: [+1.7927% +3.0005% +4.0727%] (p = 0.00 < 0.05)
                        Performance has regressed.
bincode/deserialize     time:   [43.590 ns 44.014 ns 44.433 ns]
                        change: [+0.3167% +1.1693% +2.0086%] (p = 0.01 < 0.05)
                        Change within noise threshold.
Found 6 outliers among 100 measurements (6.00%)
  6 (6.00%) high mild

--- Bincode ---
Memory before: 0 bytes
Memory after serialize: 34980 bytes
Memory after deserialize: 130 bytes
Memory used during Bincode: 35110 bytes
---------------------

bcs/serialize           time:   [70.838 ns 71.091 ns 71.341 ns]
                        change: [-0.5128% +0.5360% +1.7766%] (p = 0.39 > 0.05)
                        No change in performance detected.
Found 7 outliers among 100 measurements (7.00%)
  6 (6.00%) high mild
  1 (1.00%) high severe
bcs/deserialize         time:   [61.319 ns 61.643 ns 62.015 ns]
                        change: [-0.4917% +0.3975% +1.2275%] (p = 0.37 > 0.05)
                        No change in performance detected.
Found 15 outliers among 100 measurements (15.00%)
  4 (4.00%) high mild
  11 (11.00%) high severe

--- BCS ---
Memory before: 0 bytes
Memory after serialize: 726 bytes
Memory after deserialize: 307 bytes
Memory used during BCS: 1033 bytes
---------------------

protobuf/serialize      time:   [66.901 ns 67.099 ns 67.323 ns]
                        change: [-3.0751% -2.1961% -1.3021%] (p = 0.00 < 0.05)
                        Performance has improved.
Found 9 outliers among 100 measurements (9.00%)
  6 (6.00%) high mild
  3 (3.00%) high severe
protobuf/deserialize    time:   [79.465 ns 79.598 ns 79.751 ns]
                        change: [-3.8159% -3.0714% -2.3337%] (p = 0.00 < 0.05)
                        Performance has improved.
Found 10 outliers among 100 measurements (10.00%)
  5 (5.00%) high mild
  5 (5.00%) high severe

--- Protobuf ---
Memory before: 0 bytes
Memory after serialize: 783 bytes
Memory after deserialize: 153 bytes
Memory used during Protobuf: 936 bytes
---------------------

serde_json/serialize    time:   [70.481 ns 70.881 ns 71.289 ns]
                        change: [-0.9514% -0.0928% +0.7345%] (p = 0.83 > 0.05)
                        No change in performance detected.
Found 4 outliers among 100 measurements (4.00%)
  3 (3.00%) high mild
  1 (1.00%) high severe
serde_json/deserialize  time:   [118.50 ns 118.76 ns 119.06 ns]
                        change: [-2.4760% -1.5247% -0.6483%] (p = 0.00 < 0.05)
                        Change within noise threshold.
Found 9 outliers among 100 measurements (9.00%)
  3 (3.00%) high mild
  6 (6.00%) high severe

--- Serde JSON ---
Memory before: 0 bytes
Memory after serialize: 769 bytes
Memory after deserialize: 695 bytes
Memory used during Serde JSON: 1464 bytes
---------------------

borsh/serialize         time:   [31.574 ns 31.624 ns 31.683 ns]
                        change: [-1.3856% -0.5978% +0.1742%] (p = 0.12 > 0.05)
                        No change in performance detected.
Found 14 outliers among 100 measurements (14.00%)
  4 (4.00%) high mild
  10 (10.00%) high severe
borsh/deserialize       time:   [69.287 ns 69.423 ns 69.581 ns]
                        change: [-1.0156% -0.4342% +0.1571%] (p = 0.17 > 0.05)
                        No change in performance detected.
Found 15 outliers among 100 measurements (15.00%)
  6 (6.00%) high mild
  9 (9.00%) high severe

--- Borsh ---
Memory before: 0 bytes
Memory after serialize: 736 bytes
Memory after deserialize: 352 bytes
Memory used during Borsh: 1088 bytes
---------------------

Total estimated serialize time of group 'bincode': 31.902 ns
Total estimated deserialize time of group 'bincode': 44.014 ns
Total estimated serialize ops of group 'bincode': 31345959 ops/sec
Total estimated deserialize ops of group 'bincode': 22720093 ops/sec
Total estimated serialize time of group 'bcs': 71.091 ns
Total estimated deserialize time of group 'bcs': 61.643 ns
Total estimated serialize ops of group 'bcs': 14066415 ops/sec
Total estimated deserialize ops of group 'bcs': 16222547 ops/sec
Total estimated serialize time of group 'protobuf': 67.099 ns
Total estimated deserialize time of group 'protobuf': 79.598 ns
Total estimated serialize ops of group 'protobuf': 14903386 ops/sec
Total estimated deserialize ops of group 'protobuf': 12563200 ops/sec
Total estimated serialize time of group 'serde_json': 70.881 ns
Total estimated deserialize time of group 'serde_json': 118.762 ns
Total estimated serialize ops of group 'serde_json': 14108133 ops/sec
Total estimated deserialize ops of group 'serde_json': 8420194 ops/sec
Total estimated serialize time of group 'borsh': 31.624 ns
Total estimated deserialize time of group 'borsh': 69.423 ns
Total estimated serialize ops of group 'borsh': 31621394 ops/sec
Total estimated deserialize ops of group 'borsh': 14404395 ops/sec

----------------------- Serialization Benchmark Results ------------------------
Format       | Serialization Time (ns) | Serialization Ops/sec | Deserialization Time (ns) | Deserialization Ops/sec
-------------+----------------------+----------------------+----------------------+---------------------
bcs          | 71.09                | 14066415 ops/sec     | 61.64                | 16222547 ops/sec    
bincode      | 31.90                | 31345959 ops/sec     | 44.01                | 22720093 ops/sec    
borsh        | 31.62                | 31621394 ops/sec     | 69.42                | 14404395 ops/sec    
protobuf     | 67.10                | 14903386 ops/sec     | 79.60                | 12563200 ops/sec    
serde_json   | 70.88                | 14108133 ops/sec     | 118.76               | 8420194 ops/sec     
--------------------------------------------------------------------------------

```


## Conclusion

This project serves as a benchmark comparison of several serialization formats used in Rust, providing insights into which format might be best suited for your use case based on both performance and memory efficiency. The project can be extended to include additional serialization formats or refined to add more specific benchmarking metrics.

## Contributing

Contributions are welcome! Here are some ways you can contribute:

- Add benchmarks for additional serialization formats
- Improve the benchmarking methodology
- Enhance documentation
- Report issues or suggest improvements

## License

[MIT License](LICENSE)

## Acknowledgements

- This project uses [Criterion](https://github.com/bheisler/criterion.rs) for benchmarking
- Special thanks to all the developers of the serialization libraries being compared

