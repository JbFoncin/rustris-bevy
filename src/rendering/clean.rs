use bevy::prelude::*;

use crate::{core::gamegrid::GameGrid, rendering::{background::BackGroundBlocks, grid::PlayableAreaFixedBlocks, score::ScoreContent, shared::RenderingHistory, tetromino::CurrentTetrominoBlocks}};

pub fn clean_playable_fixed_grid(rendering_history_q: Query<&RenderingHistory>,
                                 window_query: Query<&Window>,
                                 gamegrid_query: Query<&GameGrid>,
                                 entities_query: Query<Entity, With<PlayableAreaFixedBlocks>>,
                                 mut commands: Commands) {
    
    let Ok(window) = window_query.single() else {return;};
    let Ok(gamegrid) = gamegrid_query.single() else {return;};
    let Ok(rendering_history) = rendering_history_q.single() else {return;};

    if (window.height(), window.width()) == rendering_history.previous_screen_hw &&
       rendering_history.previous_grid == gamegrid.grid { return; }

    entities_query.iter().for_each(|entity| commands.entity(entity).despawn());

}

pub fn clean_current_tet(rendering_history_q: Query<&RenderingHistory>,
                         gamegrid_query: Query<&GameGrid>,
                         window_query: Query<&Window>,
                         entities_query: Query<Entity, With<CurrentTetrominoBlocks>>,
                         mut commands: Commands) {

    let Ok(window) = window_query.single() else {return;};
    let Ok(gamegrid) = gamegrid_query.single() else {return;};
    let Ok(rendering_history) = rendering_history_q.single() else {return;};

    if (window.height(), window.width()) == rendering_history.previous_screen_hw &&
       rendering_history.previous_tet.mask == gamegrid.current_tetromino.mask &&
       rendering_history.previous_tet_coord == gamegrid.tet_coord { return; }

    entities_query.iter().for_each(|entity| commands.entity(entity).despawn());
}    

pub fn clean_background(rendering_history_q: Query<&RenderingHistory>,
                        window_query: Query<&Window>,
                        entities_query: Query<Entity, With<BackGroundBlocks>>,
                        mut commands: Commands) {

    let Ok(rendering_history) = rendering_history_q.single() 
        else {return;};
    let Ok(window) = window_query.single() else {return;};

    if rendering_history.previous_screen_hw == (window.height(), window.width())
        {return;}

    entities_query.iter().for_each(|entity| commands.entity(entity).despawn());

}

pub fn clean_score(rendering_history_q: Query<&RenderingHistory>,
                   window_query: Query<&Window>,
                   entities_query: Query<Entity, With<ScoreContent>>,
                   gamegrid_query: Query<&GameGrid>,
                   mut commands: Commands) {
                    
    let Ok(rendering_history) = rendering_history_q.single() 
        else {return;};
    let Ok(window) = window_query.single() else {return;};
    let Ok(gamegrid) = gamegrid_query.single() else {return;};

    if rendering_history.previous_lines_removed == gamegrid.lines_removed &&
       rendering_history.previous_screen_hw == (window.height(), window.width())
       {return;}

    entities_query.iter().for_each(|entity| commands.entity(entity).despawn());
}