use bevy::prelude::*;

use crate::rendering::block::make_block_meshes;
use crate::rendering::shared::CoordConverter;
use crate::rendering::shared::RenderingParams;

#[allow(dead_code)]
#[derive(Component)]
pub struct PlayableAreaFixedBlocks;

pub fn render_playable_area_fixed_blocks(mut resources: RenderingParams,
                                         mut commands: Commands) {

    let Ok(gamegrid) = resources.gamegrid_query.single() 
        else { return; };
    let Ok(window) = resources.window_query.single() 
        else { return; };
    let Ok(rendering_history) = resources.rendering_history_query.single() 
        else {return;} ;

    if gamegrid.grid == rendering_history.previous_grid ||
       rendering_history.previous_screen_hw == (window.height(), window.width()) 
       { return; }
       
    let coord_converter: CoordConverter = CoordConverter::new(window);

    let (lighter_edges,
         darker_edges, 
         inner_square) = make_block_meshes(coord_converter.block_size);

    let lighter_handles = lighter_edges.map(|x| resources.meshes.add(x));
    let darker_handles = darker_edges.map(|x| resources.meshes.add(x));
    let inner_handles = inner_square.map(|x| resources.meshes.add(x));
    
    for (i, column) in gamegrid.grid.iter().enumerate() {
        for (j, elem) in column.iter().enumerate() {
            
            let Some(color) = elem else { continue; };

            let lighter = Color::from(color.lighter(0.1));
            let darker = Color::from(color.darker(0.1));

            let coord: (f32, f32) = coord_converter.playable_grid_idx_to_center(i, j);
            
            let lighter_material = ColorMaterial::from(lighter);
            let lighter_material_handle = resources.materials.add(lighter_material);

            let transform = Transform::from_xyz(coord.0, coord.1, 0.0);

            for handle in lighter_handles.iter() {   

                commands.spawn((
                    PlayableAreaFixedBlocks, 
                    Mesh2d(handle.clone()), 
                    MeshMaterial2d(lighter_material_handle.clone()), 
                    transform
                ));
                }

            let darker_material = ColorMaterial::from(darker);
            let darker_material_handle = resources.materials.add(darker_material);

            for handle in darker_handles.iter() {

                commands.spawn((
                     PlayableAreaFixedBlocks, 
                     Mesh2d(handle.clone()), 
                     MeshMaterial2d(darker_material_handle.clone()), 
                     transform
                    ));
            }
            let inner_material = ColorMaterial::from(Color::from(color.clone()));
            let inner_material_handle = resources.materials.add(inner_material);

            for handle in inner_handles.iter() {

                commands.spawn((
                    PlayableAreaFixedBlocks, 
                    Mesh2d(handle.clone()), 
                    MeshMaterial2d(inner_material_handle.clone()), 
                    transform
                ));
            }
        }
    }
}