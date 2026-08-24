# Security and persistence contract

## Authority boundary

Shared Auth is the sole identity authority. Apostille Me accepts only a verified,
active Shared Auth subject and session for its exact API audience. Tenant IDs,
organization IDs, memberships, and roles are never taken from identity claims:
the product resolves them from `apme_tenant_memberships` for every request and
WebSocket subscription. A valid identity without an active product membership
has no Apostille Me access.

The Rust API pins the official Shared Auth guard and client at immutable commits.
The guard performs local ES256/JWKS validation; protected introspection supplies
the immediate session/revocation decision. Missing configuration, unavailable
verification dependencies, expired tokens, revoked sessions, subject/session
mismatches, and the wrong audience all fail closed.

HTTP and WebSocket authorization use the same sequence:

1. Verify identity and the exact audience without logging the credential.
2. Confirm the session is active through protected introspection.
3. Resolve the requested tenant and role from PostgreSQL.
4. Apply product capabilities and tenant predicates before returning data or
   creating a subscription.

Tenant selection is explicit. Every mutable row and subscription carries a
tenant UUID; server-side queries never infer a tenant from a case identifier.
Logs and errors use internal correlation IDs and must not contain bearer tokens,
session credentials, raw document data, client references, or object-store keys.

## Durable mutation contract

Apply `sql/002_tenant_case_persistence.sql` in a transaction before serving
traffic. Create commands are serialized by `(tenant_id, idempotency_key)` and
persist the canonical request SHA-256 with the result. An identical retry returns
the original case and event; reuse with a different request fails without a
mutation. Transitions lock the case, compare `expected_version`, enforce the
workflow edge, and append exactly one event. A competing stale transition fails
without an event or audit receipt.

The idempotency key is tenant-scoped, not globally scoped. Request hashes are
computed from a versioned canonical representation in the API; generated clients
must reproduce the fixtures in `fixtures/` before a release is promoted.

## Documents, retention, and audit

PostgreSQL stores sanitized case metadata plus an opaque encrypted-object
reference, the ciphertext SHA-256, and the encryption-key version. A reference
containing a URL is rejected. Plain document bytes, presigned URLs, encryption
keys, and bearer credentials have no persistence column.

Case events and tenant audit receipts are append-only hash chains. Their SHA-256
inputs include the previous digest and a UTC timestamp representation, making
verification stable across session time zones and backup/restore. Run
`apme_verify_case_audit(case_id)` and `apme_verify_tenant_audit(tenant_id)` during
restore and integrity canaries. A missing, reordered, or modified historical
entry makes verification fail.

`apme_apply_retention(now, actor)` deterministically tombstones expired cases,
scrubs encrypted references and identifying operational metadata, and records a
deletion event and audit receipt. Legal-held cases are excluded. Appointment and
deadline rows remain tenant-bound and must be included in the same backup and
restore boundary. Key rotation updates encrypted objects out of band; the product
must validate ciphertext SHA-256 before atomically replacing the opaque reference
and key version.

## Deployment gates

Before production promotion:

- Run Rust unit tests and both JSON fixtures in every supported generated client.
- Run two-session PostgreSQL create/transition concurrency canaries.
- Exercise two tenants concurrently over REST and WebSocket and prove that no
  event or case crosses the selected tenant.
- Prove expired and revoked sessions fail for both transports.
- Exercise allowlisted CORS preflight and reject an unlisted origin.
- Restore a backup into an isolated database, then verify row counts, encrypted
  object-reference continuity, every case event chain, and every tenant receipt
  chain.
- Rotate an encryption key version against disposable ciphertext and verify the
  old and new references without exposing object credentials.

The contract and database canaries do not replace environment-specific backup,
object-store, key-management, or Shared Auth deployment drills.
