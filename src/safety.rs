use crate::core::{Device, RomInspection};
use serde::Serialize;
use std::fs;
use std::path::Path;
use std::time::{SystemTime, UNIX_EPOCH};

pub const EVIDENCE_SCHEMA_VERSION: &str = "1.0";
pub const SUPPORT_MATRIX_VERSION: &str = "1.0";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Operation {
    Diagnose,
    Prepare,
    Flash,
    Unlock,
    Relock,
    Erase,
    Format,
    PartitionWrite,
}

#[derive(Debug, Clone, Default)]
pub struct GateEvidence {
    pub brand: Option<String>,
    pub codename: Option<String>,
    pub identity_checked: bool,
    pub identity_conflict: bool,
    pub rollback_index_known: bool,
    pub artifact_hash_verified: bool,
    pub official_provenance_verified: bool,
}

#[derive(Debug, Clone, Serialize)]
pub struct GateDecision {
    pub operation: String,
    pub allowed: bool,
    pub status: String,
    pub reasons: Vec<String>,
}

pub fn evaluate_gate(operation: Operation, evidence: &GateEvidence) -> GateDecision {
    if operation == Operation::Diagnose {
        return GateDecision {
            operation: operation_name(operation).into(),
            allowed: true,
            status: "read_only_only".into(),
            reasons: vec![
                "Pemeriksaan hanya menjalankan perintah baca-saja; identitas yang tidak pasti tidak memperluas cakupan dukungan.".into(),
                "Hasil diagnosis tidak mengizinkan persiapan atau perubahan pada perangkat.".into(),
            ],
        };
    }

    let mut reasons = vec![
        "Operasi ini belum tersedia. DroidNow tidak menjalankan perintah yang mengubah perangkat."
            .into(),
    ];
    if evidence.identity_conflict {
        reasons.push("Berhenti: data identitas perangkat berkonflik.".into());
    }
    if !evidence.identity_checked {
        reasons.push("Berhenti: pemeriksaan silang identitas belum selesai.".into());
    }
    if evidence
        .brand
        .as_deref()
        .is_none_or(|brand| !brand.eq_ignore_ascii_case("Xiaomi"))
    {
        reasons.push(
            "Berhenti: merek perangkat belum cocok dengan cakupan validasi yang tersedia.".into(),
        );
    }
    if evidence.codename.as_deref().is_none_or(str::is_empty) {
        reasons.push("Berhenti: kode perangkat tidak terbaca.".into());
    }
    if !evidence.rollback_index_known {
        reasons.push(
            "Berhenti: rollback index tidak diketahui; nilai kosong bukan bukti aman.".into(),
        );
    }
    if !evidence.artifact_hash_verified {
        reasons.push("Berhenti: SHA-256 berkas belum cocok dengan katalog tepercaya.".into());
    }
    if !evidence.official_provenance_verified {
        reasons.push("Berhenti: sumber resmi belum dapat dipastikan.".into());
    }
    GateDecision {
        operation: operation_name(operation).into(),
        allowed: false,
        status: "blocked_fail_closed".into(),
        reasons,
    }
}

