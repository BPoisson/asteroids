use ggez::glam::Vec2;

pub const GRID_SIZE: Vec2 = Vec2::new(100.0, 100.0);
pub const GRID_CELL_DIM: f32 = 10.0;

pub const SCREEN_SIZE: Vec2 =
    Vec2::new(
        GRID_SIZE.x * GRID_CELL_DIM,
        GRID_SIZE.y * GRID_CELL_DIM
    );

pub const FRAMES_PER_SECOND: f32 = 60.0;
pub const MILLIS_PER_FRAME: u64 = (1.0 / FRAMES_PER_SECOND * 1000.0) as u64;