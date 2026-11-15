# Arah desain DroidNow

Produk desktop untuk diagnosis perangkat, dengan workspace status-kontrol ringkas. ENERGY 2 / RHYTHM 2 / MOTION 1. Dasar charcoal, teks cool-white, teal sebagai aksen DroidNow, dan muted violet hanya sebagai penanda pilihan sekunder.

## Identitas
DroidNow membantu memeriksa koneksi ADB/Fastboot, membaca informasi dasar perangkat, dan meninjau berkas lokal. Nama dan tagline DroidNow menjadi identitas aplikasi. Nada antarmuka profesional, jelas, dan langsung.

## Komposisi
Layar Perangkat mengutamakan status koneksi dan satu tindakan utama. Saat data tersedia, Model, versi Android, dan koneksi menjadi metrik ringkas. Serial, kode perangkat, bootloader, serta hasil probe berada di rincian. Aktivitas, Panduan, dan Tentang tetap menjadi tujuan sekunder yang nyata. USB baca-saja, reboot, dan pemeriksaan ROM lokal berada di Pemeriksaan lanjutan.

## Bahasa visual
Charcoal `#0A0B0D` menjadi dasar, permukaan menggunakan abu gelap, teks utama `#F0F0F1`, teks sekunder `#BCBDC2`, dan garis `#53585E`. Teal `#49C9B1` mempertahankan identitas DroidNow dan menandai aksi utama. Muted violet `#836984` hanya memberi nuansa pada navigasi terpilih; tidak dipakai sebagai status atau tombol utama. Hijau, amber, dan merah hanya membawa makna status. Permukaan datar, radius kecil, tanpa glow atau kartu dekoratif. Sans bawaan menjaga keterbacaan; monospace hanya untuk nilai teknis. Gerak terbatas pada umpan balik kontrol.

## Alasan keputusan
[Acti – Wireless iPhone Microphone Dashboard](https://dribbble.com/shots/27883216-Acti-Wireless-iPhone-Microphone-Dashboard-UI) menjadi referensi untuk status perangkat yang ringkas, metrik relevan, dan satu area kontrol yang terorganisasi. DroidNow mengadaptasi bahasa visualnya ke jendela desktop, bukan ke tata letak ponsel, kontrol mikrofon, daftar sesi, atau data contoh. Struktur dan identitas tetap khusus DroidNow.

Status harus berasal dari hasil nyata. Jangan menampilkan angka, perangkat, fitur, atau klaim dukungan rekaan. Mode USB tetap baca-saja. Pemeriksaan ROM tetap lokal dan tidak menjalankan atau mengirim berkas, serta tidak memberi izin menulis firmware. Setiap tujuan reboot tetap meminta konfirmasi. Jangan menambahkan flash, penulisan UBL/EDL, erase, format, relock, atau penulisan partisi.
