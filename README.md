# RTC Unit — API and client

This repository contains `rtc-api`, the pure RTC protocol package (entry
coordinates, the five frame shapes, the reply-hole mark, and the failure
domain), and `rtc-client`, the guest-side client built on that protocol
(`now`, `arm`, and the `Alarm` returned by a successful `arm`). It has no
server, declaration, device code, runtime, `UnitFile`, or image.

Both packages are derived from the frozen Source tree under
`source-readonly/programs/src/driver/rtc/`. The API files were transferred
earlier from Source commit `ac5b61d8880c499f8cee4c60929c06cc9f4820d8` and are
unchanged in this slice. The client is transferred from the frozen Source
commit `94e114614bec53424ac55e640fc80cf19a3e7107`, path
`programs/src/driver/rtc/client.rs`.

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

The dependencies are the public `env`, `ipc`, `resource`, `wire`,
`system-support`, and `system-api` providers, pinned by complete Git revision,
plus the in-repository `rtc-api` at path `../api`. The host-only protocol checks
live in `tests/api` and are excluded from the root workspace.

## Candidate validation

Root's earlier candidate proof
`/workspace/evidence/router-next/rtc-api-candidate/proof.json` records raw exit
0 for every API gate:

- 12 original protocol items/impls token-identical, with unique definitions
- `Fail` byte-identical to Source
- original entry prefix preserved
- online and offline locked metadata
- 11 actual host codec tests
- host and RV debug/release library builds

The client slice was independently checked against Source `94e1146`: all 12
protocol item/impl bodies, `Fail` bytes, and the `now`, `arm`, `why`, and
`Alarm` method bodies are preserved. Online and locked/offline metadata select
one `rtc-api` identity. Host and RISC-V debug/release builds pass with
`nightly-2026-09-04` and warnings denied; all 11 existing protocol tests pass
with their normal flags. A separate consumer compiles the public signatures
and proves that `rtc_client::Fail` is the same type as `rtc_api::Fail`.

The initial build's unused import was removed by the same worker. A separate
attempt to deny all warnings in the existing test crate failed on its
non-test dead-code warnings; its normal test run passes unchanged. No syscall
runtime simulation, QEMU, or whole-machine acceptance was run for this slice.
Independent review and public clean-checkout proof remain separate gates;
server, declaration, runtime, and Source intake remain pending.

The public repository is `shuyra-arche/driver-rtc`. See `PROVENANCE.md` for the
source map, recorded candidate hashes, authors, and license statement.
