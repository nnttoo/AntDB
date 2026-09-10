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