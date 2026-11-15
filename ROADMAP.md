# ROADMAP DroidNow

## 1. Arah produk dan cakupan

**Tujuan jangka panjang:** membangun alat diagnostik dan pemulihan Android yang berguna lintas merek, keluarga perangkat, dan model. DroidNow dirancang dengan lapisan umum Android serta adapter dan bukti dukungan yang spesifik per merek/OEM dan perangkat.

**Cakupan merek tahap awal bersifat multi-merek.** Xiaomi adalah salah satu keluarga merek yang masuk prioritas awal, bukan batas produk. Daftar merek lain untuk gelombang awal belum dirinci dalam dokumen ini; nama merek dan urutannya perlu ditetapkan sebelum klaim dukungan dibuat.

**Redmi 4A (`rolex`) adalah satu contoh perangkat Xiaomi untuk riset/validasi awal.** Ia bukan sasaran produk, bukan satu-satunya perangkat yang hendak didukung, dan bukan alasan untuk membatasi produk pada Xiaomi. Hasil uji satu perangkat tidak membuktikan dukungan bagi seluruh model, SoC, region, firmware, maupun metode pemulihan.

### Tingkat dukungan

- **Terdeteksi:** ADB/Fastboot atau pemindaian generik melihat perangkat. Ini bukan validasi model.
- **Diagnostik tersedia:** aplikasi dapat membaca informasi yang tersedia tanpa mengubah perangkat; hasil dan batas validasi tetap harus dijelaskan.
- **Divalidasi per model/varian:** bukti untuk identitas, SoC/SKU, region, versi sistem, dan jalur pemulihan tertentu sudah dicatat.
- **Operasi pemulihan didukung:** hanya dapat diklaim untuk pasangan perangkat-paket-metode yang telah lulus gate keselamatan dan diverifikasi secara eksplisit.

## 2. Kondisi implementasi saat ini

| Area | Status saat ini | Batas klaim |
|---|---|---|
| Pemindaian dan diagnosis ADB/Fastboot | Tersedia sebagai pemeriksaan baca-saja dasar | Perangkat terdeteksi tidak otomatis berarti model atau firmware telah divalidasi |
| Pemeriksaan folder ROM lokal | Tersedia baca-saja: menghitung hash lokal dan membandingkan petunjuk codename/region | Belum ada katalog hash tepercaya; hasil cocok hanya kandidat, bukan bukti resmi atau izin menulis |
| Mulai ulang | Tersedia dengan konfirmasi eksplisit | Bukan tindakan flash atau perbaikan |
| Dukungan per merek/perangkat | Bertahap; Xiaomi salah satu keluarga awal yang dituju | Redmi 4A (`rolex`) hanya satu contoh dengan validasi terbatas; status firmware/ARB/AVB/provenance tetap belum dipastikan |
| Flash, unlock/relock bootloader, hapus/format data, tulis partisi, bypass | Belum diimplementasikan | Tidak boleh ditampilkan sebagai kemampuan yang sudah tersedia |

## 3. Daftar keluarga merek dan contoh validasi

| Keluarga/cakupan | Peran dalam roadmap | Status bukti |
|---|---|---|
| Xiaomi | Salah satu keluarga merek prioritas pada dukungan awal multi-merek | Kebijakan dan riset awal tersedia; setiap model, SKU, SoC, region, firmware, dan operasi tetap perlu validasi tersendiri |
| Redmi 4A (`rolex`) | Contoh perangkat dalam keluarga Xiaomi untuk riset/validasi | Bukan sasaran tunggal atau batas produk; bukti firmware dan status ARB/AVB belum memadai untuk klaim pemulihan |
| Keluarga merek lain pada gelombang awal | Termasuk dalam arah multi-merek; nama dan urutan belum ditetapkan di roadmap ini | Jangan mengklaim dukungan sebelum adapter, matriks bukti, dan pengujian per model tersedia |
| Android generik | Lapisan umum untuk deteksi, koneksi, dan diagnostik yang tidak spesifik OEM | Keberhasilan membaca perangkat tidak membuktikan dukungan firmware/pemulihan lintas merek |

## 4. Kontrak keselamatan

