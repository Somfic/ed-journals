use serde::{Deserialize, Serialize};

use crate::modules::odyssey::Item;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct BackpackEvent {
    pub items: Vec<BackpackEventObject>,
    pub components: Vec<BackpackEventObject>,
    pub consumables: Vec<BackpackEventObject>,
    pub data: Vec<BackpackEventObject>,
}

// TODO this is the same as ship_locker_event
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct BackpackEventObject {
    pub name: Item,

    #[serde(rename = "Name_Localised")]
    pub name_localized: Option<String>,

    #[serde(rename = "OwnerID")]
    pub owner_id: u64,

    #[serde(rename = "MissionID")]
    pub mission_id: Option<u64>,
    pub count: u16,
}
