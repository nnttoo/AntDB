import Redis from 'ioredis';

const redis = new Redis({ host: '127.0.0.1', port: 6379 });

// ==========================================
// 1. FUNGSI MANDIRI UNTUK UJIAN STRING
// ==========================================
async function testStringRound(redisInstance: Redis, key: string, payload: string): Promise<boolean> {
  const startStr = Date.now();
  try {
    await redisInstance.set(key, payload);
    const strResult = await redisInstance.get(key);
    const durationStr = (Date.now() - startStr) / 1000;

    const isMatch = strResult === payload;
    if (isMatch) {
      console.log(`✅ String (SET/GET) Success! Time taken: ${durationStr.toFixed(3)}s`);
      return true;
    } else {
      console.log(`❌ String Mismatch detected! Time taken: ${durationStr.toFixed(3)}s`);
      return false;
    }
  } catch (err) {
    console.error(`🚨 String Operations Error:`, err);
    return false;
  }
}

// ==========================================
// 2. FUNGSI MANDIRI UNTUK UJIAN HASH
// ==========================================
async function testHashRound(redisInstance: Redis, key: string, payload: string, sizeLabel: string): Promise<boolean> {
  const startHash = Date.now();
  try {
    await redisInstance.hset(key, 'large_field', payload, 'meta_info', sizeLabel);
    const hashResult = await redisInstance.hget(key, 'large_field');
    const durationHash = (Date.now() - startHash) / 1000;

    const isMatch = hashResult === payload;
    if (isMatch) {
      console.log(`✅ Hash (HSET/HGET) Success! Time taken: ${durationHash.toFixed(3)}s`);
      return true;
    } else {
      console.log(`❌ Hash Mismatch detected! Time taken: ${durationHash.toFixed(3)}s`);
      return false;
    }
  } catch (err) {
    console.error(`🚨 Hash Operations Error:`, err);
    return false;
  }
}

// ==========================================
// MAIN RUNNER (PARALLEL EXECUTION)
// ==========================================
async function runMegaPayloadTest(): Promise<void> {
  const argSize = process.argv[2];
  let TARGET_TOTAL_MB = 400; // Default target 

  if (argSize) {
    const parsedSize = parseInt(argSize.replace(/mb/i, ''), 10);
    if (!isNaN(parsedSize) && parsedSize > 0) {
      TARGET_TOTAL_MB = parsedSize;
    }
  }

  const ITERATIONS = 5;
  const PAYLOAD_SIZE_MB = TARGET_TOTAL_MB / (ITERATIONS * 4);

  console.log(`\n🌋 [MEGA PAYLOAD MODE ACTIVATED] Starting AntDB Large Object Test...`);
  console.log(`🎯 Target Total Data Processed : ${TARGET_TOTAL_MB} MB`);
  console.log(`📦 Calculated Payload Size     : ${PAYLOAD_SIZE_MB.toFixed(2)} MB per operation`);
  console.log(`🔀 Execution Mode              : PARALLEL (Promise.all)`);

  try {
    const payloadLength = Math.floor(PAYLOAD_SIZE_MB * 1024 * 1024);
    const megaStringPayload = "M".repeat(payloadLength);

    const startTime = Date.now();
    let stringErrors = 0;
    let hashErrors = 0;

    for (let i = 0; i < ITERATIONS; i++) {
      console.log(`\n⏳ [Round ${i + 1}/${ITERATIONS}] Injecting ${PAYLOAD_SIZE_MB.toFixed(2)}MB data concurrently...`);
      const stringKey = `mega:str:${i}`;
      const hashKey = `mega:hash:${i}`;
      const sizeLabel = `${PAYLOAD_SIZE_MB.toFixed(2)}MB_TEST`;

      // Menjalankan ujian String dan Hash secara bersamaan (Paralel)
      const [stringSuccess, hashSuccess] = await Promise.all([
        testStringRound(redis, stringKey, megaStringPayload),
        testHashRound(redis, hashKey, megaStringPayload, sizeLabel)
      ]);

      if (!stringSuccess) stringErrors++;
      if (!hashSuccess) hashErrors++;
    }

    const totalTime = (Date.now() - startTime) / 1000;
    const actualTotalProcessed = (payloadLength * ITERATIONS * 4) / (1024 * 1024);

    console.log(`\n================ 👑 MEGA TEST COMPLETED 👑 ================`);
    console.log(`⏱️ Total Duration       : ${totalTime.toFixed(3)} seconds`);
    console.log(`📊 Total Data Processed  : ${actualTotalProcessed.toFixed(2)} MB (Target: ${TARGET_TOTAL_MB} MB)`);
    console.log(`-----------------------------------------------------------`);
    console.log(`❌ Mega String Errors   : ${stringErrors}`);
    console.log(`❌ Mega Hash Mismatches : ${hashErrors}`);
    console.log(`===========================================================\n`);

    if (stringErrors === 0 && hashErrors === 0) {
      console.log("🏆 ABSOLUTE MONSTER ENGINE!");
      console.log(`AntDB successfully processed ${actualTotalProcessed.toFixed(2)} MB in parallel without cracking! 🛐🦀`);
    } else {
      console.log("💥 CRITICAL: Engine anomalies or mismatches detected!");
    }
  } catch (error) {
    console.error(`\n🚨 Emergency System Failure:`, error);
  } finally {
    redis.disconnect();
  }
}

runMegaPayloadTest();