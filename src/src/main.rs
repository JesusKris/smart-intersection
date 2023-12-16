use macroquad::prelude::*;
use smart_road::car::cars::CarTraits;
use smart_road::config::init_config;
use smart_road::state::frames::{draw_menu_frame, draw_paused_frame, draw_running_frame};
use smart_road::state::{AnimationState, GlobalState};
use std::thread::sleep;
use std::time::Duration;

#[macroquad::main(init_config)]
async fn main() {
    let mut global_state = GlobalState::new();

    loop {
        if global_state.get_animation_state() == AnimationState::Menu {
            draw_menu_frame(&mut global_state)
        }

        if global_state.get_animation_state() == AnimationState::Running {
            clear_background(DARKGRAY);

            if global_state.get_intersection().has_changed() {
                global_state.recalculate_positions();
            }

            global_state.get_intersection().draw();

            global_state.get_cars().move_cars(&mut global_state);

            global_state
                .get_cars()
                .remove_finished_cars(&mut global_state);

            global_state.get_cars().draw_cars(&global_state);

            // Time elapsed
            let mut old_stats = global_state.get_statistics();

            old_stats.set_animation_time(old_stats.get_animation_time() + 1.0 / 60.0);

            global_state.set_statistics(old_stats);

            draw_running_frame(&global_state)
        }

        if global_state.get_animation_state() == AnimationState::Paused {
            clear_background(DARKGRAY);

            if global_state.get_intersection().has_changed() {
                global_state.recalculate_positions();
            }

            global_state.get_intersection().draw();

            global_state.get_cars().draw_cars(&global_state);

            draw_paused_frame(&mut global_state);
        };

        global_state.handle_keypress().await;

        sleep(Duration::new(0, 1_000_000_000u32 / 60));

        next_frame().await;
    }
}
