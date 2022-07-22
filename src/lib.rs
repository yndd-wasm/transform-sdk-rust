pub mod metadata;
pub use wapc_guest;

use crate::metadata::ProtocolVersion;

pub fn protocol_version_guest(_payload: &[u8]) -> wapc_guest::CallResult {
    Ok(serde_json::to_vec(&ProtocolVersion::default())?)
}
