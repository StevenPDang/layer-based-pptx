# Spec: Rust Document Engine

## Objective

Build a portable Rust engine that owns the presentation document model and PPTX
processing for the React editor. The first milestone supports creating a deck
with positioned text layers and exporting it as a valid `.pptx`. Later
milestones add safe PPTX import and round-trip preservation without changing the
editor's internal layer-based model.

The engine must remain independent of React and Electron. Thin adapters expose
the same versioned command contract to the browser through WebAssembly and to
the initial Electron application through a newline-delimited JSON sidecar.

## Tech Stack

- Rust stable, using the 2024 edition
- `serde` and `serde_json` for the document and command contracts
- OOXML ZIP/XML processing behind a dedicated PPTX adapter
- `wasm-bindgen` for the future browser adapter
- React, TypeScript, and Electron as consumers of the engine API

Exact crate dependencies and versions are selected during implementation and
recorded in `Cargo.lock`.

## Commands

These commands become required when the Rust workspace is introduced:

```bash
cargo build --workspace
cargo test --workspace
cargo fmt --all --check
cargo clippy --workspace --all-targets --all-features -- -D warnings
pnpm build
pnpm lint
```

During the specification-only phase, the existing application is verified with
`pnpm build` and `pnpm lint`.

## Project Structure

```text
crates/
  presentation-model/  Format-independent document and layer types
  pptx/                 OOXML import, export, and preservation logic
  engine-protocol/      Versioned commands, responses, and errors
  engine-cli/           Newline-delimited JSON sidecar for Electron
  engine-wasm/          Browser adapter; added after the desktop milestone
fixtures/
  presentations/        Small source documents used by integration tests
  pptx/                 Reference and generated PPTX packages
react/                  React editor and, initially, Electron integration
docs/                   Product and engine specifications
```

Dependencies flow inward: adapters and PPTX processing may depend on the model
and protocol, while the model must not depend on OOXML, Electron, React, or
WebAssembly.

## Code Style

Rust types use `UpperCamelCase`, modules and functions use `snake_case`, and
serialized fields use `camelCase`. Public types are documented and fallible
operations return typed errors rather than panicking.

```rust
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct TextLayer {
    pub id: LayerId,
    pub frame: Frame,
    pub text: String,
}

pub fn export_pptx(document: &Presentation) -> Result<Vec<u8>, ExportError> {
    validate(document)?;
    pptx::write(document)
}
```

Run `cargo fmt`; treat all Clippy warnings as errors. Avoid leaking OOXML names
or transport-specific types into `presentation-model`.

## Testing Strategy

- Unit tests live beside Rust modules and cover validation, geometry, IDs, and
  JSON serialization.
- Crate integration tests live under each crate's `tests/` directory and cover
  protocol request/response behavior and PPTX package structure.
- Golden fixtures verify deterministic semantic output. Tests compare relevant
  XML and relationships, not raw ZIP bytes or timestamps.
- Electron integration tests start the sidecar and cover one successful command,
  malformed input, and graceful shutdown.
- A release checkpoint manually opens generated files in current PowerPoint and
  LibreOffice without a repair warning.
- No percentage coverage target is required initially; every supported command,
  validation rule, and reported bug requires a regression test.

## Boundaries

- **Always:** validate documents at engine boundaries; use stable IDs and
  explicit units; version serialized commands; preserve unknown imported PPTX
  parts when safe; run formatting, linting, and tests before merging.
- **Ask first:** add a runtime dependency; change the serialized document or
  command contract incompatibly; discard unsupported imported PPTX content;
  expand the first milestone beyond positioned text layers.
- **Never:** place UI or Electron logic in the core model; panic on user-provided
  files or commands; silently drop unsupported PPTX content; commit secrets,
  generated build output, or proprietary PPTX fixtures.

## Success Criteria

Issue #7 is complete when this specification is reviewed and stored in the
repository. The first implementation milestone is complete when:

1. A Rust `Presentation` containing one slide and positioned text layers
   round-trips through JSON without data loss.
2. The model exports to a `.pptx` that opens in PowerPoint and LibreOffice
   without repair warnings and preserves text and basic geometry.
3. Electron can start the Rust sidecar, send a correlated versioned request,
   receive a structured response, and shut it down cleanly.
4. Invalid documents and malformed commands return structured errors without
   crashing the process.
5. Rust workspace tests, formatting, Clippy, React build, and React lint pass.

## Open Questions

- Which geometry unit is canonical internally: English Metric Units, points, or
  a unit-independent integer scale?
- Which text properties join plain content in the first export milestone?
- What compatibility policy and size limit govern preservation of unknown PPTX
  package parts during import?
- When should Electron move from the debuggable sidecar protocol to a native or
  WebAssembly binding, if ever? The change requires profiling evidence.

These questions do not block the text-layer export milestone; each must be
resolved in the living spec before its affected feature is implemented.
