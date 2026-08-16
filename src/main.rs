use bevy::prelude::*;
use crate::{core::actions::{apply_player_action, make_tet_fall}, rendering::{background::render_background, 
                        clean::{clean_background, 
                                clean_current_tet, 
                                clean_playable_fixed_grid}, 
                        grid::render_playable_area_fixed_blocks, 
                        shared::update_rendering_history, 
                        tetromino::render_current_tetronimo}, startup::init};
mod core;
mod startup;
mod rendering;


fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(ClearColor(Color::srgb(0.0, 0.0, 0.0)))
        .add_systems(Startup, (init, render_background).chain())
        .add_systems(Update, 
            (
                (
                    make_tet_fall,
                    apply_player_action
                ),
                (
                    clean_background,
                    clean_current_tet,
                    clean_playable_fixed_grid
                ),
                (
                    render_background,
                    render_playable_area_fixed_blocks,
                    render_current_tetronimo
                )
            ).chain()
        )
        .add_systems(PostUpdate, update_rendering_history)
        .run();
}
