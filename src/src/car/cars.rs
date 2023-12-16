use macroquad::{prelude::GREEN, shapes::draw_circle};

use crate::state::GlobalState;

use super::{Car, Direction};

// #[derive(Debug, Clone, PartialEq)]
pub type Cars = Vec<Car>;

pub trait CarTraits {
    fn new() -> Vec<Car>;
    fn add_car(&mut self, new_value: Car);
    fn remove_finished_cars(&mut self, global_state: &mut GlobalState);
    fn move_cars(&mut self, global_state: &mut GlobalState);
    fn draw_cars(&self, global_state: &GlobalState);
}

impl CarTraits for Vec<Car> {
    fn new() -> Vec<Car> {
        Vec::new()
    }

    fn add_car(&mut self, new_value: Car) {
        self.push(new_value)
    }

    fn remove_finished_cars(&mut self, global_state: &mut GlobalState) {
        let car_width = global_state
            .get_intersection()
            .get_dimensions()
            .get_car_width();

        let x_max = global_state.get_intersection().get_dimensions().get_x_max();
        let y_max = global_state.get_intersection().get_dimensions().get_y_max();
        let center = global_state
            .get_intersection()
            .get_dimensions()
            .get_center();

        let mut new_cars = Cars::new();

        for car in self {
            match car.get_current_direction() {
                Direction::North => {
                    if car.get_y() > center.y - y_max - car_width {
                        new_cars.push(car.clone());
                        continue;
                    }
                    compare_car_statistics(car, global_state)
                }
                Direction::South => {
                    if car.get_y() < y_max + center.y + car_width {
                        new_cars.push(car.clone());
                        continue;
                    }
                    compare_car_statistics(car, global_state)
                }
                Direction::West => {
                    if car.get_x() > center.x - x_max - car_width {
                        new_cars.push(car.clone());
                        continue;
                    }
                    compare_car_statistics(car, global_state)
                }
                Direction::East => {
                    if car.get_x() < x_max + center.x + car_width {
                        new_cars.push(car.clone());
                        continue;
                    }
                    compare_car_statistics(car, global_state)
                }
                _ => unreachable!(),
            }
        }
        global_state.set_cars(new_cars);
    }

    fn move_cars(&mut self, global_state: &mut GlobalState) {
        for car in &mut *self {
            let window_width = global_state.get_intersection().get_window_width();
            let window_height = global_state.get_intersection().get_window_height();

            if !car.is_leaving_intersection() {
                match car.get_current_direction() {
                    Direction::South => {
                        if car.get_y() >= window_height / 2.0 + car.get_turn_offset() {
                            car.change_direction();
                            car.leaving_intersection(true);
                            car.calculate_rotation();
                            add_statistics_car(global_state)
                        }
                    }
                    Direction::North => {
                        if car.get_y() <= window_height / 2.0 + car.get_turn_offset() {
                            car.change_direction();
                            car.leaving_intersection(true);
                            car.calculate_rotation();
                            add_statistics_car(global_state)

                        }
                    }
                    Direction::East => {
                        if car.get_x() >= window_width / 2.0 + car.get_turn_offset() {
                            car.change_direction();
                            car.leaving_intersection(true);
                            car.calculate_rotation();
                            add_statistics_car(global_state)

                        }
                    }
                    Direction::West => {
                        if car.get_x() <= window_width / 2.0 + car.get_turn_offset() {
                            car.change_direction();
                            car.leaving_intersection(true);
                            car.calculate_rotation();
                            add_statistics_car(global_state)

                        }
                    }
                    _ => unreachable!(),
                }
            }

            car.set_driving_time(car.get_driving_time() + 1.0 / 60.0);
            car.adjust_speed(&global_state);
            get_max_min_speed(car, global_state);
        }

        global_state.set_cars(self.to_owned());
    }

    fn draw_cars(&self, global_state: &GlobalState) {
        // Helper for debugging - remove later
        fn draw_point(x: f32, y: f32) {
            draw_circle(x, y, 2.0, GREEN)
        }

        for car in self {
            car.draw(&global_state);
            draw_point(car.get_x(), car.get_y())
        }
    }
}

fn add_statistics_car(global_state: &mut GlobalState) {
    let mut old_stats = global_state.get_statistics();

    old_stats.set_max_vehicles(old_stats.get_max_vehicles() + 1.0);

    global_state.set_statistics(old_stats);
}


fn compare_car_statistics(finished_car: &Car, global_state: &mut GlobalState) {
    let mut statistics = global_state.get_statistics();

    if statistics.get_max_time() <= finished_car.get_driving_time() {
        statistics.set_max_time(finished_car.get_driving_time())
    }

    if statistics.get_min_time() == 0.0
        || statistics.get_min_time() >= finished_car.get_driving_time()
    {
        statistics.set_min_time(finished_car.get_driving_time())
    }

    global_state.set_statistics(statistics)
}

fn get_max_min_speed(car: &Car, global_state: &mut GlobalState) {
    let mut statistics = global_state.get_statistics();

    if statistics.get_max_speed() <= car.get_speed() {
        statistics.set_max_speed(car.get_speed())
    }

    if statistics.get_min_speed() == -0.1
        || statistics.get_min_speed() > car.get_speed() && car.get_speed() != 0.0
    {
        statistics.set_min_speed(car.get_speed())
    }

    global_state.set_statistics(statistics)
}
