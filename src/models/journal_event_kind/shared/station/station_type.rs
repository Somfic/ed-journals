use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[cfg_attr(test, derive(PartialEq))]
pub enum StationType {
    Orbis,
    Coriolis,
    Ocellus,
    Outpost,
    FleetCarrier,
    MegaShip,
    CraterOutpost,
    CraterPort,
    OnFootSettlement,

    #[cfg(not(feature = "strict"))]
    #[serde(untagged)]
    Unknown(String),
}

// impl FromStr for StationType {
//     type Err = String;
//
//     fn from_str(s: &str) -> Result<Self, Self::Err> {
//         match s {
//             "Orbis" => Ok(StationType::Orbis),
//             "Coriolis" => Ok(StationType::Coriolis),
//             "Ocellus" => Ok(StationType::Ocellus),
//             "Outpost" => Ok(StationType::Outpost),
//             "FleetCarrier" => Ok(StationType::FleetCarrier),
//
//             #[cfg(not(feature = "strict"))]
//             _ => Ok(StationType::Unknown(s.to_string())),
//
//             #[cfg(feature = "strict")]
//             _ => Err(s.to_string()),
//         }
//     }
// }
//
// from_str_deserialize_impl!(StationType);