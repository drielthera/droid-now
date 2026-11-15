use crate::core::RomInspection;
use serde::Serialize;
use std::fs;
use std::io::Read;
use std::path::Path;

const MAX_ANDROID_INFO_SIZE: u64 = 1024 * 1024;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum RomCheckStatus {
    Mismatch,
    CandidateMatch,
    InsufficientEvidence,
}

#[derive(Debug, Clone, Serialize)]
pub struct RomPackageCheck {
    pub status: RomCheckStatus,
    pub device_codename: Option<String>,
    pub package_codename: Option<String>,
    pub codename_matches: Option<bool>,
    pub device_region: Option<String>,
    pub package_region: Option<String>,
    pub region_matches: Option<bool>,
    pub writes_allowed: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PackageHints {
    pub codename: Option<String>,
    pub region: Option<String>,
    pub conflicting_codename_sources: bool,
}

pub fn inspect_package_hints(path: &Path, folder_name: &str) -> PackageHints {
    let (name_codename, name_region) = hints_from_folder_name(folder_name);
    let (metadata_codename, conflicting_metadata) = match read_android_info_codename(path) {
        Ok(codename) => (codename, false),
        Err(()) => (None, true),
    };
    let conflicting_codename_sources = conflicting_metadata
        || match (&name_codename, &metadata_codename) {
            (Some(name), Some(metadata)) => !name.eq_ignore_ascii_case(metadata),
            _ => false,
        };
    let codename = if conflicting_codename_sources {
        None
    } else {
        metadata_codename.or(name_codename)
    };
    PackageHints {
        codename,
        region: name_region,
        conflicting_codename_sources,
    }
}

pub fn check_package(device_info: Option<&str>, rom: &RomInspection) -> RomPackageCheck {
    let (device_codename, device_region) = device_hints(device_info);
    let package_codename = normalize_codename(&rom.codename_hint);
    let package_region = normalize_region(&rom.region_hint);
    let codename_matches = pair_match(device_codename.as_deref(), package_codename.as_deref());
    let region_matches = pair_match(device_region.as_deref(), package_region.as_deref());

    let mismatch = rom.package_identity_conflict
        || codename_matches == Some(false)
        || region_matches == Some(false);
    let status = if mismatch {
        RomCheckStatus::Mismatch
    } else if codename_matches == Some(true) && region_matches == Some(true) {
        RomCheckStatus::CandidateMatch
    } else {
        RomCheckStatus::InsufficientEvidence
    };

    RomPackageCheck {
        status,
        device_codename,
        package_codename,
        codename_matches,
        device_region,
        package_region,
        region_matches,
        // This preflight is informational only; it never authorizes a write.
        writes_allowed: false,
    }
}

fn pair_match(left: Option<&str>, right: Option<&str>) -> Option<bool> {
    match (left, right) {
        (Some(left), Some(right)) => Some(left.eq_ignore_ascii_case(right)),
        _ => None,
    }
}

fn device_hints(info: Option<&str>) -> (Option<String>, Option<String>) {
    let mut codenames = Vec::new();
    let mut regions = Vec::new();
    if let Some(info) = info {
        for line in info.lines() {
            let Some((label, raw_value)) = line.split_once(':') else {
                continue;
            };
            let label = label.trim().to_ascii_lowercase();
            let value = raw_value.trim();
            if matches!(label.as_str(), "codename" | "product") {
                if let Some(value) = normalize_codename(value) {
                    codenames.push(value);
                }
            } else if matches!(
                label.as_str(),
                "vendor region" | "miui region" | "wilayah vendor" | "region / variant" | "build"
            ) {
                if let Some(value) = normalize_region(value) {
                    regions.push(value);
                }
            }
        }
    }
    (unique_hint(codenames), unique_hint(regions))
}

fn unique_hint(values: Vec<String>) -> Option<String> {
    let mut unique = values;
    unique.sort();
    unique.dedup();
    (unique.len() == 1).then(|| unique.remove(0))
}

fn normalize_codename(value: &str) -> Option<String> {
    let value = value.trim();
    let lower = value.to_ascii_lowercase();
    if value.is_empty()
        || [
            "tidak terbaca",
            "tidak tersedia",
            "unknown",
            "unreadable",
            "not available",
        ]
        .iter()
        .any(|marker| lower.contains(marker))
    {
        return None;
    }
    let normalized = value
        .chars()
        .filter(|character| character.is_ascii_alphanumeric())
        .flat_map(char::to_lowercase)
        .collect::<String>();
    (!normalized.is_empty()).then_some(normalized)
}

fn normalize_region(value: &str) -> Option<String> {
    let compact = value
        .trim()
        .chars()
        .filter(|character| character.is_ascii_alphanumeric())
        .map(|character| character.to_ascii_uppercase())
        .collect::<String>();
    if compact.is_empty()
        || [
            "TIDAKTERBACA",
            "TIDAKTERSEDIA",
            "UNKNOWN",
            "UNREADABLE",
            "NOTAVAILABLE",
        ]
        .iter()
        .any(|marker| compact.contains(marker))
    {
        return None;
    }
    let region = match compact.as_str() {
        "GLOBAL" | "MI" | "MIXM" => "global",
        "EEA" | "EU" | "EUXM" => "eea",
        "CN" | "CHINA" | "CNXM" => "china",
        "IN" | "INDIA" | "INXM" => "india",
        "ID" | "INDONESIA" | "IDXM" => "indonesia",
        "RU" | "RUSSIA" | "RUXM" => "russia",
        "TR" | "TURKEY" | "TRXM" => "turkey",
        "TW" | "TAIWAN" | "TWXM" => "taiwan",
        "JP" | "JAPAN" | "JPXM" => "japan",
        "KR" | "KOREA" | "KRXM" => "korea",
        "BR" | "BRAZIL" | "BRXM" => "brazil",
        "MX" | "MEXICO" | "MXXM" => "mexico",
        _ => {
            let region = [
                ("MIXM", "global"),
                ("EUXM", "eea"),
                ("CNXM", "china"),
                ("INXM", "india"),
                ("IDXM", "indonesia"),
                ("RUXM", "russia"),
                ("TRXM", "turkey"),
                ("TWXM", "taiwan"),
                ("JPXM", "japan"),
                ("KRXM", "korea"),
                ("BRXM", "brazil"),
                ("MXXM", "mexico"),
            ]
            .iter()
            .find_map(|(suffix, region)| compact.ends_with(suffix).then_some(*region))?;
            region
        }
    };
    Some(region.into())
}

fn hints_from_folder_name(folder_name: &str) -> (Option<String>, Option<String>) {
    let tokens = folder_name
        .to_ascii_lowercase()
        .split(|character: char| !character.is_ascii_alphanumeric())
        .filter(|part| !part.is_empty())
        .map(str::to_owned)
        .collect::<Vec<_>>();
    if tokens.is_empty() {
        return (None, None);
    }

    let mut candidate_index = None;
    if tokens.first().is_some_and(|token| token == "miui") && tokens.len() > 1 {
        candidate_index = Some(1);
    } else if tokens.len() > 3 && tokens[0] == "xiaomi" && tokens[1] == "eu" && tokens[2] == "multi"
    {
        candidate_index = Some(3);
    }

    let mut region = None;
    let mut region_index = None;
    for (index, token) in tokens.iter().enumerate() {
        if tokens.len() > 3
            && tokens[0] == "xiaomi"
            && tokens[1] == "eu"
            && tokens[2] == "multi"
            && index == 1
        {
            continue;
        }
        if let Some((codename, found_region)) = split_attached_region(token) {
            if !codename.is_empty() {
                candidate_index.get_or_insert(index);
                region = Some(found_region);
                region_index = Some(index);
                break;
            }
        }
        if let Some(found_region) = region_from_package_token(token) {
            // Region abbreviations are accepted only in an explicit package-name position.
            if index > 0
                || matches!(
                    token.as_str(),
                    "global" | "eea" | "china" | "india" | "indonesia"
                )
            {
                region = Some(found_region);
                region_index = Some(index);
                break;
            }
        }
    }
    if candidate_index.is_none() {
        if let Some(index) = region_index.filter(|index| *index > 0) {
            candidate_index = Some(index - 1);
        } else if let Some(images_index) = tokens.iter().position(|token| token == "images") {
            if images_index > 0 {
                candidate_index = Some(images_index - 1);
            }
        } else if tokens.iter().any(|token| token == "rolex") {
            // Preserve the original, explicit codename hint for the project's initial target.
            candidate_index = tokens.iter().position(|token| token == "rolex");
        }
    }

    let mut codename = candidate_index.and_then(|index| tokens.get(index).cloned());
    if let Some(value) = codename.as_deref() {
        if let Some((stripped, attached_region)) = split_attached_region(value) {
            if !stripped.is_empty() {
                codename = Some(stripped);
                region.get_or_insert(attached_region);
            }
        }
    }
    let codename = codename.and_then(|value| normalize_codename(&value));
    (codename, region)
}

fn split_attached_region(token: &str) -> Option<(String, String)> {
    for (suffix, region) in [
        ("indonesia", "indonesia"),
        ("global", "global"),
        ("china", "china"),
        ("india", "india"),
        ("russia", "russia"),
        ("turkey", "turkey"),
        ("taiwan", "taiwan"),
        ("eea", "eea"),
    ] {
        if token.len() > suffix.len() && token.ends_with(suffix) {
            return Some((
                token[..token.len() - suffix.len()].to_owned(),
                region.into(),
            ));
        }
    }
    None
}

fn region_from_package_token(token: &str) -> Option<String> {
    match token {
        "global" | "mi" | "mixm" => Some("global".into()),
        "eea" | "eu" | "euxm" => Some("eea".into()),
        "china" | "cn" | "cnxm" => Some("china".into()),
        "india" | "in" | "inxm" => Some("india".into()),
        "indonesia" | "id" | "idxm" => Some("indonesia".into()),
        "russia" | "ru" | "ruxm" => Some("russia".into()),
        "turkey" | "tr" | "trxm" => Some("turkey".into()),
        "taiwan" | "tw" | "twxm" => Some("taiwan".into()),
        "japan" | "jp" | "jpxm" => Some("japan".into()),
        "korea" | "kr" | "krxm" => Some("korea".into()),
        "brazil" | "br" | "brxm" => Some("brazil".into()),
        "mexico" | "mx" | "mxxm" => Some("mexico".into()),
        _ => None,
    }
}

fn read_android_info_codename(folder: &Path) -> Result<Option<String>, ()> {
    let mut values = Vec::new();
    for relative in ["android-info.txt", "images/android-info.txt"] {
        let path = folder.join(relative);
        let Ok(metadata) = fs::symlink_metadata(&path) else {
            continue;
        };
        if !metadata.file_type().is_file() || metadata.len() > MAX_ANDROID_INFO_SIZE {
            continue;
        }
        let Ok(file) = fs::File::open(path) else {
            continue;
        };
        let mut contents = String::new();
        if file
            .take(MAX_ANDROID_INFO_SIZE + 1)
            .read_to_string(&mut contents)
            .is_err()
            || contents.len() as u64 > MAX_ANDROID_INFO_SIZE
        {
            continue;
        }
        for line in contents.lines() {
            let line = line.trim().trim_start_matches('#').trim();
            let line = line.strip_prefix("require ").unwrap_or(line);
            let Some((key, value)) = line.split_once('=').or_else(|| line.split_once(':')) else {
                continue;
            };
            if key.trim().eq_ignore_ascii_case("product") {
                let value = value.split('#').next().unwrap_or(value).trim();
                if value.is_empty()
                    || !value.chars().all(|character| {
                        character.is_ascii_alphanumeric() || "._-".contains(character)
                    })
                {
                    return Err(());
                }
                values.push(normalize_codename(value).ok_or(())?);
            }
        }
    }
    if values.is_empty() {
        Ok(None)
    } else {
        unique_hint(values).ok_or(()).map(Some)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::RomInspection;
    use std::time::{SystemTime, UNIX_EPOCH};

    fn inspection(codename: &str, region: &str, conflict: bool) -> RomInspection {
        RomInspection {
            folder_name: "miui_rolexGlobal_V1".into(),
            codename_hint: codename.into(),
            region_hint: region.into(),
            package_identity_conflict: conflict,
            entries: vec![],
            script_hint: "not detected".into(),
            provenance_status: "unverified".into(),
            verified_catalog_match: false,
            warning: "local hash only".into(),
        }
    }

    fn temp_dir(label: &str) -> std::path::PathBuf {
        let id = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        let path = std::env::temp_dir().join(format!("droidnow-rom-{label}-{id}"));
        fs::create_dir_all(&path).unwrap();
        path
    }

    #[test]
    fn parses_standard_xiaomi_folder_codename_and_region_hints() {
        for (name, codename, region) in [
            (
                "miui_MUNCHGlobal_V14.0.4.0.TMCMIXM",
                "munch",
                Some("global"),
            ),
            ("miui_rolex_China_images_V12", "rolex", Some("china")),
            ("munch_global_images_V1", "munch", Some("global")),
            ("xiaomi.eu_multi_rolex_2025", "rolex", None),
        ] {
            let (actual_codename, actual_region) = hints_from_folder_name(name);
            assert_eq!(actual_codename.as_deref(), Some(codename), "{name}");
            assert_eq!(actual_region.as_deref(), region, "{name}");
        }
    }

    #[test]
    fn matching_codename_and_region_is_only_a_candidate_and_never_allows_writing() {
        let rom = inspection("rolex", "Global", false);
        let check = check_package(Some("product: rolex\nregion / variant: global"), &rom);
        assert_eq!(check.status, RomCheckStatus::CandidateMatch);
        assert_eq!(check.codename_matches, Some(true));
        assert_eq!(check.region_matches, Some(true));
        assert!(!check.writes_allowed);
    }

    #[test]
    fn different_codename_or_region_is_blocked_even_when_other_evidence_matches() {
        let code_mismatch = check_package(
            Some("product: munch\nregion / variant: Global"),
            &inspection("rolex", "global", false),
        );
        assert_eq!(code_mismatch.status, RomCheckStatus::Mismatch);
        assert_eq!(code_mismatch.codename_matches, Some(false));
        assert!(!code_mismatch.writes_allowed);

        let region_mismatch = check_package(
            Some("codename: rolex\nVendor region: ID"),
            &inspection("rolex", "Global", false),
        );
        assert_eq!(region_mismatch.status, RomCheckStatus::Mismatch);
        assert_eq!(region_mismatch.region_matches, Some(false));
    }

    #[test]
    fn missing_or_conflicting_evidence_never_becomes_a_match() {
        let missing = check_package(None, &inspection("rolex", "global", false));
        assert_eq!(missing.status, RomCheckStatus::InsufficientEvidence);
        assert!(!missing.writes_allowed);

        let conflict = check_package(
            Some("codename: rolex\nVendor region: Global"),
            &inspection("rolex", "global", true),
        );
        assert_eq!(conflict.status, RomCheckStatus::Mismatch);
    }

    #[test]
    fn explicit_android_info_product_is_read_without_executing_it() {
        let folder = temp_dir("android-info");
        fs::write(folder.join("android-info.txt"), "require product=rolex\n").unwrap();
        let hints = inspect_package_hints(&folder, "miui_rolexGlobal_V1");
        assert_eq!(hints.codename.as_deref(), Some("rolex"));
        assert_eq!(hints.region.as_deref(), Some("global"));
        assert!(!hints.conflicting_codename_sources);
        let _ = fs::remove_dir_all(folder);
    }

    #[test]
    fn conflicting_folder_and_metadata_codename_is_reported_as_conflict() {
        let folder = temp_dir("conflict");
        fs::write(folder.join("android-info.txt"), "require product=munch\n").unwrap();
        let hints = inspect_package_hints(&folder, "miui_rolexGlobal_V1");
        assert!(hints.conflicting_codename_sources);
        assert!(hints.codename.is_none());
        let _ = fs::remove_dir_all(folder);
    }

    #[test]
    fn ambiguous_android_info_products_are_a_conflict() {
        let folder = temp_dir("ambiguous");
        fs::write(
            folder.join("android-info.txt"),
            "require product=rolex\nrequire product=munch\n",
        )
        .unwrap();
        let hints = inspect_package_hints(&folder, "miui_rolexGlobal_V1");
        assert!(hints.conflicting_codename_sources);
        assert!(hints.codename.is_none());
        let _ = fs::remove_dir_all(folder);
    }
}
