pub const MOVEMENT_THRESHOLD: f32 = 0.01;
pub const WALL_OVERLAP_MODIFIER: f32 = 1.25;
pub const FLOOR_Y_OFFSET: u8 = 200;
pub const MOVEMENT_COOLDOWN: f32 = 1.0; // one second cooldown
pub const TITLE: &str = "Maze Ascension: The Labyrinth of Echoes";

// Base score constants
pub const BASE_FLOOR_SCORE: usize = 100;

// Floor progression constants
pub const FLOOR_PROGRESSION_MULTIPLIER: f32 = 1.2;
pub const MIN_TIME_MULTIPLIER: f32 = 0.2; // Minimum score multiplier for time
pub const TIME_BONUS_MULTIPLIER: f32 = 1.5;
// Time scaling constants
pub const BASE_PERFECT_TIME: f32 = 10.0; // Base time for floor 1
pub const TIME_INCREASE_FACTOR: f32 = 0.15; // Each floor adds 15% more time

// Constants for camera control

pub const BASE_ZOOM_SPEED: f32 = 10.0;
#[cfg(not(target_family = "wasm"))]
pub const SCROLL_MODIFIER: f32 = 1.;
#[cfg(target_family = "wasm")]
pub const SCROLL_MODIFIER: f32 = 0.01;
pub const MIN_ZOOM: f32 = 50.0;
pub const MAX_ZOOM: f32 = 2500.0;
