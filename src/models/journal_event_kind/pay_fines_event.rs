use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[cfg_attr(test, derive(PartialEq))]
#[serde(rename_all = "PascalCase")]
pub struct PayFinesEvent {
    pub amount: u64,
    pub broker_percentage: f32,
    pub all_fines: bool,
    pub faction: Option<String>,

    #[serde(rename = "ShipID")]
    pub ship_id: u8,
}