use bevy::color::palettes::css::GRAY;
use bevy::prelude::*;
use crate::core::gamegrid::{GRID_HEIGHT, GRID_WIDTH};
use crate::core::tetrominos::Coord;
use crate::rendering::block::make_block_meshes;
use crate::rendering::shared::{CoordConverter, RenderingParams};

pub const GAME_HEIGHT: i8 = 22;
pub const GAME_WIDTH: i8 = 18;

#[derive(Component)]
pub struct BackGroundBlocks;

pub fn render_background(rendering_params: RenderingParams,
                         mut commands: Commands) {


    let Ok(window) = rendering_params.window_query.single() else {return;};
    let Ok(rendering_history) = rendering_params.rendering_history_query.single() 
        else {return;};

    if rendering_history.previous_screen_hw == (window.height(), window.width())
       {return;}

    let coord_converter = CoordConverter::new(window);

    let mut meshes = rendering_params.meshes;
    let mut materials = rendering_params.materials;

    let lighter_gray = GRAY.lighter(0.1);
    let darker_gray = GRAY.darker(0.1);

    let (lighter_meshes,
         darker_meshes,
         inner_meshes) = make_block_meshes(coord_converter.block_size);

    let lighter_meshes_handles = lighter_meshes.map(|x| meshes.add(x));
    let darker_meshes_handles = darker_meshes.map(|x| meshes.add(x));
    let inner_meshes_handles = inner_meshes.map(|x| meshes.add(x));


    let lighter_material = ColorMaterial::from(Color::from(lighter_gray));
    let darker_material = ColorMaterial::from(Color::from(darker_gray));
    let normal_material = ColorMaterial::from(Color::from(GRAY));

    let lighter_handle = materials.add(lighter_material);
    let darker_handle = materials.add(darker_material);
    let normal_handle = materials.add(normal_material);

    let mut spawn = |coord: Coord| 
        {   
            let (x, y) = coord_converter.background_idx_to_center(
                coord.x as usize, 
                coord.y as usize
            );

            let transform = Transform::from_xyz(x, y, 0.0);

            for light_mesh_h in lighter_meshes_handles.iter() {
                commands.spawn((BackGroundBlocks,
                                       Mesh2d(light_mesh_h.clone()),
                                       MeshMaterial2d(lighter_handle.clone()),
                                       transform));
            }
            for dark_mesh_h in darker_meshes_handles.iter() {
                commands.spawn((BackGroundBlocks,
                                       Mesh2d(dark_mesh_h.clone()),
                                       MeshMaterial2d(darker_handle.clone()),
                                       transform));
            }
            for inner_mesh_h in inner_meshes_handles.iter() {
                commands.spawn((BackGroundBlocks,
                                       Mesh2d(inner_mesh_h.clone()),
                                       MeshMaterial2d(normal_handle.clone()),
                                       transform));
            }
        };

    //bottom line

    (0..GAME_WIDTH).for_each(|x| spawn(Coord{x: x, y: 0}));
    for row in 1..(GRID_HEIGHT + 1) {
        (0..1).chain(GRID_WIDTH..GAME_WIDTH).for_each(|x| spawn(Coord { x, y: row }))
    }
    (0..GAME_WIDTH).for_each(|x| spawn(Coord{x: x, y: GAME_HEIGHT-1}));
}

