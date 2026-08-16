use bevy::prelude::*;

use crate::core::gamegrid::GameGrid;

pub fn make_tet_fall(mut local_timer: Local<Option<Timer>>,
                     time: Res<Time>,
                     mut gamegrid_query: Query<&mut GameGrid>) {
    
    let Ok(mut gamegrid) = gamegrid_query.single_mut() else {return;};

    let timer = local_timer.get_or_insert_with(|| {
        Timer::from_seconds(1.0, TimerMode::Repeating)
    });

    if timer.tick(time.delta()).just_finished() {
        gamegrid.move_tet_down();
    }

}

pub fn apply_player_action(mut gamegrid_query: Query<&mut GameGrid>,
                           input: Res<ButtonInput<KeyCode>>) {
    
    let Ok(mut gamegrid) = gamegrid_query.single_mut() else {return;};

    if input.just_pressed(KeyCode::ArrowUp) { gamegrid.change_tet_mask(); }
    if input.just_pressed(KeyCode::ArrowLeft) { gamegrid.move_tet_left(); }
    if input.just_pressed(KeyCode::ArrowRight) { gamegrid.move_tet_right(); }
    if input.just_pressed(KeyCode::ArrowDown) { gamegrid.dump_tet(); }

}