1. **Read-only sebagai default.** Probe identitas, mode, slot, versi, status bootloader/Verified Boot, dan rollback index tidak boleh mengubah data.
2. **Gate sebelum tulis.** Sebelum setiap operasi destruktif atau penulisan, DroidNow memeriksa model, SKU/codename, provenance hardware/region, region/variant, status bootloader, versi Android/MIUI/HyperOS, slot A/B bila ada, hash artefak, kompatibilitas partisi, dan rollback metadata bila tersedia.
3. **Tidak ada inferensi berlebihan.** Nomor MIUI, tanggal, API level, label `Global`/`China`, boot normal, atau signature/VBMeta saja tidak membuktikan firmware resmi dan cocok. Pisahkan hasil `signature_valid`, `vbmeta_consistent`, `attestation_verified`, dan `official_provenance`.
4. **Anti-rollback adalah hard stop.** AVB dapat menolak image dengan `rollback_index` lebih rendah dari `stored_rollback_index`; indeks tersimpan diperbarui sebelum boot. [1] Karena matriks perangkat lintas merek/model/region tidak tersedia, `getvar anti` yang kosong bukan bukti bahwa perangkat bebas ARB.
5. **Relock paling berisiko.** Relock hanya boleh tersedia setelah kecocokan stock dibuktikan eksplisit untuk hardware, codename, region, build, hash, partisi, dan provenance. Jika ada keraguan, sediakan tombol **Berhenti dan eskalasi**.
6. **EDL/test point adalah opsi pemulihan terakhir, bukan jalur universal.** Pada sebagian perangkat dengan bootloader terkunci, flashing Fastboot normal tidak tersedia; bila pemulihan Android/Fastboot normal juga tidak mungkin, EDL/test point dapat menjadi jalur terakhir. Deteksi Qualcomm 9008, bila tersedia, hanya observasi baca-saja dan bukan izin flash. Penulisan EDL tidak termasuk P0; kelak hanya boleh lewat alur khusus perangkat/SoC dengan paket dan programmer terverifikasi serta otorisasi vendor yang sah. Jangan memberi instruksi pin test point generik.
7. **Tidak ada bypass.** Unlock timer, akun OEM, Find Device, activation lock, ARB, EDL authorization, atau kontrol keamanan OEM tidak boleh diakali.
8. **Privasi dan audit lokal.** Arsipkan manifest, hash, metadata, keputusan gate, log, dan hasil pascarepair dengan IMEI/serial/account identifier dimasking; jangan menyimpan identifier mentah sebagai provenance.

## 5. Urutan prioritas

| Fase | Fokus | Status | Prioritas | Prasyarat keluar |
|---|---|---|---:|---|
| 0 | Matriks dukungan lintas merek dan safety | Fondasi belum lengkap | P0 | Katalog bukti, stop condition, dan gate aktif sebelum operasi berisiko |
| 1 | Diagnostik Android umum + adapter keluarga merek awal (termasuk Xiaomi) | Sebagian tersedia; perlu validasi per perangkat | P0 | Probe baca-saja stabil; identitas dan tingkat validasi ditampilkan jujur |
| 2 | Autentisitas/forensik read-only | Sebagian tersedia; katalog/provenance belum lengkap | P0 | Provenance berlapis dan katalog/hash tepercaya tersedia |
| 3 | Servis non-destruktif | Direncanakan | P1 | Fase 0–2 lulus; tindakan aman dan pembatalan jelas |
| 4 | Restore ROM resmi per model dan OEM | Belum tersedia; hanya kandidat terverifikasi di masa depan | P1 | Paket, target, metode OEM, backup, dan rencana pemulihan cocok eksplisit |
| 5 | Unlock bootloader per OEM | Ditunda; opt-in bila dikembangkan | P2 | Konfirmasi wipe, backup teruji, alur resmi OEM, dan uji non-produksi |
| 6 | Custom recovery/ROM | Ditunda | P3 | Kontrak keselamatan khusus perangkat; bukan jalur pemulihan umum |
| 7 | Relock bootloader | Ditunda paling ketat | P3 | Bukti kecocokan stock lengkap dan boot pascarestore berhasil; default tetap berhenti |
| 8 | Perluasan ke keluarga merek di luar gelombang awal | Direncanakan setelah fondasi | P4 | Adapter terisolasi dan matriks bukti tersendiri; tidak mewarisi asumsi OEM lain |

