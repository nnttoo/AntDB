import Redis from 'ioredis';

// Increase max length limits for massive data handling
// 
const redis = new Redis({
  host: '127.0.0.1',
  port: 6379 
});

async function runMegaPayloadTest(): Promise<void> {
  console.log("🌋 [MEGA PAYLOAD MODE ACTIVATED] Starting AntDB Large Object Test...");
  
  // Generating exactly 20MB of text data (1 char = 1 byte)
  const PAYLOAD_SIZE_MB = 20;
  console.log(`📦 Generating massive ${PAYLOAD_SIZE_MB}MB string payload in memory...`);
  const megaStringPayload = "M".repeat(PAYLOAD_SIZE_MB * 1024 * 1024); 
  
  const ITERATIONS = 5; // Tested multiple times sequentially to check stability
  const startTime = Date.now();
  
  let stringErrors = 0;
  let hashErrors = 0;

  try {
    for (let i = 0; i < ITERATIONS; i++) {
      console.log(`\n⏳ [Round ${i + 1}/${ITERATIONS}] Injecting 20MB data...`);
      
      const stringKey = `mega:str:${i}`;
      const hashKey = `mega:hash:${i}`;

      // --- STEP 1: TEST MASSIVE STRING (SET & GET) ---
      const startStr = Date.now();
      await redis.set(stringKey, megaStringPayload);
      const strResult = await redis.get(stringKey);
      const durationStr = (Date.now() - startStr) / 1000;
      
      if (strResult !== megaStringPayload) {
        stringErrors++;
        console.log(`❌ String Mismatch on Round ${i + 1}`);
      } else {
        console.log(`✅ String (SET/GET) Success! Time taken: ${durationStr.toFixed(3)}s`);
      }

      // --- STEP 2: TEST MASSIVE HASH (HSET & HGET) ---
      const startHash = Date.now();
      // Writing the 20MB string inside a specific hash field
      await redis.hset(hashKey, 'large_field', megaStringPayload, 'meta_info', '20MB_TEST');
      
      const hashResult = await redis.hget(hashKey, 'large_field');
      const durationHash = (Date.now() - startHash) / 1000;

      if (hashResult !== megaStringPayload) {
        hashErrors++;
        console.log(`❌ Hash Mismatch on Round ${i + 1}`);
      } else {
        console.log(`✅ Hash (HSET/HGET) Success! Time taken: ${durationHash.toFixed(3)}s`);
      }
    }

    const totalTime = (Date.now() - startTime) / 1000;
    const totalDataProcessed = PAYLOAD_SIZE_MB * ITERATIONS * 4; // SET, GET, HSET, HGET

    console.log(`\n================ 👑 MEGA TEST COMPLETED 👑 ================`);
    console.log(`⏱️ Total Duration       : ${totalTime.toFixed(3)} seconds`);
    console.log(`📊 Total Data Processed  : ${totalDataProcessed} MB`);
    console.log(`-----------------------------------------------------------`);
    console.log(`❌ Mega String Errors   : ${stringErrors}`);
    console.log(`❌ Mega Hash Mismatches : ${hashErrors}`);
    console.log(`===========================================================\n`);

    if (stringErrors === 0 && hashErrors === 0) {
      console.log("🏆 ABSOLUTE MONSTER ENGINE!");
      console.log("AntDB successfully allocated and parsed 20MB single payloads without cracking! 🛐🦀");
    } else {
      console.log("💥 CRITICAL: The engine choked on large allocations!");
      console.log("Time to debug Rust's memory buffers! 🤪");
    }

  } catch (error) {
    console.error("\n🚨 Emergency! The server crashed or refused the massive 20MB payload:", error);
  } finally {
    redis.disconnect();
  }
}

runMegaPayloadTest();