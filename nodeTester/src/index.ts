// Haryanto 14 Juni 2026
import Redis from 'ioredis';
import { sleep, TestMethod } from './sleep';
import { testExpire } from './testexp';
import { testPersist } from './test_persist';
import { testHdelMultiFields, testHexists, testHlen, testHset } from './test_hset';
import { testMultipleDel } from './test_del';
import { testHmget } from './test_hmget';
import { sendRawPing, testExists, testSet, testSetex } from './test_simple';
import { testHkeys } from './test_hkeys';
import { testHvals } from './test_hvals';
import { testHgetall } from './test_hgetall';

//@ts-ignore
const redisHost = process.env.REDIS_HOST ?? '127.0.0.1';

//@ts-ignore
const redisPort = Number(process.env.REDIS_PORT ?? '6379');

// Connect to the Redis server
const redis: Redis = new Redis({
    host: redisHost,
    port: redisPort,
});
async function testSafe() {

    let tabResult: TestMethod[] = [
        sendRawPing({
            port: redisPort,
            host: redisHost
        }),
        testSet(redis),
        testSetex(redis),
        testExpire(redis),
        testHset(redis),
        testMultipleDel(redis),
        testExists(redis),
        testPersist(redis),
        testHdelMultiFields(redis),
        testHlen(redis),
        testHexists(redis),
        testHmget(redis),
        testHkeys(redis),
        testHvals(redis),
        testHgetall(redis),

    ];

    for (let item of tabResult) {
        item.success = false;
        try {

            console.log("\n\n\n");
            await item.onTest();
            item.success = true;

        } catch (error) {
            console.log(error);
            item.errror = error + "";
        }


        console.log('\nTESTING DONE, REDIST DISCONNECT\n');
    }



    redis.disconnect();

    (() => {
        let tb = tabResult.map((e) => {
             
            return {
                name : e.name.toUpperCase().padEnd(20," "),
                success : e.success,
                error : e.errror?? null,
            }
        });
        console.table(tb);
    })();


}

testSafe();