---

## Fase 0 — Matriks dukungan dan safety

### Tujuan
Membangun batas dukungan lintas merek yang dapat diaudit sebelum DroidNow menyentuh perangkat atau firmware.

### Pekerjaan

- Definisikan matriks per **OEM/keluarga merek, model, SKU, codename, SoC, hardware/region, varian, era firmware, Android, MIUI/HyperOS bila relevan, mode bootloader/fastbootd, slot, AVB, dan ARB**.
- Catat keluarga merek gelombang awal dan urutannya. Xiaomi adalah salah satunya; jangan menyiratkan seluruh produk hanya untuk Xiaomi. Mulai riset perangkat dengan Redmi 4A `rolex` sebagai satu contoh saja, bukan batas produk. Klaim komunitas bahwa rolex tidak memiliki ARB hanya hipotesis read-only, bukan jaminan [2].
- Bangun katalog firmware dengan tipe paket, build, region, ukuran, SHA-256, URL sumber, tanggal, metadata rollback, status verifikasi, dan provenance.
- Tetapkan stop condition: identitas konflik/tidak terbaca, paket tidak cocok, hash gagal, rollback tidak dapat dinilai, toolchain tidak stabil, interupsi, atau recovery memerlukan bypass.
- Rancang schema arsip kasus: perangkat, operasi, bukti, keputusan gate, error, hasil, dan identifier yang dimasking.
- Pisahkan kemampuan **diagnose**, **prepare**, **write**, **unlock**, dan **relock**; tidak ada transisi otomatis dari diagnosis lulus ke operasi berisiko.

### Keluaran

- Matriks dukungan versioned.
- Daftar adapter keluarga merek gelombang awal dan policy per OEM; daftar codename/region hanya untuk model yang buktinya benar-benar tersedia.
- Katalog firmware terverifikasi.
- State machine safety gate dan daftar stop condition.
- Format arsip/log bermasking serta threat model.

### Kontrol risiko

Cakupan produk adalah Android lintas merek; Xiaomi hanya salah satu keluarga dalam gelombang awal. Itu bukan klaim bahwa semua model Xiaomi atau keluarga lain telah divalidasi. Setiap model/SKU/SoC/region tetap perlu bukti tersendiri. Xiaomi menyatakan ketersediaan update bersifat spesifik model dan region [15], dan jadwal resminya mencantumkan lini Xiaomi, REDMI, serta POCO dengan waktu ketersediaan yang dapat bervariasi [16]. Tolak label region saja sebagai dasar kecocokan. Xiaomi tidak menerbitkan matriks publik bahwa semua paket/region aman untuk semua hardware [3].

---

## Fase 1 — Diagnostik Android umum dan adapter keluarga merek awal

### Tujuan
Mendapatkan gambaran perangkat Android secara read-only melalui lapisan umum, lalu menambahkan interpretasi spesifik OEM hanya melalui adapter yang buktinya tersedia. Xiaomi termasuk keluarga awal; pemeriksaan Redmi 4A tetap satu contoh, bukan cakupan eksklusif.

### Pekerjaan

