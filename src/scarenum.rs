use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct ScarEnum {
    pub name: String,
    pub values: Vec<ScarEnumValue>
}

#[derive(Serialize, Deserialize, Clone)]
pub struct ScarEnumValue {
    pub name: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>
}
