
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



export async function testHdelMultiFields(redis: Redis) {
    // Haryanto 06 July 2026
    console.log("=== TEST HDEL MULTI FIELDS ===");
    const key = "testhash:hdelmulti";
    const field1 = "version";
    const value1 = "1.0.0";
    const field2 = "status";
    const value2 = "stable";
    const field3 = "license";
    const value3 = "MIT";

    // 1. Clean up any existing keys to keep the test isolated
    await redis.del(key);

    console.log('Preparing multiple fields with HSET...');
    await redis.hset(key, field1, value1);
    await redis.hset(key, field2, value2);
    await redis.hset(key, field3, value3);

    // 2. Delete two fields at once (one existing, one non-existing)
    const nonExistingField = "author";
    console.log(`Deleting '${field1}' (exists) and '${nonExistingField}' (does not exist)...`);

    // HDEL should only count successfully deleted fields, so it must return 1
    const firstDeleteResult = await redis.hdel(key, field1, nonExistingField);
    console.log('HDEL multi return value (expected 1):', firstDeleteResult);

    if (firstDeleteResult !== 1) {
        throw new Error(`Assertion Failed: HDEL should return 1 when deleting one existing and one non-existing field, but got ${firstDeleteResult}`);
    }

    // 3. Delete the remaining fields to check if the main key is automatically removed
    console.log(`Deleting remaining fields: '${field2}' and '${field3}'...`);
    const secondDeleteResult = await redis.hdel(key, field2, field3);
    console.log('HDEL multi return value (expected 2):', secondDeleteResult);

    if (secondDeleteResult !== 2) {
        throw new Error(`Assertion Failed: HDEL should return 2 when deleting two existing fields, but got ${secondDeleteResult}`);
    }

    // 4. Verify that the entire hash key no longer exists in the database
    const keyExists = await redis.exists(key);
    console.log('Checking if the main hash key still exists (expected 0):', keyExists);

    if (keyExists !== 0) {
        throw new Error(`Assertion Failed: Main hash key '${key}' should be fully removed from database after all fields are deleted, but EXISTS returned ${keyExists}`);
    }

    console.log("✅ TEST HDEL MULTI FIELDS PASSED SUCCESSFULLY!");
}