fn operation_name(operation: Operation) -> &'static str {
    match operation {
        Operation::Diagnose => "diagnose",
        Operation::Prepare => "prepare",
        Operation::Flash => "flash",
        Operation::Unlock => "unlock",
        Operation::Relock => "relock",
        Operation::Erase => "erase",
        Operation::Format => "format",
        Operation::PartitionWrite => "partition_write",
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct SupportedTarget {
    pub brand: &'static str,
    pub model: &'static str,
    pub codename: &'static str,
    pub support_scope: &'static str,
    pub supported_regions: Vec<&'static str>,
    pub firmware_catalog_verified_entries: usize,
    pub arb_status: &'static str,
    pub avb_status: &'static str,
    pub write_operations_supported: bool,
}

#[derive(Debug, Clone, Serialize)]
pub struct SupportMatrix {
    pub version: &'static str,
    pub policy: &'static str,
    pub targets: Vec<SupportedTarget>,
    pub other_devices: &'static str,
}

pub fn support_matrix() -> SupportMatrix {
    SupportMatrix {
        version: SUPPORT_MATRIX_VERSION,
        policy: "Tujuan produk adalah diagnostik untuk seluruh model Xiaomi; validasi awal baru mencakup Redmi 4A. Jangan menyimpulkan kecocokan firmware atau izin perubahan perangkat dari nama, versi, wilayah, tanda tangan, maupun nilai getvar yang kosong.",
        targets: vec![SupportedTarget {
            brand: "Xiaomi",
            model: "Redmi 4A",
            codename: "rolex",
            support_scope: "diagnostik baca-saja terbatas; identitas perangkat tetap harus dibaca langsung dari perangkat",
            supported_regions: vec!["belum divalidasi"],
            firmware_catalog_verified_entries: 0,
            arb_status: "belum dapat dipastikan; getvar anti yang kosong bukan bukti bebas anti-rollback",
            avb_status: "belum dapat dipastikan melalui pemeriksaan ini",
            write_operations_supported: false,
        }],
        other_devices: "Model Xiaomi lain dan perangkat Android lain dapat terdeteksi melalui ADB/Fastboot, tetapi validasi tetap bertahap per model/SoC/region dan dukungan firmware belum dibuktikan. Operasi tulis tidak didukung.",
    }
}

pub fn stop_conditions() -> Vec<&'static str> {
    vec![
        "Identitas perangkat tidak terbaca atau data dari beberapa sumber berbeda.",
        "Model, SKU/kode perangkat, asal perangkat keras, wilayah, atau varian belum cocok secara jelas.",
        "Berkas firmware tidak cocok dengan acuan tepercaya atau nilai SHA-256 berbeda.",
        "Indeks anti-rollback tidak dapat dipastikan; getvar anti yang kosong bukan bukti aman.",
        "Tools tidak stabil, waktu tunggu habis, proses terhenti, atau koneksi terputus. Jangan ulangi otomatis; kembali ke pemeriksaan baca-saja.",
        "Pemulihan memerlukan bypass Mi Account, kunci aktivasi, masa tunggu unlock, anti-rollback, otorisasi EDL, atau kendali OEM.",
        "Jika hanya mode EDL/9008 tersedia, hentikan dan hubungi pusat servis resmi.",
    ]
}

