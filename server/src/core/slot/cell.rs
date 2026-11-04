use env::PieToken;

/// 那一格：空着，或者武装着（到点时刻 + **往哪回**）
pub(crate) enum Cell {
    Idle,
    Armed { at: u64, back: PieToken },
}
