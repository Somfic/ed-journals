use std::str::FromStr;
use serde::Deserialize;
use crate::models::journal_event_kind::shared::trading::commodity::Commodity;

#[derive(Debug, Deserialize)]
#[cfg_attr(test, derive(PartialEq))]
pub enum PlanetarySignalType {
    #[serde(rename = "$SAA_SignalType_Human;")]
    Human,

    #[serde(rename = "$SAA_SignalType_Biological;")]
    Biological,

    #[serde(rename = "$SAA_SignalType_Geological;")]
    Geological,

    #[serde(rename = "$SAA_SignalType_Thargoid;")]
    Thargoid,

    #[serde(untagged)]
    Commodity(Commodity),

    #[cfg(not(feature = "strict"))]
    #[serde(untagged)]
    Unknown(String),
}