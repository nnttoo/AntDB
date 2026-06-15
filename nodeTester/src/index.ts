// Haryanto 14 Juni 2026
import Redis from 'ioredis';

//@ts-ignore
const redisHost = process.env.REDIS_HOST ?? '127.0.0.1';

//@ts-ignore
const redisPort = Number(process.env.REDIS_PORT ?? '6379');

// Connect to the Redis server
const redis: Redis = new Redis({
    host: redisHost,
    port: redisPort,
});

function sleep(n: number) {
    return new Promise((r, x) => {
        setTimeout(r, n);
    })
}

async function testServer(): Promise<void> {
    async function testSetex() {
        console.log("=== TEST SETEX ===");
        const key = "testkey";

        // 1. Simpan token dengan TTL 3 detik
        console.log('Storing token with a 3-second TTL...');
        await redis.setex(key, 3, 'XYZ123');

        // 2. Ambil langsung (harus ada nilainya: 'XYZ123')
        const tokenImmediate: string | null = await redis.get(key);
        console.log('Immediate token check:', tokenImmediate);

        // Validasi pengecekan langsung
        if (tokenImmediate !== 'XYZ123') {
            throw new Error(`Assertion Failed: Immediate token should be 'XYZ123', but got '${tokenImmediate}'`);
        }

        // 3. Tunggu selama 4 detik sampai data kedaluwarsa
        console.log('Waiting for 4 seconds...');
        await sleep(4000);

        // 4. Ambil setelah menunggu (harusnya sudah terhapus: null)
        const tokenAfterWait: string | null = await redis.get(key);
        console.log('Token after 4 seconds:', tokenAfterWait);

        // Validasi pengecekan setelah kedaluwarsa
        if (tokenAfterWait !== null) {
            throw new Error(`Assertion Failed: Token should be expired and return null, but instead found: '${tokenAfterWait}'`);
        }

        console.log("✅ TEST SETEX PASSED SUCCESSFULLY!");
    }

    async function testExpire() {
        console.log("=== TEST EXPIRE ===");
        const key = "testkey33";

        // 1. Simpan token lalu atur TTL 3 detik
        console.log('Storing token with a 3-second TTL...');
        await redis.set(key, 'XYZ123');
        await redis.expire(key, 3);

        // 2. Ambil langsung (harus ada nilainya: 'XYZ123')
        const tokenImmediate: string | null = await redis.get(key);
        console.log('Immediate token check:', tokenImmediate);

        if (tokenImmediate !== 'XYZ123') {
            throw new Error(`Assertion Failed: Immediate token should be 'XYZ123', but got '${tokenImmediate}'`);
        }

        // 3. Tunggu selama 4 detik sampai data kedaluwarsa
        console.log('Waiting for 4 seconds...');
        await sleep(4000);

        // 4. Ambil setelah menunggu (harusnya sudah terhapus: null)
        const tokenAfterWait: string | null = await redis.get(key);
        console.log('Token after 4 seconds:', tokenAfterWait);

        if (tokenAfterWait !== null) {
            throw new Error(`Assertion Failed: Token should be expired and return null, but instead found: '${tokenAfterWait}'`);
        }

        console.log("✅ TEST EXPIRE PASSED SUCCESSFULLY!");
    }

    async function testSet() {
        console.log("=== TEST SET ===");
        const key = "testkey_set";
        const value = "Hello AntDb";

        console.log('Storing value with SET...');
        await redis.set(key, value);

        const storedValue: string | null = await redis.get(key);
        console.log('Stored value check:', storedValue);

        if (storedValue !== value) {
            throw new Error(`Assertion Failed: SET should store '${value}', but got '${storedValue}'`);
        }

        console.log("✅ TEST SET PASSED SUCCESSFULLY!");
    }

    async function testHset() {
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

    async function testDel() {
        console.log("=== TEST DEL ===");
        const key = "testhash";

        console.log('Deleting key with DEL...');
        const deletedCount = await redis.del(key);
        console.log('DEL result:', deletedCount);

        if (deletedCount !== 1) {
            throw new Error(`Assertion Failed: DEL should remove 1 key, but returned '${deletedCount}'`);
        }

        console.log("✅ TEST DEL PASSED SUCCESSFULLY!");
    }

    async function testExists() {
        console.log("=== TEST EXISTS ===");
        const key = "testhash";

        const existsCount = await redis.exists(key);
        console.log('EXISTS result:', existsCount);

        if (existsCount !== 0) {
            throw new Error(`Assertion Failed: EXISTS should return 0 after DEL, but got '${existsCount}'`);
        }

        console.log("✅ TEST EXISTS PASSED SUCCESSFULLY!");
    }

    try {
        console.log('--- Starting AntDb Replica Tests ---\n');

        // 1. Test PING commands (Standard and with arguments)
        console.log('Testing PING without arguments...');
        const pingStandard = await redis.ping();
        console.log('PING result:', pingStandard); // Output yang diharapkan: PONG

        console.log('Testing PING with custom message...');
        // Catatan: ioredis mengirim argumen ping lewat pemanggilan method langsung atau .call()
        const pingWithArg = await redis.call('PING', 'Hello AntDb');
        console.log('PING with argument result:', pingWithArg); // Output yang diharapkan: Hello AntDb

        console.log('\n-----------------------------------\n'); 


        await testSet();
        console.log('\n-----------------------------------\n');
        await testSetex();
        console.log('\n-----------------------------------\n');
        await testExpire();
        
        console.log('\n-----------------------------------\n');
        await testHset();
        console.log('\n-----------------------------------\n');
        await testDel();
        console.log('\n-----------------------------------\n');
        await testExists();


    } catch (error) {
        console.error('An error occurred during testing:', error);
    } finally {
        redis.disconnect();
        console.log('\n--- Tests Completed & Disconnected ---');
    }
}

testServer();