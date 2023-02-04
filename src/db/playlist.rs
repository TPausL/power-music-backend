

use serde::{Deserialize, Serialize};

use super::{CanBeStored, DB};

#[derive(Debug, Deserialize, Serialize, Clone, CanBeStored)]
pub struct DBPlaylist {
    pub id: String,
    pub hidden: bool,
}
