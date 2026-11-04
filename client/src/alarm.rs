//! 一次**约**：那一格里收着的，就是它

use env::Wait;
use ipc::hand::Receiver;
use wire::Message;

use rtc_api::frame::Time;
use ::resource::raw::Hole;

/// 一次**约**：那一格里收着的，就是它
pub struct Alarm {
    pub(super) back: Hole,
}

impl Alarm {
    /// 等那一声。返**响的那一刻**（驱动听见闹钟时读到的计数）
    /// **无界等**：客人只有这一件事，而对面一没，这一枚孔就封印 ⇒ 当场答 `Err(())`
    /// 不是永久挂住（寿命边随它的**开者**——这一枚是客人自己铸的）
    pub fn receive(&self) -> Result<u64, ()> {
        let mut buf = Time::EMPTY;
        Receiver::<Time>::from_raw(self.back.token())
            .recv(buf.as_mut(), Wait::Forever)
            .map_err(|_| ())
    }
}
