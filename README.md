# AntDB

// Haryanto 14 June 2026

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

### Download Binary

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