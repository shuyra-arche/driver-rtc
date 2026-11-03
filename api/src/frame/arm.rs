use env::PieToken;

use super::ARM;

/// **问那一形 · 「再过多 long 叫我」**：动作码 ＋ 那一格 ＋ **一个相对量**
#[derive(env::Frame, Clone, Copy, PartialEq, Eq, Debug)]
pub struct Arm {
    pub op: u8,
    pub back: PieToken,
    pub after_ns: u64,
}

impl Arm {
    /// 编一问：`after_ns` 是**相对量**（纳秒）——绝对时刻由收帧的人算（见 Wire::Arm）
    pub fn of(back: PieToken, after_ns: u64) -> Arm {
        Arm {
            op: ARM,
            back,
            after_ns,
        }
    }
}