- Probe berulang untuk model/SKU/codename, region/variant, build/version, Android, bootloader state, Verified Boot state, mode, slot, daftar partisi, dan properti vendor yang memang didukung; `fastboot getvar anti` hanya diperlakukan sebagai bukti jika benar-benar tersedia.
- Catat versi `adb`/`fastboot`, driver, koneksi, kabel/port, serta hasil handshake tanpa menulis.
- Tampilkan ketidakpastian setiap field; hasil kosong dari `getvar anti` berarti **tidak terdeteksi**, bukan `ARB aman`.
- Wizard gejala:
  - **Fastboot saja:** force-restart resmi dengan tombol daya lebih dari 10 detik; bila tetap gagal, eskalasi service center [4].
  - **Tool/driver/script error:** cek kabel, port langsung tanpa hub, driver, platform-tools, dan log; jangan retry otomatis setelah timeout.
  - **`super`/partisi gagal:** hentikan dan validasi mode serta image; jangan memakai raw partition, flashing EDL, atau bypass otorisasi.
  - **`This MIUI version can't be installed on this device`, `Erase is not allowed in Lock State`, `system has been destroyed`, atau loop Fastboot:** jangan force-flash; pilih firmware tepat atau eskalasi.
  - **Layar hitam/EDL saja:** deteksi Qualcomm 9008, jika tersedia, hanya sebagai pemeriksaan baca-saja. Pada sebagian perangkat ber-bootloader terkunci, Fastboot tidak menyediakan jalur flash normal; bila Android/Fastboot normal tidak dapat digunakan, EDL/test point dapat dipertimbangkan sebagai pemulihan terakhir melalui servis resmi/authorized. Jangan melakukan flashing EDL pada P0 atau memberi pin test point generik.
  - **Rencana label mode darurat (baca-saja):** bila USB yang teramati menunjukkan `05c6:9008`, labeli sebagai Qualcomm QDL/EDL [17]. MediaTek BROM dan Preloader adalah label mode berbeda, bukan EDL. Berhenti pada mode/ID yang benar-benar teramati; jangan simpulkan otorisasi, ketersediaan programmer, atau kemampuan flash. Aplikasi saat ini belum mendeteksi atau memakai mode ini.
- Minta baterai minimum konservatif, cegah suspend/sleep, dan tampilkan larangan mencabut kabel saat operasi kelak.

### Keluaran

- Laporan diagnostik read-only.
- Wizard berbasis gejala dengan tindakan aman.
- Probe koneksi dan toolchain yang dapat diulang.
- Arsip kasus awal dengan identifier disamarkan.

### Kontrol risiko

AOSP menyarankan penggantian kabel, port, atau konektor saat fastboot/flash bermasalah dan menjelaskan bahwa flashall memeriksa kompatibilitas bootloader/radio [5]. Kasus toolchain dapat gagal setelah sebagian partisi ditulis; pembaruan platform-tools menyelesaikan satu kasus `super`, tetapi bukan aturan universal [6]. Karena itu timeout/interupsi selalu kembali ke probe read-only dan gate baru.

---

## Fase 2 — Autentisitas dan forensik read-only

### Tujuan
Membedakan bukti integritas dari bukti provenance tanpa mengubah perangkat.

### Pekerjaan

- Hash SHA-256 paket dan komponen; cocokkan hanya dengan nilai pembanding yang diterbitkan sumber tepercaya.
- Pra-pemeriksaan read-only membandingkan petunjuk codename/region paket dari nama folder atau `android-info.txt` dengan identitas perangkat; mismatch berarti berhenti, sementara kandidat cocok tetap tidak memberi izin tulis dan tidak membuktikan provenance.
- Validasi signature/VBMeta, hash/hashtree descriptor, chained partitions, vbmeta digest, dan rollback metadata bila tersedia.
- Tampilkan empat klaim terpisah: `signature_valid`, `vbmeta_consistent`, `attestation_verified`, `official_provenance`.
- Klasifikasikan sumber: `official_source`, `official_mirror_with_matching_hash`, `community_archive_unverified`, `unknown`.
- Jika attestation kelak didukung, validasi rantai sertifikat, root, hardware security level, signature, dan revocation secara terpisah/off-device; hasil parsial tetap `tidak terverifikasi` [7].
- Simpan URL, tipe paket, region, versi, ukuran, hash, waktu, versi tool, dan keputusan gate ke arsip bermasking.

### Keluaran

- Laporan forensik read-only.
- Manifest dan hash katalog firmware.
- Bukti provenance berlapis dan alasan setiap status.
- Ekspor arsip kasus lokal yang dapat diaudit.

### Kontrol risiko

AVB membuktikan rantai verifikasi dan kecocokan metadata terhadap kunci/descriptor, bukan otomatis bahwa file diunduh dari Xiaomi [7]. Unlocked/custom ROM dan boot normal tidak boleh diberi badge `authentic` atau dijadikan dasar relock.

---

## Fase 3 — Servis non-destruktif

### Tujuan
Menyelesaikan diagnosis dan pemulihan resmi yang tidak menurunkan rollback index, tidak melewati otorisasi, dan tidak menulis berisiko.

### Pekerjaan

