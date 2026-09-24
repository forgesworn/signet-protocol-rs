# signet-protocol (Rust)

Rust implementation of the Signet identity protocol on Nostr: kind-21236
auth events, kind-31000 credentials, and the NIP-01 canonical event id. It is
the Rust counterpart of the `signet-protocol` npm package; the wire format is
identical and cross-impl test vectors lock the two implementations together.
`0.1.x` ships event types and the canonical event id only; Schnorr
verification and the challenge nonce table are planned, not yet present.

## Build & Test

| Command | Purpose |
|---------|---------|
| `cargo check --all-targets` | Type-check the crate and tests |
| `cargo test` | Run the test suite, including cross-impl vectors |
| `cargo clippy --all-targets -- -D warnings` | Lint |
| `cargo fmt --all -- --check` | Check formatting |

## Structure

```
src/lib.rs               crate root, re-exports the public API
src/event.rs              event types + canonical_id
tests/canonical_id_vectors.rs   cross-impl test vectors shared with the TS package
```

## Conventions

- Wire format must stay identical to the TypeScript `signet-protocol`
  package; do not change the canonical serialisation without updating the
  shared test vectors on both sides.
- `rust-version` is pinned in `Cargo.toml`; do not use newer language
  features without checking it.
- CI runs with `RUSTFLAGS="-D warnings"`, so `cargo check` and `cargo clippy`
  must be warning-free.

## Common Pitfalls

- Never trust an inbound event's `id` or `sig` field: recompute `id` via
  `canonical_id` and re-check the signature.
- `tests/canonical_id_vectors.rs` vectors are shared with the TypeScript
  package; do not change expected values without updating both sides.
