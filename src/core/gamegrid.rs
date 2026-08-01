use crate::core::tetrominos::{ Coord, Tetromino, TetrominoType };
use std::{ usize, vec::Vec };
use bevy::color::Srgba;
use strum::IntoEnumIterator;
use std::boxed::Box;
use rand::{ rng, rngs::ThreadRng, seq::SliceRandom };

pub const GRID_HEIGHT: i8 = 20;
pub const GRID_WIDTH: i8 = 10;

type Grid = Box<[[Option<Srgba>; GRID_HEIGHT as usize]; GRID_WIDTH as usize]>;

#[derive(PartialEq, Eq, Debug)]
pub enum GridError { 
    CannotAllocateNewTet
}

pub struct GameGrid {
    rand_gen: ThreadRng,
    tetrominos: Vec<Tetromino>,
    pub current_tetromino: Tetromino,
    pub grid: Grid,
    pub tet_coords: Coord
}

impl Default for GameGrid{

    fn default() -> Self {

        let mut rand_gen: ThreadRng = rng();
        
        let mut tetrominos: Vec<Tetromino> = TetrominoType::iter()
                                            .map(|tet_type: TetrominoType| 
                                                 {Tetromino::new(tet_type)})
                                            .collect();

        tetrominos.shuffle(&mut rand_gen);

        let current_tetromino: Tetromino = tetrominos.pop().unwrap();

        let grid: Grid = Box::new([[None; GRID_HEIGHT as usize]; GRID_WIDTH as usize]);

        let tet_coords: Coord = current_tetromino.get_init_coord();

        GameGrid { rand_gen, tetrominos, current_tetromino, grid, tet_coords }
    }
}

impl GameGrid {

    pub fn new() -> Self {
        Self::default()
    }

    fn renew_current_tetromino(&mut self) -> Result<(), GridError> {

        if self.tetrominos.is_empty() {
            for tet_type in TetrominoType::iter() {
                self.tetrominos.push(
                    Tetromino::new(tet_type)
                );
            }
            self.tetrominos.shuffle(&mut self.rand_gen);
        }

        let tetromino: Tetromino = self.tetrominos.pop().unwrap();
        let tet_coords: Coord = tetromino.get_init_coord();

        if self.is_move_valid(tet_coords, tetromino.mask)
            { self.current_tetromino = tetromino;
              self.tet_coords = tet_coords;
              Ok(()) }
        else
            { Err(GridError::CannotAllocateNewTet) }

    }

    fn fix_current_tetromino(&mut self) {

        let color: Srgba = self.current_tetromino.color;
        
        self.current_tetromino.mask.iter().map(|c: &Coord| c + &self.tet_coords)
                .for_each(|c: Coord|
                          {self.grid[c.x as usize][c.y as usize] = Some(color)});          
    }

    fn move_tet_left(&mut self) {
        if self.is_move_valid(Coord { x: -1, y: 0 }, self.current_tetromino.mask)
            { self.tet_coords += Coord { x: -1, y: 0 }; }
    }

    fn move_tet_right(&mut self) {
        if self.is_move_valid(Coord { x: 1, y: 0 }, self.current_tetromino.mask)
            { self.tet_coords += Coord { x: 1, y: 0 }; }
    }

    fn dump_tet(&mut self) -> Result<(), GridError>{

        let mut dump_coord: Coord = Coord{ x: 0, y: 0 };
        let coord_down: Coord = Coord{ x: 0, y: -1 };

        loop {
            let coord_to_test: Coord = self.tet_coords + dump_coord + coord_down;
            if self.is_move_valid(coord_to_test, self.current_tetromino.mask)
                { dump_coord += coord_down; }
            else { break; }
        }

        self.tet_coords += dump_coord;
        self.fix_current_tetromino();
        self.renew_current_tetromino()?;
        Ok(())

    }

    fn change_tet_mask(&mut self) {

        if self.is_move_valid(self.tet_coords, self.current_tetromino.next_mask)
            { self.current_tetromino.update_mask_and_next_one(); }
    }

    fn move_tet_down(&mut self) -> Result<(), GridError> {

        if self.is_move_valid(Coord{ x: 0, y: -1}, self.current_tetromino.mask)
            { self.tet_coords += Coord{ x: 0, y: -1 }; Ok(()) }

        else { self.fix_current_tetromino(); 
               self.renew_current_tetromino()?; 
               Ok(()) }
    }

    fn is_move_valid(&self, tet_coord: Coord, mask: &[Coord]) -> bool {

        mask.iter().map(|c: &Coord| c + &tet_coord)
                   .all(|c: Coord| { (c.x >= 0) && (c.y >= 0) &&
                                     (c.x < GRID_WIDTH) && (c.y < GRID_HEIGHT) &&
                                     self.grid[c.x as usize][c.y as usize].is_none() }) 

    }

}