use crate::modules::shared::materials::material::Material;
use crate::modules::shared::materials::material_category::MaterialCategory;
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct ScientificResearchEvent {
    #[serde(rename = "MarketID")]
    pub market_id: u64,
    pub name: Material,
    pub category: MaterialCategory,
    pub count: u8,
}