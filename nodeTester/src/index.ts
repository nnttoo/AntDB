// Haryanto 14 Juni 2026
import Redis from 'ioredis';

// Connect to the Redis server
const redis: Redis = new Redis({
    port: 6379,
    host: '127.0.0.1'
});

async function testServer(): Promise<void> {
    // 1. Test standard SET and GET
    await redis.set('projek', 'Smart Mine Tracking');
    const projek: string | null = await redis.get('projek');
    console.log('Standard GET result:', projek); // Output: Smart Mine Tracking

    // 2. Test TTL using SETEX (3 seconds lifetime)
    console.log('Storing token with a 3-second TTL...');
    await redis.setex('session_token', 3, 'XYZ123');

    // Immediate check (should exist)
    const tokenImmediate: string | null = await redis.get('session_token');
    console.log('Immediate token check:', tokenImmediate); // Output: XYZ123

    // Wait for 4 seconds to allow expiration
    console.log('Waiting for 4 seconds...');
    await new Promise<void>(resolve => setTimeout(resolve, 4000));

    // Re-check after waiting
    const tokenAfterWait: string | null = await redis.get('session_token');
    console.log('Token after 4 seconds:', tokenAfterWait); // Output: null (due to passive deletion)

    redis.disconnect();
}

testServer();