use bevy::{color::palettes::tailwind::YELLOW_500, prelude::*, sprite::Anchor, text::TextBounds};

use crate::{core::gamegrid::GameGrid, rendering::shared::{CoordConverter, RenderingHistory}};

#[derive(Resource)]
pub struct ScoreFont {
    pub font: Font
}

#[derive(Component)]
pub struct ScoreContent;

pub fn render_score(gamegrid_q: Query<&GameGrid>,
                     font_q: Query<&ScoreFont>,
                     window_q: Query<&Window>,
                     rendering_history_q: Query<&RenderingHistory>,
                     mut fonts_assets: ResMut<Assets<Font>>,
                     mut commands: Commands) {
    
    let Ok(rendering_history) = rendering_history_q.single() 
        else {return;};
    let Ok(gamegrid) = gamegrid_q.single() else {return ;};
    let Ok(font) = font_q.single() else {return ;};
    let Ok(window) = window_q.single() else {return;};

    if rendering_history.previous_screen_hw == (window.height(), window.width()) &&
       rendering_history.previous_lines_removed == gamegrid.lines_removed 
       {return;}

    let coord_converter = CoordConverter::new(window);

    let title_position = coord_converter.score_title_position();
    let value_position = coord_converter.score_value_position();
    let level_position = coord_converter.score_level_position();

    let font_handle = fonts_assets.add(font.font.clone());

    let title_font = TextFont {
        font: font_handle.clone().into(),
        font_size: coord_converter.block_size.into(),
        ..default()
    };

    let text_color = TextColor(YELLOW_500.into()) ;

    commands.spawn(
        (
            ScoreContent,
            Text2d::new("SCORE"),
            TextLayout::new(Justify::Center, LineBreak::WordBoundary),
            TextBounds::from(coord_converter.score_area_row_size()),
            Transform::from_xyz(title_position.0, title_position.1, 0.),
            title_font,
            Anchor::TOP_LEFT,
            text_color.clone()
    ));

    let value_font = TextFont {
        font: font_handle.clone().into(),
        font_size: (coord_converter.block_size / 2.).into(),
        ..default()
    };

    let score_value =  (gamegrid.lines_removed / 100 + 1).pow(3) +
                              (gamegrid.lines_removed / 10 + 1 ).pow(2) +
                               gamegrid.lines_removed * 10 - 2;

    dbg!(value_position, (window.height(), window.width()));

    commands.spawn(
        (
            ScoreContent,
            Text2d::new(format!("{:010}", score_value )),
            TextLayout::new(Justify::Center, LineBreak::WordBoundary),
            TextBounds::from(coord_converter.score_area_row_size()),
            Transform::from_xyz(value_position.0, value_position.1, 0.),
            value_font,
            Anchor::TOP_LEFT,
            text_color.clone()
        )
    );

    let level = gamegrid.lines_removed / 10;

    let level_font = TextFont {
        font: font_handle.clone().into(),
        font_size: (coord_converter.block_size / 2.).into(),
        ..default()
    };

    commands.spawn(
        (
            ScoreContent,
            Text2d::new(format!("level {:04}", level)),
            TextLayout::new(Justify::Center, LineBreak::WordBoundary),
            TextBounds::from(coord_converter.score_area_row_size()),
            Transform::from_xyz(level_position.0, level_position.1, 0.),
            level_font,
            Anchor::TOP_LEFT,
            text_color.clone()
        )
    );

}