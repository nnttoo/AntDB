// Haryanto 04 07 2026

import Redis from "ioredis";

export async function testPersist(redis: Redis) {
    console.log("\n\n\n=== TEST PERSIST ===");
    const key = "testkey_persist";

    // 1. Store data using SETEX with a 10-second TTL
    console.log('Storing token with SETEX (10-second TTL)...');
    await redis.setex(key, 10, 'PERSIST_VAL');

    // 2. Check initial TTL (should be a positive number close to 10)
    const ttlBefore: number = await redis.ttl(key);
    console.log('TTL right after SETEX (should be > 0):', ttlBefore);
    if (ttlBefore <= 0) {
        throw new Error(`Assertion Failed: TTL should be a positive number, but got ${ttlBefore}`);
    }

    // 3. Execute PERSIST command to remove the expiration time
    console.log('Executing PERSIST command...');
    const persistResult: number = await redis.persist(key);
    console.log('Persist command return value (should be 1):', persistResult);
    if (persistResult !== 1) {
        throw new Error(`Assertion Failed: PERSIST on a key with expiry should return 1, but got ${persistResult}`);
    }

    // 4. Check TTL again (should be -1 because it is now permanent)
    const ttlAfter: number = await redis.ttl(key);
    console.log('TTL after PERSIST (should be -1):', ttlAfter);
    if (ttlAfter !== -1) {
        throw new Error(`Assertion Failed: TTL for persisted key should be -1, but got ${ttlAfter}`);
    }

    // 5. Ensure the data is still accessible and not lost
    const value: string | null = await redis.get(key);
    console.log('Value check after persist:', value);
    if (value !== 'PERSIST_VAL') {
        throw new Error(`Assertion Failed: Expected value 'PERSIST_VAL', but got '${value}'`);
    }

    // 6. Check if running PERSIST on an already permanent key returns 0
    const extraPersist: number = await redis.persist(key);
    console.log('Persist on already permanent key (should be 0):', extraPersist);
    if (extraPersist !== 0) {
        throw new Error(`Assertion Failed: PERSIST on permanent key should return 0, but got ${extraPersist}`);
    }

    console.log("✅ TEST PERSIST PASSED SUCCESSFULLY!");
}