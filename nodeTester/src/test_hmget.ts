
import Redis from "ioredis";
import { TestMethod } from "./sleep";

export function testHmget(redis: Redis): TestMethod {
    return {
        name: "Hmget",
        success: false,
        async onTest() {


            // Haryanto 11 July 2026
            console.log("=== TEST HMGET ===");
            const key = "testhash:hmget";

            const timestamp = Date.now();
            const field1 = "name";
            const value1 = `Anto_${timestamp}`;
            const field2 = "role";
            const value2 = "Core_Developer";
            const field3 = "status";
            const value3 = "active";
            const nonExistingField = "deleted_field";

            // 1. Clean up any existing keys to keep the test isolated
            await redis.del(key);

            // 2. Test HMGET on a non-existing key (should return array of nulls)
            console.log("Testing HMGET on a non-existing key...");
            const emptyResult = await redis.hmget(key, field1, field2);
            console.log("HMGET on empty key result (expected [null, null]):", emptyResult);

            if (!Array.isArray(emptyResult) || emptyResult[0] !== null || emptyResult[1] !== null) {
                throw new Error(`Assertion Failed: HMGET on empty key should return array of nulls, but got ${JSON.stringify(emptyResult)}`);
            }

            // 3. Populate data using HSET
            console.log("Setting up multiple fields with HSET...");
            await redis.hset(key, field1, value1);
            await redis.hset(key, field2, value2);
            await redis.hset(key, field3, value3);

            // 4. Test HMGET fetching mixed existing and non-existing fields
            console.log(`Fetching mixed fields: '${field1}' (exists), '${nonExistingField}' (does not exist), '${field2}' (exists)...`);
            const mixedResult = await redis.hmget(key, field1, nonExistingField, field2);
            console.log("HMGET mixed result:", mixedResult);

            // Expected output format from ioredis/redis: [value1, null, value2]
            if (!Array.isArray(mixedResult) || mixedResult.length !== 3) {
                throw new Error(`Assertion Failed: HMGET should return an array of 3 elements, but got length ${mixedResult?.length}`);
            }

            if (mixedResult[0] !== value1) {
                throw new Error(`Assertion Failed: Index 0 should be '${value1}', but got '${mixedResult[0]}'`);
            }

            if (mixedResult[1] !== null) {
                throw new Error(`Assertion Failed: Index 1 (non-existing field) should be null, but got '${mixedResult[1]}'`);
            }

            if (mixedResult[2] !== value2) {
                throw new Error(`Assertion Failed: Index 2 should be '${value2}', but got '${mixedResult[2]}'`);
            }

            console.log("✅ TEST HMGET PASSED SUCCESSFULLY!");
        }

    }
}