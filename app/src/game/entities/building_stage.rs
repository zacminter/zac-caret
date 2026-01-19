use bevy::prelude::*;

/// Represents the 10 stages of building evolution (0-10)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BuildingStage {
    Empty = 0,
    Foundation = 1,
    Frame = 2,
    WallsAndRoof = 3,
    Complete = 4,
    Enhanced = 5,
    SecondFloor = 6,
    Tower = 7,
    Decorated = 8,
    Grand = 9,
    Monument = 10,
}

impl BuildingStage {
    pub fn from_u8(stage: u8) -> Self {
        match stage.min(10) {
            0 => BuildingStage::Empty,
            1 => BuildingStage::Foundation,
            2 => BuildingStage::Frame,
            3 => BuildingStage::WallsAndRoof,
            4 => BuildingStage::Complete,
            5 => BuildingStage::Enhanced,
            6 => BuildingStage::SecondFloor,
            7 => BuildingStage::Tower,
            8 => BuildingStage::Decorated,
            9 => BuildingStage::Grand,
            _ => BuildingStage::Monument,
        }
    }

    pub fn as_u8(&self) -> u8 {
        *self as u8
    }

    /// Generate mesh for this stage (primitive shapes for now)
    pub fn generate_mesh(&self) -> Mesh {
        match self {
            BuildingStage::Empty => {
                // Small ground marker
                Mesh::from(Cuboid::new(0.5, 0.1, 0.5))
            }
            BuildingStage::Foundation => {
                // Flat foundation stones
                Mesh::from(Cuboid::new(2.0, 0.3, 2.0))
            }
            BuildingStage::Frame => {
                // Taller frame structure
                Mesh::from(Cuboid::new(1.8, 1.5, 1.8))
            }
            BuildingStage::WallsAndRoof => {
                // Basic building shape
                Mesh::from(Cuboid::new(2.0, 2.0, 2.0))
            }
            BuildingStage::Complete => {
                // Slightly taller
                Mesh::from(Cuboid::new(2.0, 2.5, 2.0))
            }
            BuildingStage::Enhanced => {
                // Wider base
                Mesh::from(Cuboid::new(2.5, 2.5, 2.5))
            }
            BuildingStage::SecondFloor => {
                // Taller multi-story
                Mesh::from(Cuboid::new(2.5, 3.5, 2.5))
            }
            BuildingStage::Tower => {
                // Add height for tower
                Mesh::from(Cuboid::new(2.5, 4.5, 2.5))
            }
            BuildingStage::Decorated => {
                // Slightly larger
                Mesh::from(Cuboid::new(2.8, 4.5, 2.8))
            }
            BuildingStage::Grand => {
                // Impressive size
                Mesh::from(Cuboid::new(3.0, 5.0, 3.0))
            }
            BuildingStage::Monument => {
                // Maximum grandeur
                Mesh::from(Cuboid::new(3.5, 6.0, 3.5))
            }
        }
    }

    /// Get material color for this stage
    pub fn get_color(&self) -> Color {
        match self {
            BuildingStage::Empty => Color::srgb(0.5, 0.4, 0.3), // Dirt brown
            BuildingStage::Foundation => Color::srgb(0.6, 0.6, 0.6), // Stone gray
            BuildingStage::Frame => Color::srgb(0.7, 0.5, 0.3), // Wood
            BuildingStage::WallsAndRoof => Color::srgb(0.8, 0.7, 0.5), // Light wood
            BuildingStage::Complete => Color::srgb(0.6, 0.5, 0.4), // Finished wood
            BuildingStage::Enhanced => Color::srgb(0.7, 0.6, 0.5), // Polished
            BuildingStage::SecondFloor => Color::srgb(0.75, 0.65, 0.55), // Refined
            BuildingStage::Tower => Color::srgb(0.8, 0.7, 0.6), // Noble
            BuildingStage::Decorated => Color::srgb(0.85, 0.75, 0.65), // Elegant
            BuildingStage::Grand => Color::srgb(0.9, 0.8, 0.7), // Majestic
            BuildingStage::Monument => Color::srgb(0.95, 0.85, 0.6), // Golden accents
        }
    }
}
