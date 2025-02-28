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

