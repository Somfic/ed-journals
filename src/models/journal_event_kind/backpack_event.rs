use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[cfg_attr(test, derive(PartialEq))]
#[serde(rename_all = "PascalCase")]
pub struct BackpackEvent {
    pub items: Vec<BackpackEventObject>,
    pub components: Vec<BackpackEventObject>,
    pub consumables: Vec<BackpackEventObject>,
    pub data: Vec<BackpackEventObject>,
}

// TODO this is the same as ship_locker_event
#[derive(Debug, Deserialize)]
#[cfg_attr(test, derive(PartialEq))]
#[serde(rename_all = "PascalCase")]
pub struct BackpackEventObject {
    // TODO check if this can be an enum
    pub name: String,

    #[serde(rename = "Name_Localised")]
    pub name_localised: String,
    pub owner_id: u64,

    #[serde(rename = "MissionID")]
    pub mission_id: Option<u64>,
    pub count: u16,
}