
import Redis from "ioredis";
import { TestMethod } from "./sleep";

export function testHset(redis: Redis): TestMethod {

    return {
        name: "hset",
        success: false,
        async onTest() {
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
    };

}



export function testHdelMultiFields(redis: Redis): TestMethod {

    return {
        name: "Hdel_MultiFields",
        success: false,
        async onTest() {
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
    }


}

export function testHlen(redis: Redis): TestMethod {
    return {
        name: "hlen",
        success: false,
        async onTest() {
            // Haryanto 11 July 2026
            console.log("=== TEST HLEN ===");
            const key = "testhash:hlen";

            // Generate unique field names using timestamp + random suffix
            const timestamp = Date.now();
            const field1 = `field1_${timestamp}`;
            const field2 = `field2_${timestamp}`;
            const field3 = `field3_${timestamp}`;

            const value1 = `val_${timestamp}`;
            const value2 = `val_${timestamp + 1}`;
            const value3 = `val_${timestamp + 2}`;

            // 1. Clean up any existing keys to keep the test isolated
            await redis.del(key);

            // 2. Test HLEN on non-existing key (should return 0)
            console.log("Checking HLEN on a non-existing key...");
            const initialLen = await redis.hlen(key);
            console.log("HLEN return value (expected 0):", initialLen);
            if (initialLen !== 0) {
                throw new Error(`Assertion Failed: HLEN on empty key should return 0, but got ${initialLen}`);
            }

            // 3. Insert fields and check HLEN incrementally
            console.log("Adding fields to hash...");
            await redis.hset(key, field1, value1);
            await redis.hset(key, field2, value2);
            await redis.hset(key, field3, value3);

            const fullLen = await redis.hlen(key);
            console.log("HLEN return value after adding 3 fields (expected 3):", fullLen);
            if (fullLen !== 3) {
                throw new Error(`Assertion Failed: HLEN should return 3, but got ${fullLen}`);
            }

            // 4. Delete one field and check if HLEN decreases to 2
            console.log(`Deleting '${field1}' to verify HLEN update...`);
            await redis.hdel(key, field1);

            const afterOneDeleteLen = await redis.hlen(key);
            console.log("HLEN return value after 1 deletion (expected 2):", afterOneDeleteLen);
            if (afterOneDeleteLen !== 2) {
                throw new Error(`Assertion Failed: HLEN should decrease to 2, but got ${afterOneDeleteLen}`);
            }

            // 5. Delete the remaining fields and check if HLEN returns to 0
            console.log("Deleting remaining fields...");
            await redis.hdel(key, field2, field3);

            const finalLen = await redis.hlen(key);
            console.log("HLEN return value after all deletions (expected 0):", finalLen);
            if (finalLen !== 0) {
                throw new Error(`Assertion Failed: HLEN should be 0 after deleting all fields, but got ${finalLen}`);
            }

            console.log("✅ TEST HLEN PASSED SUCCESSFULLY!");
        }
    }

}

export function testHexists(redis: Redis): TestMethod {
    return {
        name: "Hexists",
        success: false,
        async onTest() {
            // Haryanto 11 July 2026
            console.log("=== TEST HEXISTS ===");
            const key = "testhash:hexists";

            // Generate unique field name using timestamp
            const timestamp = Date.now();
            const existingField = `field_exist_${timestamp}`;
            const nonExistingField = `field_none_${timestamp}`;
            const value = `val_${timestamp}`;

            // 1. Clean up any existing keys to keep the test isolated
            await redis.del(key);

            // 2. Test HEXISTS on a non-existing key/field (should return 0)
            console.log(`Checking HEXISTS on non-existing field '${nonExistingField}'...`);
            const initialCheck = await redis.hexists(key, nonExistingField);
            console.log("HEXISTS return value (expected 0):", initialCheck);

            if (initialCheck !== 0) {
                throw new Error(`Assertion Failed: HEXISTS should return 0 for non-existing field, but got ${initialCheck}`);
            }

            // 3. Setup field using HSET
            console.log(`Setting field '${existingField}' with HSET...`);
            await redis.hset(key, existingField, value);

            // 4. Test HEXISTS on the field that now exists (should return 1)
            console.log(`Checking HEXISTS on existing field '${existingField}'...`);
            const fieldExistsCheck = await redis.hexists(key, existingField);
            console.log("HEXISTS return value (expected 1):", fieldExistsCheck);

            if (fieldExistsCheck !== 1) {
                throw new Error(`Assertion Failed: HEXISTS should return 1 for an existing field, but got ${fieldExistsCheck}`);
            }

            // 5. Delete the field using HDEL and verify HEXISTS returns to 0
            console.log(`Deleting field '${existingField}' using HDEL to test state change...`);
            await redis.hdel(key, existingField);

            console.log(`Re-checking HEXISTS on '${existingField}' after HDEL...`);
            const afterDeleteCheck = await redis.hexists(key, existingField);
            console.log("HEXISTS return value after HDEL (expected 0):", afterDeleteCheck);

            if (afterDeleteCheck !== 0) {
                throw new Error(`Assertion Failed: HEXISTS should return 0 after the field is deleted via HDEL, but got ${afterDeleteCheck}`);
            }

            console.log("✅ TEST HEXISTS PASSED SUCCESSFULLY!");
        }
    }

}