- Jalankan hanya operasi read-only, reboot resmi, verifikasi koneksi, pengumpulan log, backup metadata sah, dan pemulihan yang tidak memodifikasi partisi kritis.
- Tampilkan preview tindakan, dampak data, prasyarat, dan tombol **Batal**.
- Setelah gangguan, lakukan probe read-only; jangan mengulang flash otomatis.
- Tawarkan pemulihan resmi non-destruktif sesuai gejala, lalu verifikasi boot normal.

### Keluaran

- Wizard servis non-destruktif.
- Log sebelum/sesudah dan arsip keputusan.
- Laporan apakah perangkat pulih, tetap Fastboot, atau harus dieskalasi.

### Kontrol risiko

Bootloader terkunci dapat membuat reflashing melalui Fastboot tidak tersedia; operasi unlock/lock juga dapat melakukan factory reset [8]. Jika Android/Fastboot normal tidak tersedia dan EDL menjadi satu-satunya jalur, perlakukan EDL/test point sebagai opsi pemulihan terakhir melalui servis resmi/authorized. Deteksi 9008 tetap baca-saja dan tidak memberi izin untuk menulis. Penulisan EDL berada di luar P0 dan, bila kelak dipertimbangkan, wajib device/SoC-specific dengan programmer Firehose dan paket bertanda tangan/terverifikasi serta otorisasi vendor yang tepat [9]; tidak boleh universal, menyertakan pin test point generik, atau melewati otorisasi akun/OEM.

---

## Fase 4 — Restore ROM resmi per model dan OEM

### Tujuan
Memulihkan firmware resmi hanya pada pasangan perangkat-paket-metode OEM yang terverifikasi. Riset Xiaomi pada roadmap ini adalah satu studi keluarga merek, bukan aturan yang dapat diterapkan ke OEM lain.

### Prasyarat wajib

- Fase 0–3 lulus.
- Model/SKU/codename, hardware provenance, region/variant, build target, mode, slot, dan paket cocok eksplisit.
- Hash dan provenance paket lulus katalog; rollback index image tidak lebih rendah dari indeks perangkat untuk lokasi yang diketahui.
- Backup sah memiliki manifest/checksum dan pengguna memahami risiko kehilangan data.
- Metode yang digunakan didukung OEM untuk model, state bootloader, dan paket yang tepat; jalur resmi dapat menolak mismatch. Tidak memakai TWRP/jalur recovery yang melewati pemeriksaan rollback.

### Pekerjaan

- Tampilkan diff perangkat-versus-paket sebelum write.
- Minta konfirmasi eksplisit untuk wipe/data loss dan risiko interupsi.
- Gunakan hanya metode OEM yang didukung untuk firmware tepat yang sudah diverifikasi, setelah prasyarat status bootloader terpenuhi melalui alur resmi OEM yang sesuai; tanpa retry otomatis setelah timeout.
- Setelah selesai, verifikasi model/codename/region/build, AVB/bootloader, slot, kemampuan boot normal, beberapa reboot, storage/radio health, dan hash/log.
- **Jangan relock otomatis pada fase ini.**

### Keluaran

- Restore report lengkap.
- Bukti kecocokan paket dan hasil verifikasi pascarepair.
- Arsip kasus lokal bermasking.

### Kontrol risiko

Rollback image lebih rendah dapat ditolak bootloader resmi; jalur pihak ketiga yang tidak memvalidasi indeks dapat menyebabkan bootloop atau hard-brick [1][10]. Kasus Xiaomi era MIUI 10 menunjukkan downgrade tertentu dapat berujung brick dan service center [10]. Kasus modern komunitas juga melaporkan mismatch region setelah relock; itu sinyal risiko, bukan bukti universal [11]. Jika bootloader terkunci membuat flashing Fastboot tidak tersedia, jangan memintas proteksi: gunakan alur UBL resmi Xiaomi bila diwajibkan oleh metode OEM, lalu hanya flash paket persis yang terverifikasi dengan metode OEM yang didukung. Bila pemulihan Android/Fastboot resmi tidak tersedia, EDL/test point tetap opsi terakhir yang memerlukan programmer/firmware khusus perangkat dan otorisasi servis vendor; deteksi 9008 bukan izin menulis.

---

## Fase 5 — Unlock bootloader per OEM

