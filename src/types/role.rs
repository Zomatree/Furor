use serde::{Deserialize, Serialize};
use crate::types::permissions::PermissionsOverwrite;

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Role {
    pub name: String,
    pub permissions: PermissionsOverwrite,
    pub colour: Option<String>,

    #[serde(default)]
    pub hoist: bool,

    #[serde(default)]
    pub rank: i16
}
