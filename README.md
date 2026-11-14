# DroidNow

Aplikasi desktop Rust berbasis egui/eframe untuk diagnostik perangkat Android. Pemindaian ADB/Fastboot dapat menemukan perangkat Android, sementara pemeriksaan model yang divalidasi terbatas pada Xiaomi Redmi 4A (`rolex`). Source dapat dibangun secara native di Windows, Linux, atau macOS.

## Status saat ini

DroidNow menyediakan **diagnostik baca-saja**. Aplikasi tidak menyediakan pemasangan firmware, membuka atau mengunci bootloader, penghapusan data, format, atau penulisan partisi.

Yang tersedia:

- Pemindaian perangkat ADB/Fastboot dan pembacaan identitas/status yang bisa diperoleh tanpa mengubah perangkat.
- Nilai yang kosong, gagal dibaca, tidak pasti, atau bertentangan dicatat sebagai `unknown`; nilai `getvar anti` yang kosong bukan bukti perangkat bebas anti-rollback.
- Matriks dukungan saat ini hanya mengklaim diagnostik terbatas untuk Redmi 4A (`rolex`). Merek/model lain bisa muncul dalam deteksi generik, tetapi tidak dianggap didukung.
- Pemeriksaan file ROM menghitung ukuran dan SHA-256 lokal. Hash lokal tidak membuktikan sumber resmi atau kecocokan firmware; katalog firmware terverifikasi saat ini masih kosong.
- Ekspor laporan JSON berisi hasil pemeriksaan dan catatan aktivitas. Beberapa data disamarkan, tetapi teks bebas masih mungkin memuat informasi sensitif; periksa laporan sebelum membagikannya.
- Pemeriksaan keamanan hanya mengizinkan diagnosis baca-saja dan menolak semua operasi yang mengubah perangkat.

## Platform-Tools dan driver

Jika ADB/Fastboot belum tersedia, tombol utama **Siapkan ADB/Fastboot** mengunduh tools resmi Google sesuai OS, memasangnya di folder pengguna, lalu memindai perangkat secara otomatis. Alternatifnya, pilih folder Platform-Tools yang berisi `adb` dan `fastboot` (atau file `.exe` di Windows).

Tombol **Pasang driver USB Windows** hanya muncul di Windows dan membuka panduan Xiaomi. DroidNow tidak mengunduh atau menjalankan installer driver. Jika memilih memasang driver, unduh dan jalankan MiFlash secara manual dari tautan pada panduan Xiaomi, lalu tinjau serta setujui UAC sendiri.

## Menjalankan dari source

Pasang Rust stable dan toolchain C/C++ yang diperlukan OS, buka folder proyek di Zed atau terminal, lalu jalankan:

```sh
cargo run
```

Untuk validasi source:

```sh
cargo fmt --check
cargo test --locked
cargo check --locked
cargo build --locked
cargo clippy --locked -- -D warnings
cargo audit
```

## Koneksi perangkat Xiaomi

### Android menyala (ADB)

1. Aktifkan Developer options dan USB debugging.
2. Hubungkan kabel data langsung ke komputer, buka kunci perangkat, dan setujui dialog otorisasi RSA.
3. DroidNow memindai saat dibuka dan setelah menyiapkan tools. Tekan **Pindai perangkat** untuk memindai lagi. Status `unauthorized` berarti dialog belum disetujui; `offline` biasanya perlu memeriksa kabel/port atau menyambungkan ulang.

### Fastboot

Matikan perangkat, tahan **Volume− + Power** untuk masuk Fastboot, sambungkan ke komputer, lalu pindai kembali. ADB dan Fastboot merupakan mode koneksi yang berbeda. Windows mungkin memerlukan driver terpisah untuk antarmuka ADB dan Android Bootloader/Fastboot; Linux dapat memerlukan aturan udev.

## Hasil uji dan batas yang belum diuji

- Linux x86_64: format, 13 tes, `cargo check`, `cargo build`, Clippy ketat, dan `cargo audit` lulus; audit tidak menemukan kerentanan yang diketahui.
- Audit tetap memberi 4 peringatan dependency tidak terawat tanpa pembaruan perbaikan tersedia: `ttf-parser`, `zbus_names`, `zvariant`, dan `zvariant_derive`.
- Belum diuji dengan perangkat fisik atau driver/USB Windows 11; macOS juga belum dibangun atau diuji.
- Source ZIP tidak menyertakan executable Windows. Build Windows hanya dibuat jika diminta.
- Kecocokan dan keamanan firmware Redmi 4A belum dapat diverifikasi; hasil pemeriksaan bukan jaminan firmware aman atau cocok.

## Berkas utama

```text
Cargo.toml       Metadata dan dependensi Rust
src/main.rs      Entry point aplikasi native
src/app.rs       UI, aktivitas, dan alur pengguna
src/core.rs      ADB/Fastboot, probes, hashing, dan parsing
src/safety.rs    Aturan dukungan, pemeriksaan keamanan, dan laporan diagnostik
```
