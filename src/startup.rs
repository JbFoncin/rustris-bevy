use bevy::prelude::*;

use crate::{core::gamegrid::GameGrid, rendering::shared::RenderingHistory};


pub fn init(mut commands: Commands) {
        
    commands.spawn(Camera2d::default());

    let gamegrid = GameGrid::default();

    commands.spawn(gamegrid.clone());

    let rendering_history = RenderingHistory::new((0.0, 0.0), gamegrid);

    commands.spawn(rendering_history);

}