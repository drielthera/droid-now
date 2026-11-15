# Deteksi mode USB

DroidNow dapat menampilkan petunjuk mode dari pasangan Vendor ID dan Product ID (VID:PID) yang terlihat pada enumerasi USB sistem. Hasil ini adalah label observasi, bukan hasil komunikasi atau verifikasi protokol perangkat.

Pemetaan yang dikenali saat ini:

| VID:PID | Label yang ditampilkan | Batas interpretasi |
|---|---|---|
| `05c6:9008` | Qualcomm EDL / QDL | ID umum yang dikaitkan dengan QDL/EDL; tidak memverifikasi protokol atau model perangkat. |
| `0e8d:2000` | MediaTek Preloader | ID umum yang dikaitkan dengan Preloader MT65xx; bukan validasi model atau firmware. |
| `0e8d:0003` | MediaTek BROM (kandidat) | Pemetaan BootROM ini ambigu; basis data `usb.ids` Linux juga mencatat pasangan yang sama untuk perangkat telepon legacy MT6227. Karena itu labelnya hanya kandidat. |

ID yang tidak dikenal tidak ditebak. Pemindaian menggunakan enumerasi ID perangkat dari host; modul tidak membuka perangkat, mengambil alih interface, membaca string perangkat, maupun mengirim control/bulk transfer atau perintah USB.

VID:PID tidak membuktikan identitas atau model perangkat, protokol aktif, otorisasi pengguna, kecocokan firmware, ataupun dukungan flashing/pemulihan. DroidNow tidak menyediakan flashing. Contoh ID di tes merupakan fixture perangkat tiruan untuk memeriksa pemetaan, bukan hasil deteksi perangkat fisik.
