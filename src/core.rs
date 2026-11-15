use crate::process::{ProcessOutput, ProcessRunner, SystemProcessRunner};
use serde::Serialize;
use sha2::{Digest, Sha256};
use std::collections::HashSet;
use std::env;
use std::fs;
use std::fs::File;
use std::io::{Read, Seek, Write};
use std::path::{Component, Path, PathBuf};
use std::sync::Arc;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

const MAX_PLATFORM_TOOLS_ARCHIVE_SIZE: u64 = 100 * 1024 * 1024;
const MAX_PLATFORM_TOOLS_EXTRACTED_SIZE: u64 = 512 * 1024 * 1024;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Device {
    pub serial: String,
    pub mode: String,
    pub state: String,
    pub details: String,
}

#[derive(Debug, Clone)]
pub struct ToolStatus {
    pub adb_available: bool,
    pub fastboot_available: bool,
    pub location: String,
}

#[derive(Debug, Clone)]
pub struct ScanResult {
    pub devices: Vec<Device>,
    pub messages: Vec<String>,
    pub tools: ToolStatus,
}

#[derive(Debug, Clone, Serialize)]
pub struct RomEntry {
    pub kind: String,
    pub name: String,
    pub size_bytes: u64,
    pub sha256: Option<String>,
    pub hash_error: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct RomInspection {
    pub folder_name: String,
    pub codename_hint: String,
    pub entries: Vec<RomEntry>,
    pub script_hint: String,
    pub provenance_status: String,
    pub verified_catalog_match: bool,
    pub warning: String,
}

#[derive(Clone)]
pub struct ToolLocator {
    directory: Option<PathBuf>,
    runner: Arc<dyn ProcessRunner>,
}

impl Default for ToolLocator {
    fn default() -> Self {
        Self {
            directory: None,
            runner: Arc::new(SystemProcessRunner),
        }
    }
}

impl ToolLocator {
    #[cfg(test)]
    fn with_runner(runner: Arc<dyn ProcessRunner>) -> Self {
        Self {
            directory: None,
            runner,
        }
    }

    pub fn set_directory(&mut self, path: PathBuf) -> Result<(), String> {
        if !path.is_dir() {
            return Err("Folder yang dipilih bukan direktori yang dapat dibaca.".to_string());
        }
        self.directory = Some(path);
        Ok(())
    }

    pub fn status(&self) -> ToolStatus {
        let installed_directory = platform_tools_data_dir()
            .ok()
            .map(|path| path.join("platform-tools"));
        let executable = |name: &str| {
            if cfg!(target_os = "windows") {
                format!("{name}.exe")
            } else {
                name.to_string()
            }
        };
        let location = self
            .directory
            .as_ref()
            .map(|path| path.display().to_string())
            .or_else(|| {
                installed_directory
                    .as_ref()
                    .filter(|path| {
                        path.join(executable("adb")).is_file()
                            || path.join(executable("fastboot")).is_file()
                    })
                    .map(|path| path.display().to_string())
            })
            .unwrap_or_else(|| "Mencari di PATH sistem".to_string());
        ToolStatus {
            adb_available: self.resolve("adb").is_some(),
            fastboot_available: self.resolve("fastboot").is_some(),
            location,
        }
    }

    fn resolve(&self, name: &str) -> Option<PathBuf> {
        let filename = if cfg!(target_os = "windows") {
            format!("{name}.exe")
        } else {
            name.to_string()
        };
        if let Some(directory) = &self.directory {
            let candidate = directory.join(&filename);
            if candidate.is_file() {
                return Some(candidate);
            }
        }
        if let Some(directory) = platform_tools_data_dir()
            .ok()
            .map(|path| path.join("platform-tools"))
        {
            let candidate = directory.join(&filename);
            if candidate.is_file() {
                return Some(candidate);
            }
        }
        env::split_paths(&env::var_os("PATH").unwrap_or_default())
            .map(|directory| directory.join(&filename))
            .find(|candidate| candidate.is_file())
    }

