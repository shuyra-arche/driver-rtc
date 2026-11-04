# Provenance

## Source anchor

- Original Source: `https://github.com/anreonyr/sqware.git`
- Frozen Source snapshot for this slice: `94e114614bec53424ac55e640fc80cf19a3e7107`
- Client Source path: `programs/src/driver/rtc/client.rs`
- API Source paths (earlier API candidate, unchanged here):
  - `programs/src/driver/rtc/api/src/lib.rs`
  - `programs/src/driver/rtc/core/fail.rs`
  - `programs/src/driver/rtc/core/frame.rs`

This snapshot contains `rtc-api`, `rtc-client`, `rtc-server`, and
`rtc-declaration`. The server and declaration are transferred from the frozen
Source commit `94e114614bec53424ac55e640fc80cf19a3e7107`; the API files were
transferred earlier from Source commit
`ac5b61d8880c499f8cee4c60929c06cc9f4820d8`; this client slice takes the frozen
snapshot `94e114614bec53424ac55e640fc80cf19a3e7107` as its Source anchor.

The public branch base for the server/declaration slice is
`7aac60314169e57265d62960a48c90797a640887` (public PR #1 base). The
`driver-shared` provider is pinned at
`e75b1e232f828666956fb7e02c61a7d84f13eb66`. The server's build script and thin
binary follow the public Router extraction pattern; they are scaffolding, not
copied Source behavior.

## File map

| Source path | Unit path | Adaptation |
| --- | --- | --- |
| `programs/src/driver/rtc/api/src/lib.rs` | `api/src/lib.rs` | Original seven lines retained; module declarations and `Fail` re-export added. |
| `programs/src/driver/rtc/core/fail.rs` | `api/src/fail.rs` | Byte-identical. |
| `programs/src/driver/rtc/core/frame.rs` | `api/src/frame/mod.rs` | Original constants, entry projection, failure-code re-exports, and frame names; no frame body here. |
| `programs/src/driver/rtc/core/frame.rs` (`Now`, `impl Now`) | `api/src/frame/now.rs` | Item/impl body verbatim; only `use` lines added. |
| `programs/src/driver/rtc/core/frame.rs` (`Arm`, `impl Arm`) | `api/src/frame/arm.rs` | Item/impl body verbatim; only `use` lines added. |
| `programs/src/driver/rtc/core/frame.rs` (`Wire`, `impl Wire`) | `api/src/frame/wire.rs` | Item/impl body verbatim; only `use` lines added. |
| `programs/src/driver/rtc/core/frame.rs` (`Time`, `impl Time`, `impl Message for Time`) | `api/src/frame/time.rs` | Item/impl bodies verbatim; only `use` lines added. |
| `programs/src/driver/rtc/core/frame.rs` (`Status`, `impl Status`, `impl Message for Status`) | `api/src/frame/status.rs` | Item/impl bodies verbatim; only `use` lines added. |
| `programs/src/driver/rtc/client.rs` (crate doc, `now`, `arm`, `why`) | `client/src/lib.rs` | Function bodies verbatim; `use` paths retargeted to `rtc_api` and `system_support`; `Alarm` moved to its own file; `#![no_std]` and `extern crate alloc` added. |
| `programs/src/driver/rtc/client.rs` (`Alarm`, `impl Alarm`) | `client/src/alarm.rs` | Item/impl bodies verbatim; only `use` lines added; field visibility `pub(super)` for construction by `arm`. |
| `programs/src/driver/rtc/mod.rs` (`E_RTC`) | `server/src/lib.rs` | Value and documentation retained; `pub const E_RTC: env::Reason = 12`. |
| `programs/src/driver/rtc/main.rs` (`ASK`, `MS`, `main` body) | `server/src/lib.rs` | `main` body becomes public `serve`; only provider paths retargeted (`programs::debug` to `crate::debug`, `programs::driver::shared` to `driver_shared`, `programs::driver::rtc::core` to `crate::core`, local `E_RTC`). |
| `programs/src/driver/rtc/main.rs` (entry shell) | `server/src/main.rs` | Thin `#[mold::entry]` binary `rtc`; calls `rtc_server::serve`. |
| `programs/src/driver/rtc/core/host.rs` (`Answer`) | `server/src/core/host/answer.rs` | Item verbatim; own file. |
| `programs/src/driver/rtc/core/host.rs` (`Ring`) | `server/src/core/host/ring.rs` | Item verbatim; own file. |
| `programs/src/driver/rtc/core/host.rs` (`Host`, `impl Host`) | `server/src/core/host/host.rs` | Item/impl bodies verbatim; `super::frame` to `rtc_api::frame`, `super::slot` to `crate::core::slot`; `Answer`/`Ring` split to sibling files. |
| `programs/src/driver/rtc/core/slot.rs` (`Cell`) | `server/src/core/slot/cell.rs` | Item verbatim; own file. |
| `programs/src/driver/rtc/core/slot.rs` (`Slot`, `impl Slot`) | `server/src/core/slot/slot.rs` | Item/impl bodies verbatim; `super::Fail` to `rtc_api::Fail`, `Cell` from sibling file. |
| `programs/src/driver/rtc/core/mod.rs` | `server/src/core/mod.rs` | Module declarations reduced to the pure server core; `frame` and `Fail` remain owned by `rtc-api`. |
| `programs/src/driver/rtc/dev/rtc.rs` | `server/src/dev/rtc.rs` | Bodies token-identical; only the builtin `core::ptr` path is absolute (`::core::ptr`) because the server root has a local `core` module. |
| `programs/src/driver/rtc/adapt/desk.rs` | `server/src/adapt/desk.rs` | Bodies token-identical; only `use` paths retargeted to `crate::core`, `crate::debug`, `crate::dev`, and `rtc_api::frame`. |
| `programs/src/driver/rtc/adapt/resident.rs` | `server/src/adapt/resident.rs` | Bodies token-identical; only `use` paths retargeted to `crate::core`, `crate::debug`, `crate::dev`, `crate::E_RTC`, `driver_shared`, and `rtc_api::frame`. |
| `programs/src/driver/rtc/adapt/mod.rs` | `server/src/adapt/mod.rs` | Module declarations verbatim. |
| `programs/src/driver/rtc/dev/mod.rs` | `server/src/dev/mod.rs` | Module declaration verbatim. |
| `crates/composition/src/units/rtc.rs` | `declaration/src/lib.rs` | `PROGRAM` becomes `UNIT`; every UnitFile field value identical; only `crate::unit` paths retargeted to the public `unit` crate. `BUILD` added from the Router extraction manifest contract. |
| not Source | `server/Cargo.toml`, `server/build.rs`, `server/src/main.rs` | Public Router extraction scaffolding: package `rtc-server`, bin `rtc`, and the published linker-script build glue. |
| not Source | `tests/server/{Cargo.toml,lib.rs}` | New host-only fixture; path-includes the production pure core and asserts compiled `rtc_declaration::UNIT`/`BUILD` fields. |

No frame shape, wire code, `BACK`, `OK`, entry coordinate, or service-side
`Fail` is duplicated in the server or declaration. The local `ASK` device
request in `server/src/lib.rs` is not the protocol `frame::ASK`; it is the
historical `google,goldfish-rtc` claim and keeps its original values. The
server's `core` and `adapt`/`dev` modules are private. The declaration owns
exactly one `UnitFile` and one `ImageBuild` and defines no `PROGRAM` alias.

The structural adaptation is the project one-structure-per-file rule: the
original grouped frame file is split by shape, and the client's single `Alarm`
struct is split from the free functions. The codec definition stays
single-source. `api/src/frame/mod.rs` contains only constants, module
declarations, and re-exports; the external `wire::OK` re-export uses the
absolute `::wire::OK` path so it cannot bind the local `wire` shape module.
`client/src/lib.rs` re-exports `rtc_api::Fail` and `Alarm`; it defines neither
`Fail` nor any frame shape.

The client keeps `use wire::Message;` because `Time::EMPTY` and `Status::EMPTY`
are associated constants of that trait. It keeps the original no-`Drop` shape,
the 20-call diagnostic cap in `why`, every seal/release cleanup path, and the
original `Denied`/timeout failure mapping.

## Historical API candidate SHA-256 values (prior API slice only)

This retained table covers the earlier API slice only. It records source hashes
at grouped-file granularity and supplies no per-item hash for the split frame
bodies. No client, server, or declaration hash is claimed here; the current
server/declaration code map is the file map above.

### Original Source

| Source path | SHA-256 |
| --- | --- |
| `programs/src/driver/rtc/api/src/lib.rs` | `981dbfeb3b0620e536aea4f005e3491752af878f42c972c1b375968372ab1125` |
| `programs/src/driver/rtc/core/fail.rs` | `579fc47039bd3e6e3f7164a13f547b8f4b3f12d248fa8a8d9ab33c1d893fa5b0` |
| `programs/src/driver/rtc/core/frame.rs` | `4462b731453bdd419bad66889396d4b75361af19874b4ff2b13000e10100b71f` |

### Candidate destination

| Candidate path | SHA-256 |
| --- | --- |
| `api/Cargo.toml` | `52c948c2a7f0d2413d1c5ad593cc9f2a72db10ef81f22b4411f3dfa9812444cb` |
| `api/src/lib.rs` | `cd23c724a4ba037edb2397b2e3d15b5311660cd47dd73d296afad920bef7bc14` |
| `api/src/fail.rs` | `579fc47039bd3e6e3f7164a13f547b8f4b3f12d248fa8a8d9ab33c1d893fa5b0` |
| `api/src/frame/mod.rs` | `8615c2a5d767106dd36ec002c6da1536511c49aa55ac3a8cceac1b2b94e7a026` |
| `api/src/frame/now.rs` | `cbf88a668112c0ea0e96f3714b062925ff8f06e3592b55e8fbb191012b6a7626` |
| `api/src/frame/arm.rs` | `04250a1153b76e7dd391a125509abc27a7d73ef9c1dd096504a22952c2bb8dd0` |
| `api/src/frame/wire.rs` | `e95f990a6e240fafd75cf0cce447d6bc53136d1063aea78c48a6b0fc01d39df0` |
| `api/src/frame/time.rs` | `2ddca3b30796c58be871ce9b24ad955167ce21ae50ac40d54fa204b19f4bd9c8` |
| `api/src/frame/status.rs` | `7824e9af91ecf6a31c1e259bd1f667da3088a114aa50e886861638ed47e41b0b` |

## Authors and license

Original Source history author, as recorded in the Router Unit provenance for the
same Source history:

- anreonyr <anreonyr@gmail.com>

The Source root has no project LICENSE. This snapshot invents none and does not
rewrite Source or prior public histories. Existing file headers and comments are
retained.

## Validation status

The historical API/client evidence remains as recorded above. The
server/declaration slice adds the current file map and host-only compiled
declaration assertions; it has no new candidate hash table. Initial validation
passed 10 server tests and a warnings-denied host library build, and preservation
passed all checks after allowing `::core::ptr` and `::unit::READY_MARK`
qualification. Final command results, including typed declaration and RV ELF
checks, belong in `VALIDATION.md` and are not claimed as passed here.

No QEMU, boot, device, syscall, or other runtime acceptance has been performed.
Remaining Vanllia/Driver intake, independent review, public clean-checkout
reproduction, and publication remain outside this repository.
