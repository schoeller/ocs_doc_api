# DocApi Operations

| Operation | Description |
|---|---|
| `CreateDocument` | Create a new empty document and set it as the active document. |
| `GetDocument` | Return a lightweight snapshot of the active document. |
| `ListEntities` | List every entity in the active document as (handle, payload) pairs. |
| `CreateEntity` | Add a new entity from an `EntityPayload`. A unique handle is allocated. |
| `ReadEntity` | Return the `EntityPayload` for the entity with the given handle. |
| `UpdateEntity` | Replace the entity at the given handle with a new payload. |
| `DeleteEntity` | Remove the entity with the given handle from the document. |
| `OffsetEntity` | Offset a supported entity by a distance. First milestone supports `LwPolyline`. |
