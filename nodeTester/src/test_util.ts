// Haryanto 12 September 2026

import Redis from "ioredis";
import { TestMethod } from "./sleep";

// Haryanto 12 September 2026

export function testKeys(redis: Redis): TestMethod {
    return {
        name: "KEYS",
        success: false,
        async onTest() {
            console.log("=== TEST KEYS ===");

            const key1 = "testkey_keys_1";
            const key2 = "testkey_keys_2";
            const otherKey = "other_testkey_3";

            const qKey1 = "hello";
            const qKey2 = "hallo";
            const qKeyNoMatch = "h123o";

            console.log('Cleaning up keys before test...');
            await redis.del(key1);
            await redis.del(key2);
            await redis.del(otherKey);
            await redis.del(qKey1);
            await redis.del(qKey2);
            await redis.del(qKeyNoMatch);

            console.log('Testing KEYS on non-matching pattern (should return empty array)...');
            let result = await redis.keys("testkey_keys_*");
            console.log(`Result:`, result);

            if (!Array.isArray(result) || result.length !== 0) {
                throw new Error(`Assertion Failed: KEYS with no matching keys should return empty array, but got '${JSON.stringify(result)}'`);
            }

            console.log('Creating test keys for "*" wildcard...');
            await redis.set(key1, "val1");
            await redis.set(key2, "val2");
            await redis.set(otherKey, "val3");

            console.log('Testing KEYS with wildcard pattern "testkey_keys_*"...');
            result = await redis.keys("testkey_keys_*");
            console.log(`Result:`, result);

            result.sort();
            const expectedStar = [key1, key2].sort();

            if (!Array.isArray(result) || JSON.stringify(result) !== JSON.stringify(expectedStar)) {
                throw new Error(`Assertion Failed: Expected pattern "testkey_keys_*" to match ${JSON.stringify(expectedStar)}, but got ${JSON.stringify(result)}`);
            }

            console.log('Creating test keys for "?" single-character wildcard...');
            await redis.set(qKey1, "val1");
            await redis.set(qKey2, "val2");
            await redis.set(qKeyNoMatch, "val3");

            console.log('Testing KEYS with single-character wildcard pattern "h?llo"...');
            result = await redis.keys("h?llo");
            console.log(`Result:`, result);

            result.sort();
            const expectedQuestion = [qKey1, qKey2].sort();

            if (!Array.isArray(result) || JSON.stringify(result) !== JSON.stringify(expectedQuestion)) {
                throw new Error(`Assertion Failed: Expected pattern "h?llo" to match ${JSON.stringify(expectedQuestion)}, but got ${JSON.stringify(result)}`);
            }

            console.log('Cleaning up created keys...');
            await redis.del(key1);
            await redis.del(key2);
            await redis.del(otherKey);
            await redis.del(qKey1);
            await redis.del(qKey2);
            await redis.del(qKeyNoMatch);

            console.log("✅ TEST KEYS PASSED SUCCESSFULLY!");
        }
    };
}