### Status
**Eksperimental, opt-in, bukan jalur pemulihan data; belum diimplementasikan. P0 tetap baca-saja.**

### Prasyarat

- Perangkat non-produksi atau data memang boleh hilang.
- Backup sah telah diuji dengan manifest/checksum; DroidNow tidak menjanjikan pemulihan data terenkripsi/lockscreen.
- Konfirmasi eksplisit bahwa unlock menghapus data; tidak ada default tersembunyi.
- Untuk perangkat Xiaomi, gunakan alur UBL resmi Xiaomi yang berlaku: masuk dengan Mi Account sendiri, lakukan pairing/binding akun dan perangkat melalui mekanisme resmi, lalu penuhi eligibility, ketentuan SIM/network/region, serta masa tunggu yang ditampilkan untuk perangkat itu. Alur ini tidak boleh diasumsikan berlaku bagi OEM lain. Timer, akun, pairing, atau otorisasi tidak boleh diakali.
- Model/codename/region/version/hash dan status bootloader cocok.

### Pekerjaan dan keluaran

- Tampilkan syarat eligibility/pairing dan masa tunggu resmi beserta ketidakpastian kebijakan per model/region; jangan mengasumsikan durasi tetap.
- Jalankan hanya alur UBL resmi Xiaomi setelah konfirmasi kedua yang menyebutkan dengan jelas bahwa data pengguna akan dihapus; jangan bypass Mi Account, Find Device, activation lock, atau kontrol OEM.
- Verifikasi pasca-unlock: bootloader state, Verified Boot warning/state, fingerprint, storage/radio health, dan status akun.
- Simpan unlock report dan arsip kasus.

### Kontrol risiko

AOSP menyatakan unlock menghapus data dan memungkinkan OS yang tidak ditandatangani root of trust; lock juga mereset perangkat [8]. Xiaomi menyebut pairing akun/perangkat, SIM, masa tunggu yang dapat berubah, dan wipe yang bergantung model/alur [12]. Find Device/lost mode dapat mencegah unlock/access dan flashing bukan bypass activation lock [13].

---

## Fase 6 — Custom recovery/ROM

### Status
**Ditunda dan tidak didukung sebagai jalur pemulihan umum.**

### Syarat jika kelak diuji

- Kontrak keselamatan khusus codename/SoC/region/firmware.
- Backup sah, data boleh hilang, dan artefak memiliki provenance serta hash yang jelas.
- Tidak ada klaim bahwa custom recovery memvalidasi rollback; tidak boleh dipakai untuk menurunkan indeks.
- Uji hanya pada perangkat non-produksi, dengan pemulihan resmi yang sudah terbukti.
- Setiap image dan partisi melewati gate; kegagalan berarti berhenti, bukan bypass.

### Keluaran dan kontrol risiko

Keluaran berupa laporan uji terisolasi, bukan fitur pemulihan umum. TWRP atau jalur pihak ketiga yang tidak melakukan pemeriksaan ARB tidak boleh digunakan untuk downgrade [10]. Tidak ada perluasan ke merek lain dari hasil fase ini.

---

## Fase 7 — Relock bootloader

### Status
**Risiko tertinggi; default ditolak.**

### Syarat keras

Relock hanya boleh ditawarkan bila seluruh bukti berikut tersedia dan konsisten:

- Hardware/SKU/codename/region/variant/provenance cocok.
- Paket **stock resmi lengkap** berasal dari `official_source` atau mirror dengan hash resmi yang cocok.
- Build, rollback state, partisi kritis, cust/metadata, slot, dan AVB policy cocok.
- Tidak ada custom recovery, root, modifikasi, atau sisa image pihak ketiga.
- Restore telah boot normal dan lolos beberapa reboot serta verifikasi pascarepair.
- Pengguna melihat ringkasan bukti, memahami wipe/risk, dan dapat memilih **Berhenti**.

### Prosedur aman

1. Jalankan final read-only gate dan tampilkan semua bukti.
2. Jika satu field `unknown`/konflik, batalkan relock dan eskalasi.
3. Jangan menawarkan relock lintas China/Global/EEA hanya berdasarkan label region.
4. Jika prosedur resmi model-spesifik tidak tersedia, jangan relock.
5. Setelah relock, verifikasi boot, model/codename/region/build, AVB/bootloader, slot, beberapa reboot, dan error state. Jika gagal, jangan force-flash atau unlock bypass.

