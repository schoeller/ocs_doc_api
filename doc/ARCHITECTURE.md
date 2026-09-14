# ocs_doc_api Architecture

This document describes the high-level architecture of `ocs_doc_api` and how it relates to the rest of the OpenCADStudio workspace.

## Goals

1. Provide a stable, language-agnostic object model for `acadrust::CadDocument`.
2. Expose a transport-agnostic runtime API (`DocApi`) that can run in-process or over IPC.
3. Keep the core crate free of `ocs_plugin_api` so it can be used by the kernel, the host, and plugins alike.
4. Generate human-readable API documentation from the same source of truth as the code.

## Layers

```text
┌─────────────────────────────────────────────┐
│  Host / Plugin / CLI                        │
│  uses DocApi trait or LocalDocApiClient     │
├─────────────────────────────────────────────┤
│  ocs_doc_api                                │
│  • schema (TypeRegistry)                    │
│  • doc_api (DocOp, Receipt, DocApi trait)   │
│  • in_process (InProcessDocApi)              │
│  • kernel_ops (KernelOps)                   │
├─────────────────────────────────────────────┤
│  acadrust  +  cadkernel                     │
│  file format        geometry kernel         │
└─────────────────────────────────────────────┘
```

## Core components

### `schema`

The `schema` module defines the public object-model types:

- `TypeRegistry` — the complete set of traced types.
- `TypeInfo` — metadata for one type: kind, fields, enum variants, capabilities.
- `Capability` — a kernel operation attached to an entity type.

The registry is generated at build time by `build.rs` and embedded as JSON. At runtime it is deserialized with `serde_json`.

### `doc_api`

The public operation API:

- `DocOp` — one document operation (CRUD, snapshot, offset, etc.).
- `Receipt` — the result of one `DocOp`.
- `DocApi` trait — `execute(&mut self, op) -> Result<Receipt, DocApiError>`.
- `DocApiExt` — typed convenience methods built on top of `DocApi`.
- `EntityPayload` — serializable entity representation using `serde_json::Value`.
- `validate_entity_payload` — checks an entity payload against the generated registry.

### `in_process`

`InProcessDocApi` wraps `acadrust::CadDocument` and implements `DocApi`. It handles entity lifecycle and delegates geometry operations to `KernelOps`.

### `kernel_ops`

`KernelOps` is a trait that maps `acadrust` entity types to `cadkernel` geometry operations. Implementations are provided for the first-milestone capabilities:

- `Line` → `to_planar_curve`
- `Circle` → `to_planar_curve`
- `LwPolyline` → `offset`

### `ocs_doc_api_ipc`

The optional IPC subcrate provides `LocalDocApiClient`, a callback-based `DocApi` implementation. The caller supplies a dispatch closure; the subcrate encodes/decodes operations as JSON bytes. This avoids a dependency on `ocs_plugin_api` in the first milestone.

## Build pipeline

```text
acadrust (with serde)
        │
        ▼
  serde-reflection
        │
        ▼
  TypeRegistry (build.rs)
        │
        ├──► object_model.json  (embedded)
        ├──► object_model_dispatch.rs  (included under kernel)
        ├──► object_model_docs.md
        └──► doc_api_ops.md
```

`kernel_capabilities.toml` is merged into the registry at build time. If tracing fails, `build.rs` falls back to a minimal registry so the crate still compiles and tests can run.

## Feature flags

| Feature | Enables | Dependencies |
|---|---|---|
| (none) | Object model, registry, docs, payload validation | `serde`, `serde_json` |
| `kernel` | `KernelOps` dispatch | `acadrust`, `cadkernel` |
| `engine` | `DocApi`, `InProcessDocApi`, `DocApiExt` | `kernel` |

## Future extensions

- Proc-macro capability discovery in `cadkernel`.
- Real `ocs_plugin_api` IPC with dedicated request/response variants.
- Typed payload builders generated from the registry.
- Language bindings generated from `object_model.json`.
