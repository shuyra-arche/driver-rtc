# RTC server and declaration extraction

Validated on 2026-11-04 with `nightly-2026-09-04`, host
`x86_64-unknown-linux-gnu`, and target `riscv64gc-unknown-none-elf`.

The base is PR #1 head `7aac60314169e57265d62960a48c90797a640887` on
`codex/rtc-client-closure`. The historical source is
`anreonyr/sqware@94e114614bec53424ac55e640fc80cf19a3e7107`.
The existing API/client commits remain ancestors; neither Source nor any
existing remote branch was rewritten.

## Results

| Check | Result |
| --- | --- |
| Existing protocol suite, unchanged | 11 passed |
| Production pure core and compiled declaration suite | 10 passed |
| Host workspace libraries, debug and release, `-Dwarnings` | Passed |
| RISC-V workspace libraries, debug and release, `-Dwarnings` | Passed |
| Standalone `rtc` RISC-V ELF, debug and release, `-Dwarnings` | Passed |
| Online and locked/offline package identities | Same 31 packages |
| Server and declaration dependency closures | No sqware, programs, or composition; one RTC API |
| Original lockfile package identities | Preserved |
| Source preservation checks | 33 passed |
| Whitespace errors | None |

The preservation checks compare all 19 moved function bodies, all five state
shapes, the device request and startup budget, and every historical `UnitFile`
field. Accepted differences are documented crate paths, explicit `::core::ptr`
and `::unit::READY_MARK` qualification, and formatting of the trailing comma
in `ctx.plate`. The device file otherwise matches the source exactly.
`api/`, `client/`, and `tests/api/` are unchanged from the base.

The ELF headers identify RISC-V ELF64 `EXEC` images, with `_start` as the entry
symbol, a first load segment at `0x10000`, and no interpreter segment. Building
and inspecting an ELF does not establish runtime success.

## Reproduce

After dependencies have been fetched, the checked commands are:

```sh
cargo metadata --locked --offline --format-version 1
cargo test --locked --offline --manifest-path tests/api/Cargo.toml
RUSTFLAGS='-Dwarnings' cargo test --locked --offline --manifest-path tests/server/Cargo.toml
RUSTFLAGS='-Dwarnings' cargo build --locked --offline --workspace --lib
RUSTFLAGS='-Dwarnings' cargo build --locked --offline --workspace --lib --release
export RUSTFLAGS='-Dwarnings -Crelocation-model=static -Cforce-frame-pointers=yes -Ccode-model=medium'
cargo build --locked --offline --workspace --lib --target riscv64gc-unknown-none-elf
cargo build --locked --offline --workspace --lib --target riscv64gc-unknown-none-elf --release
cargo build --locked --offline -p rtc-server --bin rtc --target riscv64gc-unknown-none-elf
cargo build --locked --offline -p rtc-server --bin rtc --target riscv64gc-unknown-none-elf --release
```

The unchanged historical protocol test crate emits non-test unused-item
warnings under its normal flags. No assertions were removed or relaxed.
Fresh, separate DeepSeek V4.1 Flash namer, worker, and reviewer sessions are
used for this PR; review and public-checkout evidence are recorded separately
from the build results.

## Remaining intake and acceptance

The Driver aggregate and Vanllia still need to select the published RTC Unit,
replace the legacy `prog-rtc` image and composition declaration with this
provider's `rtc` image and `rtc_declaration::{UNIT, BUILD}`, and update their
locks. All RTC API/client/server/declaration references must select the same
published full SHA so that integration retains one protocol identity.

No QEMU, boot, MMIO, interrupt delivery, syscall, cleanup-in-kernel, or whole
machine acceptance was run. The host tests cover pure decisions and compiled
declaration fields. Runtime ownership, reply cleanup and interrupt ordering
were preserved by source comparison and remain subject to later acceptance.
No PR was merged.
