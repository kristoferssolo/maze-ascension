pub const MOVEMENT_THRESHOLD: f32 = 0.01;
pub const WALL_OVERLAP_MODIFIER: f32 = 1.25;
pub const FLOOR_Y_OFFSET: u8 = 200;
pub const MOVEMENT_COOLDOWN: f32 = 1.0; // one second cooldown
pub const TITLE: &str = "Maze Ascension: The Labyrinth of Echoes";

// Base score constants
pub const BASE_FLOOR_SCORE: usize = 1000;
pub const BASE_TIME_SCORE: usize = 100;

// Floor progression constants
pub const FLOOR_DIFFICULTY_MULTIPLIER: f32 = 1.2; // Higher floors are exponentially harder
pub const MIN_TIME_MULTIPLIER: f32 = 0.1; // Minimum score multiplier for time
pub const TIME_REFERENCE_SECONDS: f32 = 60.0; // Reference time for score calculation

// Constants for camera control
pub const BASE_ZOOM_SPEED: f32 = 10.0;
pub const MIN_ZOOM: f32 = 50.0;
pub const MAX_ZOOM: f32 = 2500.0;
pub const DISTANCE_SCALE_FACTOR: f32 = 0.5; // Adjust this to control how much distance affects zoom speed
