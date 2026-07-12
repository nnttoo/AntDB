// Haryanto 12 July 2026
import Redis from "ioredis";
import { TestMethod } from "./sleep";

export function testHvals(redis: Redis): TestMethod {
    return {
        name: "Hvals",
        success: false,
        async onTest() {
            // Haryanto 12 July 2026
            console.log("=== TEST HVALS ===");
            const key = "testhash:hvals";
            const timestamp = Date.now();

            const field1 = "name";
            const value1 = `Anto_${timestamp}`;
            const field2 = "role";
            const value2 = "Core_Developer";
            const field3 = "status";
            const value3 = "active";

            // 1. Clean up any existing keys to keep the test isolated
            await redis.del(key);

            // 2. Test HVALS on a non-existing key (should return an empty array)
            console.log("Testing HVALS on a non-existing key...");
            const emptyResult = await redis.hvals(key);
            console.log("HVALS on empty key result (expected []):", emptyResult);
            if (!Array.isArray(emptyResult) || emptyResult.length !== 0) {
                throw new Error(`Assertion Failed: HVALS on empty key should return an empty array, but got ${JSON.stringify(emptyResult)}`);
            }

            // 3. Populate data using HSET
            console.log("Setting up multiple fields with HSET...");
            await redis.hset(key, field1, value1);
            await redis.hset(key, field2, value2);
            await redis.hset(key, field3, value3);

            // 4. Test HVALS fetching all values
            console.log("Fetching all values using HVALS...");
            const result = await redis.hvals(key);
            console.log("HVALS result:", result);

            if (!Array.isArray(result) || result.length !== 3) {
                throw new Error(`Assertion Failed: HVALS should return an array of 3 elements, but got length ${result?.length}`);
            }

            // Sort the result array to ensure predictable index checking regardless of internal hashmap order
            result.sort();

            if (result[0] !== "Core_Developer") {
                throw new Error(`Assertion Failed: Index 0 (sorted) should be 'Core_Developer', but got '${result[0]}'`);
            }
            if (result[1] !== "active") {
                throw new Error(`Assertion Failed: Index 1 (sorted) should be 'active', but got '${result[1]}'`);
            }
            if (result[2] !== `Anto_${timestamp}`) {
                throw new Error(`Assertion Failed: Index 2 (sorted) should be 'Anto_${timestamp}', but got '${result[2]}'`);
            }

            console.log("✅ TEST HVALS PASSED SUCCESSFULLY!");
        }
    };
}