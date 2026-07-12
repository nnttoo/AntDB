# AntDB
 

> [!WARNING]
> **Status: Under Active Development** > AntDB is currently in its early development phase and is not yet ready for production use. Features, API compliance, and binaries are being actively worked on.

AntDB is a high-performance, in-memory data store built with Rust, designed as a lightweight and fast alternative to Redis. Leveraging Rust's safety and concurrency features, AntDB aims to deliver extremely low latency and high throughput for key-value storage.

## Why AntDB?

Most Redis alternatives and modern in-memory databases are built primarily for POSIX systems, leaving Windows users reliant on WSL (Windows Subsystem for Linux) or unofficial, outdated ports. 

AntDB is built from the ground up using Rust to be **truly cross-platform**. We believe developers should have a seamless experience regardless of their operating system. AntDB officially targets and releases pre-compiled binaries via GitHub Actions for:
* **Windows** (x86_64)
* **Linux** (x86_64)
* **macOS** (Apple Silicon & Intel)
* **Linux ARM** (for Raspberry Pi and cloud ARM instances)

## Features

* **True Cross-Platform:** Native support for Windows, Linux, macOS, and ARM architectures without external dependencies.
* **100% Node.js Redis Client Compatibility:** Designed to be a drop-in replacement. You can use standard Node.js libraries like `redis` or `ioredis` seamlessly.
* **In-Memory Storage:** Fast key-value operations with efficient memory management.
* **Rust-Powered:** Zero-cost abstractions, memory safety, and high concurrency without a garbage collector.
* **Built-in Benchmark Tool:** Easily measure performance and throughput under various workloads.

## Installation & Setup

## Auto Install Linux

```sh
curl -sSO https://raw.githubusercontent.com/nnttoo/AntDB/main/install.sh && chmod +x install.sh && ./install.sh
```

## Auto Install Windows (GitBash)

```sh
rm -f install_windows_gitbash.sh antdb-server.exe && curl -sSO https://raw.githubusercontent.com/nnttoo/AntDB/main/install_windows_gitbash.sh && chmod +x install_windows_gitbash.sh && ./install_windows_gitbash.sh
```

### Download Binary Manually

You don't need to install the Rust toolchain to use AntDB. Pre-compiled binaries for all supported platforms are automatically built via GitHub Actions.

1. Go to the **Releases** page of this repository.
2. Download the binary matching your Operating System and Architecture.
3. Extract the file and run the executable.

### Running the Server

To start the AntDB server locally, simply run the executable from your terminal:

```bash
# For Linux / macOS
./antdb-server

# For Windows
antdb-server.exe
```

## RoadMap
### Implemented Commands
- String / Key Commands
  - ✅ SET
  - ✅ GET
  - ✅ SETEX
  - ✅ EXPIRE
  - ✅ DEL
  - ✅ EXISTS
- Hash Commands
  - ✅ HSET
  - ✅ HGET
 
- TTL / Expiry
  - ✅ TTL
  - ✅ PTTL
  - ✅ PERSIST
- Hash Extensions
  - ✅ HDEL
  - ✅ HLEN
  - ✅ HEXISTS
  - ✅ HMGET
  - ⏳ HKEYS
  - ⏳ HVALS
  - ⏳ HGETALL
- Advanced String Commands
  - ⏳ MSET
  - ⏳ MGET
  - ⏳ INCR
  - ⏳ DECR
  - ⏳ APPEND
  - ⏳ GETSET
- Key Utilities
  - ⏳ KEYS
  - ⏳ SCAN
- Quality / Compatibility
  - ⏳ improve Redis command compatibility and tests
  - ⏳ fix expired-key cleanup on 
  

## Benchmark Result


| *test* | *rps* | *avg_latency_ms* | *min_latency_ms* | *p50_latency_ms* | *p95_latency_ms* | *p99_latency_ms* | *max_latency_ms* |
| ----- | ----- | ----- | ----- | ----- | ----- | ----- | ----- |
| *PING_INLINE* | 67,476.38 | 0.109 | 0.008 | 0.079 | 0.263 | 0.431 | 13.663 |
| *PING_MBULK* | 118,764.84 | 0.063 | 0.008 | 0.047 | 0.151 | 0.215 | 0.767 |
| *SET* | 98,425.20 | 0.077 | 0.008 | 0.055 | 0.191 | 0.287 | 1.159 |
| *GET* | 116,550.12 | 0.065 | 0.008 | 0.047 | 0.159 | 0.223 | 0.839 |
| *HSET* | 114,416.48 | 0.069 | 0.008 | 0.055 | 0.167 | 0.231 | 0.879 |
| *HGET* | 99,206.34 | 0.077 | 0.008 | 0.055 | 0.191 | 0.287 | 1.471 |