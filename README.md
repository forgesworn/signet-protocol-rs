# signet-protocol — Rust

[![CI](https://github.com/forgesworn/signet-protocol-rs/actions/workflows/ci.yml/badge.svg)](https://github.com/forgesworn/signet-protocol-rs/actions/workflows/ci.yml)
[![crates.io](https://img.shields.io/crates/v/signet-protocol)](https://crates.io/crates/signet-protocol)
[![docs.rs](https://img.shields.io/docsrs/signet-protocol)](https://docs.rs/signet-protocol)
[![GitHub Sponsors](https://img.shields.io/github/sponsors/TheCryptoDonkey?logo=githubsponsors&color=ea4aaa&label=Sponsor)](https://github.com/sponsors/TheCryptoDonkey)

**Rust implementation of the [Signet](https://github.com/forgesworn/signet) identity protocol on Nostr.**

Mirrors the [`signet-protocol`](https://www.npmjs.com/package/signet-protocol) npm package — the wire format is identical, and an event signed by the TypeScript side verifies here (and vice versa). Cross-impl equivalence is locked by test vectors.

> Status: **0.1.x — core types + NIP-01 canonical event id shipped.** BIP-340 Schnorr verification, credential parsing, and the server-side challenge nonce table land in upcoming minors. The wire format is stable; the API surface above it will grow additively.

## Install

```toml
[dependencies]
signet-protocol = "0.1"
```

## Quick start

Compute the NIP-01 canonical event id for a Signet auth event:

```rust
use signet_protocol::{canonical_id, AUTH_EVENT_KIND};

let pubkey = [0xbbu8; 32];
let tags = vec![
    vec!["challenge".to_string(), "a".repeat(64)],
    vec!["origin".to_string(), "https://localhost:8094".to_string()],
];
let id = canonical_id(&pubkey, 1_700_000_000, AUTH_EVENT_KIND, &tags, "");
// id is the SHA-256 hash a verifier would compare against the event's `.id` field.
```

## What's shipped in 0.1

| API | Purpose |
|---|---|
| `AUTH_EVENT_KIND` (= 21236) | Kind for Sign-in-with-Signet auth events |
| `CREDENTIAL_KIND` (= 31000) | Kind for Signet credentials (display-name, age-scope, professional) |
| `SignetAuthEvent` | Typed kind-21236 event struct |
| `SignetCredential` | Typed kind-31000 credential struct |
| `canonical_id(...)` | NIP-01 canonical event id (SHA-256 over the canonical JSON encoding) |

## Planned (next minors)

- `verify::schnorr_verify_bip340` — BIP-340 Schnorr verify
- `verify::verify_auth_event` — full kind-21236 verification (id + sig + challenge + origin + skew)
- `verify::verify_credential` — kind-31000 credential verification
- `verify::extract_display_name` — read the `display-name` tag from a verified credential
- `challenge::ChallengeTable` — server-side CSRF nonce table (issue + consume + GC)
- Optional `serde` derive feature for the event types (Phase 3 wire integration)

## Why a Rust crate?

The Signet ecosystem's reference apps and SDKs are TypeScript today. Engine consumers — voxel games, embedded signing appliances, custom relays, native multiplayer servers — need verification on the Rust side, and currently re-implement the same primitives (NIP-01 canonical id, BIP-340 verify, kind-21236 shape) in each project. One crate, one source of truth, one set of test vectors.

The wire format and verification rules match the TypeScript [`signet-protocol`](https://www.npmjs.com/package/signet-protocol) package exactly — these two crates are deliberate counterparts.

## Licence

MIT — see [LICENSE](./LICENSE).
