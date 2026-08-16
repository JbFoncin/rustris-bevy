use crate::core::tetrominos::{ Coord, Tetromino, TetrominoType };
use std::{ usize, vec::Vec };
use bevy::color::Srgba;
use strum::IntoEnumIterator;
use rand::{ rng, seq::SliceRandom };
use bevy::prelude::Component;

pub const GRID_HEIGHT: i8 = 20;
pub const GRID_WIDTH: i8 = 10;

pub type Grid = [[Option<Srgba>; GRID_HEIGHT as usize]; GRID_WIDTH as usize];

#[derive(PartialEq, Eq, Debug)]
pub enum GameState { 
    Running,
    GameOver
}

#[derive(Component, Debug)]
pub struct GameGrid {
    tetrominos: Vec<Tetromino>,
    pub current_tetromino: Tetromino,
    pub grid: Grid,
    pub tet_coord: Coord,
    pub game_state: GameState,
    pub lines_removed: usize
}

impl Default for GameGrid{

    fn default() -> Self {
        
        let mut tetrominos: Vec<Tetromino> = TetrominoType::iter()
                                             .map(|tet_type: TetrominoType| 
                                                  {Tetromino::new(tet_type)})
                                             .collect();

        tetrominos.shuffle(&mut rng());

        let current_tetromino: Tetromino = tetrominos.pop().unwrap();

        let grid: Grid = [[None; GRID_HEIGHT as usize]; GRID_WIDTH as usize];

        let tet_coord: Coord = current_tetromino.get_init_coord();

        GameGrid { tetrominos, current_tetromino, grid, tet_coord, 
                   game_state: GameState::Running, lines_removed: 0 }
    }
}

impl GameGrid {

    fn renew_current_tetromino(&mut self) {

        if self.tetrominos.is_empty() {
            for tet_type in TetrominoType::iter() {
                self.tetrominos.push(
                    Tetromino::new(tet_type)
                );
            }
            self.tetrominos.shuffle(&mut rng());
        }

        let tetromino: Tetromino = self.tetrominos.pop().unwrap();
        let tet_coord: Coord = tetromino.get_init_coord();

        if self.is_move_valid(
            Coord { x: 0, y: 0 }, 
            &tet_coord, 
            tetromino.mask)
            { self.current_tetromino = tetromino;
              self.tet_coord = tet_coord; }
        else
            { self.game_state = GameState::GameOver }

    }

    fn fix_current_tetromino(&mut self) {

        let color: Srgba = self.current_tetromino.color;
        
        self.current_tetromino.mask.iter().map(|c: &Coord| c + &self.tet_coord)
                .for_each(|c: Coord|
                          {self.grid[c.x as usize][c.y as usize] = Some(color)});

        self.remove_full_lines();       
    }

    pub fn move_tet_left(&mut self) {
        if self.is_move_valid(
            Coord { x: -1, y: 0 }, 
            &self.tet_coord, 
            self.current_tetromino.mask)
            { self.tet_coord += Coord { x: -1, y: 0 }; }
    }

    pub fn move_tet_right(&mut self) {
        if self.is_move_valid(
            Coord { x: 1, y: 0 }, 
            &self.tet_coord, 
            self.current_tetromino.mask)
            { self.tet_coord += Coord { x: 1, y: 0 }; }
    }

    pub fn dump_tet(&mut self) {

        let mut dump_coord: Coord = Coord{ x: 0, y: 0 };
        let coord_down: Coord = Coord{ x: 0, y: -1 };

        loop {
            let coord_to_test: Coord = dump_coord + coord_down;
            if self.is_move_valid(
                coord_to_test, 
                &self.tet_coord, 
                self.current_tetromino.mask
            )
                { dump_coord += coord_down; }
            else { break; }
        }

        self.tet_coord += dump_coord;
        self.fix_current_tetromino();
        self.renew_current_tetromino();

    }

    pub fn change_tet_mask(&mut self) {

    if self.is_move_valid(
            Coord{ x: 0, y: -1}, 
            &self.tet_coord,  
            self.current_tetromino.next_mask
        )
            { self.current_tetromino.update_mask_and_next_one(); }
    }

    pub fn move_tet_down(&mut self) {

        if self.is_move_valid(
            Coord{ x: 0, y: -1}, 
            &self.tet_coord, 
            self.current_tetromino.mask
        )
            { self.tet_coord += Coord{ x: 0, y: -1 }; }

        else { self.fix_current_tetromino(); 
               self.renew_current_tetromino(); }
    }

    fn is_move_valid(&self, coord_change: Coord, tet_coord: &Coord, mask: &[Coord]) -> bool {

        mask.iter().map(|c: &Coord| c + tet_coord + coord_change)
                   .all(|c: Coord| { (c.x >= 0) && (c.y >= 0) &&
                                     (c.x <= GRID_WIDTH - 1) && (c.y <= GRID_HEIGHT - 1) &&
                                     self.grid[c.x as usize][c.y as usize].is_none() })
                
    }

    fn shift_rows_down(&mut self, start: usize) {

        for x in 0..(GRID_WIDTH as usize) {
            for y in start..(GRID_HEIGHT as usize - 1) {
                self.grid[x][y] = self.grid[x][y + 1]
            }
            self.grid[x][GRID_HEIGHT as usize - 1] = None;
        }
    }

    fn remove_full_lines(&mut self) {

    let mut row: usize = 0;
    
    loop {
        let is_row_full: bool = self.grid.iter()
                                         .map(|x| x[row])
                                         .all(|x| x.is_some());
        if is_row_full {
            self.grid.iter_mut().for_each(|x| x[row] = None);
            self.shift_rows_down(row);
            self.lines_removed += 1;
        }
        else 
            { row += 1 }

        if row == GRID_HEIGHT as usize - 1 
            { break; }
    }
    
    }  
}