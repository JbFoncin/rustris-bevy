use bevy::prelude::*;

const EDGE_RATIO: f32 = 0.1;

pub type LighterBlockEdges = [Triangle2d; 4];
pub type DarkerBlockEdges = [Triangle2d; 4];
pub type InnerSquare = [Triangle2d; 2];

#[derive(Component)]
struct BackGroundAreaBlocks;

pub fn make_block_meshes(block_size: f32) -> (LighterBlockEdges, DarkerBlockEdges, InnerSquare) {

    let edge_size: f32 = EDGE_RATIO * block_size;
    
    let inner_left_edge = Triangle2d::new(
        vec2(edge_size, edge_size),
        vec2(edge_size, block_size - edge_size),
        vec2(0.0, block_size)
    );
    let outer_left_edge = Triangle2d::new(
        vec2(0.0, 0.0),
        vec2(0.0, block_size),
        vec2(edge_size, block_size - edge_size)
    );
    let inner_up_edge = Triangle2d::new(
        vec2(edge_size, block_size - edge_size),
        vec2(block_size - edge_size, block_size - edge_size),
        vec2(block_size, block_size)
    );
    let outer_up_edge = Triangle2d::new(
        vec2(0.0, block_size),
        vec2(edge_size, block_size - edge_size),
        vec2(edge_size, edge_size)
    );
    let inner_right_edge = Triangle2d::new(
        vec2(block_size - edge_size, block_size - edge_size),
        vec2(block_size, block_size),
        vec2(block_size - edge_size, edge_size)
    );
    let outer_right_edge = Triangle2d::new(
        vec2(block_size , block_size),
        vec2(block_size, 0.0),
        vec2(block_size - edge_size, edge_size)
    );
    let inner_down_edge = Triangle2d::new(
        vec2(block_size, 0.0),
        vec2(block_size - edge_size, edge_size),
        vec2(edge_size, edge_size)
    );
    let outer_down_edge = Triangle2d::new(
        vec2(0.0, 0.0),
        vec2(edge_size, edge_size),
        vec2(block_size, block_size)
    );

    let inner_square_top_left = Triangle2d::new(
        vec2(edge_size, edge_size),
        vec2(edge_size, block_size - edge_size),
        vec2(block_size - edge_size, block_size - edge_size)
    );

    let inner_square_down_right = Triangle2d::new(
        vec2(edge_size, edge_size),
        vec2(block_size - edge_size, edge_size),
        vec2(block_size - edge_size, block_size - edge_size)
    );

    ([inner_left_edge, outer_left_edge, inner_up_edge, outer_up_edge],
     [inner_right_edge, outer_right_edge, inner_down_edge, outer_down_edge],
     [inner_square_top_left, inner_square_down_right])

}
