use std::str::FromStr;
use serde::Deserialize;
use serde_json::Value;
use crate::models::journal_event_kind::shared::ship::ship_module::ship_hardpoint_module::hardpoint_type::HardpointType;

#[derive(Debug, Deserialize)]
#[cfg_attr(test, derive(PartialEq))]
pub enum HardpointModule {
    #[serde(rename = "beamlaser")]
    BeamLaser,

    #[serde(rename = "pulselaser")]
    PulseLaser,

    #[serde(rename = "multicannon")]
    MultiCannon,

    #[serde(rename = "guardian_gausscannon")]
    GuardianGaussCannon,

    #[serde(rename = "atventdisruptorpylon")]
    NaniteTorpedoPylon,

    #[serde(rename = "heatsinklauncher")]
    HeatsinkLauncher,

    #[serde(alias = "causticsinklauncher")]
    CausticSinkLauncher,

    #[serde(rename = "plasmapointdefence")]
    PointDefenceTurret,

    #[serde(rename = "chafflauncher")]
    ChaffLauncher,

    #[cfg(not(feature = "strict"))]
    #[serde(untagged)]
    Unknown(String),
}

impl FromStr for HardpointModule {
    type Err = serde_json::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        serde_json::from_value(Value::String(s.to_string()))
    }
}

impl HardpointModule {
    pub fn hardpoint_type(&self) -> HardpointType {
        match self {
            HardpointModule::HeatsinkLauncher
            | HardpointModule::PointDefenceTurret
            | HardpointModule::ChaffLauncher
            | HardpointModule::CausticSinkLauncher => HardpointType::Utility,

            _ => HardpointType::FullSized,
        }
    }

    pub fn is_full_sized(&self) -> bool {
        matches!(self.hardpoint_type(), HardpointType::FullSized)
    }

    pub fn is_utility(&self) -> bool {
        matches!(self.hardpoint_type(), HardpointType::Utility)
    }
}