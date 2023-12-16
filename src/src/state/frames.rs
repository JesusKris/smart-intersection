//add drawing -> menu picture with breathing text "Press space to begin the animation"

use macroquad::{
    prelude::{Color, Vec2, WHITE},
    text::{draw_text_ex, get_text_center, TextParams},
    texture::{draw_texture_ex, DrawTextureParams},
    window,
};

use super::GlobalState;

pub fn draw_menu_frame(global_state: &mut GlobalState) {
    draw_background(global_state);

    let text_center = get_text_center(
        "SMART ROAD",
        Some(global_state.get_text_font()),
        (window::screen_width() / 8.0) as u16,
        1.0,
        0.0,
    );

    draw_text_ex(
        "SMART ROAD",
        text_center.x / 2.54,
        window::screen_height() / 2.0,
        TextParams {
            font: global_state.get_text_font(),
            font_size: (window::screen_width() / 8.0) as u16,
            font_scale: 1.0,
            font_scale_aspect: 1.0,
            color: WHITE,
            rotation: 0.0,
        },
    );

    let text_center = get_text_center(
        "Press SPACE to start",
        Some(global_state.get_text_font()),
        (window::screen_width() / 30.0) as u16,
        1.0,
        0.0,
    );

    if global_state.get_breathing_opacity() < 0.0 || global_state.get_breathing_opacity() > 1.0 {
        global_state.set_breathing_in(!global_state.get_breathing_in())
    }

    if global_state.get_breathing_in() {
        global_state.set_breathing_opacity(global_state.get_breathing_opacity() - 0.015);
    } else {
        global_state.set_breathing_opacity(global_state.get_breathing_opacity() + 0.015);
    }

    draw_text_ex(
        "Press SPACE to start",
        text_center.x * 1.91,
        window::screen_height() / 1.5,
        TextParams {
            font: global_state.get_text_font(),
            font_size: (window::screen_width() / 30.0) as u16,
            font_scale: 1.0,
            font_scale_aspect: 1.0,
            color: Color::new(1.00, 1.00, 1.00, global_state.get_breathing_opacity()),
            rotation: 0.0,
        },
    );
}

pub fn draw_running_frame(global_state: &GlobalState) {
    draw_text_ex(
        "SPACE TO PAUSE",
        10.0,
        window::screen_height() - 20.0,
        TextParams {
            font: global_state.get_text_font(),
            font_size: (window::screen_width() / 50.0) as u16,
            font_scale: 1.0,
            font_scale_aspect: 1.0,
            color: WHITE,
            rotation: 0.0,
        },
    );

    global_state.get_statistics().draw(
        1.0,
        5.0,
        window::screen_width() / 8.0,
        window::screen_height() / 8.0,
        &global_state,
    );
}

pub fn draw_paused_frame(global_state: &mut GlobalState) {
    let text_center = get_text_center(
        "PAUSED",
        Some(global_state.get_text_font()),
        (window::screen_width() / 22.0) as u16,
        1.0,
        0.0,
    );

    draw_text_ex(
        "PAUSED",
        window::screen_width() - text_center.x * 2.1,
        window::screen_height() / 15.0,
        TextParams {
            font: global_state.get_text_font(),
            font_size: (window::screen_width() / 22.0) as u16,
            font_scale: 1.0,
            font_scale_aspect: 1.0,
            color: WHITE,
            rotation: 0.0,
        },
    );

    draw_text_ex(
        "ESC TO EXIT",
        10.0,
        window::screen_height() - 20.0,
        TextParams {
            font: global_state.get_text_font(),
            font_size: (window::screen_width() / 50.0) as u16,
            font_scale: 1.0,
            font_scale_aspect: 1.0,
            color: WHITE,
            rotation: 0.0,
        },
    );

    if global_state.get_breathing_opacity() < 0.0 || global_state.get_breathing_opacity() > 1.0 {
        global_state.set_breathing_in(!global_state.get_breathing_in())
    }

    if global_state.get_breathing_in() {
        global_state.set_breathing_opacity(global_state.get_breathing_opacity() - 0.015);
    } else {
        global_state.set_breathing_opacity(global_state.get_breathing_opacity() + 0.015);
    }

    let text_center = get_text_center(
        "PRESS SPACE TO CONTINUE",
        Some(global_state.get_text_font()),
        (window::screen_width() / 35.0) as u16,
        1.0,
        0.0,
    );

    draw_text_ex(
        "PRESS SPACE TO CONTINUE",
        window::screen_width() - text_center.x * 4.0,
        window::screen_height() / 1.2,
        TextParams {
            font: global_state.get_text_font(),
            font_size: (window::screen_width() / 35.0) as u16,
            font_scale: 1.0,
            font_scale_aspect: 1.0,
            color: Color::new(1.00, 1.00, 1.00, global_state.get_breathing_opacity()),
            rotation: 0.0,
        },
    );

    global_state.get_statistics().draw(
        1.0,
        5.0,
        window::screen_width() / 8.0,
        window::screen_height() / 8.0,
        &global_state,
    );
}

fn draw_background(global_state: &GlobalState) {
    draw_texture_ex(
        global_state.get_menu_background(),
        0.0,
        0.0,
        Color::new(1.0, 1.0, 1.0, 0.5),
        DrawTextureParams {
            dest_size: Some(Vec2 {
                x: window::screen_width(),
                y: window::screen_height(),
            }),
            source: None,
            rotation: 0.0,
            flip_x: false,
            flip_y: false,
            pivot: None,
        },
    );
}
