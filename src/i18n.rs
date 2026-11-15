#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Locale {
    #[default]
    Indonesian,
    English,
}

impl Locale {
    pub const fn tag(self) -> &'static str {
        match self {
            Self::Indonesian => "ID",
            Self::English => "EN",
        }
    }
}

const TRANSLATIONS: &[(&str, &str)] = &[
    ("DroidNow siap. Sambungkan ponsel melalui kabel USB.", "DroidNow is ready. Connect a phone using a USB cable."),
    ("Memindai perangkat ADB dan Fastboot…", "Scanning for ADB and Fastboot devices…"),
    ("Memindai perangkat", "Scanning for devices"),
    ("Menunggu hasil ADB/Fastboot.", "Waiting for ADB/Fastboot results."),
    ("Mengunduh Platform-Tools resmi Google melalui HTTPS…", "Downloading the official Google Platform-Tools over HTTPS…"),
    ("Membaca identitas perangkat…", "Reading device identity…"),
    ("Mengirim perintah mulai ulang: {}…", "Sending reboot command: {}…"),
    ("Mengirim perintah mulai ulang:", "Sending reboot command:"),
    ("Membaca folder ROM tanpa menjalankan berkas…", "Inspecting the ROM folder without running any files…"),
    ("Tidak ditemukan perangkat melalui ADB atau Fastboot.", "No devices found through ADB or Fastboot."),
    ("Informasi {serial} berhasil dibaca.", "Information for {serial} was read successfully."),
    ("Informasi ", "Information for "),
    (" berhasil dibaca.", " was read successfully."),
    ("Info perangkat tidak terbaca:", "Could not read device information:"),
    ("Tunggu perangkat masuk ke mode tujuan, lalu pindai ulang.", "Wait for the device to enter the requested mode, then scan again."),
    ("Folder ROM selesai diperiksa:", "ROM folder inspection complete:"),
    ("Ditemukan", "Found"),
    ("berkas terkait.", "related files."),
    ("Pemeriksaan folder gagal:", "Folder inspection failed:"),
    ("Mulai ulang gagal:", "Reboot failed:"),
    ("Platform-Tools berhasil dipasang di", "Platform-Tools installed successfully at"),
    ("Platform-Tools terpasang, tetapi lokasinya gagal dipilih:", "Platform-Tools were installed, but their location could not be selected:"),
    ("Pemasangan Platform-Tools gagal:", "Platform-Tools installation failed:"),
    ("Folder Platform-Tools diatur:", "Platform-Tools folder set to:"),
    ("Folder tools gagal diatur:", "Could not set tools folder:"),
    ("Pilih folder Android SDK Platform-Tools", "Choose the Android SDK Platform-Tools folder"),
    ("Pilih folder ROM hasil ekstrak", "Choose the extracted ROM folder"),
    ("Pilih folder ROM sebelum memulai pemeriksaan.", "Choose a ROM folder before starting the inspection."),
    ("Simpan laporan diagnostik DroidNow", "Save DroidNow diagnostic report"),
    ("Laporan diagnostik disimpan di", "Diagnostic report saved to"),
    ("Beberapa data sensitif disamarkan; periksa kembali sebelum membagikan.", "Some sensitive data is redacted; review it before sharing."),
    ("Pilih tools", "Choose tools"),
    ("Ringkasan", "Overview"),
    ("Aktivitas", "Activity"),
    ("Tentang", "About"),
    ("Panduan", "Guide"),
    ("ADB/Fastboot tidak ditemukan", "ADB/Fastboot not found"),
    ("Siapkan tools atau pilih folder yang sudah ada.", "Set up the tools or choose an existing folder."),
    ("Tidak ada perangkat", "No device detected"),
    ("Sambungkan USB, lalu pindai.", "Connect USB, then scan."),
    ("Izin USB debugging diperlukan", "USB debugging permission required"),
    ("Setujui permintaan akses di perangkat.", "Approve the access request on the device."),
    ("Perangkat offline", "Device is offline"),
    ("Periksa kabel atau port, lalu pindai.", "Check the cable or port, then scan."),
    ("Fastboot terhubung", "Fastboot connected"),
    ("Perangkat terhubung", "Device connected"),
    ("Informasi perangkat", "Device information"),
    ("Perangkat", "Device"),
    ("Pilih perangkat", "Select a device"),
    ("Belum dipilih", "Not selected"),
    ("Membaca informasi dasar perangkat…", "Reading basic device information…"),
    ("Informasi dasar tidak dapat dibaca:", "Could not read basic information:"),
    ("Periksa koneksi dan ketersediaan tools.", "Check the connection and tool availability."),
    ("Menunggu hasil pembacaan informasi dasar…", "Waiting for basic information…"),
    ("Memindai perangkat Android yang terhubung…", "Scanning for connected Android devices…"),
    ("Sambungkan perangkat melalui USB untuk membaca informasi yang tersedia.", "Connect a device over USB to read the available information."),
    ("Model", "Model"),
    ("Kode perangkat", "Device code"),
    ("Versi Android", "Android version"),
    ("Status bootloader", "Bootloader status"),
    ("Nomor seri", "Serial number"),
    ("Koneksi", "Connection"),
    ("Detail teknis perangkat", "Technical device details"),
    ("Nilai kosong atau gagal dibaca tidak membuktikan perangkat aman.", "An empty or unreadable value does not prove that a device is safe."),
    ("Buka layar ponsel dan setujui permintaan USB debugging.", "Unlock the phone and approve the USB debugging request."),
    ("Coba kabel atau port USB lain, lalu sambungkan kembali.", "Try another USB cable or port, then reconnect."),
    ("Mode USB teramati", "Observed USB mode"),
    ("Baca-saja", "Read-only"),
    ("Petunjuk dari VID:PID teramati.", "Hint from observed VID:PID."),
    ("Memeriksa daftar perangkat USB…", "Checking the USB device list…"),
    ("Daftar perangkat USB tidak dapat dibaca saat ini.", "The USB device list cannot be read right now."),
    ("Tidak ada ID USB yang dikenali untuk petunjuk mode.", "No recognized USB IDs are available for a mode hint."),
    ("Mulai ulang perangkat", "Reboot device"),
    ("Mulai ulang ke Android", "Reboot to Android"),
    ("Buka Recovery", "Open Recovery"),
    ("Buka Fastboot", "Open Fastboot"),
    ("Periksa berkas ROM lokal", "Inspect local ROM files"),
    ("Menghitung SHA-256 lokal. Berkas tidak dijalankan atau dikirim ke perangkat.", "Calculates local SHA-256. Files are not run or sent to the device."),
    ("Folder yang akan diperiksa", "Folder to inspect"),
    ("Pilih folder", "Choose folder"),
    ("Periksa berkas", "Inspect files"),
    ("Memeriksa isi folder dan menghitung SHA-256 berkas lokal…", "Inspecting folder contents and calculating local file SHA-256…"),
    ("Pemeriksaan gagal:", "Inspection failed:"),
    ("Pilih folder yang benar, lalu jalankan pemeriksaan lagi.", "Choose the correct folder, then run the inspection again."),
    ("Folder ini tidak berisi berkas gambar atau metadata yang dikenali. Pilih folder ROM hasil ekstrak.", "This folder contains no recognized image or metadata files. Choose an extracted ROM folder."),
    ("berkas terkait ·", "related files ·"),
    ("Sumber dan kecocokan firmware belum dapat dipastikan.", "The source and firmware compatibility cannot be verified."),
    ("SHA-256 tidak tersedia", "SHA-256 is unavailable"),
    ("B · SHA-256", "B · SHA-256"),
    ("Pilih folder untuk melihat nama, ukuran, dan SHA-256 berkas yang ditemukan.", "Choose a folder to see the names, sizes, and SHA-256 hashes of files found."),
    ("Konfirmasi mulai ulang", "Confirm reboot"),
    ("Konfirmasi", "Confirmation"),
    ("Konfirmasi diperlukan.", "Confirmation required."),
    ("Batal", "Cancel"),
    ("Mulai ulang sekarang", "Reboot now"),
    ("Memindai…", "Scanning…"),
    ("Menyiapkan ADB/Fastboot…", "Preparing ADB/Fastboot…"),
    ("Memulai ulang…", "Rebooting…"),
    ("Memeriksa berkas…", "Inspecting files…"),
    ("Memeriksa USB…", "Checking USB…"),
    ("Membaca informasi perangkat…", "Reading device information…"),
    ("Siap", "Ready"),
    ("Status aplikasi:", "App status:"),
    ("Perangkat dan koneksi", "Devices and connections"),
    ("Menyiapkan…", "Preparing…"),
    ("Pindai perangkat", "Scan devices"),
    ("Siapkan ADB/Fastboot", "Set up ADB/Fastboot"),
    ("Riwayat aktivitas", "Activity history"),
    ("Belum ada aktivitas.", "No activity yet."),
    ("Catatan sesi ini tersimpan di komputer dan tidak dikirim secara otomatis.", "This session's log stays on this computer and is not sent automatically."),
    ("catatan aktivitas", "activity entries"),
    ("Ekspor laporan…", "Export report…"),
    ("Tentang DroidNow", "About DroidNow"),
    ("Diagnostik Android melalui ADB dan Fastboot.", "Android diagnostics via ADB and Fastboot."),
    ("Membaca informasi perangkat dan memeriksa berkas lokal.", "Reads device information and checks local files."),
    ("Versi aplikasi", "App version"),
    ("Panduan singkat", "Quick guide"),
    ("Lokasi tools", "Tools location"),
    ("Siapkan tools", "Set up tools"),
    ("Pilih folder Platform-Tools, atau siapkan tools resmi dari Google.", "Choose a Platform-Tools folder, or set up the official Google tools."),
    ("Hubungkan perangkat", "Connect a device"),
    ("ADB: aktifkan USB debugging dan setujui akses. Fastboot: masuk sesuai petunjuk perangkat.", "ADB: enable USB debugging and approve access. Fastboot: enter the mode using your device's instructions."),
    ("Pindai koneksi. DroidNow hanya membaca perangkat.", "Scan the connection. DroidNow only reads device information."),
    ("Batas operasi", "Operation limits"),
    ("Reboot perlu konfirmasi. Pemasangan firmware, penulisan partisi, perubahan bootloader, dan penghapusan data tidak tersedia.", "Reboot requires confirmation. Firmware installation, partition writes, bootloader changes, and data erasure are unavailable."),
    ("Tools dari folder pilihan tidak diverifikasi. Hash lokal tidak membuktikan sumber atau kecocokan firmware.", "Tools from a selected folder aren't verified. A local hash doesn't prove firmware source or compatibility."),
    ("Deteksi USB hanya membaca VID:PID teramati. Perintah USB tidak dikirim; protokol tidak diverifikasi.", "USB detection only reads observed VID:PID. No USB commands are sent; the protocol is not verified."),
    ("Tinjau hasil", "Review results"),
    ("Bahasa Indonesia", "Bahasa Indonesia"),
    ("English", "English"),
    ("Petunjuk driver USB", "USB driver guidance"),
    ("Pemasangan driver memerlukan tindakan Anda di Windows.", "Driver installation requires your action in Windows."),
    ("DroidNow tidak mengunduh atau menjalankan installer driver secara otomatis. Driver yang diperlukan bergantung pada perangkat dan versi Windows.", "DroidNow does not download or run driver installers automatically. The required driver depends on the device and Windows version."),
    ("Jika perangkat tidak terdeteksi, periksa kabel dan port USB. Pasang driver ADB/Fastboot hanya dari sumber tepercaya, lalu tinjau penerbit serta permintaan izin Windows sebelum melanjutkan.", "If the device is not detected, check the cable and USB port. Install ADB/Fastboot drivers only from a trusted source, then review the publisher and Windows permission requests before continuing."),
    ("Tutup", "Close"),
    ("membuka Recovery", "opening Recovery"),
    ("membuka Fastboot", "opening Fastboot"),
    ("memulai ulang ke Android", "rebooting to Android"),
    ("Masuk ke Recovery?", "Reboot to Recovery?"),
    ("Masuk ke Fastboot?", "Reboot to Fastboot?"),
    ("Mulai ulang perangkat?", "Reboot device?"),
    ("Perangkat akan dimulai ulang ke Recovery. Tindakan ini tidak memasang atau menghapus apa pun.", "The device will reboot to Recovery. This does not install or erase anything."),
    ("Perangkat akan dimulai ulang ke Fastboot. Tindakan ini tidak membuka kunci bootloader.", "The device will reboot to Fastboot. This does not unlock the bootloader."),
    ("Perangkat akan dimulai ulang ke Android. Simpan pekerjaan yang sedang berlangsung.", "The device will reboot to Android. Save any work in progress."),
    ("Enumerasi USB gagal:", "USB enumeration failed:"),
    ("Tidak dapat menunggu", "Could not wait for"),
    ("tidak merespons (timeout", "did not respond (timeout"),
    ("Folder yang dipilih bukan direktori yang dapat dibaca.", "The selected folder is not a readable directory."),
    ("Mencari di PATH sistem", "Searching the system PATH"),
    ("tidak ditemukan. Pilih folder Android SDK Platform-Tools.", "not found. Choose the Android SDK Platform-Tools folder."),
    ("Pemasangan otomatis belum tersedia untuk sistem operasi ini.", "Automatic setup is not available for this operating system."),
    ("Folder data aplikasi tidak dapat dibuat:", "Could not create the application data folder:"),
    ("Klien unduhan tidak dapat dibuat:", "Could not create the download client:"),
    ("Unduhan resmi Platform-Tools gagal:", "Official Platform-Tools download failed:"),
    ("Ukuran arsip Platform-Tools melebihi batas keamanan 100 MiB.", "Platform-Tools archive exceeds the 100 MiB safety limit."),
    ("Berkas unduhan sementara tidak dapat dibuat:", "Could not create temporary download file:"),
    ("Unduhan Platform-Tools terputus:", "Platform-Tools download was interrupted:"),
    ("Berkas unduhan tidak dapat disimpan:", "Could not save the downloaded file:"),
    ("Folder pemasangan sementara tidak dapat dibuat:", "Could not create the temporary installation folder:"),
    ("Arsip unduhan tidak dapat dibuka:", "Could not open the downloaded archive:"),
    ("Instalasi Platform-Tools lama tidak dapat diganti:", "Could not replace the previous Platform-Tools installation:"),
    ("Platform-Tools gagal dipasang:", "Could not install Platform-Tools:"),
    ("Arsip resmi Platform-Tools tidak valid:", "The official Platform-Tools archive is invalid:"),
    ("Isi arsip Platform-Tools tidak dapat dibaca:", "Could not read Platform-Tools archive contents:"),
    ("Nama file arsip Platform-Tools tidak valid:", "Invalid file name in Platform-Tools archive:"),
    ("Arsip Platform-Tools berisi jalur file yang tidak aman.", "Platform-Tools archive contains an unsafe file path."),
    ("Arsip Platform-Tools berisi tautan simbolik yang ditolak.", "Platform-Tools archive contains a rejected symbolic link."),
    ("Ukuran ekstraksi arsip tidak valid.", "Invalid extracted archive size."),
    ("Ukuran hasil ekstraksi melebihi batas keamanan 512 MiB.", "Extracted files exceed the 512 MiB safety limit."),
    ("Folder Platform-Tools gagal diekstrak:", "Could not extract Platform-Tools folder:"),
    ("Folder Platform-Tools gagal dibuat:", "Could not create Platform-Tools folder:"),
    ("Berkas Platform-Tools gagal dibuat:", "Could not create Platform-Tools file:"),
    ("Berkas Platform-Tools gagal diekstrak:", "Could not extract Platform-Tools file:"),
    ("Berkas Platform-Tools gagal disimpan:", "Could not save Platform-Tools file:"),
    ("Izin berkas Platform-Tools gagal diatur:", "Could not set Platform-Tools file permissions:"),
    ("Arsip unduhan tidak memuat adb dan fastboot yang diperlukan.", "The downloaded archive does not contain the required adb and fastboot files."),
    ("Folder data pengguna Windows tidak ditemukan.", "Windows user data folder was not found."),
    ("Folder data pengguna macOS tidak ditemukan.", "macOS user data folder was not found."),
    ("Folder data pengguna Linux tidak ditemukan.", "Linux user data folder was not found."),
    ("Gagal menjalankan", "Failed to run"),
    ("Serial perangkat tidak valid.", "Invalid device serial."),
    ("tidak tersedia", "unavailable"),
    ("tidak terbaca (probe gagal)", "unreadable (probe failed)"),
    ("tidak terbaca", "unreadable"),
    ("Belum terbaca", "Not read yet"),
    ("Belum tersedia", "Not available"),
    ("tidak terdeteksi (bukan bukti aman)", "not detected (this is not proof of safety)"),
    ("ADB belum berstatus device. Jika unauthorized, setujui dialog RSA di ponsel.", "ADB is not in the device state. If it is unauthorized, approve the RSA prompt on the phone."),
    ("Mode perangkat tidak dikenal.", "Unknown device mode."),
    ("rollback index (anti): tidak terdeteksi (probe Fastboot tidak berjalan; bukan bukti aman)", "rollback index (anti): not detected (Fastboot probe did not run; this is not proof of safety)"),
    ("SHA-256 berkas: belum dihitung (belum memilih berkas).", "File SHA-256: not calculated (no file selected yet)."),
    ("Sumber resmi: belum dapat dipastikan (katalog SHA-256 tepercaya belum tersedia).", "Official source: unverified (a trusted SHA-256 catalog is not available)."),
    ("Android: tidak tersedia dalam mode Fastboot", "Android: unavailable in Fastboot mode"),
    ("build: tidak tersedia dalam mode Fastboot", "build: unavailable in Fastboot mode"),
    ("Verified Boot: tidak tersedia melalui probe ini", "Verified Boot: unavailable through this probe"),
    ("ADB version: tidak diuji dalam mode Fastboot", "ADB version: not tested in Fastboot mode"),
    ("ADB version:", "ADB version:"),
    ("Fastboot version:", "Fastboot version:"),
    ("Vendor region", "Vendor region"),
    ("MIUI region", "Vendor region"),
    ("Wilayah vendor", "Vendor region"),
    ("rollback index", "rollback index"),
    ("belum dihitung (belum memilih berkas).", "not calculated (no file selected yet)."),
    ("belum dapat dipastikan (katalog SHA-256 tepercaya belum tersedia).", "unverified (a trusted SHA-256 catalog is not available)."),
    ("tidak terdeteksi (probe Fastboot tidak berjalan; bukan bukti aman)", "not detected (Fastboot probe did not run; this is not proof of safety)"),
    ("tidak tersedia dalam mode Fastboot", "unavailable in Fastboot mode"),
    ("tidak tersedia melalui probe ini", "unavailable through this probe"),
    ("tidak diuji dalam mode Fastboot", "not tested in Fastboot mode"),
    ("(kandidat)", "(candidate)"),
    ("BROM yang ambigu", "ambiguous BROM"),
    ("database USB Linux juga mencatat perangkat legacy MT6227.", "the Linux USB database also lists legacy MT6227 devices."),
    ("Qualcomm EDL / QDL", "Qualcomm EDL / QDL"),
    ("selesai dengan kode", "finished with exit code"),
    ("Fastboot tidak menyediakan perintah umum untuk reboot ke Recovery.", "Fastboot does not provide a generic command to reboot to Recovery."),
    ("Perangkat sudah berada di mode Fastboot.", "The device is already in Fastboot mode."),
    ("ADB tidak berstatus device; reboot dibatalkan.", "ADB is not in the device state; reboot cancelled."),
    ("Aksi reboot tidak dikenal.", "Unknown reboot action."),
    ("Perintah reboot gagal", "Reboot command failed"),
    ("Perintah reboot berhasil dikirim.", "Reboot command sent successfully."),
    ("Folder ROM tidak dapat dibaca:", "Could not read ROM folder:"),
    ("Tidak dapat membaca metadata file:", "Could not read file metadata:"),
    ("SHA-256 tidak dapat dihitung secara lokal", "Could not calculate SHA-256 locally"),
    ("Metadata file tidak dapat dibaca:", "Could not read file metadata:"),
    ("File tidak dapat dibuka untuk hashing:", "Could not open file for hashing:"),
    ("Metadata file terbuka tidak dapat dibaca:", "Could not read open file metadata:"),
    ("File berubah sebelum proses hashing dimulai.", "File changed before hashing started."),
    ("File tidak dapat dibaca untuk hashing:", "Could not read file for hashing:"),
    ("Ukuran file saat hashing melebihi batas.", "File size exceeded the limit during hashing."),
    ("Metadata file setelah hashing gagal dibaca:", "Could not read file metadata after hashing:"),
    ("File hilang setelah hashing:", "File disappeared after hashing:"),
    ("File berubah selama proses hashing; hash ditolak.", "File changed during hashing; hash rejected."),
    ("Folder yang dipilih tidak tersedia atau bukan direktori.", "The selected folder is unavailable or is not a directory."),
    ("tidak terbaca", "unreadable"),
    ("skrip flash_all tidak terlihat", "flash_all script not detected"),
    ("Belum dapat dipastikan: katalog SHA-256 resmi belum tersedia.", "Unverified: an official SHA-256 catalog is not available."),
    ("SHA-256 lokal hanya sidik jari berkas. Sumber resmi, kecocokan perangkat/wilayah, dan keamanan firmware tidak dipastikan. Tidak ada berkas yang dijalankan atau dikirim ke ponsel.", "A local SHA-256 is only a file fingerprint. The official source, device/region compatibility, and firmware safety are not verified. No files are run or sent to the phone."),
    ("Qualcomm EDL / QDL", "Qualcomm EDL / QDL"),
    ("MediaTek BROM (kandidat)", "MediaTek BROM (candidate)"),
    ("MediaTek Preloader", "MediaTek Preloader"),
    ("ID umum Qualcomm QDL/EDL; bukan verifikasi protokol.", "Common Qualcomm QDL/EDL ID; protocol is not verified."),
    ("0e8d:0003 adalah petunjuk BROM yang ambigu; database USB Linux juga mencatat perangkat legacy MT6227.", "0e8d:0003 is an ambiguous BROM hint; the Linux USB database also lists legacy MT6227 devices."),
    ("ID umum MediaTek MT65xx Preloader; bukan verifikasi protokol.", "Common MediaTek MT65xx Preloader ID; protocol is not verified."),
];

