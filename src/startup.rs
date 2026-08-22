use std::sync::Arc;

use bevy::prelude::*;

use crate::rendering::score::ScoreFont;
use crate::{core::gamegrid::GameGrid, 
            rendering::shared::RenderingHistory};
use linebender_resource_handle::Blob;

const FONT_BYTES: &[u8] = include_bytes!("../assets/square_sans_serif_7.ttf");

pub fn init(mut commands: Commands) {
        
    commands.spawn(Camera2d::default());

    let gamegrid = GameGrid::default();

    let rendering_history = RenderingHistory::new((0.0, 0.0), &gamegrid);

    let font = Font { data: Blob::new(Arc::new(FONT_BYTES)), alias: "score_font".into() };

    let score_font = ScoreFont{font};

    commands.insert_resource(score_font);

    commands.spawn(gamegrid);    

    commands.insert_resource(rendering_history);

}