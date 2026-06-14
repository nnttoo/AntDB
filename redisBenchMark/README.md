# Redis Benchmark Guide for AntDB

## Step-by-step

1. Start AntDB locally:
   ```bash
   cd AntDBRust
   cargo run
   ```

2. Run the benchmark container:
   ```bash
   cd redisBenchMark
   docker compose up
   ```

3. If you want to use only specific commands, run this instead:
   ```bash
   docker run --rm --add-host=host.docker.internal:host-gateway redis:7-alpine \
     redis-benchmark -h host.docker.internal -p 6379 -t set,get,ping -c 10 -n 1000 --threads 1
   ```

4. Example for hash commands (if supported by your current AntDB build):
   ```bash
   docker run --rm --add-host=host.docker.internal:host-gateway redis:7-alpine \
     redis-benchmark -h host.docker.internal -p 6379 -t hset,hget -c 10 -n 1000 --threads 1
   ```