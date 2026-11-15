# DroidNow

DroidNow adalah aplikasi desktop Rust dengan antarmuka egui/eframe untuk memeriksa perangkat Android melalui ADB dan Fastboot. Deteksi koneksi bersifat umum. Validasi identitas model saat ini hanya mencakup Xiaomi Redmi 4A (`rolex`).

## Cakupan pemeriksaan

DroidNow membaca informasi yang tersedia dari perangkat dan dapat memeriksa berkas lokal. Aplikasi tidak memasang firmware, membuka atau mengunci bootloader, menghapus atau memformat data, maupun menulis partisi. Perintah mulai ulang yang sudah tersedia selalu meminta konfirmasi.

Perangkat Android lain dapat muncul dalam pemindaian, tetapi belum divalidasi sebagai model yang didukung. Nilai yang kosong atau gagal dibaca tetap dianggap belum diketahui. Nilai `getvar anti` yang kosong bukan bukti perangkat bebas anti-rollback.

Pemeriksaan folder ROM menghitung ukuran dan SHA-256 berkas yang dikenal. Hash lokal hanya sidik jari berkas; hash itu tidak membuktikan sumber resmi atau kecocokan firmware. Katalog SHA-256 resmi yang terverifikasi belum tersedia.

Laporan JSON disimpan di komputer. DroidNow menyamarkan sejumlah pengenal, tetapi teks bebas masih dapat memuat informasi sensitif. Tinjau laporan sebelum membagikannya.

## ADB, Fastboot, dan Platform-Tools

Jika ADB atau Fastboot belum tersedia, gunakan **Siapkan ADB/Fastboot** untuk mengunduh Platform-Tools resmi dari Google, memasangnya di folder pengguna, lalu memindai perangkat. Gunakan **Pilih tools** untuk memilih folder Platform-Tools yang sudah ada.

Untuk ADB, aktifkan USB debugging, sambungkan perangkat dengan kabel data, buka kuncinya, lalu setujui permintaan RSA jika muncul. Status `unauthorized` berarti permintaan akses belum disetujui. Status `offline` biasanya perlu diperiksa lewat kabel atau port USB.

Untuk Fastboot, ikuti petunjuk model perangkat untuk masuk ke mode Fastboot, sambungkan USB, lalu pindai kembali. ADB dan Fastboot adalah mode koneksi yang berbeda. Driver Windows mungkin diperlukan untuk antarmuka ADB dan Android Bootloader/Fastboot; Linux mungkin memerlukan aturan udev.

Panduan driver Xiaomi hanya tersedia pada Windows. DroidNow membuka panduan tersebut, tetapi tidak mengunduh atau menjalankan installer driver.

## Menjalankan dari source

Pasang Rust stable dan toolchain C/C++ yang diperlukan sistem operasi, lalu jalankan dari folder proyek:

```sh
cargo run
```

Untuk memeriksa source:

```sh
cargo fmt --check
cargo test --locked
cargo check --locked
cargo build --locked
cargo clippy --locked -- -D warnings
cargo audit
```

## Hasil uji dan batas yang belum diuji

- Linux x86_64: `cargo fmt --check`, 19 tes, `cargo check`, `cargo build`, dan Clippy untuk semua target dengan peringatan sebagai error lulus. Tes mencakup timeout proses serta penghentian proses turunan yang mewarisi pipe.
- `cargo audit` tidak menemukan kerentanan yang diketahui, tetapi memberi 4 peringatan dependency tidak terawat tanpa pembaruan perbaikan yang tersedia: `ttf-parser`, `zbus_names`, `zvariant`, dan `zvariant_derive`.
- Belum diuji dengan perangkat Android fisik atau driver/USB Windows 11. macOS juga belum dibangun atau diuji.
- Build Windows tidak dibuat.
- Kecocokan dan keamanan firmware Redmi 4A belum dapat diverifikasi. Hasil pemeriksaan bukan jaminan firmware aman atau cocok.

## Berkas utama

```text
Cargo.toml       Metadata paket dan dependensi Rust
src/main.rs      Entry point aplikasi native
src/app.rs       Antarmuka, aktivitas, dan alur pengguna
src/core.rs      ADB/Fastboot, pemeriksaan, hashing, dan parsing
src/process.rs   Runner proses dengan timeout dan penghentian grup proses
src/safety.rs    Cakupan dukungan, batas operasi, dan laporan diagnostik
DESIGN.md        Arah visual dan alasan keputusan desain
```
