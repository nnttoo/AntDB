#!/bin/sh
# // Haryanto 15 06 2026
echo "--- Debugging: List isi direktori /app_source ---"
rm -rf ./build/*
rm /working_dir/AntDBRust

ls -la /app_source/


cd /app_source/

echo '--- Memulai proses kompilasi dengan cargo-zigbuild ---'
export RUSTFLAGS="-C target-cpu=native" 
export CARGO_TARGET_DIR="/build/"
cargo zigbuild --release --target x86_64-unknown-linux-musl

echo '--- Kompilasi selesai, menyalin binary ke folder output ---'

ls -la /build/
cp /build/x86_64-unknown-linux-musl/release/AntDBRust /working_dir/AntDBRust

echo '--- Selesai! File binary siap digunakan ---'