# signet-protocol — Rust

[![CI](https://github.com/forgesworn/signet-protocol-rs/actions/workflows/ci.yml/badge.svg)](https://github.com/forgesworn/signet-protocol-rs/actions/workflows/ci.yml)
[![GitHub Sponsors](https://img.shields.io/github/sponsors/TheCryptoDonkey?logo=githubsponsors&color=ea4aaa&label=Sponsor)](https://github.com/sponsors/TheCryptoDonkey)

**Rust implementation of the [Signet](https://github.com/forgesworn/signet) identity protocol on Nostr.**

Mirrors the [`signet-protocol`](https://www.npmjs.com/package/signet-protocol) npm package. The wire format is identical, and the canonical event id computed here matches the TypeScript side, locked by shared test vectors. Signature verification is planned, not yet present.

> Status: **0.1.x — core types + NIP-01 canonical event id shipped.** BIP-340 Schnorr verification, credential parsing, and the server-side challenge nonce table land in upcoming minors. The wire format is stable; the API surface above it will grow additively.

## Install

Not yet published to crates.io. Depend on the repository, pinned to a commit:

```toml
[dependencies]
signet-protocol = { git = "https://github.com/forgesworn/signet-protocol-rs", rev = "4f42cc86c80674d575802ddb77d179b5318c9329" }
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

The wire format matches the TypeScript [`signet-protocol`](https://www.npmjs.com/package/signet-protocol) package exactly, and the verification rules will follow it as they land; the two are deliberate counterparts.

## Licence

MIT — see [LICENSE](./LICENSE).
