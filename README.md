# RTC Unit

RTC provides Shuyra's time and alarm service: `rtc-client` exposes `now`,
`arm` and the returned `Alarm`.

## Layout and policy

- `api/`: protocol frames, failure domain and entry coordinates.
- `client/`: typed calls and alarm reception.
- `server/`: pure session core, MMIO adapters and the `rtc` binary.
- `unit.toml`: runtime policy requiring and following Hub and Router.
- `declaration/`: compatibility Rust Unit/build declarations.
- `tests/api/`, `tests/server/`, `tests/unit-document/`: independent host suites.

RTC belongs to the `supervised` group; membership does not itself install a
supervisor. The entry mark deliberately uses `router_api::ENTRY_MARK`, matching
the declaration and server authentication. System completes internal bootstrap
before creating ordinary Units. The freestanding server is excluded from the
root workspace's default members.

## Build and test

Install Rust through rustup; the checkout pins nightly-2026-09-04 and RV64.

```sh
for suite in api server unit-document; do
  cargo test --locked --manifest-path "tests/$suite/Cargo.toml" --target x86_64-unknown-linux-gnu || exit
done
CARGO_TARGET_RISCV64GC_UNKNOWN_NONE_ELF_RUSTFLAGS='-Crelocation-model=static -Cforce-frame-pointers=yes -Ccode-model=medium' \
  cargo build --locked --release --target riscv64gc-unknown-none-elf -p rtc-server --bin rtc
```

Uncached dependencies require network access. Host tests exercise protocol,
pure core and declaration contracts, not syscalls or MMIO. Composed runtime
checks belong to [Vanllia](https://github.com/shuyra-arche/vanllia).

See [source attribution](PROVENANCE.md) and the
[organization contribution guide](https://github.com/shuyra-arche/vanllia/blob/main/CONTRIBUTING.md).
