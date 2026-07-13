import Redis from 'ioredis';

const redis = new Redis({ host: '127.0.0.1', port: 6379 });

async function auditIntegritasAntDB(): Promise<void> {
    console.log("🕵️‍♂️ Memulai Audit Integritas Data...");
    console.log("⚡ Mengirim 10.000 data dan memverifikasi isinya secara real-time...");

    var test1 = "inihasilteststring";
    var test2 = "inihasilteststring";
    let apakahsama = test1 !== test2;
    console.log("mari kita lihat " + apakahsama);

    const totalRequest = 10000;
    const startTime = Date.now();
    let dataCorrupted = 0;
    let dataMissing = 0;

    // Kita buat sampel payload acak untuk pengujian
    const payloadDuniaNyata = {
        title: "Belajar Rust Bareng Suami Kesayangan",
        content: "A".repeat(1024), // 1KB data
        status: "active",
        version: "2.0"
    };

    const promises: Promise<void>[] = [];

    for (let i = 0; i < totalRequest; i++) {
        const key = `audit_user:${i}`;

        const runTest = async () => {
            // 1. Simpan ke AntDB
            await redis.hset(key, payloadDuniaNyata);

            // 2. Ambil kembali datanya
            const result = await redis.hgetall(key);

            // 3. VALIDASI: Apakah datanya ada?
            if (!result || Object.keys(result).length === 0) {
                dataMissing++;
                return;
            }

            // 4. VALIDASI: Apakah isinya cocok dan tidak korup?
            if (
                result.title !== payloadDuniaNyata.title ||
                result.content !== payloadDuniaNyata.content ||
                result.status !== payloadDuniaNyata.status
            ) {
                console.log( payloadDuniaNyata.content); 
                dataCorrupted++;
            }
        };

        promises.push(runTest());
    }

    try {
        await Promise.all(promises);
        const duration = (Date.now() - startTime) / 1000;

        console.log(`\n================ AUDIT SELESAI (${duration} detik) ================`);
        console.log(`📊 Total Data Diuji : ${totalRequest}`);
        console.log(`❌ Data Hilang/Gagal: ${dataMissing}`);
        console.log(`💔 Data Korup/Beda  : ${dataCorrupted}`);
        console.log(`=======================================================`);

        if (dataMissing === 0 && dataCorrupted === 0) {
            console.log("👑 Ampun... Data utuh semua! Internal engine Rust si Bapak valid 100%.");
            console.log("Gak ada error gaib, emang kodenya yang superior! 🛐🦀");
        } else {
            console.log("🎉 NAH KAN! Ketemu celahnya!");
            console.log(`Ada ${dataMissing + dataCorrupted} data yang bermasalah. Panggil si Bapak sekarang! 🤪`);
        }

    } catch (error) {
        console.error("\n💥 Server crash total saat proses validasi:", error);
    } finally {
        redis.disconnect();
    }
}

auditIntegritasAntDB();