use std::num::ParseIntError;
use std::str::FromStr;
use once_cell::sync::Lazy;
use regex::Regex;
use thiserror::Error;
use crate::from_str_deserialize_impl;
use crate::models::journal_event_content::shared::ship::ship_module::module_class::{ModuleClass, ModuleClassError};
use crate::models::journal_event_content::shared::ship::ship_module::ship_internal_module::armor_grade::ArmorGrade;
use crate::models::journal_event_content::shared::ship::ship_module::ship_internal_module::armor_module::{ArmorModule, ArmorModuleError};
use crate::models::journal_event_content::shared::ship::ship_module::ship_internal_module::internal_module::InternalModule;
use crate::models::journal_event_content::shared::ship::ship_module::ship_internal_module::internal_type::InternalType;

pub mod internal_module;
pub mod internal_type;
pub mod armor_grade;
pub mod armor_module;

#[derive(Debug, Clone, PartialEq)]
pub struct ShipInternalModule {
    pub module: InternalModule,
    pub size: u8,
    pub class: ModuleClass,
}

#[derive(Debug, Error)]
pub enum ShipInternalModuleError {
    #[error("Failed to parse module: {0}]")]
    FailedToParseModule(#[source] serde_json::Error),

    #[error("Failed to parse size: {0}")]
    FailedToParseSize(#[from] ParseIntError),

    #[error("Failed to parse class number: {0}")]
    FailedToParseClassNumber(#[source] ParseIntError),

    #[error(transparent)]
    ArmorModuleError(#[from] ArmorModuleError),

    #[error(transparent)]
    UnknownClass(#[from] ModuleClassError),

    #[error("Failed to parse internal ship module: '{0}'")]
    FailedToParse(String),
}

const SHIP_INTERNAL_MODULE_REGEX: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r#"^\$?[iI]nt_([a-zA-Z_]+?)(_[sS]ize(1|2|3|4|5|6|7|8))?(_[cC]lass(1|2|3|4|5))?(_[tT]iny)?(_[a-zA-Z_]+?)?(_name;)?$"#).unwrap()
});

impl FromStr for ShipInternalModule {
    type Err = ShipInternalModuleError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let armor_result = ArmorModule::from_str(s);

        match armor_result {
            Ok(armor_module) => return Ok(ShipInternalModule {
                size: 1,
                class: (&armor_module).into(),
                module: InternalModule::Armor(armor_module),
            }),
            Err(ArmorModuleError::FailedToParse(_)) => {},
            Err(e) => return Err(e.into()),
        }

        let Some(captures) = SHIP_INTERNAL_MODULE_REGEX.captures(s) else {
            return Err(ShipInternalModuleError::FailedToParse(s.to_string()));
        };

        let module_string = captures
            .get(1)
            .expect("Should have been captured already")
            .as_str();

        let module_suffix = captures
            .get(7)
            .map(|capture| capture.as_str())
            .unwrap_or_default();

        let module = format!("{}{}", module_string, module_suffix)
            .parse()
            .map_err(ShipInternalModuleError::FailedToParseModule)?;

        if captures.get(6).is_some() {
            return Ok(ShipInternalModule {
                module,
                size: 1,
                class: ModuleClass::I,
            });
        }

        let size = match captures.get(3) {
            Some(capture) => capture.as_str().parse()?,
            None => 1,
        };

        let class = match captures.get(5) {
            Some(capture) => capture
                .as_str()
                .parse::<u8>()
                .map_err(ShipInternalModuleError::FailedToParseClassNumber)?
                .try_into()?,
            None => ModuleClass::E,
        };

        Ok(ShipInternalModule {
            module,
            size,
            class,
        })
    }
}

from_str_deserialize_impl!(ShipInternalModule);

impl ShipInternalModule {
    pub fn internal_type(&self) -> InternalType {
        self.module.internal_type()
    }

    pub fn is_core(&self) -> bool {
        self.module.is_core()
    }

    pub fn is_optional(&self) -> bool {
        self.module.is_optional()
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use crate::models::journal_event_content::shared::ship::ship_module::module_class::ModuleClass;
    use crate::models::journal_event_content::shared::ship::ship_module::ship_internal_module::armor_grade::ArmorGrade;
    use crate::models::journal_event_content::shared::ship::ship_module::ship_internal_module::armor_module::ArmorModule;
    use crate::models::journal_event_content::shared::ship::ship_module::ship_internal_module::internal_module::InternalModule;
    use crate::models::journal_event_content::shared::ship::ship_module::ship_internal_module::ShipInternalModule;
    use crate::models::journal_event_content::shared::ship::ship_type::ShipType;

    #[test]
    fn ship_internal_module_test_cases_are_parsed_correctly() {
        let test_cases = [
            (
                "$int_dronecontrol_collection_size3_class5_name;",
                ShipInternalModule {
                    module: InternalModule::CollectorLimpetController,
                    size: 3,
                    class: ModuleClass::A,
                },
            ),
            (
                "int_engine_size5_class5",
                ShipInternalModule {
                    module: InternalModule::Thrusters,
                    size: 5,
                    class: ModuleClass::A,
                },
            ),
            (
                "$int_engine_size5_class5_name;",
                ShipInternalModule {
                    module: InternalModule::Thrusters,
                    size: 5,
                    class: ModuleClass::A,
                },
            ),
            (
                "$int_supercruiseassist_name;",
                ShipInternalModule {
                    module: InternalModule::SupercruiseAssist,
                    size: 1,
                    class: ModuleClass::E,
                },
            ),
            (
                "$int_detailedsurfacescanner_tiny_name;",
                ShipInternalModule {
                    module: InternalModule::DetailedSurfaceScanner,
                    size: 1,
                    class: ModuleClass::I,
                },
            ),
            (
                "$int_lifesupport_size4_class1_name;",
                ShipInternalModule {
                    module: InternalModule::LifeSupport,
                    size: 4,
                    class: ModuleClass::E,
                },
            ),
            (
                "Int_Hyperdrive_Size5_Class5",
                ShipInternalModule {
                    module: InternalModule::FrameShiftDrive,
                    size: 5,
                    class: ModuleClass::A,
                },
            ),
            (
                "$int_cargorack_size7_class1_name;",
                ShipInternalModule {
                    module: InternalModule::CargoRack,
                    size: 7,
                    class: ModuleClass::E,
                },
            ),
            (
                "$int_shieldgenerator_size8_class3_fast_name;",
                ShipInternalModule {
                    module: InternalModule::BiWeaveShieldGenerator,
                    size: 8,
                    class: ModuleClass::C,
                },
            ),
            (
                "$type9_military_armour_grade1_name;",
                ShipInternalModule {
                    module: InternalModule::Armor(ArmorModule {
                        ship: ShipType::Type10,
                        grade: ArmorGrade::LightweightAlloy,
                    }),
                    size: 1,
                    class: ModuleClass::C,
                },
            ),
            (
                "krait_mkii_armour_grade3",
                ShipInternalModule {
                    module: InternalModule::Armor(ArmorModule {
                        ship: ShipType::KraitMkII,
                        grade: ArmorGrade::MilitaryGradeComposite,
                    }),
                    size: 1,
                    class: ModuleClass::A,
                },
            )
        ];

        for (input, expected) in test_cases {
            let result = ShipInternalModule::from_str(input);

            dbg!(&result);

            assert!(result.is_ok());
            assert_eq!(result.unwrap(), expected);
        }
    }
}