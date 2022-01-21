use serde::{Deserialize, Serialize};
use crate::lib::types::permissions::Permissions;

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Role {
    pub name: String,
    pub permissions: Permissions,
    pub colour: Option<String>,

    #[serde(default)]
    pub hoist: bool,

    #[serde(default)]
    pub rank: i16
}
