# apme-interfaces

Canonical Rust, JSON Schema, OpenAPI, AsyncAPI, and PostgreSQL contracts for Apostille Me.

**Product:** Apostille Me — Case operations for visa and apostille consulting.

Track sanitized client references, document workflows, destination jurisdictions, appointments, deadlines, and case events for a visa and apostille consulting firm.

## Safety and production boundary

This software is an operational starter and does not provide legal advice. Keep identity documents and sensitive case files out of logs and this bootstrap data model; production use requires encryption, access controls, retention rules, auditability, and jurisdiction-specific professional review.

This repository is an executable bootstrap, not a production deployment. Before live
use, add authentication, tenant authorization, rate limits, durable migrations,
observability, backups, incident response, dependency review, and secret management.
## Contract authority

- `src/lib.rs` is the Rust model and validation surface.
- `schemas/` contains JSON Schema Draft 2020-12 wire contracts.
- `openapi.yaml` defines REST endpoints.
- `asyncapi.yaml` defines WebSocket event envelopes.
- `sql/` provides a deny-by-default PostgreSQL/Supabase migration baseline.
- `fixtures/` provides cross-language conformance examples.

Downstream services should consume a tagged release and run fixture compatibility
tests before deployment.
