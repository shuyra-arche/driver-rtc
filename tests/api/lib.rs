//! Host protocol-conformance checks for `rtc-api`.
//!
//! These are pure frame/mapping tests. They do not exercise a device, a
//! runtime, a client, or a server.

use env::wire::Span as _;
use env::{Mark, PieToken};
use wire::Message;

use rtc_api::frame::{
    self, ARM, ASK, Arm, BACK, BAD, Now, OK, PAST, Status, TAKEN, Time, Wire, code_to_fail,
    fail_to_code,
};
use rtc_api::{ENTRY_NAME, ENTRY_PATH, Fail};

const TOKEN_VALUE: usize = 0x0102_0304;
const TOKEN_LE: [u8; PieToken::WIDTH] = [0x04, 0x03, 0x02, 0x01, 0x00, 0x00, 0x00, 0x00];
const AFTER_NS: u64 = 0x1122_3344_5566_7788;
const AFTER_NS_LE: [u8; 8] = [0x88, 0x77, 0x66, 0x55, 0x44, 0x33, 0x22, 0x11];

fn token() -> PieToken {
    PieToken::mint(TOKEN_VALUE)
}

fn frame(len: usize, op: u8) -> Vec<u8> {
    let mut bytes = vec![0u8; len];
    if let Some(first) = bytes.first_mut() {
        *first = op;
    }
    bytes
}

#[test]
fn derived_lengths_are_the_protocol_sizes() {
    assert_eq!(Now::LEN, 9);
    assert_eq!(Arm::LEN, 17);
    assert_eq!(Time::LEN, 8);
    assert_eq!(Status::LEN, 1);
}

#[test]
fn now_is_action_byte_then_little_endian_token() {
    let now = Now::of(token());
    let mut bytes = [0u8; Now::LEN];
    assert_eq!(now.store_at(&mut bytes, 0), Some(Now::LEN));
    assert_eq!(bytes[0], ASK);
    assert_eq!(&bytes[1..], &TOKEN_LE);
    assert_eq!(
        &bytes[..],
        &[ASK, 0x04, 0x03, 0x02, 0x01, 0x00, 0x00, 0x00, 0x00]
    );

    let mut short = [0u8; Now::LEN - 1];
    assert_eq!(now.store_at(&mut short, 0), None);
}

#[test]
fn arm_is_action_byte_then_token_then_little_endian_relative_time() {
    let arm = Arm::of(token(), AFTER_NS);
    let mut bytes = [0u8; Arm::LEN];
    assert_eq!(arm.store_at(&mut bytes, 0), Some(Arm::LEN));
    assert_eq!(bytes[0], ARM);
    assert_eq!(&bytes[1..9], &TOKEN_LE);
    assert_eq!(&bytes[9..], &AFTER_NS_LE);
    assert_eq!(
        &bytes[..],
        &[
            ARM, 0x04, 0x03, 0x02, 0x01, 0x00, 0x00, 0x00, 0x00, 0x88, 0x77, 0x66, 0x55, 0x44,
            0x33, 0x22, 0x11,
        ]
    );

    let mut short = [0u8; Arm::LEN - 1];
    assert_eq!(arm.store_at(&mut short, 0), None);
}

#[test]
fn wire_take_accepts_exact_now_and_arm_frames() {
    let now = Now::of(token());
    let mut now_bytes = [0u8; Now::LEN];
    now.store_at(&mut now_bytes, 0).unwrap();
    let (back, asked) = Wire::take(&now_bytes).expect("an exact Now frame is a request");
    assert_eq!(back, token());
    assert_eq!(asked, Wire::Now);

    let arm = Arm::of(token(), AFTER_NS);
    let mut arm_bytes = [0u8; Arm::LEN];
    arm.store_at(&mut arm_bytes, 0).unwrap();
    let (back, asked) = Wire::take(&arm_bytes).expect("an exact Arm frame is a request");
    assert_eq!(back, token());
    assert_eq!(asked, Wire::Arm { after_ns: AFTER_NS });
}

