# Petunjuk DroidNow untuk Windows 11

## Status executable

Belum ada build Windows baru untuk source DroidNow yang memuat fondasi P0 dan tombol pemasangan Platform-Tools/driver. Executable portable yang pernah dibagikan adalah versi sebelumnya dan belum mencerminkan perubahan ini. Sesuai arahan pengguna, Windows tidak dibuild pada milestone ini.

Untuk mencoba source terbaru, pasang Rust stable (target MSVC) dan Microsoft C++ Build Tools dengan workload **Desktop development with C++**, buka folder proyek di Zed atau PowerShell, lalu jalankan:

```powershell
cargo run
```

## Platform-Tools

Source terbaru memiliki tombol **Pasang Platform-Tools**. Tombol tersebut mengunduh arsip ADB/Fastboot resmi Google untuk OS yang dipakai, memasangnya di folder data pengguna, dan menggunakan kembali folder tersebut saat aplikasi dibuka lagi.

Unduh manual hanya dari [Android Developers](https://developer.android.com/tools/releases/platform-tools). Jika memilih folder sendiri, pilih folder yang langsung berisi `adb.exe` dan `fastboot.exe`.

## Driver USB Xiaomi

Tombol **Pasang driver USB Windows** membuka [panduan Xiaomi Community](https://new.c.mi.com/mx/post/11443). DroidNow tidak menjalankan installer driver. Jika memilih opsi MiFlash pada panduan itu, unduh dan jalankan MiFlash secara manual dari tautan yang disediakan Xiaomi Community; periksa sumber/penerbitnya dan setujui prompt Administrator/UAC sendiri. Menu **Driver** pada MiFlash dapat dipakai untuk memasang ulang driver.

Jangan memasang driver dari sumber tidak tepercaya atau mematikan proteksi Windows untuk driver tak dikenal. Driver ADB dan Android Bootloader/Fastboot dapat muncul sebagai antarmuka berbeda di Device Manager.

## Menyambungkan Redmi 4A

### ADB saat Android menyala

1. Aktifkan Developer options dan USB debugging.
2. Sambungkan ponsel langsung dengan kabel data, buka kunci layar, lalu setujui dialog otorisasi RSA.
3. Di DroidNow, pilih **Pindai perangkat**. `unauthorized` berarti dialog belum disetujui; `offline` biasanya perlu memeriksa kabel/port atau menyambungkan ulang.

### Fastboot

Matikan ponsel, tahan **Volume− + Power**, sambungkan ke komputer, lalu pindai kembali. Jika tidak terdeteksi, periksa Device Manager dan driver Android Bootloader/Fastboot yang sesuai.

## Batas keselamatan

Milestone P0 mengizinkan diagnostik baca-saja dan reboot yang telah dikonfirmasi. Flash ROM, unlock/relock bootloader, erase, format, dan penulisan partisi belum tersedia. Pemeriksaan nama file dan hash lokal tidak membuktikan ROM berasal dari sumber resmi atau cocok untuk perangkat.
