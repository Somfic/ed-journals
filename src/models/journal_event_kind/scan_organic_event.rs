use serde::Deserialize;
use crate::models::journal_event_kind::shared::exploration::genus::Genus;

#[derive(Debug, Deserialize)]
#[cfg_attr(test, derive(PartialEq))]
#[serde(rename_all = "PascalCase")]
pub struct ScanOrganicEvent {
    // Possible values seem to be "Sample", "Analyze", "Log". It seems that the first scan for
    // a bio species uses `Sample`, then the second consists of two back to back events: one with
    // `Sample` and the one immediately after with `Analyze`. The contents seem to be the same. And
    // the third and last entry seems to be `Log`.
    pub scan_type: ScanOrganicEventScanType,

    pub genus: Genus,

    #[serde(rename = "Genus_Localised")]
    pub genus_localised: String,

    // TODO replace this with an enum
    pub species: String,

    #[serde(rename = "Species_Localised")]
    pub species_localised: String,

    // TODO replace this with an enum
    pub variant: String,

    #[serde(rename = "Variant_Localised")]
    pub variant_localised: String,

    pub system_address: u64,
    pub body: u8,
}

#[derive(Debug, Deserialize)]
#[cfg_attr(test, derive(PartialEq))]
#[serde(rename_all = "PascalCase")]
pub enum ScanOrganicEventScanType {
    Sample,
    Analyse,
    Log,
}