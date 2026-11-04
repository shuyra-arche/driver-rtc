//! Host checks for the pure rtc server core and the declaration contract.
//!
//! The production core is included directly; no model is duplicated and no
//! syscall, device register, or volatile access is reached.

#![cfg(test)]

#[path = "../../server/src/core/mod.rs"]
mod core;

use env::PieToken;
use rtc_api::Fail;
use rtc_api::frame::{self, Wire};

use crate::core::Host;
use crate::core::host::{Answer, Ring};
use rtc_declaration::{BUILD, UNIT};

fn token(value: usize) -> PieToken {
    PieToken::mint(value)
}

fn armed(host: &mut Host, after_ns: u64, back: PieToken, now: u64) -> u64 {
    match host.ask(Wire::Arm { after_ns }, back, now) {
        Answer::Armed { at } => at,
        _ => panic!("arm should have been accepted"),
    }
}

#[test]
fn now_answers_with_the_clock_handed_to_the_core() {
    let mut host = Host::new();
    assert!(matches!(
        host.ask(Wire::Now, token(1), 41),
        Answer::Time(41)
    ));
    assert!(matches!(
        host.ask(Wire::Now, token(1), 99),
        Answer::Time(99)
    ));
}

#[test]
fn arm_lands_at_now_plus_delta_and_rings_before_exact_after() {
    let back = token(0x22);
    let at = {
        let mut host = Host::new();
        let at = armed(&mut host, 30, back, 100);
        assert_eq!(at, 130);

        assert!(matches!(host.ring(129), Ring::Quiet));
        assert!(matches!(
            host.ring(130),
            Ring::Rang { back: b, now: 130 } if b == back
        ));
        assert!(matches!(host.ring(131), Ring::Quiet));
        at
    };

    // A fresh reservation proves the delivery clock is the observed `now`,
    // not the reserved `at` value.
    let mut late = Host::new();
    assert_eq!(armed(&mut late, 30, back, 100), at);
    assert!(matches!(
        late.ring(131),
        Ring::Rang { back: b, now: 131 } if b == back
    ));
}

#[test]
fn saturation_does_not_wrap_the_reserved_instant() {
    let back = token(3);
    let mut host = Host::new();
    let at = armed(&mut host, 10, back, u64::MAX - 1);
    assert_eq!(at, u64::MAX);
    assert!(matches!(host.ring(u64::MAX - 1), Ring::Quiet));
    assert!(matches!(
        host.ring(u64::MAX),
        Ring::Rang { back: b, now: u64::MAX } if b == back
    ));
}

#[test]
fn taken_has_precedence_and_keeps_the_first_reservation() {
    let first = token(4);
    let second = token(5);
    let mut host = Host::new();
    assert_eq!(armed(&mut host, 50, first, 10), 60);

    let refused = host.ask(Wire::Arm { after_ns: 1 }, second, 20);
    match refused {
        Answer::Refused { code, at } => {
            assert_eq!(code, frame::fail_to_code(Some(Fail::Taken)));
            assert_eq!(at, 21);
        }
        _ => panic!("a second reservation must be refused as Taken"),
    }

    assert!(matches!(host.ring(59), Ring::Quiet));
    assert!(matches!(
        host.ring(60),
        Ring::Rang { back: b, now: 60 } if b == first
    ));
}

#[test]
fn zero_delta_is_past_and_leaves_the_cell_empty() {
    let mut host = Host::new();
    match host.ask(Wire::Arm { after_ns: 0 }, token(6), 70) {
        Answer::Refused { code, at } => {
            assert_eq!(at, 70);
            assert_eq!(code, frame::fail_to_code(Some(Fail::Past)));
        }
        _ => panic!("at == now must be refused as Past"),
    }

    // The failed arm did not occupy the cell.
    let back = token(7);
    assert_eq!(armed(&mut host, 1, back, 70), 71);
    assert!(matches!(
        host.ring(71),
        Ring::Rang { back: b, now: 71 } if b == back
    ));
}

#[test]
fn refused_arm_preserves_the_previous_reservation() {
    let first = token(8);
    let second = token(9);
    let mut host = Host::new();
    assert_eq!(armed(&mut host, 40, first, 0), 40);

    // The cell is occupied, so even a past instant is reported as Taken; the
    // previous token and instant remain untouched.
    assert!(matches!(
        host.ask(Wire::Arm { after_ns: 0 }, second, 0),
        Answer::Refused { code, at: 0 } if code == frame::fail_to_code(Some(Fail::Taken))
    ));
    assert!(matches!(
        host.ring(40),
        Ring::Rang { back: b, now: 40 } if b == first
    ));

    // Once empty, a past arm is refused as Past and still leaves it empty.
    assert!(matches!(
        host.ask(Wire::Arm { after_ns: 0 }, second, 50),
        Answer::Refused { code, at: 50 } if code == frame::fail_to_code(Some(Fail::Past))
    ));
    assert!(matches!(host.ring(50), Ring::Quiet));
    let third = token(10);
    assert_eq!(armed(&mut host, 2, third, 50), 52);
    assert!(matches!(
        host.ring(52),
        Ring::Rang { back: b, now: 52 } if b == third
    ));
}

