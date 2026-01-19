use bevy::prelude::*;
use crate::game::entities::building_stage::BuildingStage;

/// Component for buildings that evolve through stages
#[derive(Component, Debug)]
pub struct StagedBuilding {
    pub current_stage: BuildingStage,
    #[allow(dead_code)]
    pub building_type: BuildingType,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BuildingType {
    TownHall,
    // Future: Workshop, Library, etc.
}

impl StagedBuilding {
    pub fn new(building_type: BuildingType, stage: u8) -> Self {
        Self {
            current_stage: BuildingStage::from_u8(stage),
            building_type,
        }
    }

    pub fn upgrade(&mut self) {
        let next_stage = (self.current_stage.as_u8() + 1).min(10);
        self.current_stage = BuildingStage::from_u8(next_stage);
    }

    pub fn set_stage(&mut self, stage: u8) {
        self.current_stage = BuildingStage::from_u8(stage);
    }
}
