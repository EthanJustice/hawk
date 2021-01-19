// std

// crates
use serde::{Deserialize, Serialize};

// local

#[derive(Serialize, Deserialize, Debug)]
pub enum ExportFileType {
    CSV,
    HTML,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ExportRequest {
    pub export_type: ExportFileType,
}
