# RTC runtime declaration

Validated on 2026-11-05 with the toolchain from
`/workspace/.vanllia-tools/env.sh`: `nightly-2026-10-09`, rustc
`1.101.0-nightly (a30aa9064 2026-10-08)`, host
`x86_64-unknown-linux-gnu`. The local branch is `codex/unit-toml-runtime`, based
on extraction commit `f14cca5`.

Root `unit.toml` adds schema 1 policy for stable id/image `rtc`. It preserves
every legacy `rtc_declaration::UNIT` field, including the original
`router_api::ENTRY_MARK` publication and the server's matching unseal mark.
System's prior Operator bootstrap replaces the old Bootstrap dependency;
`requires` includes the actual `hub` and `router` providers and `after` waits
for their readiness. RTC starts at startup and stays resident.

The old Vanllia `PRODUCT.targets.supervised = ["rtc"]` membership moves from
static composition to `control.groups = ["supervised"]`. New System derives
dynamic target membership from these groups. RTC's own supervision remains
absent; later Daemon intake must keep `supervision = "supervised"` and requires
and after dependencies on `rtc`.

| Check | Result |
| --- | --- |
| TOML versus every legacy policy field and product supervision membership | Passed |
| Complete document/record/TOML roundtrip and borrowed strings | Passed |
| Actual `runtime::UnitFile::try_from` conversion versus legacy native fields | Passed |
| Decimal/hex original marks, numeric/quoted inputs, full `u64`, negative/overflow rejection | Passed |
| Existing Cargo package/bin/image and complete `BUILD` contract | Passed |
| New native declaration suite against public Unit pin, `--locked`, `-Dwarnings` | 7 passed |
| Same public suite, `--locked --offline`, no Cargo patch | 7 passed |
| Reviewer rerun with the formal test lock and independent output directory | 7 passed; `--locked --offline`, `-Dwarnings`, no Cargo patch |
| Unchanged protocol suite | 11 passed; existing 11 non-test unused-item warnings |
| Unchanged pure-core/compiled-declaration suite, `-Dwarnings` | 10 passed |
| Existing host workspace libraries, `-Dwarnings` | Passed |
| Existing release RISC-V `rtc` binary, `-Dwarnings` | Passed |
| Production dependency pins and all three original committed locks | Unchanged |
| Formal document-test lock | 37 packages; both public Unit revisions explicit |
| Locked/offline metadata sources | 34 public external packages and three in-repository RTC packages |
| New Rust formatting and whitespace checks | Passed |

The document tests now pin the published Unit revision
`ad50b803c3bd5cd2ef90010d80badf23551e88d5`
([Unit Draft #1](https://github.com/shuyra-arche/unit/pull/1)). The formal
`tests/unit-document/Cargo.lock` records 37 packages: 34 dependencies from
public external sources and three in-repository RTC packages. The direct
`unit` dependency supplies the real Rust schema,
record encoder/decoder and runtime conversion. The `legacy-unit` alias exposes
the unchanged production revision `90396f0fda376e0693fb1b9553aa21864d241057`
for comparisons with `rtc_declaration::UNIT`; enum policies are compared by
their values across the distinct revisions. No local Unit checkout, manifest
staging, seed lock or Cargo patch is required. Earlier local-patch evidence is
superseded by the direct public-source runs below. Production pins, production
Rust, old scenarios and the three original locks remain unchanged.

Namer and reviewer independently checked the preserved fields, dependency
mapping, mark values and product group membership in the initial migration.
The same reviewer approved the public pin and formal-lock follow-up after
independently rerunning all seven tests with `--locked --offline`, `-Dwarnings`,
no Cargo patch and an independent output directory. Locked/offline metadata
confirmed both public Unit revisions and the 37-package closure; production
pins and the three original locks remain unchanged.

Run directly with the committed test manifest and lock. The first invocation
fetches any missing public dependencies; the second reuses them offline:

```sh
source /workspace/.vanllia-tools/env.sh
RUSTFLAGS='-Dwarnings' cargo test --locked \
  --manifest-path tests/unit-document/Cargo.toml \
  --target-dir /tmp/rtc-unit-public-target --target x86_64-unknown-linux-gnu
RUSTFLAGS='-Dwarnings' cargo test --locked --offline \
  --manifest-path tests/unit-document/Cargo.toml \
  --target-dir /tmp/rtc-unit-public-target --target x86_64-unknown-linux-gnu
cargo test --locked --offline --manifest-path tests/api/Cargo.toml \
  --target-dir /tmp/rtc-legacy-api-target --target x86_64-unknown-linux-gnu
RUSTFLAGS='-Dwarnings' cargo test --locked --offline --manifest-path tests/server/Cargo.toml \
  --target-dir /tmp/rtc-legacy-server-target --target x86_64-unknown-linux-gnu
RUSTFLAGS='-Dwarnings' cargo build --locked --offline --workspace --lib \
  --target-dir /tmp/rtc-libraries-runtime-target --target x86_64-unknown-linux-gnu
RUSTFLAGS='-Dwarnings -Crelocation-model=static -Cforce-frame-pointers=yes -Ccode-model=medium' \
  cargo build --locked --offline -p rtc-server --bin rtc --release \
  --target riscv64gc-unknown-none-elf --target-dir /tmp/rtc-image-runtime-target
readelf -h /tmp/rtc-image-runtime-target/riscv64gc-unknown-none-elf/release/rtc
readelf -l /tmp/rtc-image-runtime-target/riscv64gc-unknown-none-elf/release/rtc
rustfmt --edition 2024 --check tests/unit-document/lib.rs
git diff --check
git diff --exit-code -- Cargo.lock tests/api/Cargo.lock tests/server/Cargo.lock
```

ELF inspection confirms a RISC-V ELF64 `EXEC`, first load address `0x10000`, and
no interpreter segment. The binary and old suites use the unchanged public
dependency pins. Production Rust and the old
test scenarios are unchanged. No QEMU, boot, syscall, MMIO, IRQ, supervision
runtime or real-image acceptance was run. This work creates only a local
commit; it does not push, open an RTC PR, or migrate the product scenes.

## Historical server and declaration extraction

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
