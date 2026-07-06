
import Redis from "ioredis";

export async function testHset(redis: Redis) {
    console.log("=== TEST HSET ===");
    const key = "testhash";
    const field = "name";
    const value = "AntDb";

    console.log('Storing hash field with HSET...');
    await redis.hset(key, field, value);

    const storedValue: string | null = await redis.hget(key, field);
    console.log('Hash field value check:', storedValue);

    if (storedValue !== value) {
        throw new Error(`Assertion Failed: HSET should store '${value}' at field '${field}', but got '${storedValue}'`);
    }

    console.log("✅ TEST HSET PASSED SUCCESSFULLY!");
}