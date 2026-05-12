use serde::{Deserialize, Serialize};
use std::fs;

pub const SKIN_DIR: &str = "assets/skins";

const VALID_BUTTONS: &[&str] = &[
    "DPadUp",
    "DPadDown",
    "DPadLeft",
    "DPadRight",
    "South",
    "East",
    "West",
    "North",
    "LeftShoulder",
    "RightShoulder",
    "LeftTrigger",
    "RightTrigger",
    "LeftStick",
    "RightStick",
    "LeftStickPressed",
    "RightStickPressed",
    "Select",
    "Start",
    "Menu",
];

#[derive(Clone, Serialize)]
pub struct SkinEntry {
    pub name: String,
    pub path: String,
}

#[derive(Clone, Serialize)]
pub struct SkinInfo {
    pub name: String,
    pub path: String,
}

#[derive(Deserialize)]
struct SkinJson {
    name: String,
    background: JsonMedia,
    indicator: Option<JsonMedia>,
    buttons: Vec<JsonButton>,
}

#[derive(Deserialize)]
struct JsonMedia {
    image: String,
    #[allow(dead_code)]
    top: Option<i64>,
    #[allow(dead_code)]
    left: Option<i64>,
}

#[derive(Deserialize)]
struct JsonButton {
    name: String,
    #[allow(dead_code)]
    top: i64,
    #[allow(dead_code)]
    left: i64,
    image: String,
}

fn button_name_valid(name: &str) -> bool {
    VALID_BUTTONS.contains(&name)
}

pub fn validate_skin(skin_name: &str) -> Result<(), String> {
    let skin_path = format!("{}/{}/skin.json", SKIN_DIR, skin_name);
    let contents = fs::read_to_string(&skin_path)
        .map_err(|e| format!("Failed to read {}: {}", skin_path, e))?;

    let json: SkinJson = serde_json::from_str(&contents)
        .map_err(|e| format!("Failed to parse {}: {}", skin_path, e))?;

    if json.name.is_empty() {
        return Err(format!("{}: 'name' is empty", skin_path));
    }

    let bg_path = format!("{}/{}/{}", SKIN_DIR, skin_name, json.background.image);
    fs::metadata(&bg_path)
        .map_err(|e| format!("Background image not found {}: {}", bg_path, e))?;

    if let Some(ind) = &json.indicator {
        let ind_path = format!("{}/{}/{}", SKIN_DIR, skin_name, ind.image);
        fs::metadata(&ind_path)
            .map_err(|e| format!("Indicator image not found {}: {}", ind_path, e))?;
    }

    for btn in &json.buttons {
        if !button_name_valid(&btn.name) {
            return Err(format!("Invalid button name: {}", btn.name));
        }
        let btn_path = format!("{}/{}/{}", SKIN_DIR, skin_name, btn.image);
        fs::metadata(&btn_path)
            .map_err(|e| format!("Button image not found {}: {}", btn_path, e))?;
    }

    Ok(())
}

pub fn load_skin_info(skin_name: &str) -> Result<SkinInfo, String> {
    validate_skin(skin_name)?;

    let skin_path = format!("{}/{}/skin.json", SKIN_DIR, skin_name);
    eprintln!("load_skin_info: reading {}", skin_path);
    let contents = fs::read_to_string(&skin_path)
        .map_err(|e| format!("Failed to read {}: {}", skin_path, e))?;

    let json: serde_json::Value = serde_json::from_str(&contents)
        .map_err(|e| format!("Failed to parse {}: {}", skin_path, e))?;

    let name = json
        .get("name")
        .and_then(|v| v.as_str())
        .ok_or_else(|| format!("{} missing 'name' field", skin_path))?
        .to_string();

    Ok(SkinInfo {
        name,
        path: format!("/skins/{}/", skin_name),
    })
}

pub fn discover_skins() -> Vec<SkinEntry> {
    let mut skins = Vec::new();

    let entries = match fs::read_dir(SKIN_DIR) {
        Ok(e) => e,
        Err(_) => return skins,
    };

    for entry in entries.flatten() {
        let path = entry.path();
        if path.is_dir() {
            if let Some(dir_name) = path.file_name().and_then(|n| n.to_str()) {
                eprintln!("discover_skins: checking dir {}", dir_name);
                let skin_json_path = format!("{}/{}/skin.json", SKIN_DIR, dir_name);
                if fs::metadata(&skin_json_path).is_ok() {
                    eprintln!("discover_skins: found skin.json, validating {}", dir_name);
                    let result = load_skin_info(dir_name);
                    if let Ok(info) = result {
                        eprintln!("discover_skins: {} is valid, path={}", info.name, info.path);
                        skins.push(SkinEntry {
                            name: info.name,
                            path: info.path,
                        });
                    } else {
                        eprintln!("Skin validation failed for {}: {:?}", dir_name, result.err());
                    }
                }
            }
        }
    }

    skins.sort_by(|a, b| a.name.to_lowercase().cmp(&b.name.to_lowercase()));
    skins
}