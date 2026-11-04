# Provenance

## Source anchor

- Original Source: `https://github.com/anreonyr/sqware.git`
- Frozen Source snapshot for this slice: `94e114614bec53424ac55e640fc80cf19a3e7107`
  (read-only tree mounted at `source-readonly/`)
- Client Source path: `programs/src/driver/rtc/client.rs`
- API Source paths (earlier API candidate, unchanged here):
  - `programs/src/driver/rtc/api/src/lib.rs`
  - `programs/src/driver/rtc/core/fail.rs`
  - `programs/src/driver/rtc/core/frame.rs`

This snapshot contains `rtc-api` and `rtc-client` only. The RTC server,
declaration, device mapping, runtime, `UnitFile`, and image remain outside this
repository. The API files were transferred earlier from Source commit
`ac5b61d8880c499f8cee4c60929c06cc9f4820d8`; this client slice takes the frozen
snapshot `94e114614bec53424ac55e640fc80cf19a3e7107` as its Source anchor.

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

## Candidate SHA-256 values

Copied from Root's candidate proof
`/workspace/evidence/router-next/rtc-api-candidate/proof.json`. The proof records
source hashes at grouped-file granularity; it supplies no per-item hash for the
split frame bodies. These values cover the API candidate only; the client files
are hashed separately in the current Root validation evidence; no client hash
is claimed by this historical API table.

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

## Candidate status

The historical hash table above covers the API only. Current Root evidence
also records 20 preservation checks, four warnings-denied host/RISC-V
debug/release builds with nightly-2026-09-04, 11 unchanged protocol tests with
normal test flags, and an independent consumer compiling public signatures
and the shared Fail identity. The README records the two retained failed
attempts and the validation limits. Independent review, public clean-checkout
proof and publication remain separate gates. Server, declaration, runtime,
and Source intake remain pending and are not claimed here.
