mod core;
mod startup;
mod rendering;

use bevy::prelude::*;
use crate::{core::actions::{apply_player_action, make_tet_fall}, 
            rendering::{background::render_background, 
                        buttons::render_buttons, 
                        clean::{clean_background, clean_current_tet, 
                                clean_playable_fixed_grid, clean_score, clean_buttons}, 
                        grid::render_playable_area_fixed_blocks, 
                        score::render_score, 
                        shared::update_rendering_history, 
                        tetromino::render_current_tetronimo}, 
            startup::init};

pub fn create_app() -> App {

    let mut app = App::new();

    app.add_plugins(DefaultPlugins)
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
                clean_playable_fixed_grid,
                clean_score,
                clean_buttons
            ),
            (
                render_background,
                render_playable_area_fixed_blocks,
                render_current_tetronimo,
                render_score,
                render_buttons
            )
        ).chain()
    )
    .add_systems(PostUpdate, update_rendering_history);

    app
}

#[bevy_main]
fn main() {
    // Sur Android, on récupère l'app et on la lance via bevy_android
    create_app().run();
}