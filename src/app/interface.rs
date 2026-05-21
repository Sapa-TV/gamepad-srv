#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AppCommandEnum {
    None,
    EnterSkinSelectMode,
    LeaveSkinSelectMode,
    SelectNextSkin,
    SelectPrevSkin,
}
