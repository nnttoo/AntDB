// Haryanto 12 July 2026

import Redis from "ioredis";
import { TestMethod } from "./sleep"; 

export function testHkeys(redis: Redis): TestMethod {
    return {
        name: "Hkeys",
        success: false,
        async onTest() {
            // Haryanto 12 July 2026
            console.log("=== TEST HKEYS ===");
            const key = "testhash:hkeys";
            const timestamp = Date.now();
            const field1 = "name";
            const value1 = `Anto_${timestamp}`;
            const field2 = "role";
            const value2 = "Core_Developer";
            const field3 = "status";
            const value3 = "active";

            // 1. Clean up any existing keys to keep the test isolated
            await redis.del(key);

            // 2. Test HKEYS on a non-existing key (should return an empty array)
            console.log("Testing HKEYS on a non-existing key...");
            const emptyResult = await redis.hkeys(key);
            console.log("HKEYS on empty key result (expected []):", emptyResult);
            if (!Array.isArray(emptyResult) || emptyResult.length !== 0) {
                throw new Error(`Assertion Failed: HKEYS on empty key should return an empty array, but got ${JSON.stringify(emptyResult)}`);
            }

            // 3. Populate data using HSET
            console.log("Setting up multiple fields with HSET...");
            await redis.hset(key, field1, value1);
            await redis.hset(key, field2, value2);
            await redis.hset(key, field3, value3);

            // 4. Test HKEYS fetching all fields
            console.log("Fetching all fields using HKEYS...");
            const result = await redis.hkeys(key);
            console.log("HKEYS result:", result);

            if (!Array.isArray(result) || result.length !== 3) {
                throw new Error(`Assertion Failed: HKEYS should return an array of 3 elements, but got length ${result?.length}`);
            }

            // Sort the result array to ensure predictable index checking regardless of internal hashmap order
            result.sort();

            if (result[0] !== "name") {
                throw new Error(`Assertion Failed: Index 0 (sorted) should be 'name', but got '${result[0]}'`);
            }
            if (result[1] !== "role") {
                throw new Error(`Assertion Failed: Index 1 (sorted) should be 'role', but got '${result[1]}'`);
            }
            if (result[2] !== "status") {
                throw new Error(`Assertion Failed: Index 2 (sorted) should be 'status', but got '${result[2]}'`);
            }

            console.log("✅ TEST HKEYS PASSED SUCCESSFULLY!");
        }
    };
}