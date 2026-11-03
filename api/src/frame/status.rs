use env::wire::Span as _;
use wire::Message;

/// **答那一形 · 一个答码**：收下了没有（OK / TAKEN / PAST / BAD）
/// 与板 / 树那两族的 1 字节答**同名同位**（`Status`）：一格状态、没有荷载
#[derive(env::Frame, Clone, Copy, PartialEq, Eq, Debug)]
pub struct Status {
    pub status: u8,
}

impl Status {
    /// 编一答：一个答码
    pub const fn of(code: u8) -> Status {
        Status { status: code }
    }
}

impl Message for Status {
    /// 解开之后就是**那一格答码**
    type In = u8;
    type Buf = [u8; Status::LEN];
    const EMPTY: Self::Buf = [0u8; Status::LEN];

    fn store(&self, out: &mut [u8]) -> Option<usize> {
        Status::store_at(self, out, 0)
    }

    /// **恰好 1 字节**
    fn fetch(bytes: &[u8]) -> Option<u8> {
        if bytes.len() != Status::LEN {
            return None;
        }
        Some(Status::fetch_at(bytes, 0)?.0.status)
    }
}
