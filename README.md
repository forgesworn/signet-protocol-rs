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

Minimum supported Rust version: 1.74 (the `rust-version` declared in `Cargo.toml`).

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
// `id` is a raw `[u8; 32]` SHA-256 hash. hex::encode(id) (from the `hex` crate,
// which this crate does not re-export) gives the lower-case hex string to
// compare with the event's `.id`. `pubkey` is the raw 32-byte x-only key, not hex.
```

## What's shipped in 0.1

| API | Purpose |
|---|---|
| `AUTH_EVENT_KIND: u32` (= 21236) | Kind for Sign-in-with-Signet auth events |
| `CREDENTIAL_KIND: u32` (= 31000) | Kind for Signet credentials (display-name, age-scope, professional) |
| `SignetAuthEvent` | Typed kind-21236 event struct |
| `SignetCredential` | Typed kind-31000 credential struct |
| `canonical_id(pubkey: &[u8; 32], created_at: u32, kind: u32, tags: &[Vec<String>], content: &str) -> [u8; 32]` | NIP-01 canonical event id (SHA-256 over the canonical JSON encoding); hex-encode the result to compare with a Nostr `.id` |

### Struct fields

`SignetAuthEvent` (kind 21236):

| Field | Type | Meaning |
|---|---|---|
| `pubkey` | `[u8; 32]` | x-only Schnorr public key, raw bytes |
| `created_at` | `u32` | Unix timestamp in seconds |
| `kind` | `u32` | Must equal `AUTH_EVENT_KIND` for server-side verification |
| `tags` | `Vec<Vec<String>>` | NIP-01 tags; must include `["challenge", <hex>]` and `["origin", <str>]` |
| `content` | `String` | Must be the empty string for auth events |
| `id` | `[u8; 32]` | SHA-256 of the canonical serialisation; recompute with `canonical_id` rather than trusting it |
| `sig` | `[u8; 64]` | BIP-340 Schnorr signature over `id` (not verified by this crate) |
| `from_np` | `bool` | Client-asserted flag that the event was signed by a Natural Person keypair; a server that rejects Natural Persons as players should reject `true` |

`SignetCredential` (kind 31000) has the same fields and types except `from_np`, which it does not have.

Structs are data only in 0.1: build them with struct literals. They derive `Clone` and `Debug` only, and there is no constructor, parser or serde support yet, so `serde_json::from_str` into these types will not compile.

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