    fn run(&self, name: &str, args: &[String], timeout_seconds: u64) -> Result<ToolOutput, String> {
        let binary = self.resolve(name).ok_or_else(|| {
            format!("{name} tidak ditemukan. Pilih folder Android SDK Platform-Tools.")
        })?;
        self.runner
            .run(&binary, args, Duration::from_secs(timeout_seconds))
    }
}

type ToolOutput = ProcessOutput;

/// Downloads and installs Google's official Android SDK Platform-Tools for this OS.
/// The archive is extracted to the current user's application data directory; no
/// administrator privileges are needed and no device commands are run.
pub fn install_platform_tools() -> Result<PathBuf, String> {
    let (archive_name, executable_suffix) = if cfg!(target_os = "windows") {
        ("platform-tools-latest-windows.zip", ".exe")
    } else if cfg!(target_os = "macos") {
        ("platform-tools-latest-darwin.zip", "")
    } else if cfg!(target_os = "linux") {
        ("platform-tools-latest-linux.zip", "")
    } else {
        return Err("Pemasangan otomatis belum tersedia untuk sistem operasi ini.".into());
    };

    let base_dir = platform_tools_data_dir()?;
    fs::create_dir_all(&base_dir)
        .map_err(|error| format!("Folder data aplikasi tidak dapat dibuat: {error}"))?;
    let unique = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_nanos();
    let archive_path = env::temp_dir().join(format!("droidnow-platform-tools-{unique}.zip"));
    let stage_dir = base_dir.join(format!(".platform-tools-stage-{unique}"));
    let install_dir = base_dir.join("platform-tools");
    let backup_dir = base_dir.join(format!(".platform-tools-backup-{unique}"));

    let result = (|| {
        let client = reqwest::blocking::Client::builder()
            .user_agent(concat!("DroidNow/", env!("CARGO_PKG_VERSION")))
            .connect_timeout(Duration::from_secs(20))
            .timeout(Duration::from_secs(300))
            .build()
            .map_err(|error| format!("Klien unduhan tidak dapat dibuat: {error}"))?;
        let url = format!("https://dl.google.com/android/repository/{archive_name}");
        let response = client
            .get(&url)
            .send()
            .and_then(|response| response.error_for_status())
            .map_err(|error| format!("Unduhan resmi Platform-Tools gagal: {error}"))?;
        if response
            .content_length()
            .is_some_and(|size| size > MAX_PLATFORM_TOOLS_ARCHIVE_SIZE)
        {
            return Err("Ukuran arsip Platform-Tools melebihi batas keamanan 100 MiB.".into());
        }

        let mut archive_file = fs::OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(&archive_path)
            .map_err(|error| format!("Berkas unduhan sementara tidak dapat dibuat: {error}"))?;
        let downloaded = std::io::copy(
            &mut response.take(MAX_PLATFORM_TOOLS_ARCHIVE_SIZE + 1),
            &mut archive_file,
        )
        .map_err(|error| format!("Unduhan Platform-Tools terputus: {error}"))?;
        if downloaded > MAX_PLATFORM_TOOLS_ARCHIVE_SIZE {
            return Err("Ukuran arsip Platform-Tools melebihi batas keamanan 100 MiB.".into());
        }
        archive_file
            .flush()
            .map_err(|error| format!("Berkas unduhan tidak dapat disimpan: {error}"))?;
        drop(archive_file);

        fs::create_dir(&stage_dir)
            .map_err(|error| format!("Folder pemasangan sementara tidak dapat dibuat: {error}"))?;
        let archive_file = File::open(&archive_path)
            .map_err(|error| format!("Arsip unduhan tidak dapat dibuka: {error}"))?;
        extract_platform_tools_archive(archive_file, &stage_dir, executable_suffix)?;
        let staged_tools_dir = stage_dir.join("platform-tools");

        if install_dir.exists() {
            fs::rename(&install_dir, &backup_dir).map_err(|error| {
                format!("Instalasi Platform-Tools lama tidak dapat diganti: {error}")
            })?;
        }
        if let Err(error) = fs::rename(&staged_tools_dir, &install_dir) {
            if backup_dir.exists() {
                let _ = fs::rename(&backup_dir, &install_dir);
            }
            return Err(format!("Platform-Tools gagal dipasang: {error}"));
        }
        if backup_dir.exists() {
            let _ = remove_path(&backup_dir);
        }
        Ok(install_dir.clone())
    })();

    let _ = fs::remove_file(&archive_path);
    let _ = remove_path(&stage_dir);
    result
}

fn extract_platform_tools_archive<R: Read + Seek>(
    reader: R,
    stage_dir: &Path,
    executable_suffix: &str,
) -> Result<(), String> {
    let mut archive = zip::ZipArchive::new(reader)
        .map_err(|error| format!("Arsip resmi Platform-Tools tidak valid: {error}"))?;
    let mut extracted_size = 0_u64;
    for index in 0..archive.len() {
        let mut entry = archive
            .by_index(index)
            .map_err(|error| format!("Isi arsip Platform-Tools tidak dapat dibaca: {error}"))?;
        let archive_name = entry
            .name()
            .map_err(|error| format!("Nama file arsip Platform-Tools tidak valid: {error}"))?;
        let path_without_trailing_separator =
            archive_name.strip_suffix('/').unwrap_or(&archive_name);
        let has_unsafe_component = archive_name.starts_with('/')
            || archive_name.contains('\\')
            || path_without_trailing_separator
                .split('/')
                .any(|component| component.is_empty())
            || path_without_trailing_separator
                .split('/')
                .any(|component| component == "." || component == "..");
        let relative_path = entry
            .enclosed_name()
            .filter(|path| {
                !has_unsafe_component
                    && path
                        .components()
                        .all(|component| matches!(component, Component::Normal(_)))
            })
            .ok_or_else(|| "Arsip Platform-Tools berisi jalur file yang tidak aman.".to_string())?
            .to_path_buf();
        if entry.is_symlink() {
            return Err("Arsip Platform-Tools berisi tautan simbolik yang ditolak.".into());
        }
        extracted_size = extracted_size
            .checked_add(entry.size())
            .ok_or_else(|| "Ukuran ekstraksi arsip tidak valid.".to_string())?;
        if extracted_size > MAX_PLATFORM_TOOLS_EXTRACTED_SIZE {
            return Err("Ukuran hasil ekstraksi melebihi batas keamanan 512 MiB.".into());
        }

        let output_path = stage_dir.join(relative_path);
        if entry.is_dir() {
            fs::create_dir_all(&output_path)
                .map_err(|error| format!("Folder Platform-Tools gagal diekstrak: {error}"))?;
            continue;
        }
        if let Some(parent) = output_path.parent() {
            fs::create_dir_all(parent)
                .map_err(|error| format!("Folder Platform-Tools gagal dibuat: {error}"))?;
        }
        let unix_mode = entry.unix_mode();
        let mut output = File::create(&output_path)
            .map_err(|error| format!("Berkas Platform-Tools gagal dibuat: {error}"))?;
        std::io::copy(&mut entry, &mut output)
            .map_err(|error| format!("Berkas Platform-Tools gagal diekstrak: {error}"))?;
        output
            .flush()
            .map_err(|error| format!("Berkas Platform-Tools gagal disimpan: {error}"))?;
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            let filename = output_path
                .file_name()
                .and_then(|name| name.to_str())
                .unwrap_or_default();
            let mut mode = unix_mode.unwrap_or(0o644) & 0o777;
            if filename == "adb" || filename == "fastboot" {
                mode |= 0o111;
            }
            fs::set_permissions(&output_path, fs::Permissions::from_mode(mode))
                .map_err(|error| format!("Izin berkas Platform-Tools gagal diatur: {error}"))?;
        }
    }