#[test]
fn a_ring_is_delivered_at_most_once() {
    let back = token(11);
    let mut host = Host::new();
    assert_eq!(armed(&mut host, 5, back, 0), 5);
    assert!(matches!(
        host.ring(5),
        Ring::Rang { back: b, now: 5 } if b == back
    ));
    assert!(matches!(host.ring(5), Ring::Quiet));
    assert!(matches!(host.ring(u64::MAX), Ring::Quiet));
}

#[test]
fn hosts_are_independent_single_slots() {
    let one = token(12);
    let two = token(13);
    let mut first = Host::new();
    let mut second = Host::new();
    assert_eq!(armed(&mut first, 10, one, 0), 10);
    assert_eq!(armed(&mut second, 20, two, 0), 20);

    assert!(matches!(
        first.ring(10),
        Ring::Rang { back: b, now: 10 } if b == one
    ));
    assert!(matches!(second.ring(10), Ring::Quiet));
    assert!(matches!(
        second.ring(20),
        Ring::Rang { back: b, now: 20 } if b == two
    ));
}

#[test]
fn heard_counts_only_explicit_calls() {
    let mut host = Host::new();
    assert_eq!(host.heard(), 1);
    assert_eq!(host.heard(), 2);

    let back = token(14);
    assert_eq!(armed(&mut host, 1, back, 0), 1);
    assert!(matches!(
        host.ring(1),
        Ring::Rang { back: b, now: 1 } if b == back
    ));
    // A ring does not increment the delivered count by itself.
    assert_eq!(host.heard(), 3);
}

fn bootstrap_operator(dependency: &unit::Dependency) -> bool {
    matches!(
        dependency,
        unit::Dependency::Bootstrap(unit::Bootstrap::Operator)
    )
}

fn named_unit(dependency: &unit::Dependency, expected: &str) -> bool {
    matches!(dependency, unit::Dependency::Unit(name) if *name == expected)
}

#[test]
fn compiled_declaration_and_build_keep_the_rtc_unit_contract() {
    assert_eq!(UNIT.name(), "rtc");
    assert_eq!(UNIT.loader.image, "rtc");
    assert!(matches!(UNIT.loader.space, env::ProgramKind::User));
    assert!(UNIT.loader.args.is_empty());
    assert_eq!(UNIT.loader.stack, 0);

    let namespaces = UNIT.operator.namespace;
    assert_eq!(namespaces.len(), 1);
    let ns = &namespaces[0];
    assert_eq!(ns.key.name(), system_api::namespace::SERVICE.name());
    assert!(matches!(ns.placement, unit::operator::Placement::Shared));
    assert_eq!(ns.path, "rtc");
    assert!(matches!(ns.scope, unit::operator::Scope::Subtree));
    let expected_ops = system_api::operator::Ops::PART
        .union(system_api::operator::Ops::UNPART)
        .union(system_api::operator::Ops::BIND)
        .union(system_api::operator::Ops::UNBIND)
        .union(system_api::operator::Ops::FIND);
    assert_eq!(ns.ops, expected_ops);
    assert!(matches!(
        ns.permit,
        Some(system_api::operator::Permit::Public)
    ));
    assert!(ns.source_mark.is_none());
    assert_eq!(ns.bindings.len(), 1);
    let binding = &ns.bindings[0];
    assert_eq!(binding.name, rtc_api::ENTRY_NAME);
    assert_eq!(binding.mark, router_api::ENTRY_MARK);
    assert_eq!(binding.view, env::Access::STORE);
    assert!(matches!(
        binding.permit,
        system_api::operator::Permit::Public
    ));

    assert!(UNIT.identity.aliases.is_empty());
    assert!(UNIT.identity.grants.is_empty());
    assert!(UNIT.control.supervision.is_none());
    let after = UNIT.control.after.expect("rtc declares its startup order");
    assert_eq!(after.len(), 3);
    assert!(bootstrap_operator(&after[0]));
    assert!(named_unit(&after[1], "hub"));
    assert!(named_unit(&after[2], "router"));
    assert_eq!(UNIT.control.ready, unit::Ready::Signal(unit::READY_MARK));
    assert_eq!(UNIT.control.ending, Some(unit::Ending::Resident));
    assert!(UNIT.control.construction.is_empty());
    assert!(UNIT.control.retirement.is_empty());
    assert!(UNIT.supply().is_empty());

    assert_eq!(BUILD.image, "rtc");
    assert_eq!(BUILD.manifest, "server/Cargo.toml");
    assert_eq!(BUILD.package, "rtc-server");
    assert_eq!(BUILD.bin, "rtc");
    assert!(BUILD.features.is_empty());
    assert_eq!(BUILD.target, "riscv64gc-unknown-none-elf");
    assert_eq!(BUILD.profile, "release");
    assert!(BUILD.default_features);
}
