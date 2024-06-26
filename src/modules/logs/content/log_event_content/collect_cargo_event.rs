use serde::{Deserialize, Serialize};

use crate::modules::trading::Commodity;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct CollectCargoEvent {
    #[serde(rename = "Type")]
    pub kind: Commodity,

    #[serde(rename = "Type_Localised")]
    pub type_localized: Option<String>,

    pub stolen: bool,
}
