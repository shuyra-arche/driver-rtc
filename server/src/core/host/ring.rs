use env::PieToken;

/// 一次投递：清掉设备那一格（电平源）之后，那一格到点没有
pub enum Ring {
    /// 还没到点（或者是空响）
    Quiet,
    /// 到点了：往这一枚孔推"那一声"
    Rang { back: PieToken, now: u64 },
}
