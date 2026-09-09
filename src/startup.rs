use std::sync::Arc;

use bevy::asset::RenderAssetUsages;
use bevy::image::CompressedImageFormats;
use bevy::prelude::*;

use crate::rendering::background::{GAME_HEIGHT, GAME_WIDTH};
use crate::rendering::buttons::ArrowImageHandles;
use crate::rendering::score::ScoreFont;
use crate::{core::gamegrid::GameGrid, 
            rendering::shared::RenderingHistory};
use linebender_resource_handle::Blob;

const FONT_BYTES: &[u8] = include_bytes!("../assets/square_sans_serif_7.ttf");
const ARROW_UP: &[u8] = include_bytes!("../assets/arrow_up_gray.png");
const ARROW_RIGHT: &[u8] = include_bytes!("../assets/arrow_right_gray.png");
const ARROW_DOWN: &[u8] = include_bytes!("../assets/arrow_down_gray.png");
const ARROW_LEFT: &[u8] = include_bytes!("../assets/arrow_left_gray.png");

pub fn load_png_from_bytes(bytes: &[u8]) -> Image {
    Image::from_buffer(bytes, 
                       bevy::image::ImageType::Extension("png"), 
                       CompressedImageFormats::default(), 
                       true, 
                       bevy::image::ImageSampler::default(), 
                       RenderAssetUsages::default()).unwrap()
}

pub fn init(mut materials: ResMut<Assets<Image>>,
            mut window_query: Query<&mut Window>,
            mut commands: Commands) {
        
    commands.spawn(Camera2d::default());

    let gamegrid = GameGrid::default();

    let rendering_history = RenderingHistory::new((0.0, 0.0), &gamegrid);

    let font = Font { data: Blob::new(Arc::new(FONT_BYTES)), alias: "score_font".into() };

    let Ok(mut window) = window_query.single_mut() else {return;};

    let (new_height, new_width) = {
        let game_ratio = (GAME_WIDTH as f32) / (GAME_HEIGHT as f32);
        (window.height(), window.height() * game_ratio)
    };

    window.resolution.set(new_width, new_height);

    let arrow_up_img = load_png_from_bytes(ARROW_UP);
    let arrow_right_img = load_png_from_bytes(ARROW_RIGHT);
    let arrow_down_img = load_png_from_bytes(ARROW_DOWN);
    let arrow_left_img = load_png_from_bytes(ARROW_LEFT);

    let arrow_up = materials.add(arrow_up_img);
    let arrow_right = materials.add(arrow_right_img);
    let arrow_down = materials.add(arrow_down_img);
    let arrow_left = materials.add(arrow_left_img);

    let arrow_handles = ArrowImageHandles {
        arrow_up, arrow_right, arrow_down, arrow_left
    };
    commands.insert_resource(arrow_handles);

    let score_font = ScoreFont{font};

    commands.insert_resource(score_font);

    commands.spawn(gamegrid);    

    commands.insert_resource(rendering_history);

}