use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[cfg_attr(test, derive(PartialEq))]
#[serde(rename_all = "PascalCase")]
pub struct DockedEvent {
    pub station_name: String,
    pub station_type: String,
    pub star_system: String,
    pub system_address: u64,

    #[serde(rename = "MarketID")]
    pub market_id: u64,
    pub station_faction: DockedEventStationFaction,

    pub station_government: String,

    #[serde(rename = "StationGovernment_Localised")]
    pub station_government_localised: String,
    pub station_allegiance: String,
    pub station_services: Vec<String>,
    pub station_economy: String,

    #[serde(rename = "StationEconomy_Localised")]
    pub station_economy_localised: String,
    pub station_economies: Vec<DockedEventStationEconomy>,

    #[serde(rename = "DistFromStarLS")]
    pub dist_from_star_ls: f32,
}

#[derive(Debug, Deserialize)]
#[cfg_attr(test, derive(PartialEq))]
#[serde(rename_all = "PascalCase")]
pub struct DockedEventStationFaction {
    pub name: String,
    pub faction_state: String,
}

#[derive(Debug, Deserialize)]
#[cfg_attr(test, derive(PartialEq))]
#[serde(rename_all = "PascalCase")]
pub struct DockedEventStationEconomy {
    pub name: String,

    #[serde(rename = "Name_Localised")]
    pub name_localised: String,
    pub proportion: f32,
}

#[cfg(test)]
mod tests {
    use serde_json::json;
    use crate::models::journal_event_kind::docked_event::DockedEvent;

    #[test]
    fn docked_event_is_parsed_correctly() {
        let value = serde_json::from_str::<DockedEvent>(&r#"
            {
                "StationName": "Jenner Orbital",
                "StationType": "Outpost",
                "StarSystem": "Luhman 16",
                "SystemAddress": 22960358574928,
                "MarketID": 3228883456,
                "StationFaction": {
                    "Name": "Union of Luhman 16 Values Party",
                    "FactionState": "CivilWar"
                },
                "StationGovernment": "$government_Democracy;",
                "StationGovernment_Localised": "Democracy",
                "StationAllegiance": "Federation",
                "StationServices": [
                    "Dock",
                    "Autodock",
                    "BlackMarket",
                    "Commodities",
                    "Contacts",
                    "Exploration",
                    "Missions",
                    "Outfitting",
                    "CrewLounge",
                    "Rearm",
                    "Refuel",
                    "Workshop",
                    "MissionsGenerated",
                    "FlightController",
                    "StationOperations",
                    "Powerplay",
                    "SearchAndRescue"
                ],
                "StationEconomy": "$economy_Refinery;",
                "StationEconomy_Localised": "Refinery",
                "StationEconomies": [
                    {
                        "Name": "$economy_Refinery;",
                        "Name_Localised": "Refinery",
                        "Proportion": 0.76
                    },
                    {
                        "Name": "$economy_Extraction;",
                        "Name_Localised": "Extraction",
                        "Proportion": 0.24
                    }
                ],
                "DistFromStarLS": 10.061876
            }
        "#.to_string());

        assert!(value.is_ok());
    }
}