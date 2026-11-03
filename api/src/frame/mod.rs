//! 两句话、两种答形（一张字段表就是一处定义）。
//! **四张表、两个方向**：Now / Arm 是问的两形，Time / Status 是答的两形——偏移与
//! 长度全部由字段宽度求和得出（`#[derive(env::Frame)]` 那一处定义），手写的那五枚自由函数
//! （`pack_ask` / `pack_arm` / `unpack_ask` / `pack_time` / `unpack_time`）与那四个长度常量

use env::Mark;

/// 问那一句的动作码：「现在几点」
pub const ASK: u8 = 1;

/// 问那一句的动作码：「在 `at` 叫我」
pub const ARM: u8 = 2;

pub use crate::{ENTRY_NAME, ENTRY_PATH};

/// 回信孔的记号：客人每趟铸一枚、借给驱动（**收方按它验那一格**）
pub const BACK: Mark = Mark::of("rtc-back");

/// 失败域与答话那一格**一处编**：三个码与两向读法由 mold::WireCodes 从
pub use super::fail::{BAD, PAST, TAKEN, code_to_fail, fail_to_code};
/// 答话那一格：收下了——**全协议那一个"没失败"**（wire::OK），本族不再写第二遍
pub use ::wire::OK;

pub mod arm;
pub mod now;
pub mod status;
pub mod time;
pub mod wire;

pub use arm::Arm;
pub use now::Now;
pub use status::Status;
pub use time::Time;
pub use wire::Wire;
