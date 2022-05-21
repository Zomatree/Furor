use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct PermissionsOverwrite {
    a: u64,
    d: u64,
}
