import Redis from "ioredis";
import { TestMethod } from "./sleep";

export function testHgetall(redis: Redis): TestMethod {
    return {
        name: "hgetall",
        success: false,
        async onTest() {
            // Haryanto 13 July 2026
            console.log("=== TEST HGETALL ===");
            const key = "testhash:hgetall";
            const field1 = "framework";
            const value1 = "nestjs";
            const field2 = "language";
            const value2 = "typescript";

            // 1. Clean up any existing keys to keep the test isolated
            await redis.del(key);

            // 2. Test HGETALL on non-existing key (should return an empty object {})
            console.log("Checking HGETALL on a non-existing key...");
            const initialResult = await redis.hgetall(key);
            console.log("HGETALL return value (expected empty object):", initialResult);
            if (Object.keys(initialResult).length !== 0) {
                throw new Error(`Assertion Failed: HGETALL on empty key should return an empty object, but got ${JSON.stringify(initialResult)}`);
            }

            // 3. Populate hash with fields using HSET
            console.log("Setting fields with HSET...");
            await redis.hset(key, field1, value1);
            await redis.hset(key, field2, value2);

            // 4. Test HGETALL on the populated hash key
            console.log("Checking HGETALL on populated key...");
            const fullResult = await redis.hgetall(key);
            console.log("HGETALL return value:", fullResult);

            if (fullResult[field1] !== value1) {
                throw new Error(`Assertion Failed: Expected field '${field1}' to be '${value1}', but got '${fullResult[field1]}'`);
            }
            if (fullResult[field2] !== value2) {
                throw new Error(`Assertion Failed: Expected field '${field2}' to be '${value2}', but got '${fullResult[field2]}'`);
            }
            if (Object.keys(fullResult).length !== 2) {
                throw new Error(`Assertion Failed: HGETALL should return exactly 2 fields, but got ${Object.keys(fullResult).length}`);
            }

            console.log("✅ TEST HGETALL PASSED SUCCESSFULLY!");
        }
    };
}