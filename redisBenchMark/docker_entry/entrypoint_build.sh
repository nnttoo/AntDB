#!/bin/sh
# // Haryanto 15 06 2026
echo "--- Debugging: List isi direktori /app_source ---"
rm -rf ./build/*
rm /working_dir/AntDB

ls -la /app_source/


cd /app_source/

echo '--- Memulai proses kompilasi dengan cargo-zigbuild ---'
#export RUSTFLAGS="-C target-cpu=native" 
export CARGO_TARGET_DIR="/build/"
#cargo zigbuild --release --target x86_64-unknown-linux-musl
cargo build --release

echo '--- Kompilasi selesai, menyalin binary ke folder output ---'

ls -la /build/release
cp /build/release/AntDB /working_dir/AntDB

# Using ZigBUilg
#cp /build/x86_64-unknown-linux-musl/release/AntDB /working_dir/AntDB

echo '--- Selesai! File binary siap digunakan ---'
exit 0