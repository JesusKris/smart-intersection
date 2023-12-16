use macroquad::{
    prelude::WHITE,
    text::{draw_text_ex, TextParams},
};

use crate::state::GlobalState;

#[derive(Debug, Clone, PartialEq, Copy)]
pub struct Statistics {
    animation_time: f32,
    max_vehicles: f32,
    max_speed: f32,
    min_speed: f32,
    max_time: f32,
    min_time: f32,
    close_calls: f32,
}

impl Statistics {
    pub fn new() -> Statistics {
        Statistics {
            animation_time: 0.0,
            max_vehicles: 0.0,
            max_speed: 0.0,
            min_speed: -0.1,
            max_time: 0.0,
            min_time: 0.0,
            close_calls: 0.0,
        }
    }

    pub fn draw(&self, x: f32, y: f32, w: f32, h: f32, global_state: &GlobalState) {
        let stats: Vec<(&str, f32)> = vec![
            ("Animation Time", self.animation_time.floor()),
            ("Max Vehicles", self.max_vehicles),
            ("Max Speed", (self.max_speed* 10.0).floor() / 10.0),
            ("Min Speed", (self.min_speed* 10.0).floor() / 10.0),
            ("Max Time", (self.max_time * 10.0).floor() / 10.0),
            ("Min Time", (self.min_time* 10.0).floor() / 10.0),
            ("Close Calls", self.close_calls),
        ];

        let top_margin = h / stats.len() as f32 * 1.5;
        let left_margin = w / stats.len() as f32;

        let mut i = 0.0;
        for (key, val) in stats.iter() {
            i = i + 1.0;

            if *key == "Animation Time" {
                let time = Self::get_formatted_animation_time(*val);
                draw_text_ex(
                    &format!("{key}: {time}"),
                    x + left_margin,
                    y + top_margin * i,
                    TextParams {
                        font: global_state.get_text_font(),
                        font_size: (w / 10.0) as u16,
                        font_scale: 1.0,
                        font_scale_aspect: 1.0,
                        color: WHITE,
                        rotation: 0.0,
                    },
                );
                continue;
            }

            draw_text_ex(
                &format!("{key}: {val}"),
                x + left_margin,
                y + top_margin * i,
                TextParams {
                    font: global_state.get_text_font(),
                    font_size: (w / 10.0) as u16,
                    font_scale: 1.0,
                    font_scale_aspect: 1.0,
                    color: WHITE,
                    rotation: 0.0,
                },
            );
        }
    }

    fn get_formatted_animation_time(time_in_seconds: f32) -> String {
        let mut hours = (time_in_seconds / 3600.0).floor().to_string();
        let mut minutes = ((time_in_seconds - (hours.parse::<f32>().unwrap() * 3600.0)) / 60.0)
            .floor()
            .to_string();
        let mut seconds = (time_in_seconds
            - (hours.parse::<f32>().unwrap() * 3600.0)
            - (minutes.parse::<f32>().unwrap() * 60.0).floor())
        .to_string();

        let new_hours = ["0", &hours].concat();
        if hours.parse::<f32>().unwrap() < 10.0 {
            hours = new_hours;
        }

        let new_minutes = ["0", &minutes].concat();
        if minutes.parse::<f32>().unwrap() < 10.0 {
            minutes = new_minutes;
        }

        let new_seconds = ["0", &seconds].concat();
        if seconds.parse::<f32>().unwrap() < 10.0 {
            seconds = new_seconds;
        }

        return format!("{hours}:{minutes}:{seconds}");
    }
    //setters
    pub fn set_animation_time(&mut self, new_value: f32) {
        self.animation_time = new_value;
    }

    pub fn set_max_vehicles(&mut self, new_value: f32) {
        self.max_vehicles = new_value;
    }

    pub fn set_max_speed(&mut self, new_value: f32) {
        self.max_speed = new_value;
    }

    pub fn set_min_speed(&mut self, new_value: f32) {
        self.min_speed = new_value;
    }

    pub fn set_max_time(&mut self, new_value: f32) {
        self.max_time = new_value;
    }

    pub fn set_min_time(&mut self, new_value: f32) {
        self.min_time = new_value;
    }

    pub fn set_close_calls(&mut self, new_value: f32) {
        self.close_calls = new_value
    }

    //getters
    pub fn get_animation_time(&self) -> f32 {
        self.animation_time
    }

    pub fn get_max_vehicles(&self) -> f32 {
        self.max_vehicles
    }

    pub fn get_max_speed(&self) -> f32 {
        self.max_speed
    }

    pub fn get_min_speed(&self) -> f32 {
        self.min_speed
    }

    pub fn get_max_time(&self) -> f32 {
        self.max_time
    }

    pub fn get_min_time(&self) -> f32 {
        self.min_time
    }

    pub fn get_close_calls(&self) -> f32 {
        self.close_calls
    }
}