### Keluaran

- Relock eligibility report dengan bukti eksplisit.
- Hanya jika lulus: laporan relock dan verifikasi pascarepair.
- Jika tidak lulus: arsip alasan penolakan dan rujukan servis resmi.

### Kontrol risiko

Xiaomi memperingatkan firmware tidak didukung dapat menyebabkan gagal boot dan masalah kompatibilitas [3]. Laporan komunitas Xiaomi Pad 6 `pipa` dan CC9 `pyxis` menggambarkan Global/China mismatch setelah relock, Fastboot loop, pesan `This MIUI version can't be installed on this device`, `Erase is not allowed in Lock State`, dan kegagalan unlock ulang [11][14]. Laporan tersebut tidak membuktikan semua perangkat berperilaku sama, tetapi cukup untuk menjadikan relock fail-closed. **Unlock bukan cara menonaktifkan ARB.**

---

## Fase 8 — Perluas dukungan keluarga merek

### Status
**Untuk keluarga di luar gelombang awal; ditunda sampai ada prioritas, bukti, dan policy adapter terpisah.**

### Prasyarat

- Matriks resmi/terverifikasi per OEM: boot chain, rollback, bootloader, firmware provenance, relock, recovery, akun, dan servis.
- Implementasi adapter terisolasi; tidak mewarisi asumsi Xiaomi maupun asumsi OEM lain tentang AVB, EDL, ARB, atau kebijakan wipe.
- Uji read-only, katalog artefak, gate, interupsi, dan pascarepair pada setiap model sebelum write.
- Review keamanan dan incident rollback sebelum rilis.

### Keluaran dan kontrol risiko

Keluaran awal untuk setiap keluarga baru adalah diagnostik/read-only. Tidak ada klaim lintas-merek aman sampai ada bukti model-spesifik dan sumber terverifikasi.

---

## Verifikasi pascarepair dan arsip kasus

Setiap operasi write yang diizinkan wajib menghasilkan laporan berisi: model/SKU/codename, region/variant, build/version, bootloader/AVB state, slot, rollback index yang terlihat, paket dan SHA-256, provenance, versi tool, waktu, hasil boot normal, reboot test, storage/radio health, error, keputusan gate, dan alasan berhenti bila ada. IMEI, serial, akun, dan token dimasking atau di-hash satu arah.

Status akhir harus salah satu dari: **pulih dan terverifikasi**, **pulih tetapi bukti parsial**, **tetap Fastboot/EDL**, atau **eskalasi service center**. Status `pulih` tidak boleh menyamarkan bukti yang belum tersedia.

## Prioritas implementasi

1. **P0:** Fase 0–2: schema evidence, matrix, katalog/hash, probe read-only, gate, logging bermasking, stop condition.
2. **P1:** Fase 3: wizard gejala, preflight kabel/toolchain, interupsi aman, dan arsip kasus.
3. **P1 terbatas:** Fase 4: restore resmi hanya untuk paket/codename/region yang terverifikasi; retry manual berbasis bukti.
4. **P2:** Fase 5 hanya opt-in dengan wipe confirmation, binding resmi, backup teruji, dan perangkat non-produksi.
5. **P3:** Fase 6–7 tetap ditunda; relock paling ketat dan fail-closed.
6. **P4:** Fase 8 hanya untuk keluarga merek di luar gelombang awal, setelah prioritas adapter OEM dan bukti per-model tersedia.

## Gap dan ketidakpastian yang harus tetap terlihat

- Tidak ada dokumentasi Xiaomi publik lengkap yang memetakan ARB, AVB, cust, region, relock, dan EDL untuk semua model/region/era.
- Tidak ada artefak firmware `rolex` yang diverifikasi dalam riset ini; Redmi 4A adalah satu contoh riset, dan status ARB/AVB/provenance-nya tetap dapat `indeterminate`.
- Perilaku modern HyperOS, multi-lokasi rollback index, dan jalur EDL dapat berbeda menurut SoC, bootloader, region, dan paket.
- Sumber XDA/Reddit adalah laporan komunitas atau self-report: berguna untuk threat model, bukan frekuensi atau jaminan kausal universal.
- Batas baterai konservatif, probabilitas brick, dan matriks error khusus perangkat belum ditetapkan oleh sumber yang digunakan; kekosongan ini tidak terbatas pada Redmi 4A.
- Kebijakan Mi Account, masa tunggu, eligibility, dan wipe dapat berubah menurut waktu, akun, region, dan model.

