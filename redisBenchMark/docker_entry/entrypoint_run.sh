#!/bin/sh
# // Haryanto 15 06 2026

echo "=== Starting Performance Test at $(date) ==="  

echo "=== 1. Starting AntDB Server in Background ===" 
./AntDB > /dev/null 2>&1 &

echo "=== 2. Waiting 5 Seconds for CPU System to Settle ===" 
sleep 5

echo "=== 3. Running Node.js Functional Tests ===" 
# Output test Node.js langsung dialihkan ke terminal (tidak ke log file)
# cp /nodeTester/src/index.ts  /usr/src/tester/src/index.ts
cp -r /nodeTester/src /usr/src/tester/
cd /usr/src/tester && npx tsx ./src/index.ts  

echo "=========================================="
echo "       REDIS PERFORMANCE REPORT           "
echo "       Date: $(date)                      "
echo "=========================================="

echo "Running all benchmarks (PING, SET, GET, HSET) to CSV..."

# Menjalankan semua pengujian sekaligus dan menyimpannya ke satu file CSV
redis-benchmark -h 127.0.0.1 -p 6379 -c 10 -n 50000 -t  ping,set,get,hset --csv > /working_dir/benchmark_results.csv
 

echo "Running custom HGET benchmark..."
# 2. Use append (>>) to add the custom HGET test to the same CSV file
# Use -r 10000 so the requested keys/fields are randomized according to the HSET data
redis-benchmark -h 127.0.0.1 -p 6379 -c 10 -n 50000 -r 10000 --csv HGET "myhash:__rand_int__" "counter:__rand_int__" >> /working_dir/benchmark_results.csv


echo "\n=== Tests Completed. All benchmark results saved to /working_dir/benchmark_results.csv ==="

cat /working_dir/benchmark_results.csv