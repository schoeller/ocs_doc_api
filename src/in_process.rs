use crate::doc_api::{
    DocApi, DocApiError, DocOp, DocumentSnapshot, EntityPayload, Receipt,
};
use crate::schema::TypeRegistry;
use crate::{entity_type_name, Handle};
use acadrust::entities::{Entity, EntityType};
use acadrust::{CadDocument, Handle as AcadHandle};

/// An in-process `DocApi` implementation backed by `acadrust::CadDocument`.
pub struct InProcessDocApi {
    doc: CadDocument,
    name: String,
    registry: TypeRegistry,
    next_doc_handle: u64,
}

impl InProcessDocApi {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            doc: CadDocument::new(),
            name: name.into(),
            registry: crate::object_model(),
            next_doc_handle: 1,
        }
    }

    pub fn from_document(doc: CadDocument, name: impl Into<String>) -> Self {
        Self {
            doc,
            name: name.into(),
            registry: crate::object_model(),
            next_doc_handle: 1,
        }
    }

    pub fn document(&self) -> &CadDocument {
        &self.doc
    }

    pub fn document_mut(&mut self) -> &mut CadDocument {
        &mut self.doc
    }

    fn payload_to_entity(payload: &EntityPayload) -> Result<EntityType, DocApiError> {
        let variant = entity_type_name(&payload.kind);
        let wrapped = serde_json::json!({ variant: payload.data });
        serde_json::from_value(wrapped).map_err(|e| DocApiError::InvalidPayload {
            message: format!("failed to deserialize {} payload: {}", payload.kind, e),
        })
    }

    fn acad_handle(handle: Handle) -> AcadHandle {
        AcadHandle::from(handle.0)
    }
}

impl DocApi for InProcessDocApi {
    fn execute(&mut self, op: DocOp) -> Result<Receipt, DocApiError> {
        match op {
            DocOp::CreateDocument { name } => {
                self.doc = CadDocument::new();
                self.name = name;
                let handle = Handle(self.next_doc_handle);
                self.next_doc_handle += 1;
                Ok(Receipt::DocumentCreated { handle })
            }
            DocOp::GetDocument => {
        let entities: Result<Vec<_>, _> = self
            .doc
            .entities()
            .map(crate::doc_api::entity_to_payload)
            .collect();
                Ok(Receipt::DocumentSnapshot(DocumentSnapshot {
                    name: self.name.clone(),
                    entities: entities?,
                }))
            }
            DocOp::ListEntities => {
                let mut list = Vec::new();
                for entity in self.doc.entities() {
                    let payload = crate::doc_api::entity_to_payload(entity)?;
                    let handle = Handle(entity.common().handle.value());
                    list.push((handle, payload));
                }
                Ok(Receipt::EntityList { entities: list })
            }
            DocOp::CreateEntity { payload } => {
                crate::validate_entity_payload(&payload, &self.registry)?;
                let mut entity = Self::payload_to_entity(&payload)?;
                entity.common_mut().handle = AcadHandle::NULL;
                let handle = self
                    .doc
                    .add_entity(entity)
                    .map_err(|e| DocApiError::KernelOpFailed {
                        op: "create_entity".into(),
                        message: e.to_string(),
                    })?;
                Ok(Receipt::EntityCreated {
                    handle: Handle(handle.value()),
                })
            }
            DocOp::ReadEntity { handle } => {
                let entity = self.doc.get_entity(Self::acad_handle(handle)).ok_or(
                    DocApiError::EntityNotFound { handle },
                )?;
                Ok(Receipt::EntityRead {
                    handle,
                    payload: crate::doc_api::entity_to_payload(entity)?,
                })
            }
            DocOp::UpdateEntity { handle, payload } => {
                crate::validate_entity_payload(&payload, &self.registry)?;
                let mut entity = Self::payload_to_entity(&payload)?;
                entity.common_mut().handle = Self::acad_handle(handle);
                let arc = std::sync::Arc::new(entity);
                self.doc
                    .replace_entity_arc(Self::acad_handle(handle), arc)
                    .ok_or(DocApiError::EntityNotFound { handle })?;
                Ok(Receipt::EntityUpdated { handle })
            }
            DocOp::DeleteEntity { handle } => {
                self.doc
                    .remove_entity(Self::acad_handle(handle))
                    .ok_or(DocApiError::EntityNotFound { handle })?;
                Ok(Receipt::EntityDeleted { handle })
            }
            DocOp::OffsetEntity {
                handle,
                distance,
                side,
            } => {
                let entity = self.doc.get_entity(Self::acad_handle(handle)).ok_or(
                    DocApiError::EntityNotFound { handle },
                )?;
                let lwpolyline = match entity {
                    EntityType::LwPolyline(p) => p.clone(),
                    other => {
                        let source_kind =
                            crate::doc_api::entity_to_payload(other)?.kind;
                        return Err(DocApiError::KernelOpFailed {
                            op: "offset".into(),
                            message: format!("offset is not supported for {}", source_kind),
                        });
                    }
                };
                let side = side.unwrap_or_else(|| {
                    // Default side: positive Y of the entity bounding-box center.
                    let bb = lwpolyline.bounding_box();
                    [
                        (bb.min.x + bb.max.x) * 0.5,
                        (bb.min.y + bb.max.y) * 0.5 + distance.abs(),
                    ]
                });
                let offsets =
                    crate::kernel_ops::KernelOps::offset(&lwpolyline, distance, side);
                let mut results = Vec::new();
                for mut offset_entity in offsets {
                    offset_entity.common.handle = AcadHandle::NULL;
                    let added = self
                        .doc
                        .add_entity(EntityType::LwPolyline(offset_entity))
                        .map_err(|e| DocApiError::KernelOpFailed {
                            op: "offset".into(),
                            message: e.to_string(),
                        })?;
                    let read_back = self.doc.get_entity(added).unwrap();
                    results.push(crate::doc_api::entity_to_payload(read_back)?);
                }
                Ok(Receipt::OffsetResult {
                    source: handle,
                    results,
                })
            }
            DocOp::Unsupported => Err(DocApiError::NotImplemented {
                op: "unsupported".into(),
            }),
        }
    }
}
