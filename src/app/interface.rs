use serde::Serialize;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub enum AppCommandEnum {
    #[serde(rename = "0")]
    None,
    #[serde(rename = "e")]
    EnterSkinSelectMode,
    #[serde(rename = "l")]
    LeaveSkinSelectMode,
    #[serde(rename = "n")]
    SelectNextSkin,
    #[serde(rename = "p")]
    SelectPrevSkin,
}