    let staged_tools_dir = stage_dir.join("platform-tools");
    for binary in [
        format!("adb{executable_suffix}"),
        format!("fastboot{executable_suffix}"),
    ] {
        if !staged_tools_dir.join(binary).is_file() {
            return Err("Arsip unduhan tidak memuat adb dan fastboot yang diperlukan.".into());
        }
    }
    Ok(())
}

fn platform_tools_data_dir() -> Result<PathBuf, String> {
    #[cfg(target_os = "windows")]
    let base = env::var_os("LOCALAPPDATA")
        .or_else(|| env::var_os("APPDATA"))
        .map(PathBuf::from)
        .ok_or_else(|| "Folder data pengguna Windows tidak ditemukan.".to_string())?;

    #[cfg(target_os = "macos")]
    let base = env::var_os("HOME")
        .map(PathBuf::from)
        .map(|home| home.join("Library").join("Application Support"))
        .ok_or_else(|| "Folder data pengguna macOS tidak ditemukan.".to_string())?;

    #[cfg(target_os = "linux")]
    let base = env::var_os("XDG_DATA_HOME")
        .map(PathBuf::from)
        .or_else(|| env::var_os("HOME").map(|home| PathBuf::from(home).join(".local/share")))
        .ok_or_else(|| "Folder data pengguna Linux tidak ditemukan.".to_string())?;

    #[cfg(not(any(target_os = "windows", target_os = "macos", target_os = "linux")))]
    return Err("Pemasangan otomatis belum tersedia untuk sistem operasi ini.".into());

    #[cfg(any(target_os = "windows", target_os = "macos", target_os = "linux"))]
    Ok(base.join("DroidNow"))
}

fn remove_path(path: &Path) -> std::io::Result<()> {
    match fs::symlink_metadata(path) {
        Ok(metadata) if metadata.file_type().is_dir() => fs::remove_dir_all(path),
        Ok(_) => fs::remove_file(path),
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => Ok(()),
        Err(error) => Err(error),
    }
}

fn parse_adb_devices(text: &str) -> Vec<Device> {
    let mut found = Vec::new();
    for line in text.lines().map(str::trim) {
        if line.is_empty() || line.starts_with("List of devices") || line.starts_with("* daemon") {
            continue;
        }
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() >= 2 && ["device", "unauthorized", "offline"].contains(&parts[1]) {
            found.push(Device {
                serial: parts[0].to_string(),
                mode: "adb".to_string(),
                state: parts[1].to_string(),
                details: parts[2..].join(" "),
            });
        }
    }
    found
}

fn parse_fastboot_devices(text: &str) -> Vec<Device> {
    text.lines()
        .map(str::trim)
        .filter_map(|line| {
            let parts: Vec<&str> = line.split_whitespace().collect();
            (parts.len() >= 2 && parts[1].eq_ignore_ascii_case("fastboot")).then(|| Device {
                serial: parts[0].to_string(),
                mode: "fastboot".to_string(),
                state: "fastboot".to_string(),
                details: parts[2..].join(" "),
            })
        })
        .collect()
}

fn parse_getvar(text: &str, key: &str) -> String {
    for line in text.lines() {
        let line = line.trim().trim_start_matches("(bootloader)").trim();
        if let Some((found_key, value)) = line.split_once(':') {
            if found_key.trim().eq_ignore_ascii_case(key) {
                return value.trim().to_string();
            }
        }
    }
    "tidak tersedia".to_string()
}

fn valid_serial(serial: &str) -> bool {
    !serial.is_empty()
        && serial.len() <= 128
        && serial
            .chars()
            .next()
            .is_some_and(|character| character.is_ascii_alphanumeric())
        && serial
            .chars()
            .all(|character| character.is_ascii_alphanumeric() || "._:-".contains(character))
}

