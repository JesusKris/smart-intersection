pub mod frames;

use crate::{
    car::{
        cars::{CarTraits, Cars},
        Car, Direction,
    },
    intersection::Intersection,
    statistics::Statistics,
};
use macroquad::{
    prelude::{is_key_pressed, ImageFormat, KeyCode, Texture2D},
    text::{load_ttf_font_from_bytes, Font},
    window::next_frame,
};

#[derive(Debug, Clone, PartialEq)]
/// State contains
pub struct GlobalState {
    //assets
    car_sprite: Texture2D,
    menu_background: Texture2D,
    text_font: Font,

    //animation
    intersection: Intersection,
    cars: Cars,
    statistics: Statistics,
    animation_state: AnimationState,

    //main menu states
    breathing_opacity: f32,
    breathing_in: bool,
}

impl GlobalState {
    pub fn new() -> GlobalState {
        GlobalState {
            car_sprite: Texture2D::from_file_with_format(
                include_bytes!("../../assets/cars.png"),
                Some(ImageFormat::Png),
            ),
            menu_background: Texture2D::from_file_with_format(
                include_bytes!("../../assets/menu_background.png"),
                Some(ImageFormat::Png),
            ),

            text_font: load_ttf_font_from_bytes(include_bytes!(
                "../../assets/LuckiestGuy-Regular.ttf"
            ))
            .unwrap(),

            intersection: Intersection::new(),
            cars: Cars::new(),
            statistics: Statistics::new(),
            animation_state: AnimationState::Menu,
            breathing_opacity: 1.0,
            breathing_in: true,
        }
    }

    // FIGURE OUT THE CORRECT CALCULATIONS!
    pub fn recalculate_positions(&mut self) {
        let dimensions = self.get_intersection().get_dimensions();
        let old_x_max = self.get_intersection().get_dimensions().get_x_max();
        let old_y_max = self.get_intersection().get_dimensions().get_y_max();
        let old_center = self.get_intersection().get_dimensions().get_center();

        self.set_intersection(Intersection::new());

        let new_x_max = self.get_intersection().get_dimensions().get_x_max();
        let new_y_max = dimensions.get_y_max();
        let x_ratio = old_x_max / new_x_max;
        let y_ratio = new_y_max / old_y_max;

        let center = self.get_intersection().get_dimensions().get_center();

        let mut cars = self.cars.clone();
        for car in &mut *cars {
            match car.get_current_direction() {
                Direction::North | Direction::South => {
                    let old_y_from_center = old_center.y - car.get_y();
                    let new_y = center.y - (old_y_from_center / y_ratio);
                    car.set_y(new_y);
                }
                Direction::East | Direction::West => {
                    let old_x_from_center = old_center.x - car.get_x();
                    let new_x = center.x - (old_x_from_center / x_ratio);
                    car.set_x(new_x);
                }
                _ => unreachable!(),
            }
        }

        self.set_cars(cars.to_owned())
    }

    pub async fn handle_keypress(&mut self) {
        if self.get_animation_state() == AnimationState::Menu {} // This is unnecessary

        // Space is always "play" or "pause" - I think it is better and more intuitive from UX perspective
        if is_key_pressed(KeyCode::Space) {
            self.toggle_animation_state();
            next_frame().await
        };

        if self.get_animation_state() == AnimationState::Running {
            let cars = &mut self.get_cars();
            let mut car: Option<Car> = None;

            if is_key_pressed(KeyCode::Down) {
                car = Some(Car::new(Direction::South, &self));
            };
            if is_key_pressed(KeyCode::Up) {
                car = Some(Car::new(Direction::North, &self));
            };
            if is_key_pressed(KeyCode::Right) {
                car = Some(Car::new(Direction::East, &self));
            };
            if is_key_pressed(KeyCode::Left) {
                car = Some(Car::new(Direction::West, &self));
            };

            if is_key_pressed(KeyCode::R) {
                car = Some(Car::new(Direction::Random, &self));
            };

            if let Some(mut new_car) = car {
                if !new_car.same_lane_is_clear(self) {
                    return;
                } else {
                    cars.add_car(new_car);
                    self.set_cars(cars.to_owned());
                    // println!("{:?}", new_car);
                }
            };
        }

        if self.get_animation_state() == AnimationState::Paused {
            if is_key_pressed(KeyCode::Escape) {
                self.set_animation_state(AnimationState::Menu);
                self.reset();
            } else {
                self.set_animation_state(AnimationState::Paused);
            }
        }
    }

    fn reset(&mut self) {
        self.intersection = Intersection::new();
        self.cars = Cars::new();
        self.statistics = Statistics::new();
        self.animation_state = AnimationState::Menu;
        self.breathing_opacity = 1.0;
        self.breathing_in = true;
    }

    //setters

    pub fn set_intersection(&mut self, new_value: Intersection) {
        self.intersection = new_value;
    }

    pub fn set_cars(&mut self, new_value: Cars) {
        self.cars = new_value;
    }

    pub fn set_statistics(&mut self, new_value: Statistics) {
        self.statistics = new_value;
    }

    pub fn set_animation_state(&mut self, new_value: AnimationState) {
        self.animation_state = new_value;
    }

    pub fn toggle_animation_state(&mut self) {
        match self.animation_state {
            AnimationState::Running => self.set_animation_state(AnimationState::Paused),
            _ => self.set_animation_state(AnimationState::Running),
        }
    }

    pub fn set_breathing_opacity(&mut self, new_value: f32) {
        self.breathing_opacity = new_value
    }

    pub fn set_breathing_in(&mut self, new_value: bool) {
        self.breathing_in = new_value
    }

    //getters

    pub fn get_intersection(&self) -> Intersection {
        self.intersection
    }

    pub fn get_cars(&self) -> Cars {
        self.cars.clone()
    }

    pub fn get_statistics(&self) -> Statistics {
        self.statistics
    }

    pub fn get_animation_state(&self) -> AnimationState {
        self.animation_state.clone()
    }

    pub fn get_car_sprite(&self) -> Texture2D {
        self.car_sprite.clone()
    }

    pub fn get_menu_background(&self) -> Texture2D {
        self.menu_background.clone()
    }

    pub fn get_text_font(&self) -> Font {
        self.text_font
    }

    pub fn get_breathing_opacity(&self) -> f32 {
        self.breathing_opacity
    }

    pub fn get_breathing_in(&self) -> bool {
        self.breathing_in
    }
}

#[derive(Debug, Clone, PartialEq, Copy)]
pub enum AnimationState {
    Menu,
    Running,
    Paused,
}