#[derive(Debug, Clone, Serialize)]
pub struct EvidenceField {
    pub field: String,
    pub value: Option<String>,
    pub source: String,
    pub status: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct DeviceEvidence {
    pub mode: String,
    pub serial_masked: String,
    pub fields: Vec<EvidenceField>,
}

#[derive(Debug, Clone, Serialize)]
pub struct ArchivedDevice {
    serial_masked: String,
    mode: String,
    state: String,
    details: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct ArchivedRomFile {
    pub kind: String,
    pub name: String,
    pub size_bytes: u64,
    pub sha256_local: Option<String>,
    pub hash_error: Option<String>,
    pub trusted_catalog_match: bool,
}

#[derive(Debug, Clone, Serialize)]
pub struct ArchivedRomInspection {
    pub folder_name: String,
    pub codename_hint: String,
    pub files: Vec<ArchivedRomFile>,
    pub script_hint: String,
    pub provenance_status: String,
    pub verified_catalog_match: bool,
    pub warning: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct CaseArchive {
    pub evidence_schema_version: &'static str,
    pub captured_at_unix_seconds: u64,
    pub application_version: &'static str,
    pub support_matrix: SupportMatrix,
    pub capability_boundary: Vec<&'static str>,
    pub stop_conditions: Vec<&'static str>,
    pub devices: Vec<ArchivedDevice>,
    pub selected_device_evidence: Option<DeviceEvidence>,
    pub rom_inspection: Option<ArchivedRomInspection>,
    pub gate_decisions: Vec<GateDecision>,
    pub session_log: Vec<String>,
    pub privacy_note: &'static str,
}

fn is_unknown(value: &str) -> bool {
    let value = value.trim();
    let lower = value.to_ascii_lowercase();
    value.is_empty()
        || lower.contains("unknown")
        || lower.contains("indeterminate")
        || lower.contains("tidak terbaca")
        || lower.contains("tidak terdeteksi")
        || lower.contains("tidak tersedia")
        || lower.contains("tidak dihitung")
        || lower.contains("belum dikumpulkan")
}

pub fn mask_identifier(identifier: &str) -> String {
    let chars: Vec<char> = identifier.chars().collect();
    if chars.len() <= 4 {
        return "****".into();
    }
    format!(
        "{}****{}",
        chars[..2].iter().collect::<String>(),
        chars[chars.len() - 2..].iter().collect::<String>()
    )
}

fn redact_sensitive_text(text: &str, raw_identifiers: &[&str]) -> String {
    let mut redacted = text.to_string();
    for identifier in raw_identifiers.iter().filter(|value| !value.is_empty()) {
        redacted = redacted.replace(identifier, &mask_identifier(identifier));
    }
    redacted
        .lines()
        .map(redact_sensitive_line)
        .collect::<Vec<_>>()
        .join("\n")
}

fn redact_sensitive_line(line: &str) -> String {
    let lower = line.to_ascii_lowercase();
    for key in ["imei", "serial", "account", "token", "auth id"] {
        let Some(start) = lower.find(key) else {
            continue;
        };
        let suffix_start = start + key.len();
        if let Some((offset, delimiter)) = line[suffix_start..]
            .char_indices()
            .find(|(_, character)| *character == ':' || *character == '=')
        {
            let delimiter_end = suffix_start + offset + delimiter.len_utf8();
            return format!("{}[MASKED]", &line[..delimiter_end]);
        }
    }
    line.to_string()
}

fn evidence_from_info(mode: &str, serial: &str, info: Option<&str>) -> DeviceEvidence {
    let mut fields = Vec::new();
    let device_source = if mode.eq_ignore_ascii_case("fastboot") {
        "fastboot getvar (read-only)"
    } else {
        "adb shell getprop (read-only)"
    };
    if let Some(info) = info {
        for line in info.lines() {
            if let Some((field, raw_value)) = line.split_once(':') {
                let field = field.trim().to_string();
                let value = raw_value.trim();
                let lower_field = field.to_ascii_lowercase();
                let source = if lower_field == "adb version" {
                    "adb version (local toolchain)"
                } else if lower_field == "fastboot version" {
                    "fastboot --version (local toolchain)"
                } else if lower_field == "sha-256 berkas" {
                    "perhitungan SHA-256 lokal; belum ada pembanding tepercaya"
                } else if lower_field == "sumber resmi" {
                    "katalog SHA-256 resmi, yang belum tersedia"
                } else {
                    device_source
                };
                fields.push(EvidenceField {
                    field,
                    value: (!is_unknown(value)).then(|| raw_value.trim().to_string()),
                    source: source.into(),
                    status: if is_unknown(value) {
                        "unknown"
                    } else {
                        "observed"
                    }
                    .into(),
                });
            }
        }
    }
    for field in [
        "model",
        "brand",
        "codename",
        "product",
        "SKU / hardware",
        "MIUI region",
        "region / variant",
        "Android",
        "build",
        "bootloader unlocked",
        "Verified Boot",
        "AVB device state",
        "slot",
        "secure",
        "rollback index (anti)",
        "ADB version",
        "Fastboot version",
        "SHA-256 berkas",
        "Sumber resmi",
    ] {
        if !fields
            .iter()
            .any(|item| item.field.eq_ignore_ascii_case(field))
        {
            fields.push(EvidenceField {
                field: field.into(),
                value: None,
                source: "not collected / no trusted source configured".into(),
                status: "unknown".into(),
            });
        }
    }
    DeviceEvidence {
        mode: mode.into(),
        serial_masked: mask_identifier(serial),
        fields,
    }
}

pub fn create_case_archive(
    devices: &[Device],
    selected: Option<&Device>,
    device_info: Option<&str>,
    rom_inspection: Option<&RomInspection>,
    session_log: &[String],
) -> CaseArchive {
    let mut identifiers = devices
        .iter()
        .map(|device| device.serial.as_str())
        .collect::<Vec<_>>();
    identifiers.sort_by_key(|identifier| std::cmp::Reverse(identifier.len()));
    let archived_devices = devices
        .iter()
        .map(|device| ArchivedDevice {
            serial_masked: mask_identifier(&device.serial),
            mode: device.mode.clone(),
            state: device.state.clone(),
            details: redact_sensitive_text(&device.details, &identifiers),
        })
        .collect();
    let sanitized_info = device_info.map(|value| redact_sensitive_text(value, &identifiers));
    let selected_device_evidence = selected
        .map(|device| evidence_from_info(&device.mode, &device.serial, sanitized_info.as_deref()));
    let archived_rom_inspection = rom_inspection.map(|inspection| ArchivedRomInspection {
        folder_name: redact_sensitive_text(&inspection.folder_name, &identifiers),
        codename_hint: inspection.codename_hint.clone(),
        files: inspection
            .entries
            .iter()
            .map(|entry| ArchivedRomFile {
                kind: entry.kind.clone(),
                name: redact_sensitive_text(&entry.name, &identifiers),
                size_bytes: entry.size_bytes,
                sha256_local: entry.sha256.clone(),
                hash_error: entry.hash_error.clone(),
                trusted_catalog_match: false,
            })
            .collect(),
        script_hint: redact_sensitive_text(&inspection.script_hint, &identifiers),
        provenance_status: inspection.provenance_status.clone(),
        verified_catalog_match: inspection.verified_catalog_match,
        warning: inspection.warning.clone(),
    });
    let mut gate_evidence = GateEvidence::default();
    if let Some(info) = sanitized_info.as_deref() {
        for line in info.lines() {
            let Some((label, value)) = line.split_once(':') else {
                continue;
            };
            match label.trim().to_ascii_lowercase().as_str() {
                "brand" => {
                    if !is_unknown(value) {
                        gate_evidence.brand = Some(value.trim().into());
                    }
                }
                "codename" | "product" => {
                    if !is_unknown(value) {
                        gate_evidence.codename = Some(value.trim().into());
                    }
                }
                "rollback index (anti)" => {
                    gate_evidence.rollback_index_known = !is_unknown(value)
                        && !value.to_ascii_lowercase().contains("tidak terdeteksi");
                }
                _ => {}
            }
        }
    }
    let gate_decisions = [
        Operation::Diagnose,
        Operation::Prepare,
        Operation::Flash,
        Operation::Unlock,
        Operation::Relock,
        Operation::Erase,
        Operation::Format,
        Operation::PartitionWrite,
    ]
    .into_iter()
    .map(|operation| evaluate_gate(operation, &gate_evidence))
    .collect();
    CaseArchive {
        evidence_schema_version: EVIDENCE_SCHEMA_VERSION,
        captured_at_unix_seconds: SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs(),
        application_version: env!("CARGO_PKG_VERSION"),
        support_matrix: support_matrix(),
        capability_boundary: vec![
            "diagnose: read-only probes only",
            "prepare: unavailable",
            "write / flash / unlock / relock / erase / format / partition write: blocked",
            "reboot: existing explicitly confirmed convenience action; not a write/repair authorization",
        ],
        stop_conditions: stop_conditions(),
        devices: archived_devices,
        selected_device_evidence,
        rom_inspection: archived_rom_inspection,
        gate_decisions,
        session_log: session_log
            .iter()
            .map(|entry| redact_sensitive_text(entry, &identifiers))
            .collect(),
    privacy_note: "Local export only. Known device serials and common IMEI/serial/account/token/auth ID key-value forms are redacted on a best-effort basis. Review the archive manually before sharing; free-form identifiers may remain. No archive is uploaded automatically.",
    }
}

pub fn write_case_archive(path: &Path, archive: &CaseArchive) -> Result<(), String> {
    let json = serde_json::to_vec_pretty(archive)
        .map_err(|error| format!("Arsip evidence tidak dapat diserialisasi: {error}"))?;
    fs::write(path, json).map_err(|error| format!("Arsip lokal tidak dapat disimpan: {error}"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn support_matrix_is_explicitly_limited_and_unknown() {
        let matrix = support_matrix();
        assert_eq!(matrix.targets.len(), 1);
        let target = &matrix.targets[0];
        assert_eq!(target.codename, "rolex");
        assert_eq!(target.firmware_catalog_verified_entries, 0);
        assert!(!target.write_operations_supported);
        assert!(target.arb_status.contains("belum dapat dipastikan"));
    }

    #[test]
    fn every_non_diagnostic_operation_is_blocked_even_with_complete_evidence() {
        let evidence = GateEvidence {
            brand: Some("Xiaomi".into()),
            codename: Some("rolex".into()),
            identity_checked: true,
            identity_conflict: false,
            rollback_index_known: true,
            artifact_hash_verified: true,
            official_provenance_verified: true,
        };
        for operation in [
            Operation::Prepare,
            Operation::Flash,
            Operation::Unlock,
            Operation::Relock,
            Operation::Erase,
            Operation::Format,
            Operation::PartitionWrite,
        ] {
            let decision = evaluate_gate(operation, &evidence);
            assert!(!decision.allowed, "{operation:?} must remain unavailable");
            assert_eq!(decision.status, "blocked_fail_closed");
        }
    }

    #[test]
    fn unknown_rollback_and_identity_conflict_are_explicit_stop_reasons() {
        let decision = evaluate_gate(
            Operation::Flash,
            &GateEvidence {
                identity_conflict: true,
                ..GateEvidence::default()
            },
        );
        let reasons = decision.reasons.join(" ");
        assert!(reasons.contains("berkonflik"));
        assert!(reasons.contains("rollback index tidak diketahui"));
        assert!(reasons.contains("sumber resmi belum dapat dipastikan"));
    }

    #[test]
    fn diagnostic_gate_is_read_only_and_does_not_claim_write_eligibility() {
        let decision = evaluate_gate(Operation::Diagnose, &GateEvidence::default());
        assert!(decision.allowed);
        assert_eq!(decision.status, "read_only_only");
    }

    #[test]
    fn archives_mask_identifiers_and_retain_unknown_evidence() {
        let device = Device {
            serial: "ABC1234567".into(),
            mode: "fastboot".into(),
            state: "fastboot".into(),
            details: "serial: ABC1234567".into(),
        };
        let rom = RomInspection {
            folder_name: "rolex-test-package".into(),
            codename_hint: "rolex".into(),
            entries: vec![crate::core::RomEntry {
                kind: "IMAGE".into(),
                name: "images/boot.img".into(),
                size_bytes: 3,
                sha256: Some(
                    "ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad".into(),
                ),
                hash_error: None,
            }],
            script_hint: "skrip tidak terlihat".into(),
            provenance_status: "Belum dapat dipastikan: katalog SHA-256 tepercaya belum tersedia."
                .into(),
            verified_catalog_match: false,
            warning: "Hash lokal bukan bukti sumber resmi".into(),
        };
        let archive = create_case_archive(
            std::slice::from_ref(&device),
            Some(&device),
            Some("product: rolex\nrollback index (anti): tidak tersedia\nIMEI: 123456789012345"),
            Some(&rom),
            &["Ditemukan perangkat ABC1234567".into()],
        );
        let json = serde_json::to_string(&archive).unwrap();
        assert!(!json.contains("ABC1234567"));
        assert!(!json.contains("123456789012345"));
        assert!(json.contains("AB****67"));
        assert!(json.contains("belum dapat dipastikan"));
        assert!(json.contains("ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad"));
        assert!(json.contains("trusted_catalog_match\":false"));
        assert!(archive.gate_decisions.iter().any(|decision| {
            decision.operation == "flash"
                && !decision.allowed
                && decision.status == "blocked_fail_closed"
        }));
        assert!(archive.gate_decisions.iter().any(|decision| {
            decision.operation == "diagnose"
                && decision.allowed
                && decision.status == "read_only_only"
        }));
        assert!(archive
            .selected_device_evidence
            .as_ref()
            .unwrap()
            .fields
            .iter()
            .any(|field| {
                field.field == "rollback index (anti)"
                    && field.status == "unknown"
                    && field.value.is_none()
            }));
        assert!(serde_json::from_str::<serde_json::Value>(&json).is_ok());
        let redacted = redact_sensitive_text("debug IMEI=123456789012345 token:secret-value", &[]);
        assert!(!redacted.contains("123456789012345"));
        assert!(!redacted.contains("secret-value"));
        assert!(redacted.contains("[MASKED]"));
    }

    #[test]
    fn masks_short_identifiers_without_leaking_them() {
        assert_eq!(mask_identifier("A1"), "****");
        assert_eq!(mask_identifier("ABCDE"), "AB****DE");
    }
}