pub fn scan_devices(tools: &ToolLocator) -> ScanResult {
    let mut devices = Vec::new();
    let mut messages = Vec::new();
    for (tool, args, parser) in [
        (
            "adb",
            vec!["devices".to_string(), "-l".to_string()],
            parse_adb_devices as fn(&str) -> Vec<Device>,
        ),
        (
            "fastboot",
            vec!["devices".to_string()],
            parse_fastboot_devices as fn(&str) -> Vec<Device>,
        ),
    ] {
        match tools.run(tool, &args, 25) {
            Ok(output) => {
                let combined = format!("{}\n{}", output.stdout, output.stderr);
                devices.extend(parser(&combined));
                if output.code.unwrap_or(0) != 0 && combined.trim().is_empty() {
                    messages.push(format!("{tool} selesai dengan kode {:?}.", output.code));
                }
            }
            Err(error) => messages.push(error),
        }
    }
    let mut seen = HashSet::new();
    devices.retain(|device| seen.insert((device.mode.clone(), device.serial.clone())));
    ScanResult {
        devices,
        messages,
        tools: tools.status(),
    }
}

pub fn read_device_info(device: &Device, tools: &ToolLocator) -> Result<String, String> {
    if !valid_serial(&device.serial) {
        return Err("Serial perangkat tidak valid.".to_string());
    }
    match device.mode.as_str() {
        "adb" if device.state == "device" => {
            let mut lines = Vec::new();
            for (label, property) in [
                ("model", "ro.product.model"),
                ("brand", "ro.product.brand"),
                ("codename", "ro.product.device"),
                ("SKU / hardware", "ro.boot.hardware.sku"),
                ("MIUI region", "ro.miui.region"),
                ("Android", "ro.build.version.release"),
                ("build", "ro.build.display.id"),
                ("bootloader locked (flash.locked)", "ro.boot.flash.locked"),
                ("Verified Boot", "ro.boot.verifiedbootstate"),
                ("AVB device state", "ro.boot.vbmeta.device_state"),
                ("slot", "ro.boot.slot_suffix"),
            ] {
                let value = tools
                    .run(
                        "adb",
                        &[
                            "-s".into(),
                            device.serial.clone(),
                            "shell".into(),
                            "getprop".into(),
                            property.into(),
                        ],
                        12,
                    )
                    .map(|output| {
                        if !tool_succeeded(&output) {
                            "tidak terbaca (probe gagal)".into()
                        } else if output.stdout.trim().is_empty()
                            && !output.stderr.trim().is_empty()
                        {
                            output.stderr.trim().replace('\n', " ")
                        } else {
                            output.stdout.trim().replace('\n', " ")
                        }
                    })
                    .unwrap_or_else(|_| "tidak terbaca".into());
                lines.push(format!(
                    "{label}: {}",
                    if value.is_empty() {
                        "tidak terbaca"
                    } else {
                        &value
                    }
                ));
            }
            lines.push(
                "rollback index (anti): tidak terdeteksi (probe Fastboot tidak berjalan; bukan bukti aman)".into(),
            );
            lines.push(format!(
                "ADB version: {}",
                read_tool_version(tools, "adb", &["version".into()])
            ));
            lines.push(format!(
                "Fastboot version: {}",
                read_tool_version(tools, "fastboot", &["--version".into()])
            ));
            lines.push("SHA-256 berkas: belum dihitung (belum memilih berkas).".into());
            lines.push(
                "Sumber resmi: belum dapat dipastikan (katalog SHA-256 tepercaya belum tersedia)."
                    .into(),
            );
            Ok(lines.join("\n"))
        }
        "fastboot" => {
            let mut lines = Vec::new();
            for (label, key) in [
                ("product", "product"),
                ("SKU / hardware", "sku"),
                ("region / variant", "variant"),
                ("bootloader unlocked", "unlocked"),
                ("rollback index (anti)", "anti"),
                ("slot", "current-slot"),
                ("mode (is-userspace)", "is-userspace"),
                ("bootloader version", "version-bootloader"),
                ("secure", "secure"),
            ] {
                let value = read_fastboot_var(tools, &device.serial, key);
                let value = if value == "tidak tersedia" {
                    "tidak terdeteksi (bukan bukti aman)".to_string()
                } else {
                    value
                };
                lines.push(format!("{label}: {value}"));
            }
            lines.extend([
                "Android: tidak tersedia dalam mode Fastboot".into(),
                "build: tidak tersedia dalam mode Fastboot".into(),
                "Verified Boot: tidak tersedia melalui probe ini".into(),
                "ADB version: tidak diuji dalam mode Fastboot".into(),
                format!(
                    "Fastboot version: {}",
                    read_tool_version(tools, "fastboot", &["--version".into()])
                ),
                "SHA-256 berkas: belum dihitung (belum memilih berkas).".into(),
                "Sumber resmi: belum dapat dipastikan (katalog SHA-256 tepercaya belum tersedia)."
                    .into(),
            ]);
            Ok(lines.join("\n"))
        }
        "adb" => Err(
            "ADB belum berstatus device. Jika unauthorized, setujui dialog RSA di ponsel.".into(),
        ),
        _ => Err("Mode perangkat tidak dikenal.".to_string()),
    }
}

