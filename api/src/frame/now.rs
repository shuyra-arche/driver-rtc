use env::PieToken;

use super::ASK;

/// **问那一形 · 「现在几点」**：动作码 ＋ 那一格
/// 动作码由 Now::of 钉进来（表那一格是裸字节，是构造那一手保证的）
#[derive(env::Frame, Clone, Copy, PartialEq, Eq, Debug)]
pub struct Now {
    pub op: u8,
    pub back: PieToken,
}

impl Now {
    /// 编一问：`back` = "我借给你的那枚回信孔**在你表里**是几号"
    pub fn of(back: PieToken) -> Now {
        Now { op: ASK, back }
    }
}