pub fn tr(locale: Locale, source: &'static str) -> &'static str {
    match locale {
        Locale::Indonesian => source,
        Locale::English => TRANSLATIONS
            .iter()
            .find_map(|(indonesian, english)| (*indonesian == source).then_some(*english))
            .unwrap_or(source),
    }
}

/// Translate a displayed message, including stable phrases around dynamic values.
pub fn translate_text(locale: Locale, source: &str) -> String {
    if locale == Locale::Indonesian {
        return source.to_owned();
    }
    if let Some((_, english)) = TRANSLATIONS
        .iter()
        .find(|(indonesian, _)| *indonesian == source)
    {
        return (*english).to_owned();
    }

    let mut phrases = TRANSLATIONS.to_vec();
    phrases.sort_by_key(|(indonesian, _)| std::cmp::Reverse(indonesian.len()));
    phrases
        .into_iter()
        .fold(source.to_owned(), |text, (indonesian, english)| {
            text.replace(indonesian, english)
        })
}

#[cfg(test)]
mod tests {
    use super::{tr, translate_text, Locale};

    #[test]
    fn defaults_and_switches_languages() {
        assert_eq!(Locale::default(), Locale::Indonesian);
        assert_eq!(Locale::Indonesian.tag(), "ID");
        assert_eq!(Locale::English.tag(), "EN");
        assert_eq!(tr(Locale::English, "Ringkasan"), "Overview");
        assert_eq!(tr(Locale::Indonesian, "Ringkasan"), "Ringkasan");
    }

    #[test]
    fn translates_dynamic_messages_and_keeps_values() {
        let source = "Pemeriksaan folder gagal: Permission denied";
        assert_eq!(
            translate_text(Locale::English, source),
            "Folder inspection failed: Permission denied"
        );
    }

    #[test]
    fn compact_ui_copy_is_available_in_both_locales() {
        let copy = [
            "Menghitung SHA-256 lokal. Berkas tidak dijalankan atau dikirim ke perangkat.",
            "Memindai perangkat",
            "Petunjuk driver USB",
            "ADB/Fastboot tidak ditemukan",
            "Tidak ada perangkat",
            "Izin USB debugging diperlukan",
            "Perangkat offline",
            "Petunjuk dari VID:PID teramati.",
            "Konfirmasi diperlukan.",
            "Belum ada aktivitas.",
            "Panduan singkat",
            "Lokasi tools",
            "Batas operasi",
            "Reboot perlu konfirmasi. Pemasangan firmware, penulisan partisi, perubahan bootloader, dan penghapusan data tidak tersedia.",
            "Tools dari folder pilihan tidak diverifikasi. Hash lokal tidak membuktikan sumber atau kecocokan firmware.",
            "Deteksi USB hanya membaca VID:PID teramati. Perintah USB tidak dikirim; protokol tidak diverifikasi.",
        ];
        for source in copy {
            assert_eq!(tr(Locale::Indonesian, source), source);
            assert_ne!(tr(Locale::English, source), source, "{source}");
        }
    }
}