fn read_fastboot_var(tools: &ToolLocator, serial: &str, key: &str) -> String {
    let Ok(output) = tools.run(
        "fastboot",
        &["-s".into(), serial.into(), "getvar".into(), key.into()],
        12,
    ) else {
        return "tidak tersedia".into();
    };
    if !tool_succeeded(&output) {
        return "tidak tersedia".into();
    }
    parse_getvar(&format!("{}\n{}", output.stdout, output.stderr), key)
}

fn read_tool_version(tools: &ToolLocator, tool: &str, args: &[String]) -> String {
    match tools.run(tool, args, 12) {
        Ok(output) if !tool_succeeded(&output) => "tidak terbaca".into(),
        Ok(output) => format!("{}\n{}", output.stdout, output.stderr)
            .lines()
            .map(str::trim)
            .find(|line| !line.is_empty())
            .unwrap_or("tidak terbaca")
            .to_string(),
        Err(_) => "tidak terbaca".into(),
    }
}

fn tool_succeeded(output: &ToolOutput) -> bool {
    output.code == Some(0)
}

fn build_reboot_command(
    device: &Device,
    action: &str,
) -> Result<(&'static str, Vec<String>), String> {
    if !valid_serial(&device.serial) {
        return Err("Serial perangkat tidak valid.".to_string());
    }
    let command = match (device.mode.as_str(), action) {
        ("adb", "recovery") if device.state == "device" => (
            "adb",
            vec![
                "-s".into(),
                device.serial.clone(),
                "reboot".into(),
                "recovery".into(),
            ],
        ),
        ("adb", "bootloader") if device.state == "device" => (
            "adb",
            vec![
                "-s".into(),
                device.serial.clone(),
                "reboot".into(),
                "bootloader".into(),
            ],
        ),
        ("adb", "system") if device.state == "device" => (
            "adb",
            vec!["-s".into(), device.serial.clone(), "reboot".into()],
        ),
        ("fastboot", "system") => (
            "fastboot",
            vec!["-s".into(), device.serial.clone(), "reboot".into()],
        ),
        ("fastboot", "recovery") => {
            return Err(
                "Fastboot tidak menyediakan perintah umum untuk reboot ke Recovery.".into(),
            );
        }
        ("fastboot", "bootloader") => return Err("Perangkat sudah berada di mode Fastboot.".into()),
        ("adb", _) if device.state != "device" => {
            return Err("ADB tidak berstatus device; reboot dibatalkan.".into());
        }
        _ => return Err("Aksi reboot tidak dikenal.".into()),
    };
    Ok(command)
}

pub fn reboot_device(device: &Device, action: &str, tools: &ToolLocator) -> Result<String, String> {
    let (tool, args) = build_reboot_command(device, action)?;
    let output = tools.run(tool, &args, 15)?;
    let combined = format!("{}{}", output.stdout, output.stderr)
        .trim()
        .to_string();
    if output.code.unwrap_or(1) != 0 {
        return Err(if combined.is_empty() {
            format!("Perintah reboot gagal (exit {:?}).", output.code)
        } else {
            combined
        });
    }
    Ok(if combined.is_empty() {
        "Perintah reboot berhasil dikirim.".into()
    } else {
        combined
    })
}

fn is_image(path: &Path) -> bool {
    matches!(
        path.extension()
            .and_then(|extension| extension.to_str())
            .unwrap_or_default()
            .to_ascii_lowercase()
            .as_str(),
        "img" | "bin" | "elf" | "mbn"
    )
}

fn list_folder(folder: &Path, prefix: &str, entries: &mut Vec<RomEntry>) -> Result<(), String> {
    let mut children = fs::read_dir(folder)
        .map_err(|error| format!("Folder ROM tidak dapat dibaca: {error}"))?
        .filter_map(Result::ok)
        .collect::<Vec<_>>();
    children.sort_by_key(|entry| entry.file_name().to_string_lossy().to_lowercase());
    for entry in children {
        let file_type = entry
            .file_type()
            .map_err(|error| format!("Tidak dapat membaca metadata file: {error}"))?;
        if !file_type.is_file() {
            continue;
        }
        let path = entry.path();
        let extension = path
            .extension()
            .and_then(|value| value.to_str())
            .unwrap_or_default()
            .to_ascii_lowercase();
        let kind = if is_image(&path) {
            Some("IMAGE")
        } else if ["bat", "sh", "txt", "cfg", "xml"].contains(&extension.as_str()) {
            Some("SCRIPT / META")
        } else {
            None
        };
        if let Some(kind) = kind {
            let (size_bytes, sha256, hash_error) = match sha256_file(&path) {
                Ok((size, hash)) => (size, Some(hash), None),
                Err(_) => (
                    fs::metadata(&path)
                        .map(|metadata| metadata.len())
                        .unwrap_or(0),
                    None,
                    Some("SHA-256 tidak dapat dihitung secara lokal".into()),
                ),
            };
            entries.push(RomEntry {
                kind: kind.to_string(),
                name: format!("{prefix}{}", entry.file_name().to_string_lossy()),
                size_bytes,
                sha256,
                hash_error,
            });
        }
    }
    Ok(())
}

