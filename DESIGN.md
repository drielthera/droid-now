# Arah desain DroidNow

Pembacaan desain: aplikasi desktop diagnostik Android untuk pengguna dan teknisi, dengan visual utilitarian grafit dan aksen teal yang tenang, dial ENERGY 2 / RHYTHM 2 / MOTION 1.

## Identitas

DroidNow membantu memeriksa koneksi ADB/Fastboot, membaca informasi dasar perangkat, dan meninjau berkas lokal. Deteksi koneksi bersifat umum untuk perangkat Android yang terlihat melalui ADB/Fastboot. Validasi identitas model saat ini hanya mencakup Xiaomi Redmi 4A (`rolex`); antarmuka harus menyebut batas ini sebagai cakupan pemeriksaan, bukan menjadikan merek sebagai identitas seluruh aplikasi.

Nada antarmuka profesional, langsung, dan mudah dipahami. Jelaskan tindakan serta batasnya dengan kata-kata sehari-hari. Hindari jargon milestone, promosi, klaim tanpa bukti, dan istilah yang menyiratkan dukungan model lebih luas daripada yang telah divalidasi.

## Bahasa visual

- Dasar grafit gelap mengikuti referensi UI yang diberikan dan menjaga antarmuka utilitas desktop tetap fokus.
- Teal menandai tindakan utama dan status siap. Amber menyampaikan peringatan yang perlu dibaca; merah menandai kegagalan. Warna status selalu disertai label teks.
- Tipografi sans-serif bawaan menjaga keterbacaan tanpa dependensi font tambahan. Monospace hanya untuk nilai teknis seperti serial, hash, dan versi tool.
- Panel utama mengutamakan pemindaian dan hasil perangkat. Kolom samping memuat kesiapan tools, batas keamanan, dan aktivitas. Urutan ini mengikuti tugas yang dilakukan pengguna, bukan template dashboard.
- Ikon garis yang digambar aplikasi dipakai hanya untuk konsep yang relevan seperti perangkat, folder, pemindaian, informasi, dan keamanan.
- Gerak dibatasi pada umpan balik interaksi seperti hover atau fokus. Tidak ada dekorasi bergerak.

## Keputusan desain

- Aksen teal menarik perhatian ke pemindaian dan status koneksi yang siap, sehingga tindakan rutin mudah ditemukan.
- Latar grafit dipertahankan karena sesuai dengan utilitas diagnostik dan referensi proyek, bukan sekadar karena tampil teknis.
- Kolom samping merangkum prasyarat, keamanan, dan aktivitas tanpa menambah navigasi atau statistik yang tidak membantu pemeriksaan.
- Tepi abu kebiruan pada kontrol idle, hover, dan terbuka memberi kontras minimal 3:1 terhadap permukaan dan fill tombol, tanpa memperluas penggunaan aksen teal.
- Jarak rapat mengelompokkan label dan nilai di dalam kartu; sela yang lebih lebar memisahkan tugas agar layar cepat dipindai.
- Panduan memakai tiga langkah karena alur nyata terdiri dari menyiapkan tools, menyambungkan perangkat, lalu meninjau hasil.
- Ikon mulai ulang dipakai untuk ketiga tujuan boot karena semuanya mengirim perintah reboot; label tombol menyebut tujuan masing-masing.
- Label model yang tervalidasi dipisahkan dari identitas umum Android agar cakupan produk tetap jujur.
- Pemilih perangkat diberi label terlihat; saat popup tertutup, panah atas/bawah berpindah perangkat dan hanya memuat ulang informasi baca-saja.
- Nilai yang belum dibaca, proses berjalan, dan kegagalan dibedakan dengan teks agar pengguna tidak perlu menafsirkan simbol atau warna saja.
- Banner cakupan membungkus teks ketika kolom menyempit agar batas keselamatan tidak hilang dari tampilan.

## Dial

- ENERGY 2: tampilan tenang dengan satu tindakan utama yang jelas.
- RHYTHM 2: struktur shell konsisten dengan variasi ukuran dan kepadatan sesuai peran tiap bagian.
- MOTION 1: hanya umpan balik hover dan fokus, tanpa animasi dekoratif.
