use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[cfg_attr(test, derive(PartialEq))]
#[serde(rename_all = "PascalCase")]
pub struct DockingRequestedEvent {
    pub station_name: String,
    pub station_type: String,

    #[serde(rename = "MarketID")]
    pub market_id: u64,

    // TODO check actual type and value. Is this an array or enum or something?
    pub landing_pads: String,
}