fn sha256_file(path: &Path) -> Result<(u64, String), String> {
    let path_before =
        fs::metadata(path).map_err(|error| format!("Metadata file tidak dapat dibaca: {error}"))?;
    let mut file = File::open(path)
        .map_err(|error| format!("File tidak dapat dibuka untuk hashing: {error}"))?;
    let file_before = file
        .metadata()
        .map_err(|error| format!("Metadata file terbuka tidak dapat dibaca: {error}"))?;
    let path_mtime_before = path_before.modified().ok();
    let file_mtime_before = file_before.modified().ok();
    if path_mtime_before.is_none()
        || file_mtime_before.is_none()
        || path_before.len() != file_before.len()
        || path_mtime_before != file_mtime_before
    {
        return Err("File berubah sebelum proses hashing dimulai.".into());
    }
    let mut hasher = Sha256::new();
    let mut buffer = [0_u8; 64 * 1024];
    let mut bytes_read = 0_u64;
    loop {
        let read = file
            .read(&mut buffer)
            .map_err(|error| format!("File tidak dapat dibaca untuk hashing: {error}"))?;
        if read == 0 {
            break;
        }
        bytes_read = bytes_read
            .checked_add(read as u64)
            .ok_or_else(|| "Ukuran file saat hashing melebihi batas.".to_string())?;
        hasher.update(&buffer[..read]);
    }
    let file_after = file
        .metadata()
        .map_err(|error| format!("Metadata file setelah hashing gagal dibaca: {error}"))?;
    let path_after =
        fs::metadata(path).map_err(|error| format!("File hilang setelah hashing: {error}"))?;
    let file_mtime_after = file_after.modified().ok();
    let path_mtime_after = path_after.modified().ok();
    if file_mtime_after.is_none()
        || path_mtime_after.is_none()
        || bytes_read != file_before.len()
        || file_before.len() != file_after.len()
        || file_mtime_before != file_mtime_after
        || path_after.len() != file_after.len()
        || path_mtime_after != file_mtime_after
    {
        return Err("File berubah selama proses hashing; hash ditolak.".into());
    }
    let hash = hasher
        .finalize()
        .iter()
        .map(|byte| format!("{byte:02x}"))
        .collect();
    Ok((bytes_read, hash))
}

