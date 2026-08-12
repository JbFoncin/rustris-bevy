use bevy::prelude::*;

use crate::{rendering::{block::make_block_meshes, shared::{CoordConverter, RenderingParams}}};


#[allow(dead_code)]
#[derive(Component)]
pub struct CurrentTetrominoBlocks;

pub fn render_current_tetronimo(mut resources: RenderingParams, mut commands: Commands) {

    let Ok(rendering_history) = resources.rendering_history_query.single() 
        else { return; };
    let Ok(gamegrid) = resources.gamegrid_query.single()
        else { return; };

    if gamegrid.current_tetromino.mask == rendering_history.previous_tet.mask &&
       gamegrid.tet_coords == rendering_history.previous_tet_coord {
            return;
       }

    let Ok(window) = resources.window_query.single()
        else {return;};

    let tet_coords = gamegrid.current_tetromino.mask.map(
        |x| x + gamegrid.tet_coords
    );

    let coord_converter = CoordConverter::new(window);

    let (lighter_meshes,
         darker_meshes,
         inner_meshes) = make_block_meshes(coord_converter.block_size);

    let lighter_meshes_h = lighter_meshes.map(|x| {
        resources.meshes.add(x)
    });
    let darker_meshes_h = darker_meshes.map(|x| {
        resources.meshes.add(x)
    });
    let inner_meshes_h = inner_meshes.map(
        |x| resources.meshes.add(x)
    );

    let inner_color = Color::from(gamegrid.current_tetromino.color);
    let darker_color = Color::from(inner_color.darker(0.1));
    let lighter_color = Color::from(inner_color.lighter(0.1));

    let lighter_color_h = resources.materials.add(lighter_color);
    let darker_color_h = resources.materials.add(darker_color);
    let inner_color_h = resources.materials.add(inner_color);

    tet_coords.iter().for_each(
        |c| {
            let (x, y) = coord_converter.playable_grid_idx_to_center(
                c.x as usize, 
                c.y as usize
            );
            let transform = Transform::from_xyz(x, y, 0.0);

            for light_mesh_h in lighter_meshes_h.iter() {
                commands.spawn((
                    CurrentTetrominoBlocks,
                    Mesh2d(light_mesh_h.clone()),
                    MeshMaterial2d(lighter_color_h.clone()),
                    transform
                ));
            }

            for dark_mesh_h in darker_meshes_h.iter() {
                commands.spawn((
                    CurrentTetrominoBlocks,
                    Mesh2d(dark_mesh_h.clone()),
                    MeshMaterial2d(darker_color_h.clone()),
                    transform
                ));
            }
            
            for inner_mesh_h in inner_meshes_h.iter() {
                commands.spawn((
                    CurrentTetrominoBlocks,
                    Mesh2d(inner_mesh_h.clone()),
                    MeshMaterial2d(inner_color_h.clone()),
                    transform
                ));
            }
        });
}