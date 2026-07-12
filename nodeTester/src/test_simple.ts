
import Redis from "ioredis"
import * as net from 'net';
import { sleep, TestMethod } from "./sleep";

export function testSetex(redis: Redis): TestMethod {
    return {
        name: "SETEX",
        success: false,
        async onTest() {
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
    };

}
export function sendRawPing(arg: {
    port: number,
    host: string,
}): TestMethod {
    let p: TestMethod = {
        name: "PING",
        success: false,
        async onTest() {
            console.log("Testing raw ping--------------------------")

            let port = arg.port;
            let host = arg.host;
            let response = await new Promise((resolve, reject) => {
                const client: net.Socket = net.createConnection({ port, host }, () => {
                    // Send exactly "PING\r\n" as raw text bytes
                    client.write("PING\r\n");
                });

                client.on('data', (data: Buffer) => {
                    resolve(data.toString());
                    client.end(); // Clean up connection
                });

                client.on('error', (err: Error) => {
                    reject(err);
                });
            });

            console.log(response + ":---");

            if (response == "+PONG\r\n") {
                console.log("✅ TEST sendRawPing PASSED SUCCESSFULLY!");
            } else {
                throw (`Assertion Failed: sendRawPing`);
            }
        },
    };

    return p;
}

export async function testRedisPing(redis: Redis) {
    // 1. Test PING commands (Standard and with arguments)
    console.log('Testing PING without arguments...');
    const pingStandard = await redis.ping();
    console.log('PING result:', pingStandard); // Output yang diharapkan: PONG

    console.log('Testing PING with custom message...');

    var haloDB = "Hello AntDb";
    const pingWithArg = await redis.call('PING', haloDB);
    console.log('PING with argument result:', pingWithArg); // Output yang diharapkan: Hello AntDb

    if (pingWithArg != haloDB) {
        throw new Error(`Assertion Failed: Ping should store  '${haloDB}', but got '${pingWithArg}'`);
    }


    console.log("✅ TEST Ping PASSED SUCCESSFULLY!");
}

export function testSet(redis: Redis): TestMethod {
    return {
        name: "SET",
        success: false,
        async onTest() {
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
    };

} 

export function testExists(redis: Redis): TestMethod {
    return {
        name: "existts",
        success: false,
        async onTest() {

            console.log("=== TEST EXISTS ===");
            const key = "testtestexistskeys";

            const existsCount = await redis.exists(key);
            console.log('EXISTS result:', existsCount);

            if (existsCount !== 0) {
                throw new Error(`Assertion Failed: EXISTS should return 0 after DEL, but got '${existsCount}'`);
            }

            console.log("✅ TEST EXISTS PASSED SUCCESSFULLY!");
        }
    };
}