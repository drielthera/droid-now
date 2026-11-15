# Arah desain DroidNow

Design Read: utilitas desktop diagnosis perangkat Android untuk pengguna teknis, dalam bahasa visual dashboard layanan yang terang dan ringkas; ENERGY 1 / RHYTHM 2 / MOTION 1.

## Identitas
DroidNow memakai tagline "Your devices. One solution". UI tetap berpusat pada data perangkat, koneksi ADB/Fastboot, pemeriksaan USB baca-saja, reboot, dan preflight ROM lokal. Font sans bawaan dipakai untuk keterbacaan; monospace hanya untuk nilai teknis.

## Komposisi
Arah komposisi mengikuti referensi Acti yang dipilih pengguna: sidebar kiri yang sempit, breadcrumb dan aksi tools di bar atas, kartu perangkat lebar di area utama, panel pemeriksaan/tindakan di kanan, lalu aktivitas terbaru. Sidebar hanya memuat tujuan nyata: Perangkat, Aktivitas, Panduan, Tentang. Status aplikasi tetap di kaki sidebar; bahasa ID/EN berada di bar atas di samping aksi tools agar selalu terlihat.

Kartu utama mengutamakan satu keputusan: siapkan ADB/Fastboot atau pindai perangkat. Ikon ponsel garis sederhana dipakai karena langsung mewakili perangkat yang diperiksa, bukan sebagai ilustrasi dekoratif. Model, Android, dan status koneksi hanya berasal dari hasil nyata. Jika belum ada perangkat, kartu menampilkan empty state dan tidak mengisi metrik contoh. Detail teknis berada di disclosure. Panel kanan memuat ketersediaan tools, deteksi USB baca-saja, reboot dengan konfirmasi, dan pemeriksaan ROM lokal yang dilipat. Aktivitas terbaru hanya berasal dari log sesi nyata; tidak ada tanggal, durasi, perangkat, atau angka contoh.

## Bahasa visual dan alasan
Tema off-white `#F4F5F3` dengan permukaan putih dipilih karena referensi desktop pengguna terang; satu tema dipakai untuk menjaga kecocokan visual yang diminta, bukan karena anggapan bahwa alat teknis harus gelap. Teks charcoal `#202422` dan muted `#5B635F` memberi kontras terukur di atas 4.5:1. Teal `#16796A` menandai identitas DroidNow dan status koneksi nyata saja. Tombol aksi utama charcoal mengikuti hierarki CTA pada referensi; amber dan merah hanya untuk peringatan dan kesalahan.

Sidebar aktif memakai latar teal pucat serta penanda kecil agar lokasi navigasi jelas tanpa mengecat semua kontrol. Kartu berbeda ukuran sesuai perannya: kartu perangkat paling lebar, pemeriksaan lebih sempit, aktivitas membentang di bawah. Garis tipis dan radius sedang memisahkan permukaan tanpa bayangan besar atau glow. Jarak lebar mengelompokkan area utama; label dan nilai tetap rapat. Ikon digambar khusus untuk perangkat, aktivitas, dan pemeriksaan agar maknanya langsung relevan. Gerak hanya untuk indikator proses dan umpan balik kontrol.

Referensi komposisi: [Acti - Wireless iPhone Microphone Dashboard](https://dribbble.com/shots/27883216-Acti-Wireless-iPhone-Microphone-Dashboard-UI). DroidNow mengadaptasi tata letak desktop dan hierarki visualnya, bukan konten mikrofon, kontrol, atau data sesi.

## Batas keselamatan
Status harus berasal dari hasil nyata. Jangan menampilkan angka, perangkat, fitur, atau klaim dukungan rekaan. Mode USB tetap baca-saja. Pemeriksaan ROM tetap lokal, tidak menjalankan atau mengirim berkas, dan tidak memberi izin menulis firmware. Setiap tujuan reboot meminta konfirmasi. Jangan menambahkan flash, penulisan UBL/EDL, erase, format, relock, atau penulisan partisi.
