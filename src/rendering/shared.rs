use bevy::prelude::*;
use crate::core::gamegrid::{GameGrid, Grid};
use crate::core::tetrominos::{Tetromino, Coord};

#[derive(Resource)]
pub struct RenderingHistory { 
    pub previous_grid: Grid,
    pub previous_screen_hw: (f32, f32),
    pub previous_tet: Tetromino,
    pub previous_tet_coord: Coord,
}

impl RenderingHistory {
    pub fn new(window: &Window, gamegrid: GameGrid) -> Self {
        RenderingHistory { 
            previous_grid: gamegrid.grid.clone(), 
            previous_screen_hw: (window.height(), window.width()), 
            previous_tet: gamegrid.current_tetromino.clone(), 
            previous_tet_coord: gamegrid.tet_coords 
        }
    }
}

use bevy::prelude::*;
use crate::rendering::background::{ GAME_HEIGHT, GAME_WIDTH };
const INTERACTIVE_AREA_ORIGIN: (usize, usize) = (1, 1);

pub struct CoordConverter {
    win_h: f32,
    win_w: f32,
    pub block_size: f32
}

impl CoordConverter {

    pub fn new(window: &Window) -> CoordConverter {
        let block_size: f32 = (window.height() / GAME_HEIGHT as f32)
                                .min(window.width() / GAME_WIDTH as f32);

        CoordConverter { win_h: window.height(),
                         win_w: window.width(),
                         block_size: block_size }
    }

    pub fn down_left_to_center(&self, x: f32, y:f32) -> (f32, f32) {
        (x - self.win_w / 2.0, y - self.win_h / 2.0)
    }

    pub fn playable_grid_idx_to_center(&self, row_index: usize, col_index: usize) -> (f32, f32) {
        let down_left_coord:(f32, f32)  = 
                ((INTERACTIVE_AREA_ORIGIN.0 + row_index) as f32 * self.block_size,
                 (INTERACTIVE_AREA_ORIGIN.1 + col_index) as f32 * self.block_size);

        self.down_left_to_center(down_left_coord.0, down_left_coord.1) 
    }
}