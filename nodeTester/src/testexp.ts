import Redis from "ioredis";
import { sleep } from "./sleep";

export async function testExpire(redis: Redis) {
    console.log("=== TEST EXPIRE & TTL ===");
    const key = "testkey33";

    // 1. Simpan token lalu atur TTL 3 detik
    console.log('Storing token with a 3-second TTL...');
    await redis.set(key, 'XYZ123');
    await redis.expire(key, 3);

    // 2. Ambil langsung dan cek sisa TTL-nya
    const tokenImmediate: string | null = await redis.get(key);
    const ttlImmediate: number = await redis.ttl(key);
    console.log('Immediate token check:', tokenImmediate);
    console.log('Immediate TTL check (should be around 3):', ttlImmediate);

    if (tokenImmediate !== 'XYZ123') {
        throw new Error(`Assertion Failed: Immediate token should be 'XYZ123', but got '${tokenImmediate}'`);
    }
    if (ttlImmediate <= 0) {
        throw new Error(`Assertion Failed: TTL should be a positive number close to 3, but got ${ttlImmediate}`);
    }

    // 3. Tunggu selama 1.5 detik untuk mengecek penyusutan TTL
    console.log('Waiting for 1.5 seconds...');
    await sleep(1500);

    const ttlMidWay: number = await redis.ttl(key);
    console.log('TTL after 1.5 seconds (should be around 1 or 2):', ttlMidWay);
    if (ttlMidWay <= 0 || ttlMidWay > 2) {
        throw new Error(`Assertion Failed: TTL should have decreased, but got ${ttlMidWay}`);
    }

    // 4. Tunggu sisa waktu sampai benar-benar kedaluwarsa (total tunggu tambahan 2.5 detik)
    console.log('Waiting for another 2.5 seconds...');
    await sleep(2500);

    // 5. Ambil setelah menunggu (harusnya sudah terhapus dan TTL mengembalikan -2)
    const tokenAfterWait: string | null = await redis.get(key);
    const ttlAfterWait: number = await redis.ttl(key);
    console.log('Token after wait:', tokenAfterWait);
    console.log('TTL after wait (should be -2):', ttlAfterWait);

    if (tokenAfterWait !== null) {
        throw new Error(`Assertion Failed: Token should be expired and return null, but instead found: '${tokenAfterWait}'`);
    }
    if (ttlAfterWait !== -2) {
        throw new Error(`Assertion Failed: TTL for expired or non-existent key should return -2, but got ${ttlAfterWait}`);
    }

    // 6. Opsional: Cek key yang ada tanpa expired (harus mengembalikan -1)
    const permanentKey = "permanentKey33";
    await redis.set(permanentKey, "Forever");
    const ttlPermanent: number = await redis.ttl(permanentKey);
    console.log('TTL for permanent key (should be -1):', ttlPermanent);
    if (ttlPermanent !== -1) {
        throw new Error(`Assertion Failed: TTL for key without expiry should return -1, but got ${ttlPermanent}`);
    }

    console.log("✅ TEST EXPIRE & TTL PASSED SUCCESSFULLY!");
} 