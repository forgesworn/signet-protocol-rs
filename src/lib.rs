//! # signet-protocol — Rust
//!
//! Rust implementation of the [Signet](https://github.com/forgesworn/signet)
//! identity protocol on Nostr. This crate is the Rust counterpart of the
//! `signet-protocol` npm package; the wire format is identical and the
//! canonical event id computed here matches the TypeScript side.
//!
//! ## Status
//!
//! `0.1.x` ships the cross-impl-locked primitives that any Signet consumer
//! needs first: event types, the NIP-01 canonical event id, and the kind
//! constants. Schnorr verification, server-side challenge nonce table, and
//! credential parsing land in follow-up minors.
//!
//! ## Quick start
//!
//! ```
//! use signet_protocol::{canonical_id, AUTH_EVENT_KIND};
//!
//! let pubkey = [0u8; 32];
//! let id = canonical_id(&pubkey, 1700000000, AUTH_EVENT_KIND, &[], "");
//! assert_eq!(id.len(), 32);
//! ```

mod event;

pub use event::{
    canonical_id, SignetAuthEvent, SignetCredential, AUTH_EVENT_KIND, CREDENTIAL_KIND,
};
