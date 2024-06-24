use std::path::PathBuf;
use serde::{Serialize, Deserialize};

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Metadata {
    pub template: Option<PathBuf>
}
