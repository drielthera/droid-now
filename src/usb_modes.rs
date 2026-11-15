//! Read-only USB VID:PID hints for Qualcomm EDL/QDL and MediaTek BROM/Preloader.
//!
//! This module intentionally does not call `DeviceInfo::open`, claim interfaces,
//! read strings, send control/bulk transfers, or issue device commands. It only
//! consumes VID/PID values supplied by the host's existing USB enumeration.

use nusb::MaybeFuture;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UsbModeHint {
    QualcommEdlQdl,
    MediatekBromCandidate,
    MediatekPreloader,
}

impl UsbModeHint {
    pub fn label(self) -> &'static str {
        match self {
            Self::QualcommEdlQdl => "Qualcomm EDL / QDL",
            Self::MediatekBromCandidate => "MediaTek BROM (kandidat)",
            Self::MediatekPreloader => "MediaTek Preloader",
        }
    }

    #[cfg(test)]
    pub fn note(self) -> &'static str {
        match self {
            Self::QualcommEdlQdl => "ID umum Qualcomm QDL/EDL; bukan verifikasi protokol.",
            Self::MediatekBromCandidate => {
                "0e8d:0003 adalah petunjuk BROM yang ambigu; database USB Linux juga mencatat perangkat legacy MT6227."
            }
            Self::MediatekPreloader => "ID umum MediaTek MT65xx Preloader; bukan verifikasi protokol.",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DetectedUsbMode {
    pub vendor_id: u16,
    pub product_id: u16,
    pub mode: UsbModeHint,
}

impl DetectedUsbMode {
    pub fn usb_id(self) -> String {
        format!("{:04x}:{:04x}", self.vendor_id, self.product_id)
    }
}

/// Return a cautious mode hint for USB IDs with a documented common mapping.
/// Unknown IDs are deliberately not guessed.
pub fn classify_usb_id(vendor_id: u16, product_id: u16) -> Option<UsbModeHint> {
    match (vendor_id, product_id) {
        (0x05c6, 0x9008) => Some(UsbModeHint::QualcommEdlQdl),
        // Commonly used for MediaTek BootROM, but the Linux usb.ids database
        // labels the same pair as a legacy MT6227 phone; display as a candidate.
        (0x0e8d, 0x0003) => Some(UsbModeHint::MediatekBromCandidate),
        (0x0e8d, 0x2000) => Some(UsbModeHint::MediatekPreloader),
        _ => None,
    }
}

/// Convert enumerated VID/PID pairs to recognized mode hints.
pub fn recognized_modes_from_ids(
    ids: impl IntoIterator<Item = (u16, u16)>,
) -> Vec<DetectedUsbMode> {
    let mut recognized = ids
        .into_iter()
        .filter_map(|(vendor_id, product_id)| {
            classify_usb_id(vendor_id, product_id).map(|mode| DetectedUsbMode {
                vendor_id,
                product_id,
                mode,
            })
        })
        .collect::<Vec<_>>();
    recognized.sort_by_key(|item| (item.vendor_id, item.product_id));
    recognized
}

/// Enumerate device IDs without opening a device or sending any USB transfers.
pub fn scan_usb_modes() -> Result<Vec<DetectedUsbMode>, String> {
    let devices = nusb::list_devices()
        .wait()
        .map_err(|error| format!("Enumerasi USB gagal: {error}"))?;
    Ok(recognized_modes_from_ids(
        devices.map(|device| (device.vendor_id(), device.product_id())),
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn classifies_known_qualcomm_edl_qdl_id() {
        assert_eq!(
            classify_usb_id(0x05c6, 0x9008),
            Some(UsbModeHint::QualcommEdlQdl)
        );
    }

    #[test]
    fn classifies_known_mediatek_ids_with_brom_ambiguity_explicit() {
        assert_eq!(
            classify_usb_id(0x0e8d, 0x0003),
            Some(UsbModeHint::MediatekBromCandidate)
        );
        assert!(UsbModeHint::MediatekBromCandidate.note().contains("ambigu"));
        assert_eq!(
            classify_usb_id(0x0e8d, 0x2000),
            Some(UsbModeHint::MediatekPreloader)
        );
    }

    #[test]
    fn unknown_usb_ids_are_not_guessed() {
        assert_eq!(classify_usb_id(0x1234, 0xabcd), None);
        assert_eq!(classify_usb_id(0x05c6, 0x9001), None);
        assert_eq!(classify_usb_id(0x0e8d, 0xffff), None);
    }

    #[cfg(target_os = "linux")]
    #[test]
    fn linux_enumeration_result_fixture_keeps_only_recognized_modes() {
        // Linux-style `lsusb` VID:PID values represented as parsed IDs. This
        // tests the exact mapping/filter path without requiring attached hardware.
        let ids = [
            (0x05c6, 0x9008),
            (0x0e8d, 0x2000),
            (0x0e8d, 0x0003),
            (0x1234, 0xabcd),
        ];
        let result = recognized_modes_from_ids(ids);
        assert_eq!(result.len(), 3);
        assert_eq!(result[0].usb_id(), "05c6:9008");
        assert_eq!(result[0].mode, UsbModeHint::QualcommEdlQdl);
        assert_eq!(result[1].usb_id(), "0e8d:0003");
        assert_eq!(result[1].mode, UsbModeHint::MediatekBromCandidate);
        assert_eq!(result[2].usb_id(), "0e8d:2000");
        assert_eq!(result[2].mode, UsbModeHint::MediatekPreloader);
    }
}
