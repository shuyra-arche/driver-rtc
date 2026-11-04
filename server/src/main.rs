#![no_std]
#![no_main]

//! rtc 服务的外部入口；流程与判定都在 `rtc_server::serve`。

#[mold::entry]
fn main() -> Result<(), driver_shared::fail::Fail> {
    rtc_server::serve()
}
