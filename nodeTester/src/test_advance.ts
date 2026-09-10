// Haryanto 10 September 2026

import Redis from "ioredis";
import { TestMethod } from "./sleep";

export function testMSet(redis: Redis): TestMethod {
    return {
        name: "MSET",
        success: false,
        async onTest() {
            console.log("=== TEST MSET ===");
            const kvMap = {
                "testkey_mset_1": "Hello AntDb 1",
                "testkey_mset_2": "Hello AntDb 2",
                "testkey_mset_3": "Hello AntDb 3"
            };

            console.log('Storing multiple values with MSET...');
            await redis.mset(kvMap);

            console.log('Verifying stored values...');
            for (const [key, expectedValue] of Object.entries(kvMap)) {
                const storedValue: string | null = await redis.get(key);
                console.log(`Key: ${key} | Stored value check:`, storedValue);

                if (storedValue !== expectedValue) {
                    throw new Error(`Assertion Failed: MSET for '${key}' should store '${expectedValue}', but got '${storedValue}'`);
                }
            }

            console.log("✅ TEST MSET PASSED SUCCESSFULLY!");
        }
    };
}

export function testMGet(redis: Redis): TestMethod {
    return {
        name: "MGET",
        success: false,
        async onTest() {
            console.log("=== TEST MGET ===");

            const testData: Record<string, string> = {
                "testkey_mget_1": "Value AntDb 1",
                "testkey_mget_2": "Value AntDb 2",
                "testkey_mget_3": "Value AntDb 3"
            };

            const keysToFetch = [...Object.keys(testData), "non_existing_key_mget"];

            console.log('Preparing test data using MSET...');
            await redis.mset(testData);

            console.log('Fetching multiple keys using MGET...');
            const results: (string | null)[] = await redis.mget(...keysToFetch);

            console.log('Verifying fetched values...');

            // Verify existing keys
            const keysList = Object.keys(testData);
            for (let i = 0; i < keysList.length; i++) {
                const key = keysList[i];
                const expectedValue = testData[key];
                const actualValue = results[i];

                console.log(`Key: ${key} | Expected: '${expectedValue}' | Got: '${actualValue}'`);

                if (actualValue !== expectedValue) {
                    throw new Error(`Assertion Failed: MGET for '${key}' should return '${expectedValue}', but got '${actualValue}'`);
                }
            }

            // Verify non-existing key returns null
            const nonExistingIndex = keysToFetch.length - 1;
            const nonExistingResult = results[nonExistingIndex];
            console.log(`Key: non_existing_key_mget | Expected: null | Got:`, nonExistingResult);

            if (nonExistingResult !== null) {
                throw new Error(`Assertion Failed: MGET for non-existing key should return null, but got '${nonExistingResult}'`);
            }

            console.log("✅ TEST MGET PASSED SUCCESSFULLY!");
        }
    };
}

export function testIncr(redis: Redis): TestMethod {
    return {
        name: "INCR",
        success: false,
        async onTest() {
            console.log("=== TEST INCR ===");

            const key = "testkey_incr";

            console.log('Cleaning up key before test...');
            await redis.del(key);

            console.log('Testing INCR on a non-existing key (should initialize to 0 and become 1)...');
            let result = await redis.incr(key);
            console.log(`Result:`, result);

            if (result !== 1) {
                throw new Error(`Assertion Failed: INCR on non-existing key should return 1, but got '${result}'`);
            }

            console.log('Testing INCR on an existing numeric key (should increment to 2)...');
            result = await redis.incr(key);
            console.log(`Result:`, result);

            if (result !== 2) {
                throw new Error(`Assertion Failed: INCR should increment value to 2, but got '${result}'`);
            }

            console.log('Verifying final stored value with GET...');
            const storedValue = await redis.get(key);
            console.log(`Stored value check:`, storedValue);

            if (storedValue !== "2") {
                throw new Error(`Assertion Failed: Final stored value should be '2', but got '${storedValue}'`);
            }

            console.log("✅ TEST INCR PASSED SUCCESSFULLY!");
        }
    };
}