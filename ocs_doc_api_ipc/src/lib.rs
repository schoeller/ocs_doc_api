use ocs_doc_api::{DocApi, DocApiError, DocOp, Receipt};

/// A local callback-based `DocApi` client.
///
/// The host provides a `dispatch` closure that receives `(tab_id, bytes)` and
/// returns response bytes. This avoids a dependency on `ocs_plugin_api` for the
/// first milestone.
pub struct LocalDocApiClient<'a> {
    tab_id: u64,
    dispatch: &'a mut dyn FnMut(u64, &[u8]) -> Vec<u8>,
}

impl<'a> LocalDocApiClient<'a> {
    pub fn new(tab_id: u64, dispatch: &'a mut dyn FnMut(u64, &[u8]) -> Vec<u8>) -> Self {
        Self { tab_id, dispatch }
    }
}

impl DocApi for LocalDocApiClient<'_> {
    fn execute(&mut self, op: DocOp) -> Result<Receipt, DocApiError> {
        let bytes = encode_op(&op)?;
        let response = (self.dispatch)(self.tab_id, &bytes);
        let receipt = decode_receipt(&response)?;
        match receipt {
            Receipt::Error { message } => Err(DocApiError::KernelOpFailed {
                op: "ipc".into(),
                message,
            }),
            other => Ok(other),
        }
    }
}

/// Encode a `DocOp` using JSON bytes.
pub fn encode_op(op: &DocOp) -> Result<Vec<u8>, DocApiError> {
    serde_json::to_vec(op).map_err(|e| DocApiError::Serialize {
        message: e.to_string(),
    })
}

/// Decode raw bytes into a `DocOp`.
pub fn decode_op(bytes: &[u8]) -> Result<DocOp, DocApiError> {
    serde_json::from_slice(bytes).map_err(|e| DocApiError::Deserialize {
        message: e.to_string(),
    })
}

/// Encode a receipt for the wire, converting errors into `Receipt::Error`.
pub fn encode_receipt(result: Result<Receipt, DocApiError>) -> Result<Vec<u8>, DocApiError> {
    let receipt = match result {
        Ok(receipt) => receipt,
        Err(err) => Receipt::Error {
            message: err.to_string(),
        },
    };
    serde_json::to_vec(&receipt).map_err(|e| DocApiError::Serialize {
        message: e.to_string(),
    })
}

/// Decode raw bytes into a `Receipt`.
pub fn decode_receipt(bytes: &[u8]) -> Result<Receipt, DocApiError> {
    serde_json::from_slice(bytes).map_err(|e| DocApiError::Deserialize {
        message: e.to_string(),
    })
}
