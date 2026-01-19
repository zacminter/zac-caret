use bevy::prelude::*;
use crate::game::components::{StagedBuilding, BuildingType};

pub struct TownHall {
    pub _level: u8,
    pub _worker_capacity: u8,
}

impl TownHall {
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self {
            _level: 1,
            _worker_capacity: 5,
        }
    }

    /// Spawn the town hall entity with staging component
    pub fn spawn(commands: &mut Commands, stage: u8) -> Entity {
        commands
            .spawn((
                StagedBuilding::new(BuildingType::TownHall, stage),
                SpatialBundle::from_transform(Transform::from_xyz(0.0, 0.0, 0.0)),
            ))
            .id()
    }

    pub fn _max_workers(&self) -> u8 {
        5 + (self._level - 1) * 2
    }

    pub fn _upgrade_requirements(&self) -> _TownHallUpgradeReq {
        match self._level {
            1 => _TownHallUpgradeReq { _projects: 1, _workers: 2 },
            2 => _TownHallUpgradeReq { _projects: 2, _workers: 4 },
            3 => _TownHallUpgradeReq { _projects: 3, _workers: 6 },
            _ => _TownHallUpgradeReq { 
                _projects: self._level as usize, 
                _workers: self._level as usize * 2 
            },
        }
    }
}

pub struct _TownHallUpgradeReq {
    pub _projects: usize,
    pub _workers: usize,
}
