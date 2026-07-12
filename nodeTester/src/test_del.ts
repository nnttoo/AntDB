// Haryanto 07 July 2026
import Redis from "ioredis";
import { TestMethod } from "./sleep";

// Haryanto 07 July 2026

export function testMultipleDel(redis: Redis): TestMethod {
    return {
        name: "multiple_del",
        success: false,
        async onTest() {
            console.log("=== TEST MULTIPLE DEL ===");

            // 1. Setup sample keys that exist
            console.log('Setting up test keys...');
            await redis.set("key_del_1", "value1");
            await redis.set("key_del_2", "value2");

            console.log('Deleting multiple keys with DEL...');
            // We pass 2 existing keys and 1 non-existing key
            const deletedCount = await redis.del("key_del_1", "key_del_2", "key_del_3");
            console.log('DEL result:', deletedCount);

            // 2. Assertion for the returned count (should be 2)
            if (deletedCount !== 2) {
                throw new Error(`Assertion Failed: DEL should remove 2 keys, but returned '${deletedCount}'`);
            }

            // 3. Double check using GET (should return null for deleted keys)
            const val1 = await redis.get("key_del_1");
            const val2 = await redis.get("key_del_2");

            if (val1 !== null || val2 !== null) {
                throw new Error(`Assertion Failed: Keys were reported deleted but GET did not return null`);
            }

            console.log("✅ TEST MULTIPLE DEL PASSED SUCCESSFULLY!");
        }
    };

}