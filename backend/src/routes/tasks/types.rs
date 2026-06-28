use serde::{Deserialize, Serialize};

/// Persisted kanban state. Mirrors the WASM `BoardData` in
/// `frontend/src/types.rs`. The `version` field implements optimistic
/// concurrency: the client must send back the version it loaded; on
/// mismatch the server returns 409 Conflict and the client refetches.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Column {
    pub name: String,
    pub tasks: Vec<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Board {
    pub name: String,
    pub columns: indexmap::IndexMap<String, Column>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct BoardData {
    /// Monotonic version. Incremented on every successful save. Missing
    /// in legacy files (defaulted to 0 on read).
    #[serde(default)]
    pub version: u64,
    pub boards: indexmap::IndexMap<String, Board>,
    #[serde(rename = "activeBoard", default)]
    pub active_board: String,
}

impl Default for BoardData {
    fn default() -> Self {
        Self {
            version: 0,
            boards: indexmap::IndexMap::new(),
            active_board: String::new(),
        }
    }
}
