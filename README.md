# RTC Unit — API, client, server, and declaration

This repository contains `rtc-api`, the pure RTC protocol package (entry
coordinates, the five frame shapes, the reply-hole mark, and the failure
domain); `rtc-client`, the guest-side client built on that protocol (`now`,
`arm`, and the `Alarm` returned by a successful `arm`); `rtc-server`, the
focused server with the pure session core, device MMIO adapters, and the
standalone `rtc` binary; and `rtc-declaration`, the single `UnitFile` and
`BUILD` image manifest for that Unit.

The API files were transferred earlier from Source commit
`ac5b61d8880c499f8cee4c60929c06cc9f4820d8`. The client, server, and declaration
use frozen Source commit `94e114614bec53424ac55e640fc80cf19a3e7107`; this slice
starts from the public RTC API/client PR #1 base
`7aac60314169e57265d62960a48c90797a640887`. The extracted static is named
`UNIT`; it preserves historical `PROGRAM` fields, with only `crate::unit` paths
retargeted to the public `unit` crate.

The entry constants stay in `api/src/lib.rs`; `Fail` stays in `api/src/fail.rs`;
each frame shape and its impls live in its own file under `api/src/frame/`;
`api/src/frame/mod.rs` owns the constants and re-exports only. In `rtc-client`,
`client/src/lib.rs` holds the crate documentation plus `now`, `arm`, and the
private `why` diagnostic helper; `client/src/alarm.rs` holds the single `Alarm`
struct and its `receive`. Function and impl bodies are preserved verbatim; only
module/import scaffolding changed. The protocol's unique `Fail` and `frame`
definitions stay in `rtc-api` and are not duplicated: `rtc-client` re-exports
`rtc_api::Fail` and imports `rtc_api::frame`. The `Alarm::back` field is
`pub(super)` only so `arm` can construct the value.

`rtc-server` splits the Source files without designing new behavior:
`server/src/lib.rs` owns `E_RTC = 12`, the local `Ask`, the bounded `MS`, and
`serve` (the historical `main` body); `server/src/main.rs` is the thin
`#[mold::entry]` binary named `rtc`; `server/src/core/{host,slot}/` hold one
type per file (`Host`, `Answer`, `Ring`, `Slot`, `Cell`); `server/src/dev/rtc.rs`
owns the register order; `server/src/adapt/{desk,resident}.rs` own the syscall
and MMIO glue. The server imports `Context`, `Device`/`Hub`/`Ask`, and `Fail`
from the public `driver-shared` provider pinned at
`e75b1e232f828666956fb7e02c61a7d84f13eb66`. `rtc-api` remains the unique owner
of the protocol shapes, codes, entry coordinates, and the service-side `Fail`.
The source `Fail` and frame files are not copied into the server.

`rtc-declaration` holds exactly one `UnitFile` (`UNIT`) and one `ImageBuild`
(`BUILD`). `BUILD` names image `rtc`, manifest `server/Cargo.toml`, package
`rtc-server`, bin `rtc`, target `riscv64gc-unknown-none-elf`, and release
profile. The build script consumes `execution`'s published
`DEP_EXECUTION_LINKER_SCRIPT`, matching the public Router extraction pattern.
The standalone server is a workspace member but is excluded from
`default-members`, because a freestanding image binary is not a host default
target.

The dependencies are public provider crates pinned by complete HTTPS Git
revision, plus the in-repository `rtc-api` at path `../api`. `rtc-api` remains
the only RTC codec and service-side `Fail` owner; `rtc-server` adds the public
`driver-shared` runtime helper crate and the provider crates it names. The
host-only protocol checks live in `tests/api`; host-only pure-core and
declaration-contract checks live in `tests/server`. Both test crates are
excluded from the root workspace and are their own workspaces.

## Preservation adaptations

Function and impl bodies are transferred verbatim apart from crate-path
rewrites and the one-type-per-file splits. Two qualifications are required by
the new crate roots: `dev/rtc.rs` uses `::core::ptr` because the server has a
local `core` module, and `serve` uses `::unit::READY_MARK` because `env::unit`
is also imported. The server keeps `rtc-api` as the only owner of frames,
wire codes, entry coordinates, and the service-side `Fail`; it imports
`Context`, `Device`/`Hub`/`Ask`, and `Fail` from `driver-shared` at
`e75b1e232f828666956fb7e02c61a7d84f13eb66`. The declaration owns exactly one
`UnitFile` and one `ImageBuild`.

## Validation

See `VALIDATION.md` for validation results and exact commands. The host suites
cover 11 protocol tests and 10 server/declaration tests; they do not execute
syscalls or MMIO.

Representative commands:

```sh
cargo test --manifest-path tests/api/Cargo.toml
cargo test --manifest-path tests/server/Cargo.toml
cargo build -p rtc-api -p rtc-client -p rtc-declaration
cargo build -p rtc-server --lib
cargo build -p rtc-server --lib --release --target riscv64gc-unknown-none-elf
RUSTFLAGS='-Crelocation-model=static -Cforce-frame-pointers=yes -Ccode-model=medium' cargo build -p rtc-server --bin rtc --release --target riscv64gc-unknown-none-elf
```

## Intake and runtime acceptance

All four packages are implemented. Remaining work is intake into Vanllia/Driver
and runtime acceptance. No QEMU, boot, device, syscall, or whole-machine runtime
acceptance is performed or claimed. Adapter and MMIO behavior is covered by body
preservation and review, not the host suite. Independent review and public
clean-checkout proof are not claimed as completed; final records belong in
`VALIDATION.md`. The public repository is `shuyra-arche/driver-rtc`.