pub fn inspect_rom_folder(path: &Path) -> Result<RomInspection, String> {
    if !path.is_dir() {
        return Err("Folder yang dipilih tidak tersedia atau bukan direktori.".into());
    }
    let folder_name = path
        .file_name()
        .map(|name| name.to_string_lossy().into_owned())
        .unwrap_or_else(|| path.display().to_string());
    let mut entries = Vec::new();
    list_folder(path, "", &mut entries)?;
    let images_folder = path.join("images");
    if images_folder.is_dir() {
        list_folder(&images_folder, "images/", &mut entries)?;
    }
    entries.sort_by_key(|entry| entry.name.to_lowercase());
    let all_names = entries
        .iter()
        .map(|entry| entry.name.to_ascii_lowercase())
        .collect::<Vec<_>>()
        .join(" ");
    let codename_hint =
        if folder_name.to_ascii_lowercase().contains("rolex") || all_names.contains("rolex") {
            "rolex".to_string()
        } else {
            "tidak terbaca".to_string()
        };
    let scripts = entries
        .iter()
        .filter(|entry| {
            entry.name.to_ascii_lowercase().ends_with("flash_all.bat")
                || entry.name.to_ascii_lowercase().ends_with("flash_all.sh")
        })
        .map(|entry| entry.name.as_str())
        .collect::<Vec<_>>();
    let script_hint = if scripts.is_empty() {
        "skrip flash_all tidak terlihat".to_string()
    } else {
        scripts.join(", ")
    };
    Ok(RomInspection {
        folder_name,
        codename_hint,
        entries: entries.into_iter().take(150).collect(),
        script_hint,
        provenance_status: "Belum dapat dipastikan: katalog SHA-256 resmi belum tersedia.".into(),
        verified_catalog_match: false,
        warning: "SHA-256 lokal hanya sidik jari berkas. Sumber resmi, kecocokan perangkat/wilayah, dan keamanan firmware tidak dipastikan. Tidak ada berkas yang dijalankan atau dikirim ke ponsel.".into(),
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;
    use std::io::Cursor;
    use std::sync::Mutex;
    use zip::write::SimpleFileOptions;
    use zip::ZipWriter;

    #[derive(Debug, Clone, PartialEq, Eq)]
    struct RecordedCall {
        program: PathBuf,
        args: Vec<String>,
        timeout: Duration,
    }

    #[derive(Default)]
    struct MockRunner {
        calls: Mutex<Vec<RecordedCall>>,
        stdout_by_program: Mutex<HashMap<String, String>>,
    }

    impl ProcessRunner for MockRunner {
        fn run(
            &self,
            program: &Path,
            args: &[String],
            timeout: Duration,
        ) -> Result<ProcessOutput, String> {
            let name = program
                .file_name()
                .and_then(|name| name.to_str())
                .unwrap_or_default()
                .to_string();
            self.calls.lock().unwrap().push(RecordedCall {
                program: program.to_path_buf(),
                args: args.to_vec(),
                timeout,
            });
            let stdout = self
                .stdout_by_program
                .lock()
                .unwrap()
                .get(&name)
                .cloned()
                .unwrap_or_default();
            Ok(ProcessOutput {
                stdout,
                stderr: String::new(),
                code: Some(0),
            })
        }
    }

    fn test_temp_dir(label: &str) -> PathBuf {
        let unique = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        let path = env::temp_dir().join(format!("droidnow-{label}-{unique}"));
        fs::create_dir_all(&path).unwrap();
        path
    }

    fn fake_tool_directory() -> PathBuf {
        let path = test_temp_dir("mock-platform-tools");
        let suffix = if cfg!(target_os = "windows") {
            ".exe"
        } else {
            ""
        };
        for name in ["adb", "fastboot"] {
            fs::write(path.join(format!("{name}{suffix}")), b"mock executable").unwrap();
        }
        path
    }

    fn mock_tools(runner: Arc<MockRunner>) -> (ToolLocator, PathBuf) {
        let directory = fake_tool_directory();
        let mut tools = ToolLocator::with_runner(runner);
        tools.set_directory(directory.clone()).unwrap();
        (tools, directory)
    }

    fn zip_bytes(entries: &[(&str, &[u8])]) -> Vec<u8> {
        let mut writer = ZipWriter::new(Cursor::new(Vec::new()));
        for (name, contents) in entries {
            writer
                .start_file(
                    *name,
                    SimpleFileOptions::default().compression_method(zip::CompressionMethod::Stored),
                )
                .unwrap();
            writer.write_all(contents).unwrap();
        }
        writer.finish().unwrap().into_inner()
    }

    #[test]
    fn parses_adb_online_offline_and_unauthorized_devices() {
        let devices = parse_adb_devices("List of devices attached\nABC123 device product:rolex model:Redmi_4A transport_id:1\nOFF456 offline transport_id:2\nXYZ unauthorized usb:1-2\n* daemon started successfully\n");
        assert_eq!(devices.len(), 3);
        assert_eq!(devices[0].serial, "ABC123");
        assert_eq!(devices[0].state, "device");
        assert!(devices[0].details.contains("product:rolex"));
        assert_eq!(devices[1].serial, "OFF456");
        assert_eq!(devices[1].state, "offline");
        assert_eq!(devices[2].state, "unauthorized");
    }

    #[test]
    fn mock_process_runner_drives_adb_scan_without_hardware() {
        let runner = Arc::new(MockRunner::default());
        runner.stdout_by_program.lock().unwrap().insert(
            "adb".into(),
            "List of devices attached\nOFF456 offline transport_id:2\nXYZ unauthorized usb:1-2\n"
                .into(),
        );
        let (tools, directory) = mock_tools(runner.clone());
        let result = scan_devices(&tools);
        let _ = fs::remove_dir_all(&directory);

        assert_eq!(result.devices.len(), 2);
        assert_eq!(result.devices[0].state, "offline");
        assert_eq!(result.devices[1].state, "unauthorized");
        assert_eq!(runner.calls.lock().unwrap().len(), 2);
    }

    #[test]
    fn reboot_dispatch_uses_only_explicit_safe_argument_combinations() {
        let runner = Arc::new(MockRunner::default());
        let (tools, directory) = mock_tools(runner.clone());
        let cases = [
            (
                Device {
                    serial: "SERIAL-1".into(),
                    mode: "adb".into(),
                    state: "device".into(),
                    details: String::new(),
                },
                "recovery",
                "adb",
                vec!["-s", "SERIAL-1", "reboot", "recovery"],
            ),
            (
                Device {
                    serial: "SERIAL-1".into(),
                    mode: "adb".into(),
                    state: "device".into(),
                    details: String::new(),
                },
                "bootloader",
                "adb",
                vec!["-s", "SERIAL-1", "reboot", "bootloader"],
            ),
            (
                Device {
                    serial: "SERIAL-1".into(),
                    mode: "adb".into(),
                    state: "device".into(),
                    details: String::new(),
                },
                "system",
                "adb",
                vec!["-s", "SERIAL-1", "reboot"],
            ),
            (
                Device {
                    serial: "SERIAL-1".into(),
                    mode: "fastboot".into(),
                    state: "fastboot".into(),
                    details: String::new(),
                },
                "system",
                "fastboot",
                vec!["-s", "SERIAL-1", "reboot"],
            ),
        ];

        for (device, action, _, _) in &cases {
            reboot_device(device, action, &tools).unwrap();
        }
        let calls = runner.calls.lock().unwrap();
        assert_eq!(calls.len(), cases.len());
        for (call, (_, _, expected_tool, expected_args)) in calls.iter().zip(cases.iter()) {
            let expected_name = if cfg!(target_os = "windows") {
                format!("{expected_tool}.exe")
            } else {
                (*expected_tool).to_string()
            };
            assert_eq!(
                call.program.file_name().unwrap().to_string_lossy(),
                expected_name
            );
            assert_eq!(
                call.args,
                expected_args
                    .iter()
                    .map(|arg| (*arg).to_string())
                    .collect::<Vec<_>>()
            );
            assert_eq!(call.timeout, Duration::from_secs(15));
        }
        drop(calls);

        for (device, action) in [
            (
                Device {
                    serial: "SERIAL-1".into(),
                    mode: "fastboot".into(),
                    state: "fastboot".into(),
                    details: String::new(),
                },
                "recovery",
            ),
            (
                Device {
                    serial: "SERIAL-1".into(),
                    mode: "fastboot".into(),
                    state: "fastboot".into(),
                    details: String::new(),
                },
                "bootloader",
            ),
            (
                Device {
                    serial: "SERIAL-1".into(),
                    mode: "adb".into(),
                    state: "unauthorized".into(),
                    details: String::new(),
                },
                "recovery",
            ),
            (
                Device {
                    serial: "SERIAL-1".into(),
                    mode: "adb".into(),
                    state: "device".into(),
                    details: String::new(),
                },
                "erase",
            ),
            (
                Device {
                    serial: "--help".into(),
                    mode: "adb".into(),
                    state: "device".into(),
                    details: String::new(),
                },
                "system",
            ),
        ] {
            assert!(reboot_device(&device, action, &tools).is_err());
        }
        assert_eq!(runner.calls.lock().unwrap().len(), cases.len());
        let _ = fs::remove_dir_all(directory);
    }

    #[test]
    fn parses_fastboot_and_ignores_diagnostics() {
        let devices = parse_fastboot_devices(
            "ABC123\tfastboot\nfastboot: error: no devices found\nWaiting for device...\n",
        );
        assert_eq!(devices.len(), 1);
        assert_eq!(devices[0].serial, "ABC123");
        assert_eq!(devices[0].mode, "fastboot");
    }

    #[test]
    fn parses_getvar_bootloader_output() {
        let text = "(bootloader) product: rolex\n(bootloader) unlocked: no\n";
        assert_eq!(parse_getvar(text, "product"), "rolex");
        assert_eq!(parse_getvar(text, "unlocked"), "no");
        assert_eq!(parse_getvar(text, "missing"), "tidak tersedia");
    }

    #[test]
    fn validates_serial_before_platform_tools_use() {
        assert!(valid_serial("ABC:123-xyz_01"));
        assert!(!valid_serial("--help"));
        assert!(!valid_serial("serial with spaces"));
    }

    #[test]
    fn failed_or_missing_exit_code_is_not_successful_probe_evidence() {
        assert!(!tool_succeeded(&ToolOutput {
            stdout: "product: rolex".into(),
            stderr: String::new(),
            code: Some(1),
        }));
        assert!(!tool_succeeded(&ToolOutput {
            stdout: "product: rolex".into(),
            stderr: String::new(),
            code: None,
        }));
        assert!(tool_succeeded(&ToolOutput {
            stdout: "product: rolex".into(),
            stderr: String::new(),
            code: Some(0),
        }));
    }

    #[test]
    fn computes_sha256_known_vector_for_rom_file() {
        let unique = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        let path = env::temp_dir().join(format!("droidnow-sha256-{unique}.img"));
        fs::write(&path, b"abc").unwrap();
        let (size, hash) = sha256_file(&path).unwrap();
        let _ = fs::remove_file(path);
        assert_eq!(size, 3);
        assert_eq!(
            hash,
            "ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad"
        );
    }

    #[test]
    fn rom_inspection_hashes_locally_without_claiming_catalog_match() {
        let unique = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        let folder = env::temp_dir().join(format!("rolex-test-{unique}"));
        fs::create_dir_all(folder.join("images")).unwrap();
        fs::write(folder.join("images/boot.img"), b"abc").unwrap();
        let inspection = inspect_rom_folder(&folder).unwrap();
        let _ = fs::remove_dir_all(&folder);
        let boot = inspection
            .entries
            .iter()
            .find(|entry| entry.name == "images/boot.img")
            .unwrap();
        assert_eq!(boot.size_bytes, 3);
        assert_eq!(
            boot.sha256.as_deref(),
            Some("ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad")
        );
        assert!(!inspection.verified_catalog_match);
        assert!(inspection
            .provenance_status
            .starts_with("Belum dapat dipastikan"));
        assert!(inspection.warning.contains("tidak dipastikan"));
    }

    #[test]
    fn malformed_platform_tools_archive_is_rejected() {
        let root = test_temp_dir("malformed-zip");
        let stage = root.join("stage");
        fs::create_dir(&stage).unwrap();
        let error =
            extract_platform_tools_archive(Cursor::new(b"not a zip archive".to_vec()), &stage, "")
                .unwrap_err();
        assert!(error.contains("tidak valid"));
        let _ = fs::remove_dir_all(root);
    }

    #[test]
    fn path_traversal_platform_tools_archives_are_rejected_without_writing_outside_stage() {
        for malicious_name in ["../escape.txt", "platform-tools/../../escape.txt"] {
            let root = test_temp_dir("traversal-zip");
            let stage = root.join("stage");
            fs::create_dir(&stage).unwrap();
            let bytes = zip_bytes(&[(malicious_name, b"outside")]);
            let error = extract_platform_tools_archive(Cursor::new(bytes), &stage, "")
                .expect_err("archive traversal must be rejected");
            assert!(error.contains("jalur file yang tidak aman"));
            assert!(!root.join("escape.txt").exists());
            let _ = fs::remove_dir_all(root);
        }
    }
}
