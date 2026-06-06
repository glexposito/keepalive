use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComponentGroup {
    pub id: Uuid,
    pub name: String,
    pub display_order: i32,
}
