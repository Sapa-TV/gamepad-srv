#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AppActionEnum {
    None,
    EnterSkinSelectMode,
    LeaveSkinSelectMode,
    SelectNextSkin,
    SelectPrevSkin,
}
