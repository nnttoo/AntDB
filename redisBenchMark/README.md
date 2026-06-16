# Redis Benchmark Guide for AntDB

This directory provides automated scripts to compile and benchmark AntDB using Docker containers. All commands are managed via `package.json` as shortcuts for easier execution.

---

## Prerequisites

Ensure you have the following installed and running on your machine:
* **Node.js** (required only to execute the `npm run` shortcuts)
* **Docker** and **Docker Compose**

---

## Script Explanations

The `package.json` file contains two main pipelines (Compilation and Benchmarking), each split into configuration, building, and running phases:

### 1. Compilation Pipeline (Rust Environment)
Used to set up and compile the AntDB Rust implementation inside a container.
* `npm run compile_compose`: Specifies the Docker Compose configuration file (`docker-compose-build.yml`).
* `npm run compile_build`: Builds the Docker image for the compilation environment.
* `npm run compile_up`: Compiles the Rust project and runs the container, forcing a recreate to ensure a clean build.

### 2. Benchmark Pipeline (Redis Benchmark Environment)
Used to build and execute the benchmarking suite against the compiled AntDB.
* `npm run bmark_compose`: Specifies the Docker Compose configuration file for testing (`docker-compose-build-run.yml`).
* `npm run bmark_build`: Builds the Docker image for the benchmark runner.
* `npm run bmark_up`: Launches the benchmark suite, cleaning up any orphaned containers.

---

## Quick Start Guide

To compile the project and run the complete benchmark suite sequentially in one go, execute:

```bash
npm run run_all