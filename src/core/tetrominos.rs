use strum_macros::EnumIter;
use bevy::color::{ Srgba, palettes::{basic::RED, tailwind::{BLUE_500, GRAY_500, GREEN_500, YELLOW_500}} };
use crate::core::{ gamegrid::{GRID_HEIGHT, GRID_WIDTH}, 
                   masks::* };

const CYAN: Srgba = Srgba::rgb(0.0, 1.0, 1.0);
const MAGENTA: Srgba = Srgba::rgb(1.0, 0.0, 1.0);

#[derive(EnumIter)]
pub enum TetrominoType { I, O, T, L, J, Z, S }

#[derive(Debug, Clone, Copy)]
pub struct Coord {
    pub x: i8,
    pub y: i8
}

impl std::ops::Add for Coord {

    type Output = Coord;

    fn add(self, rhs: Self) -> Self::Output {
        Coord{ x: self.x + rhs.x,
               y: self.y + rhs.y }
    }
}

impl std::ops::AddAssign for Coord {

    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x; self.y += rhs.y;    
    }    
}

impl std::ops::Add for &Coord {

    type Output = Coord;

    fn add(self, rhs: Self) -> Self::Output {
        Coord{ x: self.x + rhs.x,
               y: self.y + rhs.y }
    }

}

impl Eq for Coord {}

impl std::cmp::PartialEq for Coord {
    
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Tetromino {
    blocks_masks: &'static [[Coord; 4]],
    mask_idx: usize,
    pub color: Srgba,
    pub mask: &'static[Coord; 4],
    pub next_mask: &'static[Coord; 4]
}

impl Tetromino {

    pub fn new(t_type: TetrominoType) -> Tetromino {

        match t_type {

            TetrominoType::O => { 
                
                Tetromino { blocks_masks: MASKS_O,
                            mask_idx: 0,
                            color: CYAN,
                            mask: &MASKS_O[0],
                            next_mask: &MASKS_O[0] }}

            TetrominoType::I => { 

                Tetromino { blocks_masks: MASKS_I,
                            mask_idx: 0,
                            color: RED,
                            mask: &MASKS_I[0],
                            next_mask: &MASKS_I[1] }}

            TetrominoType::J => {                                
                
                Tetromino { blocks_masks: MASKS_J,
                            mask_idx: 0, 
                            color: MAGENTA,
                            mask: &MASKS_J[0],
                            next_mask: &MASKS_J[1] }}

            TetrominoType::S => { 
                
                Tetromino { blocks_masks: MASKS_S,
                            mask_idx: 0,
                            color: BLUE_500,
                            mask: &MASKS_S[0],
                            next_mask: &MASKS_S[1] }}

            TetrominoType::Z => { 
                
                Tetromino { blocks_masks: MASKS_Z,
                            mask_idx: 0,
                            color: GREEN_500,
                            mask: &MASKS_Z[0],
                            next_mask: &MASKS_Z[1] }}

            TetrominoType::L => { 
                
                Tetromino { blocks_masks: MASKS_L,
                            mask_idx: 0,
                            color: YELLOW_500,
                            mask: &MASKS_L[0],
                            next_mask: &MASKS_L[1] }}

            TetrominoType::T => { 
                
                Tetromino { blocks_masks: MASKS_T,
                            mask_idx: 0,
                            color: GRAY_500,
                            mask: &MASKS_T[0],
                            next_mask: &MASKS_T[1] }}
                            
        }
    }

    pub fn update_mask_and_next_one(&mut self) -> () {

        self.mask_idx = (self.mask_idx + 1) % self.blocks_masks.len();
        self.mask = &self.blocks_masks[self.mask_idx];

        let next_mask_idx: usize = (self.mask_idx + 1) % self.blocks_masks.len();
        self.next_mask = &self.blocks_masks[next_mask_idx];

    } 

    pub fn get_width(&self) -> i8 
        { self.mask.iter().map(|x: &Coord| x.x ).max().unwrap() }

    pub fn get_height(&self) -> i8 
        { self.mask.iter().map(|x: &Coord| x.y ).max().unwrap() }

    pub fn get_init_coord(&self) -> Coord {
        let tet_height: i8 = self.get_height();
        let tet_width: i8 = self.get_width(); 

        Coord { x: ((GRID_WIDTH - tet_width) / 2) - 1, 
                y:  GRID_HEIGHT - tet_height - 1 }
    }

}