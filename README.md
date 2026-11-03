# RTC Unit — API

This repository currently contains only `rtc-api`, the pure RTC protocol package:
entry coordinates, the five frame shapes, the reply-hole mark, and the failure
domain. It has no client, server, declaration, device code, runtime, `UnitFile`,
or image.

The package is derived from Source
`/workspace/sqware-next` at commit
`ac5b61d8880c499f8cee4c60929c06cc9f4820d8`, paths
`programs/src/driver/rtc/api/src/lib.rs`, `programs/src/driver/rtc/core/fail.rs`,
and `programs/src/driver/rtc/core/frame.rs`. The entry constants stay in
`api/src/lib.rs`; `Fail` stays in `api/src/fail.rs`; each frame shape and its
impls live in its own file under `api/src/frame/`; `api/src/frame/mod.rs` owns
the constants and re-exports only. Item and impl bodies are preserved verbatim;
only module/import scaffolding and the entry re-export projection changed.

The only dependencies are the public `env`, `wire`, `mold`, and `system-api`
providers, pinned by complete Git revision. The host-only protocol checks live
in `tests/api` and are excluded from the root workspace.

## Candidate validation

Root's candidate proof
`/workspace/evidence/router-next/rtc-api-candidate/proof.json` records raw exit
0 for every gate:

- 12 original protocol items/impls token-identical, with unique definitions
- `Fail` byte-identical to Source
- original entry prefix preserved
- online and offline locked metadata
- 11 actual host codec tests
- host and RV debug/release library builds

These are candidate checks only. The public clean-checkout proof, publication,
client, server, declaration, runtime, and Source intake remain pending.

The public repository is `shuyra-arche/rtc`. See `PROVENANCE.md` for the source
map, verified candidate hashes, authors, and license statement.