## Referensi

[1]: https://source.android.com/docs/security/features/verifiedboot "Verified Boot | Android Open Source Project"

[2]: https://xdaforums.com/t/redmi-4a-rolex-anti-rollback-protection.3841927/ "Redmi 4a (Rolex) - Anti Rollback Protection"

[3]: https://www.mi.com/global/support/faq/details/KA-07238/ "Boot Loader Lock Use Introduction FAQ"

[4]: https://www.mi.com/global/support/faq/details/KA-550626/ "Why is the phone stuck in the Fastboot interface?"

[5]: https://source.android.com/docs/setup/test/running "Flash with Fastboot"

[6]: https://forum.fairphone.com/t/flashing-fp5-with-fastboot-error-could-not-flash-the-super-partition-on-device/106592 "Flashing FP5 with fastboot error ‘Could not flash the super partition on device’"

[7]: https://android.googlesource.com/platform/external/avb/+/master/README.md "Android Verified Boot 2.0 README"

[8]: https://source.android.com/docs/core/architecture/bootloader/locking_unlocking "Lock and unlock the bootloader"

[9]: https://www.96boards.org/documentation/consumer/guides/qdl.md.html "Qualcomm USB flashing tool"

[10]: https://www.xda-developers.com/xiaomi-anti-rollback-protection-brick-phone/ "Xiaomi's Anti-Rollback Protection Explained: How to avoid bricking your phone"

[11]: https://xdaforums.com/t/locking-bootloader-on-a-different-global-region-rom.4563349/ "Locking bootloader on a different global region rom?"

[12]: https://www.mi.com/uk/support/faq/details/KA-07238/ "Boot Loader Lock Use Introduction FAQ"

[13]: https://privacy.mi.com/support/faq?locale=en "Xiaomi Privacy Support FAQs"

[14]: https://xdaforums.com/t/xiaomi-pad-6-pipa-hard-brick-locked-bootloader-region-mismatch-cn-hardware-global-rom.4781752/ "Xiaomi Pad 6 (pipa) - Hard Brick - Locked Bootloader - Region Mismatch (CN Hardware / Global ROM)"

[15]: https://www.mi.com/global/support/faq/details/KA-483292/ "The REDMI Note 13 Pro+ 5G cannot get a Xiaomi HyperOS update"

[16]: https://www.mi.com/global/hyperos "Xiaomi HyperOS release schedule and supported models"

[17]: https://docs.qualcomm.com/bundle/publicresource/topics/80-70018-251/rb3_gen2_force_edl_mode.html "Force RB3 into emergency download mode (Qualcomm)"

### Sumber tambahan yang digunakan dalam riset tetapi tidak dijadikan dasar klaim universal

- https://indianexpress.com/article/technology/mobile-tabs/downgrading-your-xiaomi-redmi-note-5-pro-might-brick-it-heres-why-5256424/ — *MIUI 10 Global Beta 8.7.5 adds anti-rollback protection: Do not downgrade your Redmi Note 5 Pro*.
- https://www.reddit.com/r/Xiaomi/comments/ex1ucm/did_i_brick_my_phone_by_locking_the_bootloader/ — *Did I brick my phone by locking the bootloader? (Xiaomi CC9)*.
- https://new.c.mi.com/ng/post/10512 — *How to unlock the bootloader of your Xiaomi phone. Tips, tricks and troubleshooting*.
- https://developer.android.com/privacy-and-security/security-key-attestation — *Verify hardware-backed key pairs with key attestation*.
- https://tgrush.bearblog.dev/android-verified-boot/ — *Android Verified Boot isn't a Silver Bullet for security*.
- https://www.xda-developers.com/xiaomi-edl-unbrick-authorized-mi-accounts/ — *Xiaomi needs a better way to unbrick its devices instead of relying on Authorized Mi Accounts*.
