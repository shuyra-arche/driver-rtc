#![no_std]

//! 实时钟驱动（rtc）的装配声明。
//! **U 态**：持有 `rtc@101000`（11 号线），武装闹钟、到点自己拉线；客人定的闹钟到点就清掉
//! 那一格、把"那一声"推回去。它是"抽象等第二个实例"的那个第二例。

use unit::{Ending, ImageBuild, UnitFile};

pub static UNIT: UnitFile = UnitFile {
    name: "rtc",
    loader: unit::LoaderConfig::user("rtc"),
    operator: unit::OperatorConfig {
        namespace: &[unit::operator::Namespace {
            key: system_api::namespace::SERVICE,
            placement: unit::operator::Placement::Shared,
            path: "rtc",
            scope: unit::operator::Scope::Subtree,
            ops: system_api::operator::Ops::PART
                .union(system_api::operator::Ops::UNPART)
                .union(system_api::operator::Ops::BIND)
                .union(system_api::operator::Ops::UNBIND)
                .union(system_api::operator::Ops::FIND),
            permit: Some(system_api::operator::Permit::Public),
            source_mark: None,
            bindings: &[unit::operator::Binding {
                name: rtc_api::ENTRY_NAME,
                mark: router_api::ENTRY_MARK,
                view: env::Access::STORE,
                permit: system_api::operator::Permit::Public,
            }],
        }],
    },
    identity: unit::Identity {
        aliases: &[],
        grants: &[],
    },
    control: unit::ControlConfig {
        supervision: None,
        after: Some(&[
            unit::Dependency::Bootstrap(unit::Bootstrap::Operator),
            unit::Dependency::Unit("hub"),
            unit::Dependency::Unit("router"),
        ]),
        ready: unit::Ready::Signal(unit::READY_MARK),
        ending: Some(Ending::Resident),

        construction: &[],
        retirement: &[],
    },
    supply: unit::SupplyConfig { bindings: &[] },
};

pub const BUILD: ImageBuild = ImageBuild {
    image: "rtc",
    manifest: "server/Cargo.toml",
    package: "rtc-server",
    bin: "rtc",
    features: &[],
    target: "riscv64gc-unknown-none-elf",
    profile: "release",
    default_features: true,
};
