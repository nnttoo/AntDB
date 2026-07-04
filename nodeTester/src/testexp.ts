// Haryanto 04 07 2026

import Redis from "ioredis";
import { sleep } from "./sleep";

export async function testExpire(redis: Redis) {
    console.log("=== TEST EXPIRE, TTL, & PTTL ===");
    const key = "testkey33";

    // 1. Simpan token lalu atur TTL 3 detik
    console.log('Storing token with a 3-second TTL...');
    await redis.set(key, 'XYZ123');
    await redis.expire(key, 3);

    // 2. Ambil langsung dan cek sisa TTL serta PTTL-nya
    const tokenImmediate: string | null = await redis.get(key);
    const ttlImmediate: number = await redis.ttl(key);
    const pttlImmediate: number = await redis.pttl(key); // Menggunakan pttl() bawaan ioredis
    console.log('Immediate token check:', tokenImmediate);
    console.log('Immediate TTL check (seconds):', ttlImmediate);
    console.log('Immediate PTTL check (milliseconds):', pttlImmediate);

    if (tokenImmediate !== 'XYZ123') {
        throw new Error(`Assertion Failed: Immediate token should be 'XYZ123', but got '${tokenImmediate}'`);
    }
    if (ttlImmediate <= 0) {
        throw new Error(`Assertion Failed: TTL should be a positive number close to 3, but got ${ttlImmediate}`);
    }
    // PTTL harusnya bernilai ribuan karena menggunakan milidetik (misal di kisaran 2500 - 3000 ms)
    if (pttlImmediate <= 0 || pttlImmediate > 3000) {
        throw new Error(`Assertion Failed: PTTL should be a positive number close to 3000, but got ${pttlImmediate}`);
    }

    // 3. Tunggu selama 1.5 detik untuk mengecek penyusutan TTL dan PTTL
    console.log('Waiting for 1.5 seconds...');
    await sleep(1500);

    const ttlMidWay: number = await redis.ttl(key);
    const pttlMidWay: number = await redis.pttl(key);
    console.log('TTL after 1.5 seconds:', ttlMidWay);
    console.log('PTTL after 1.5 seconds (should be around 1000 - 1500):', pttlMidWay);
    
    if (ttlMidWay <= 0 || ttlMidWay > 2) {
        throw new Error(`Assertion Failed: TTL should have decreased, but got ${ttlMidWay}`);
    }
    if (pttlMidWay <= 0 || pttlMidWay > 1500) {
        throw new Error(`Assertion Failed: PTTL should have decreased below 1500, but got ${pttlMidWay}`);
    }

    // 4. Tunggu sisa waktu sampai benar-benar kedaluwarsa (total tunggu tambahan 2.5 detik)
    console.log('Waiting for another 2.5 seconds...');
    await sleep(2500);

    // 5. Ambil setelah menunggu (harusnya sudah terhapus, TTL & PTTL mengembalikan -2)
    const tokenAfterWait: string | null = await redis.get(key);
    const ttlAfterWait: number = await redis.ttl(key);
    const pttlAfterWait: number = await redis.pttl(key);
    console.log('Token after wait:', tokenAfterWait);
    console.log('TTL after wait (should be -2):', ttlAfterWait);
    console.log('PTTL after wait (should be -2):', pttlAfterWait);

    if (tokenAfterWait !== null) {
        throw new Error(`Assertion Failed: Token should be expired and return null, but instead found: '${tokenAfterWait}'`);
    }
    if (ttlAfterWait !== -2) {
        throw new Error(`Assertion Failed: TTL for expired or non-existent key should return -2, but got ${ttlAfterWait}`);
    }
    if (pttlAfterWait !== -2) {
        throw new Error(`Assertion Failed: PTTL for expired or non-existent key should return -2, but got ${pttlAfterWait}`);
    }

    // 6. Cek key yang ada tanpa expired (TTL & PTTL harus mengembalikan -1)
    const permanentKey = "permanentKey33";
    await redis.set(permanentKey, "Forever");
    const ttlPermanent: number = await redis.ttl(permanentKey);
    const pttlPermanent: number = await redis.pttl(permanentKey);
    console.log('TTL for permanent key (should be -1):', ttlPermanent);
    console.log('PTTL for permanent key (should be -1):', pttlPermanent);
    
    if (ttlPermanent !== -1) {
        throw new Error(`Assertion Failed: TTL for key without expiry should return -1, but got ${ttlPermanent}`);
    }
    if (pttlPermanent !== -1) {
        throw new Error(`Assertion Failed: PTTL for key without expiry should return -1, but got ${pttlPermanent}`);
    }

    console.log("✅ TEST EXPIRE, TTL, & PTTL PASSED SUCCESSFULLY!");
}