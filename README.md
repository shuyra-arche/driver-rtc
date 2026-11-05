# RTC Unit — API, client, server, and declaration

This repository contains `rtc-api`, the pure RTC protocol package (entry
coordinates, the five frame shapes, the reply-hole mark, and the failure
domain); `rtc-client`, the guest-side client built on that protocol (`now`,
`arm`, and the `Alarm` returned by a successful `arm`); `rtc-server`, the
focused server with the pure session core, device MMIO adapters, and the
standalone `rtc` binary; and `rtc-declaration`, the single `UnitFile` and
`BUILD` image manifest for that Unit. Root `unit.toml` carries the corresponding
schema 1 runtime policy; the compiled declaration remains its preservation
reference.

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

## Runtime declaration

`unit.toml` uses stable id `rtc` and image `rtc`, the existing Cargo binary in
`server/Cargo.toml` (package `rtc-server`, entry `server/src/main.rs`). Build
metadata stays in `BUILD`: target `riscv64gc-unknown-none-elf`, release profile,
default features enabled, and no additional features. The runtime declaration
preserves the user address space, empty arguments and default stack; the
service/shared/rtc subtree namespace and its 279 operation bits; public entry
publication with STORE view; empty identity, supply, construction and retirement
policies; absent supervision and source mark; READY_MARK signal; and resident
ending. The entry mark deliberately remains `router_api::ENTRY_MARK`, matching
both the legacy RTC declaration and the server's `unseal` call.

`control.creation = "startup"` declares RTC startup. `requires = ["hub", "router"]`
includes the actual provider closure, while `after = ["hub", "router"]` waits
for those providers to become ready. System completes Operator bootstrap before
creating Units, so the old `Bootstrap::Operator` dependency has no standalone
Unit counterpart in either list.

The old Vanllia `PRODUCT` composition put RTC in `targets.supervised = ["rtc"]`.
That product relationship moves from static composition into RTC's runtime
declaration as `control.groups = ["supervised"]`; System derives the dynamic
target membership from startup Units' groups. RTC's own `supervision` remains
absent. The later Daemon declaration must use `supervision = "supervised"` and
both `requires = ["rtc"]` and `after = ["rtc"]` to retain the consumer side of
this relationship. This change records RTC's side without migrating Daemon or
the old scenarios.

Full-width marks are quoted decimal strings. Schema 1 also accepts numeric
values and quoted `0x` hexadecimal strings, and records encode native `u64`
values. `tests/unit-document` compares every policy field with `UNIT`, exercises
the complete binary record and actual runtime conversion, checks borrowed
strings, validates signed/overflow rejection and the full `u64` range, and
checks the Cargo image against `BUILD`.

The document/record/runtime suite pins the public Unit revision
`ad50b803c3bd5cd2ef90010d80badf23551e88d5`
([Unit Draft #1](https://github.com/shuyra-arche/unit/pull/1)) and commits its
own `tests/unit-document/Cargo.lock`. The `legacy-unit` test alias keeps the
compiled declaration's original Unit types available for comparison across
the two revisions. Production dependencies retain the legacy Unit pin
`90396f0fda376e0693fb1b9553aa21864d241057`; the root and old test locks stay
unchanged. Run the suite directly, without a local Unit checkout or Cargo patch:

```sh
source /workspace/.vanllia-tools/env.sh
RUSTFLAGS='-Dwarnings' cargo test --locked \
  --manifest-path tests/unit-document/Cargo.toml \
  --target-dir /tmp/rtc-unit-public-target --target x86_64-unknown-linux-gnu
RUSTFLAGS='-Dwarnings' cargo test --locked --offline \
  --manifest-path tests/unit-document/Cargo.toml \
  --target-dir /tmp/rtc-unit-public-target --target x86_64-unknown-linux-gnu
```

The first command fetches any missing public dependencies; the second uses
the same committed lock and cached sources offline.

## Validation

See `VALIDATION.md` for validation results and exact commands. The host suites
cover 11 protocol tests, 10 server/declaration tests, and seven runtime
declaration tests; they do not execute syscalls or MMIO.

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
preservation and review, not the host suite. Current review results and the
validation scope are recorded in `VALIDATION.md`. The public
repository is `shuyra-arche/driver-rtc`.
