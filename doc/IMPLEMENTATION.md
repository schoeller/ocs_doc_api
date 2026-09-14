# ocs_doc_api Implementation Notes

This document describes how the crate is implemented and where to find specific pieces of logic.

## Build-time code generation

### `build.rs`

`build.rs` is responsible for producing the artifacts that `src/lib.rs` embeds:

1. Trace selected `acadrust` roots with `serde-reflection`.
2. Map the `serde-reflection::Registry` into the local `schema::TypeRegistry`.
3. Load `kernel_capabilities.toml` and merge capabilities into the registry.
4. Generate Markdown docs from the registry and `doc_api_ops.toml`.
5. Write `object_model.json`, `object_model_dispatch.rs`, and the Markdown files to `OUT_DIR`.

Because `build.rs` cannot depend on the crate it is building, it includes `src/schema.rs` directly via `#[path = "src/schema.rs"] mod schema;`. This guarantees the generated JSON matches the runtime schema exactly.

### Tracing strategy

The build script traces:

- `CadDocument`
- `HeaderVariables`
- `EntityType`
- `EntityCommon`
- `ObjectType`

`serde-reflection` needs concrete samples for non-unit enum variants. The build script registers samples for `Line`, `Circle`, `LwPolyline`, `MText`, and `Point`. If tracing fails for any reason, `build.rs` prints a warning and falls back to a minimal registry containing `Line` and `LwPolyline`.

### Capability merge

`kernel_capabilities.toml` contains entries like:

```toml
[[capability]]
entity = "Line"
name = "to_planar_curve"
output = "Option<cadkernel::space::PlanarCurve>"
```

The build script validates that the entity string exists in the registry and attaches the capability to that type.

## Runtime implementation

### `src/schema.rs`

Defines the public schema types. `TypeRegistry` is deserialized from the embedded `object_model.json`.

### `src/doc_api.rs`

- `DocOp` and `Receipt` are internally tagged enums so JSON payloads are self-describing.
- `Handle` is defined in `src/lib.rs` and re-used here.
- `validate_entity_payload` checks that a payload's `kind` maps to a known struct type and that all required fields are present.
- `entity_to_payload` serializes an `acadrust::EntityType` and splits the externally tagged wrapper into `kind` + `data`.

### `src/in_process.rs`

`InProcessDocApi` stores:

- `doc: CadDocument`
- `name: String`
- `registry: TypeRegistry`
- `next_doc_handle: u64`

`CreateDocument` resets `doc`, sets `name`, and allocates a monotonically increasing document handle.

`CreateEntity` validates the payload, deserializes it into an `EntityType`, clears the handle so `CadDocument::add_entity` allocates a fresh one, and returns the allocated handle.

`UpdateEntity` deserializes the payload, forces the entity's handle to the requested handle, and uses `CadDocument::replace_entity_arc`.

`OffsetEntity` matches on `EntityType::LwPolyline`, runs `KernelOps::offset`, adds each result entity to the document, and returns their payloads.

### `src/kernel_ops.rs`

`KernelOps` provides default no-op methods. Specific entity types override them:

- `Line::to_planar_curve` builds a `cadkernel::geom2d::Curve::Line` from `start`/`end`.
- `Circle::to_planar_curve` builds a `cadkernel::geom2d::Curve::Circle` from `center`/`radius`.
- `LwPolyline::offset` converts to a `cadkernel::geom2d::Polyline`, calls `cadkernel::geom2d::offset::offset_polyline`, and converts each result back.

### `src/convert.rs`

Conversion helpers between `acadrust` and `cadkernel` types. Polyline conversion preserves the source `LwPolyline`'s elevation, normal, and common data.

### `src/bin/generate_docs.rs`

A small CLI that writes the embedded Markdown docs to disk. It uses `clap` for argument parsing and requires no special features.

## IPC subcrate

### `ocs_doc_api_ipc/src/lib.rs`

`LocalDocApiClient` implements `DocApi` by encoding operations as JSON bytes and passing them to a caller-provided dispatch closure. The closure receives `(tab_id, bytes)` and returns response bytes.

`encode_receipt` converts `Result<Receipt, DocApiError>` into a `Receipt::Error` on failure so the wire format is always a single `Receipt`. `LocalDocApiClient::execute` maps an incoming `Receipt::Error` back to `DocApiError::KernelOpFailed`.

JSON was chosen over `bincode` because `EntityPayload` contains `serde_json::Value`, which `bincode` cannot deserialize.

## Testing

Tests live in `tests/` and are gated by feature flags:

- `core_tests.rs` — always compiled; tests JSON parsing, capability lookup, registry round-trip, and embedded docs.
- `engine_tests.rs` — compiled under `engine`; tests payload validation, in-process offset, and `doc_api_ops.toml` sync.
- `kernel_tests.rs` — compiled under `kernel`; tests `KernelOps` dispatch.

The IPC subcrate has its own `tests/ipc_tests.rs`.

## Known limitations

- The object model is traced from a representative set of entity samples, not exhaustive enumeration of all 41 entity types.
- `OffsetEntity` only supports `LwPolyline` in the first milestone.
- `OpenDocument` and `SaveDocument` are not implemented.
- Typed payload builders are not generated; callers construct `serde_json::Value` payloads.
