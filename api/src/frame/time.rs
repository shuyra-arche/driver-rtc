use env::wire::Span as _;
use wire::Message;

/// **答那一形 · 一个时刻**：驱动读设备那一刻的纳秒计数（u64 LE）
#[derive(env::Frame, Clone, Copy, PartialEq, Eq, Debug)]
pub struct Time {
    pub ns: u64,
}

impl Time {
    /// 编一答：一个时刻
    pub const fn of(ns: u64) -> Time {
        Time { ns }
    }
}

impl Message for Time {
    /// 解开之后就是**那个时刻**——读的人不必再念一遍"它叫 `ns`"
    type In = u64;
    /// 定长一答（Time::LEN）
    type Buf = [u8; Time::LEN];
    const EMPTY: Self::Buf = [0u8; Time::LEN];

    /// 要的是定长数组、返 `()`，两回事
    fn store(&self, out: &mut [u8]) -> Option<usize> {
        Time::store_at(self, out, 0)
    }

    fn fetch(bytes: &[u8]) -> Option<u64> {
        if bytes.len() != Time::LEN {
            return None;
        }
        Some(Time::fetch_at(bytes, 0)?.0.ns)
    }
}
