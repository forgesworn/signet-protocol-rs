//! Signet event types + NIP-01 canonical event id.
//!
//! The wire shape matches the TypeScript `signet-protocol` package exactly.
//! An event signed by the TS path verifies here, and vice versa — locked by
//! cross-impl test vectors in `tests/canonical_id_vectors.rs`.

use sha2::{Digest, Sha256};

/// Signet auth event kind. Used by `Sign in with Signet` flows.
pub const AUTH_EVENT_KIND: u32 = 21236;

/// Verifiable attestation event kind (NIP-VA). Signet credential profile —
/// display-name, age-scope, professional credentials — share this kind and
/// differentiate via the `type` tag.
pub const CREDENTIAL_KIND: u32 = 31000;

/// Signed Nostr auth event (kind-21236). Server-side verification recomputes
/// `id` and re-checks the Schnorr signature against `pubkey` — never trust
/// the inbound `id` or `sig` fields without that recheck.
#[derive(Clone, Debug)]
pub struct SignetAuthEvent {
    /// x-only Schnorr public key (32 bytes).
    pub pubkey: [u8; 32],
    /// Unix timestamp (seconds since epoch).
    pub created_at: u32,
    /// Must equal [`AUTH_EVENT_KIND`] for server-side verification.
    pub kind: u32,
    /// NIP-01 tags. Must include `["challenge", <hex>]` and `["origin", <str>]`.
    pub tags: Vec<Vec<String>>,
    /// Per Signet spec must be the empty string for auth events.
    pub content: String,
    /// SHA-256 of the canonical NIP-01 serialisation. Recompute via
    /// [`canonical_id`] and compare before trusting.
    pub id: [u8; 32],
    /// BIP-340 Schnorr signature over `id`.
    pub sig: [u8; 64],
    /// Client-asserted "signed by a Natural Person keypair". Servers that
    /// reject NP-as-player should reject `true` outright.
    pub from_np: bool,
}

/// Signed kind-31000 event carrying a Signet credential.
#[derive(Clone, Debug)]
pub struct SignetCredential {
    pub pubkey: [u8; 32],
    pub created_at: u32,
    pub kind: u32,
    pub tags: Vec<Vec<String>>,
    pub content: String,
    pub id: [u8; 32],
    pub sig: [u8; 64],
}

/// Compute the NIP-01 canonical event id.
///
/// The canonical serialisation is compact JSON (`,` and `:` separators, no
/// spaces), UTF-8, lower-case hex for the pubkey. The result is the SHA-256
/// of that byte sequence.
///
/// Cross-impl equivalence with the TypeScript `signet-protocol` package is
/// locked by `tests/canonical_id_vectors.rs`.
pub fn canonical_id(
    pubkey: &[u8; 32],
    created_at: u32,
    kind: u32,
    tags: &[Vec<String>],
    content: &str,
) -> [u8; 32] {
    let hex_pk = hex::encode(pubkey);
    // serde_json's default output is compact (`,` + `:` separators) and emits
    // integers without trailing `.0`, matching the reference impl.
    let payload = serde_json::json!([0, hex_pk, created_at, kind, tags, content]);
    let bytes = serde_json::to_vec(&payload).expect("canonical json should never fail");

    let mut h = Sha256::new();
    h.update(&bytes);
    h.finalize().into()
}