#[test]
fn wire_take_rejects_empty_and_every_non_exact_length() {
    assert!(Wire::take(&[]).is_none());

    for len in [8usize, 10, 16, 18] {
        assert!(
            Wire::take(&frame(len, ASK)).is_none(),
            "ASK must refuse a {len}-byte frame"
        );
        assert!(
            Wire::take(&frame(len, ARM)).is_none(),
            "ARM must refuse a {len}-byte frame"
        );
    }

    assert!(Wire::take(&frame(17, ASK)).is_none());
    assert!(Wire::take(&frame(9, ARM)).is_none());
}

#[test]
fn wire_take_rejects_unknown_action_codes() {
    assert_eq!(ASK, 1);
    assert_eq!(ARM, 2);

    for op in [0u8, 3, 4, u8::MAX] {
        assert!(
            Wire::take(&frame(Now::LEN, op)).is_none(),
            "unknown action code {op} at Now length"
        );
        assert!(
            Wire::take(&frame(Arm::LEN, op)).is_none(),
            "unknown action code {op} at Arm length"
        );
    }
}

#[test]
fn time_fetch_requires_exactly_eight_bytes() {
    let encoded = AFTER_NS.to_le_bytes();
    assert_eq!(Time::of(AFTER_NS).ns, AFTER_NS);
    assert_eq!(Time::fetch(&encoded), Some(AFTER_NS));
    assert_eq!(Time::fetch(&encoded[..7]), None);
    assert_eq!(Time::fetch(&[0u8; 9]), None);
    assert_eq!(Time::fetch(&[]), None);
}

#[test]
fn status_fetch_requires_exactly_one_byte() {
    assert_eq!(Status::of(PAST).status, PAST);
    assert_eq!(Status::fetch(&[PAST]), Some(PAST));
    assert_eq!(Status::fetch(&[]), None);
    assert_eq!(Status::fetch(&[PAST, OK]), None);
}

#[test]
fn failure_codes_map_both_ways_and_unknown_codes_are_none() {
    assert_eq!(OK, 0);
    assert_eq!(TAKEN, 1);
    assert_eq!(PAST, 2);
    assert_eq!(BAD, 3);

    assert_eq!(fail_to_code(None), OK);
    assert_eq!(fail_to_code(Some(Fail::Taken)), TAKEN);
    assert_eq!(fail_to_code(Some(Fail::Past)), PAST);
    assert_eq!(fail_to_code(Some(Fail::Denied)), BAD);

    assert_eq!(code_to_fail(OK), None);
    assert_eq!(code_to_fail(TAKEN), Some(Fail::Taken));
    assert_eq!(code_to_fail(PAST), Some(Fail::Past));
    assert_eq!(code_to_fail(BAD), Some(Fail::Denied));
    assert_eq!(code_to_fail(4), None);
    assert_eq!(code_to_fail(u8::MAX), None);
}

#[test]
fn entry_coordinates_and_back_mark_are_the_existing_names() {
    assert_eq!(ENTRY_NAME, "entry");
    assert_eq!(ENTRY_PATH.as_str(), "rtc/entry");
    assert_eq!(frame::ENTRY_NAME, ENTRY_NAME);
    assert_eq!(frame::ENTRY_PATH, ENTRY_PATH);

    assert_eq!(BACK, Mark::of("rtc-back"));
    assert_ne!(BACK, Mark::NONE);
}

#[test]
fn api_source_has_no_server_device_or_shared_runtime_ownership() {
    let source = [
        include_str!("../../api/src/lib.rs"),
        include_str!("../../api/src/fail.rs"),
        include_str!("../../api/src/frame/mod.rs"),
        include_str!("../../api/src/frame/now.rs"),
        include_str!("../../api/src/frame/arm.rs"),
        include_str!("../../api/src/frame/wire.rs"),
        include_str!("../../api/src/frame/time.rs"),
        include_str!("../../api/src/frame/status.rs"),
    ]
    .concat();

    for symbol in [
        "Host",
        "Slot",
        "Answer",
        "Ring",
        "E_RTC",
        "View",
        "router_api",
        "hub_",
        "ipc",
        "resource",
        "programs",
        "system_client",
    ] {
        assert!(
            !source.contains(symbol),
            "rtc-api must not contain {symbol}"
        );
    }
}
