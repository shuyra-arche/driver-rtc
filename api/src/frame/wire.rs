use env::PieToken;
use env::wire::Span as _;

use super::{ARM, ASK, Arm, Now};

/// **解出来的一问**——与板 / 树 / 名册 / 盟籍四族同一个名字同一个位置（"解出来的一问"叫 `Wire`）
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Wire {
    /// 「现在几点」
    Now,
    /// 「**再过多 long** 叫我」（**相对量**，纳秒）
    Arm { after_ns: u64 },
}

impl Wire {
    /// 解一问：返 `(那一格, 那一问)`。**不是那个形状就答 `None`**（别人往这扇门推别的东西时
    /// 不猜、不动账、也不回话——那一格读得出来也不答，因为没有可信的"往哪回"可言：动作码不认
    /// 就问不出这一帧该有多长）
    pub fn take(bytes: &[u8]) -> Option<(PieToken, Wire)> {
        match *bytes.first()? {
            ASK if bytes.len() == Now::LEN => {
                let ask = Now::fetch_at(bytes, 0)?.0;
                Some((ask.back, Wire::Now))
            }
            ARM if bytes.len() == Arm::LEN => {
                let ask = Arm::fetch_at(bytes, 0)?.0;
                Some((
                    ask.back,
                    Wire::Arm {
                        after_ns: ask.after_ns,
                    },
                ))
            }
            _ => None,
        }
    }
}
