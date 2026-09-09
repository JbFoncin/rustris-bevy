use bevy::{color::palettes::css::GRAY, prelude::*};

use crate::{core::gamegrid::GameGrid, 
            rendering::shared::{CoordConverter, RenderingHistory}};

#[derive(Resource)]
pub struct ArrowImageHandles {
    pub arrow_up: Handle<Image>,
    pub arrow_right: Handle<Image>,
    pub arrow_down: Handle<Image>,
    pub arrow_left: Handle<Image>
}

pub fn render_buttons(rendering_history_q: Query<&RenderingHistory>,
                      arrow_images_q: Query<&ArrowImageHandles>,
                      window_query: Query<&Window>,
                      mut commands: Commands) {

    if !cfg!(target_os="android") {return;}

    let Ok(window) = window_query.single() else {return;};
    let Ok(arrow_images) = arrow_images_q.single() else {return;};
    let Ok(rendering_history) = rendering_history_q.single() 
        else {return;};

    if rendering_history.previous_screen_hw == (window.height(), window.width())
        {return;}

    let coord_converter = CoordConverter::new(window);
    let button_size = coord_converter.block_size * 2.;
    
    commands.spawn(
        (
            Button,
            Node {
                width: Val::Px(coord_converter.block_size * 6.),
                height: Val::Px(coord_converter.block_size * 6.),
                position_type: PositionType::Absolute,
                left: Val::Px(12. * coord_converter.block_size),
                bottom: Val::Px(6. * coord_converter.block_size),
                ..default()
            },
            Pickable::IGNORE,
            BackgroundColor(GRAY.into())
        )
    ).with_children(|parent| 
        {
            parent.spawn(
                (
                Button,
                Node {
                    position_type: PositionType::Absolute,
                    width: Val::Px(button_size),
                    height: Val::Px(button_size),
                    left: Val::Px(button_size),
                    bottom: Val::Px(2. * button_size),
                    ..default()
                },
                ImageNode::new(arrow_images.arrow_up.clone()),
                )
            ).observe(|_event: On<Pointer<Press>>, mut gamegrid_query: Query<&mut GameGrid>|{
                let Ok(mut gamegrid) = gamegrid_query.single_mut() else {return;};
                gamegrid.change_tet_mask();
            });
            
            parent.spawn(
                (
                Button,
                Node {
                    position_type: PositionType::Absolute,
                    width: Val::Px(button_size),
                    height: Val::Px(button_size),
                    left: Val::Px(2. * button_size),
                    bottom: Val::Px(button_size),
                    ..default()
                },
                ImageNode::new(arrow_images.arrow_right.clone())
                )
            ).observe(|_event: On<Pointer<Press>>, mut gamegrid_query: Query<&mut GameGrid>|{
                let Ok(mut gamegrid) = gamegrid_query.single_mut() else {return;};
                gamegrid.move_tet_right();
            });
            
            parent.spawn(
                (
                Button,
                Node {
                    position_type: PositionType::Absolute,
                    width: Val::Px(button_size),
                    height: Val::Px(button_size),
                    left: Val::Px(button_size),
                    bottom: Val::Px(0.),
                    ..default()
                },
                ImageNode::new(arrow_images.arrow_down.clone())
                )
            ).observe(|_event: On<Pointer<Press>>, mut gamegrid_query: Query<&mut GameGrid>|{
                let Ok(mut gamegrid) = gamegrid_query.single_mut() else {return;};
                gamegrid.dump_tet();
            });
            
            parent.spawn(
                (
                Button,
                Node {
                    position_type: PositionType::Absolute,
                    width: Val::Px(button_size),
                    height: Val::Px(button_size),
                    left: Val::Px(0.),
                    bottom: Val::Px(button_size),
                    ..default()
                },
                ImageNode::new(arrow_images.arrow_left.clone())
                )
            ).observe(|_event: On<Pointer<Press>>, mut gamegrid_query: Query<&mut GameGrid>|{
                let Ok(mut gamegrid) = gamegrid_query.single_mut() else {return;};
                gamegrid.move_tet_left();
            });
        });
}