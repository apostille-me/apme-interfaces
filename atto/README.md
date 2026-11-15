# AttoLang case-status contract fixture

`atto/case_status.atto` is an experimental, self-contained AttoLang
consumer of the six `CaseStatus` wire values from `schemas/case.json` and
`src/lib.rs`. Its `main()` is a positive compiler fixture whose expected
guest exit code is 42. It does not parse case payloads or grant access to
client documents; no real identity or case data is included.

Rust's `tests/atto_case_status_contract.rs` validates that the JSON Schema,
Rust serialization, and the Atto source's status mappings agree. This is a
**host-only source and wire-contract test**, not evidence that every Atto
compiler/backend can execute the fixture.

Runtime admission requires a pinned AttoLang compiler commit, a successful
compile of the committed `.atto` source, and guest execution confirming exit
42. Report targets independently as passed/failed/unsupported/not executed.
The canonical `apme-interfaces` wire schema remains authoritative; any
future Atto imports must preserve the exact snake_case values.
