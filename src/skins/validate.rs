use serde::Deserialize;
use std::{fs, path::Path};
use strum::{EnumMessage, IntoEnumIterator};

use crate::{
    error::{AppError, AppResult},
    gamepad::buttons::ButtonEnum,
};

#[derive(Deserialize)]
struct SkinJson {
    name: String,
    body: JsonMedia,
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

fn check_btn_name(button: &str) -> AppResult<()> {
    ButtonEnum::iter()
        .any(|btn| btn.get_message() == Some(button))
        .ok_or(AppError::Skin(format!("Button name invalid: {}", button)))
}

fn check_file(path: &Path) -> AppResult<()> {
    fs::metadata(&path)
        .map_err(|err| AppError::Skin(format!("File not found {}: {}", path.display(), err)))?;
    Ok(())
}

pub fn validate_get_name(path: &Path) -> AppResult<String> {
    let json_path = path.join("skin.json");

    let contents = fs::read_to_string(&json_path)
        .map_err(|err| AppError::Skin(format!("Error read skin.json: {err}")))?;

    let json: SkinJson = serde_json::from_str(&contents)
        .map_err(|err| AppError::Skin(format!("Failed to parse: {}", err)))?;

    if json.name.is_empty() {
        return Err(AppError::Skin(format!(
            "{}: field 'name' is empty",
            json_path.display()
        )));
    }

    check_file(&path.join(json.body.image))?;

    if let Some(ind) = json.indicator {
        check_file(&path.join(ind.image))?;
    }

    for btn in json.buttons {
        check_btn_name(&btn.name)?;
        check_file(&path.join(btn.image))?;
    }

    Ok(json.name)
}
