use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct CommunityGoalDiscardEvent {
    #[serde(rename = "CGID")]
    pub cgid: u64,
    pub name: String,
    pub system: String,
}
