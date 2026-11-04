#![no_std]

//! rtc@101000 的持有者（设备树里那条 11 号线），兼报时服务。
//! 唯一入口是 [`serve`]；设备面、纯核与常驻面都是实现私有。

extern crate alloc;

pub(crate) use system_support::debug;

use crate::core::Host;
use dev::rtc as device;
use driver_shared::context::{Context, Step};
use driver_shared::device::{Ask, Device, Hub};
use driver_shared::fail::Fail;
use env::unit;
use env::{Access, PieKind, Policy, Wait};
use router_api::ENTRY_MARK;
use system_client::operator;

/// It dies during startup/residency here.
pub const E_RTC: env::Reason = 12;

const ASK: Ask = Ask {
    class: "google,goldfish-rtc",
    name: None,
    kind: PieKind::Pole,
    access: Access::FETCH_STORE,
    policy: Policy::ONLY,
};

/// 等板 / 等树 / 办一趟登记的总上限（毫秒）。**必须有界**
const MS: usize = 1000;

pub fn serve() -> Result<(), Fail> {
    // 解门牌 → 上板 ＋ 开会话 → 上树落门牌。门牌**公开可查**（Mine::No）：谁都能查、谁都能用。
    let entry = env::hole::unseal(ENTRY_MARK, env::HoleLimits::default())
        .map_err(|_| Fail::at(E_RTC, "tree"))?;
    let ctx = Context::open(unit::sire(), Wait::AtMost(MS)).map_err(|s| {
        Fail::at(
            E_RTC,
            match s {
                Step::Tree => "tree",
            },
        )
    })?;
    // 找设备账那两枚面 → 认领一台（类 `google,goldfish-rtc`）→ 开图 → 自证。
    let tree = operator::Face::from(&ctx.session);
    let hub = Hub::find(&tree, E_RTC, Wait::AtMost(MS))?;
    let deed = hub.claim(&tree, &ASK, E_RTC, Wait::AtMost(MS))?;
    debug!("rtc: claimed {}", deed.name.as_str());
    let dev = Device::open(deed.token).map_err(|_| Fail::at(E_RTC, "device open failed"))?;
    // 自证：那对纳秒格子读两次（两次不同 ⇒ 它是活的）。
    let (t0, t1) = (device::now(dev.view()), device::now(dev.view()));
    debug!("rtc: time {t0} -> {t1}");

    // 占线（契里那个号；"这台是哪条线"那条权威在设备账那一台）。
    let line = ctx
        .line(deed.line, Wait::AtMost(MS))
        .map_err(|_| Fail::at(E_RTC, "line"))?;
    debug!("rtc: line occupied");

    ctx.plate(entry, rtc_api::frame::ENTRY_NAME, Wait::AtMost(MS));

    // **报"答得动了"**（Setup::Ready）：牌子落了才算——装配者等它才往下起别人，于是"排在第几号"
    let _ = ipc::session::establish::lend(env::unit::sire(), ::unit::READY_MARK);

    adapt::resident::run(entry, &dev, line, &mut Host::new())
}

mod adapt;
mod core;
mod dev;
