use serde::{Deserialize, Serialize};

use crate::modules::ship::{ShipModule, ShipSlot, ShipType};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct LoadoutEvent {
    pub ship: ShipType,

    #[serde(rename = "ShipID")]
    pub ship_id: u32,
    pub ship_name: String,
    pub ship_ident: String,
    pub modules: Vec<LoadoutModule>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct LoadoutModule {
    pub slot: ShipSlot,
    pub item: ShipModule,
    pub on: bool,
    pub priority: u8,
    pub health: f32,

    // TODO check when this value is used
    pub value: Option<u32>,
    pub ammo_in_clip: Option<u32>,
}
