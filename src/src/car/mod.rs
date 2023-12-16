pub mod cars;

use crate::constants::SPRITE_CARS;
use crate::state::GlobalState;
use macroquad::prelude::{draw_texture_ex, Color, DrawTextureParams, Rect, Vec2};
use rand::Rng;
use std::f32::consts::PI;

#[derive(Debug, Clone, PartialEq, Copy)]
pub struct Car {
    sprite: (f32, f32, f32, f32),
    x: f32,
    y: f32,
    rotation: f32,
    leaving_intersection: bool,
    target_speed: f32,
    speed: f32,
    direction_from: Direction,
    current_direction: Direction,
    lane: Lane,
    turn_offset: f32,
    driving_time: f32,
}

impl Car {
    pub fn new(mut direction: Direction, global_state: &GlobalState) -> Self {
        let mut rng = rand::thread_rng();
        let s = rng.gen_range(0..20);
        let sprite = SPRITE_CARS[s];
        let intersection = global_state.get_intersection();
        let dimensions = intersection.get_dimensions();
        let center = dimensions.get_center();
        let lane = Self::select_random_lane();
        let x_max = dimensions.get_x_max();
        let y_max = dimensions.get_y_max();

        if direction == Direction::Random {
            direction = Self::select_random_direction();
        }

        Car {
            sprite,
            x: match direction {
                Direction::South | Direction::North => match lane {
                    Lane::Left => intersection.get_lanes(direction).left_axis,
                    Lane::Middle => intersection.get_lanes(direction).middle_axis,
                    Lane::Right => intersection.get_lanes(direction).right_axis,
                },
                Direction::West => center.x + x_max,
                Direction::East => center.x - x_max,
                _ => unreachable!(),
            },
            y: match direction {
                Direction::East | Direction::West => match lane {
                    Lane::Left => intersection.get_lanes(direction).left_axis,
                    Lane::Middle => intersection.get_lanes(direction).middle_axis,
                    Lane::Right => intersection.get_lanes(direction).right_axis,
                },
                Direction::North => center.y + y_max,
                Direction::South => center.y - y_max,
                _ => unreachable!(),
            },
            rotation: match direction {
                Direction::South => PI,
                Direction::West => 1.5 * PI,
                Direction::North => 0.0,
                Direction::East => 0.5 * PI,
                _ => unreachable!(),
            },
            leaving_intersection: false,
            target_speed: 1.0,
            speed: 0.0, // TODO: Different speeds
            direction_from: direction,
            current_direction: direction,
            lane,
            turn_offset: match direction {
                Direction::North | Direction::West => match lane {
                    Lane::Right => 2.5 * dimensions.get_lane_width(),
                    Lane::Middle => 0.0,
                    Lane::Left => -0.5 * dimensions.get_lane_width(),
                },
                Direction::South | Direction::East => match lane {
                    Lane::Right => -2.5 * dimensions.get_lane_width(),
                    Lane::Middle => 0.0,
                    Lane::Left => 0.5 * dimensions.get_lane_width(),
                },
                _ => unreachable!(),
            },
            driving_time: 0.0,
        }
    }

    fn select_random_lane() -> Lane {
        let mut rng = ::rand::thread_rng();
        match rng.gen_range(0..7) {
            0 => Lane::Left,
            1 => Lane::Middle,
            2 => Lane::Right,
            3 => Lane::Middle,
            4 => Lane::Middle,
            5 => Lane::Right,
            6 => Lane::Right,
            7 => Lane::Middle,
            _ => unreachable!(),
        }
    }

    fn select_random_direction() -> Direction {
        let mut rng = ::rand::thread_rng();
        match rng.gen_range(0..4) {
            0 => Direction::North,
            1 => Direction::South,
            2 => Direction::East,
            3 => Direction::West,
            _ => unreachable!(),
        }
    }

    pub fn calculate_rotation(&mut self) {
        self.rotation = match self.current_direction {
            Direction::South => PI,
            Direction::West => 1.5 * PI,
            Direction::North => 0.0,
            Direction::East => 0.5 * PI,
            _ => unreachable!(),
        }
    }

    pub fn adjust_speed(&mut self, global_state: &GlobalState) {
        if self.same_lane_is_clear(global_state) {
            self.set_target_speed(2.2);
            if self.is_at_intersection(global_state) {
                self.set_target_speed(1.2);

                if self.crossing_lane_is_clear(global_state) {
                    self.set_target_speed(1.2)
                } else {
                    self.set_target_speed(0.0);
                }
            }
        } else {
            self.set_target_speed(0.0);
        }

        if self.get_speed() < self.get_target_speed() {
            self.set_speed(self.get_speed() + 0.15)
        } else if self.get_speed() > self.get_target_speed() && self.get_speed() > 0.15 {
            self.set_speed(self.get_speed() - 0.15)
        } else {
            self.set_speed(0.0);
        }

        self.update_coords(global_state)
    }

    fn update_coords(&mut self, global_state: &GlobalState) {
        let intersection = global_state.get_intersection();
        let speed_unit = global_state
            .get_intersection()
            .get_dimensions()
            .get_speed_unit();

        match self.current_direction {
            Direction::North => {
                self.y = self.y - speed_unit * self.get_speed();
                match self.lane {
                    Lane::Left => self.x = intersection.get_lanes(Direction::North).left_axis,
                    Lane::Middle => self.x = intersection.get_lanes(Direction::North).middle_axis,
                    Lane::Right => self.x = intersection.get_lanes(Direction::North).right_axis,
                }
            }
            Direction::East => {
                self.x = self.x + speed_unit * self.get_speed();
                match self.lane {
                    Lane::Left => self.y = intersection.get_lanes(Direction::East).left_axis,
                    Lane::Middle => self.y = intersection.get_lanes(Direction::East).middle_axis,
                    Lane::Right => self.y = intersection.get_lanes(Direction::East).right_axis,
                }
            }
            Direction::South => {
                self.y = self.y + speed_unit * self.get_speed();
                match self.lane {
                    Lane::Left => self.x = intersection.get_lanes(Direction::South).left_axis,
                    Lane::Middle => self.x = intersection.get_lanes(Direction::South).middle_axis,
                    Lane::Right => self.x = intersection.get_lanes(Direction::South).right_axis,
                }
            }
            Direction::West => {
                self.x = self.x - speed_unit * self.get_speed();
                match self.lane {
                    Lane::Left => self.y = intersection.get_lanes(Direction::West).left_axis,
                    Lane::Middle => self.y = intersection.get_lanes(Direction::West).middle_axis,
                    Lane::Right => self.y = intersection.get_lanes(Direction::West).right_axis,
                }
            }
            _ => unreachable!(),
        };
    }

    pub fn same_lane_is_clear(&mut self, global_state: &GlobalState) -> bool {
        let direction = self.get_current_direction();
        let dimensions = global_state.get_intersection().get_dimensions();
        let safety_distance = dimensions.get_safety_distance();
        let car_width = dimensions.get_car_width();
        let cars = global_state.get_cars();

        let safe_distance = |x: f32| -> f32 {
            if self.is_at_intersection(global_state) {
                return x;
            } else {
                return 1.0 * dimensions.get_lane_width();
            }
        };

        for car in cars {
            let speed = car.get_speed();
            if car.get_current_direction() == self.get_current_direction()
                && car.get_lane() == self.get_lane()
            {
                match direction {
                    Direction::South => {
                        if car.get_y()
                            < self.get_y() + 1.0 * car_width + safe_distance(safety_distance * 1.8)
                            && car.get_y() > self.get_y()
                            && car.get_y() != self.get_y()
                        {
                            self.set_target_speed(speed - speed * 0.1);
                            return false;
                        }
                    }
                    Direction::West => {
                        if car.get_x()
                            > self.get_x() - 1.0 * car_width - safe_distance(safety_distance * 1.8)
                            && car.get_x() < self.get_x()
                            && car.get_x() != self.get_x()
                        {
                            self.set_target_speed(speed - speed * 0.1);
                            return false;
                        }
                    }
                    Direction::North => {
                        if car.get_y()
                            > self.get_y() - 1.0 * car_width - safe_distance(safety_distance * 1.5)
                            && car.get_y() < self.get_y()
                            && car.get_y() != self.get_y()
                        {
                            self.set_target_speed(speed - speed * 0.1);
                            return false;
                        }
                    }
                    Direction::East => {
                        if car.get_x()
                            < self.get_x() + 1.0 * car_width + safe_distance(safety_distance * 1.5)
                            && car.get_x() > self.get_x()
                            && car.get_x() != self.get_x()
                        {
                            self.set_target_speed(speed - speed * 0.1);
                            return false;
                        }
                    }

                    _ => unreachable!(),
                }
            }
        }
        return true;
    }

    pub fn crossing_lane_is_clear(&mut self, global_state: &GlobalState) -> bool {
        let direction = self.get_current_direction();
        let dimensions = global_state.get_intersection().get_dimensions();
        let center = dimensions.get_center();
        let car_width = dimensions.get_car_width();
        let lane_width = dimensions.get_lane_width();
        let cars = global_state.get_cars();

        for car in cars {
            let is_not_self = car.get_y() != self.get_y() && car.get_x() != self.get_x();
            match direction {
                Direction::North => match self.lane {
                    Lane::Left => {
                        // Self is on the intersection but hasn't crossed middle
                        if self.get_y() < center.y + 3.0 * lane_width && self.get_y() > center.y {
                            if car.get_y() > self.get_y() - 2.0 * car_width - 1.2 * lane_width // look ahead n number of lanes
                            && car.get_y() < self.get_y() - 0.6 * car_width  // front of car
                            && car.get_x() > self.get_x() - 0.6 * lane_width  // left hand side
                            && car.get_x() < self.get_x() + 0.7 * lane_width  // right hand side
                            && is_not_self
                            && (car.get_current_direction() == Direction::West || car.get_current_direction() == Direction::East)
                            {
                                if !(car.get_speed() == 0.0
                                    && self.narrow_crossing_lane_is_clear(global_state))
                                {
                                    return false;
                                }
                            }
                        }
                        // crossed the middle of intersection
                        if self.get_y() < center.y && self.get_y() > center.y - 3.0 * lane_width {
                            if car.get_y() > self.get_y() - 2.0 * car_width - 1.0 * lane_width // look ahead n number of lanes
                            && car.get_y() < self.get_y() - 0.6 * car_width  // front of car
                            && car.get_x() > self.get_x() - 0.6 * lane_width  // left hand side
                            && car.get_x() < self.get_x() + 0.7 * lane_width  // right hand side
                            && is_not_self
                            && (car.get_current_direction() == Direction::West || car.get_current_direction() == Direction::East)
                            {
                                if !(car.get_speed() == 0.0
                                    && self.narrow_crossing_lane_is_clear(global_state))
                                {
                                    return false;
                                }
                            }
                        }
                    }
                    Lane::Middle => {
                        //Self is on the intersection but hasn't crossed middle
                        if self.get_y() < center.y + 3.0 * lane_width && self.get_y() > center.y {
                            if car.get_y() > self.get_y() - 2.0 * car_width - 1.0 * lane_width // look ahead n number of lanes
                            && car.get_y() < self.get_y() - 0.6 * car_width  // front of car
                            && car.get_x() > self.get_x() - 0.6 * lane_width  // left hand side
                            && car.get_x() < self.get_x() + 0.7 * lane_width  // right hand side
                            && is_not_self
                            && (car.get_current_direction() == Direction::West || car.get_current_direction() == Direction::East)
                            {
                                if !(car.get_speed() == 0.0
                                    && self.narrow_crossing_lane_is_clear(global_state))
                                {
                                    return false;
                                }
                            }
                        }
                        // crossed the middle of intersection
                        if self.get_y() < center.y && self.get_y() > center.y - 3.0 * lane_width {
                            if car.get_y() > self.get_y() - 2.0 * car_width - 1.0 * lane_width // look ahead n number of lanes
                            && car.get_y() < self.get_y() - 0.6 * car_width  // front of car
                            && car.get_x() > self.get_x() - 0.6 * lane_width  // left hand side
                            && car.get_x() < self.get_x() + 0.7 * lane_width  // right hand side
                            && is_not_self
                            && (car.get_current_direction() == Direction::West || car.get_current_direction() == Direction::East)
                            {
                                if !(car.get_speed() == 0.0
                                    && self.narrow_crossing_lane_is_clear(global_state))
                                {
                                    return false;
                                }
                            }
                        }
                    }
                    Lane::Right => {
                        if car.get_y() > self.get_y() - 1.0 * lane_width
                        && car.get_y() < self.get_y() - 0.6 * car_width  // front of car
                        && car.get_x() > self.get_x() - 0.5 * lane_width  // left hand side
                        && car.get_x() < self.get_x() + 0.5 * lane_width  // right hand side
                        && is_not_self
                        // && car.get_current_direction() != Direction::North
                        {
                            self.set_target_speed(0.2);
                            return false;
                        }
                    }
                },

                Direction::South => match self.lane {
                    Lane::Left => {
                        if self.get_y() < center.y && self.get_y() > center.y - 3.0 * lane_width {
                            if car.get_y() < self.get_y() + 2.0 * car_width + 2.0 * lane_width // look ahead n number of lanes
                            && car.get_y() > self.get_y() + 0.6 * car_width  // front of car
                            && car.get_x() < self.get_x() + 0.6 * lane_width  // left hand side
                            && car.get_x() > self.get_x() - 0.6 * lane_width  // right hand side
                            && is_not_self
                            && (car.get_current_direction() == Direction::West || car.get_current_direction() == Direction::East)
                            {
                                if !(car.get_speed() == 0.0
                                    && self.narrow_crossing_lane_is_clear(global_state))
                                {
                                    return false;
                                }
                            }
                        }
                        // crossed the middle of intersection
                        if self.get_y() > center.y && self.get_y() < center.y + 3.0 * lane_width {
                            if car.get_y() > self.get_y() - 1.0 * lane_width // look ahead n number of lanes
                            && car.get_y() > self.get_y() + 0.6 * car_width  // front of car
                            && car.get_x() < self.get_x() + 0.6 * lane_width  // left hand side
                            && car.get_x() > self.get_x() - 0.6 * lane_width  // right hand side
                            && is_not_self
                            && (car.get_current_direction() == Direction::West || car.get_current_direction() == Direction::East)
                            {
                                if !(car.get_speed() == 0.0
                                    && self.narrow_crossing_lane_is_clear(global_state))
                                {
                                    return false;
                                }
                            }
                        }
                    }
                    Lane::Middle => {
                        // Self is on the intersection but hasn't crossed middle
                        if self.get_y() < center.y && self.get_y() > center.y - 3.0 * lane_width {
                            if car.get_y() < self.get_y() + 2.0 * car_width + 2.0 * lane_width // look ahead n number of lanes
                            && car.get_y() > self.get_y() + 0.6 * car_width  // front of car
                            && car.get_x() < self.get_x() + 0.6 * lane_width  // left hand side
                            && car.get_x() > self.get_x() - 0.6 * lane_width  // right hand side
                            && is_not_self
                            && (car.get_current_direction() == Direction::West || car.get_current_direction() == Direction::East)
                            {
                                if !(car.get_speed() == 0.0
                                    && self.narrow_crossing_lane_is_clear(global_state))
                                {
                                    return false;
                                }
                            }
                        }
                        // crossed the middle of intersection
                        if self.get_y() > center.y && self.get_y() < center.y + 3.0 * lane_width {
                            if car.get_y() > self.get_y() - 2.0 * car_width - 1.0 * lane_width // look ahead n number of lanes
                            && car.get_y() > self.get_y() + 0.6 * car_width  // front of car
                            && car.get_x() < self.get_x() + 0.6 * lane_width  // left hand side
                            && car.get_x() > self.get_x() - 0.6 * lane_width  // right hand side
                            && is_not_self
                            && (car.get_current_direction() == Direction::West || car.get_current_direction() == Direction::East)
                            {
                                if !(car.get_speed() == 0.0
                                    && self.narrow_crossing_lane_is_clear(global_state))
                                {
                                    return false;
                                }
                            }
                        }
                    }
                    Lane::Right => {
                        if car.get_y() < self.get_y() + 1.2 * lane_width
                        && car.get_y() > self.get_y() + 0.5 * car_width // front of car
                        && car.get_x() < self.get_x() + 0.5 * lane_width // left hand side
                        && car.get_x() > self.get_x() - 0.7 * car_width // right hand side
                        && is_not_self
                        // && car.get_current_direction() != Direction::South
                        {
                            self.set_target_speed(0.2);
                            return false;
                        }
                    }
                },

                Direction::West => match self.lane {
                    Lane::Left => {
                        if self.get_x() < center.x + 3.0 * lane_width && self.get_x() > center.x {
                            if car.get_x() > self.get_x() - 3.0 * lane_width
                            && car.get_x() < self.get_x() - 0.6 * car_width
                            && car.get_y() < self.get_y() + 0.6 * lane_width // left hand
                            && car.get_y() > self.get_y() - 0.6 * lane_width // right hand
                            && is_not_self
                            && car.current_direction != Direction::West
                            && car.current_direction != Direction::East
                            {
                                if !(car.get_speed() == 0.0
                                    && self.narrow_crossing_lane_is_clear(global_state))
                                {
                                    return false;
                                }
                            }
                        }

                        if self.get_x() < center.x && self.get_x() > center.x - 3.0 * lane_width {
                            if car.get_x() > self.get_x() - 1.0 * lane_width
                            && car.get_x() < self.get_x() - 0.6 * car_width
                            && car.get_y() < self.get_y() + 0.6 * lane_width // left hand
                            && car.get_y() > self.get_y() - 0.7 * lane_width // right hand
                            && is_not_self
                            && car.current_direction != Direction::West
                            && car.current_direction != Direction::East
                            {
                                if !(car.get_speed() == 0.0
                                    && self.narrow_crossing_lane_is_clear(global_state))
                                {
                                    return false;
                                }
                            }
                        }
                    }
                    Lane::Middle => {
                        if self.get_x() < center.x + 3.0 * lane_width && self.get_x() > center.x {
                            if car.get_x() > self.get_x() - 1.2 * lane_width
                            && car.get_x() < self.get_x() - 0.6 * car_width
                            && car.get_y() < self.get_y() + 0.7 * lane_width // Left hand aka down
                            && car.get_y() > self.get_y() - 0.7 * lane_width // Right hand aka up
                            && is_not_self
                            && (car.get_current_direction() == Direction::North
                            || car.get_current_direction() == Direction::South)
                            {
                                if !(car.get_speed() == 0.0
                                    && self.narrow_crossing_lane_is_clear(global_state))
                                {
                                    return false;
                                }
                            }
                        }

                        if self.get_x() > center.x - 3.0 * lane_width && self.get_x() < center.x {
                            if car.get_x() > self.get_x() - 1.2 * lane_width
                            && car.get_x() < self.get_x() - 0.6 * car_width
                            && car.get_y() < self.get_y() + 0.7 * lane_width // Left hand aka down
                            && car.get_y() > self.get_y() - 0.8 * lane_width // Right hand aka up
                            && is_not_self
                            && (car.get_current_direction() == Direction::North
                            || car.get_current_direction() == Direction::South)
                            {
                                if !(car.get_speed() == 0.0
                                    && self.narrow_crossing_lane_is_clear(global_state))
                                {
                                    return false;
                                }
                            }
                        }
                    }
                    Lane::Right => {
                        if car.get_x() > self.get_x() - 1.0 * lane_width
                            && car.get_x() < self.get_x() - 0.5 * car_width
                            && car.get_y() < self.get_y() + 0.5 * lane_width // left hand
                            && car.get_y() > self.get_y() - 0.5 * lane_width // right hand
                            && is_not_self
                        {
                            return false;
                        }
                    }
                },
                Direction::East => match self.lane {
                    Lane::Left => {
                        // before crossing middle of intersection
                        if self.get_x() > center.x - 3.0 * lane_width && self.get_x() < center.x {
                            if car.get_x() < self.get_x() + 2.5 * lane_width
                                && car.get_x() > self.get_x() + 0.6 * car_width // front of car
                                && car.get_y() > self.get_y() - 0.6 * lane_width // left hand
                                && car.get_y() < self.get_y() + 0.7 * lane_width // right hand
                                && is_not_self
                            && car.current_direction != Direction::East
                            && car.current_direction != Direction::West
                            {
                                if !(car.get_speed() == 0.0
                                    && self.narrow_crossing_lane_is_clear(global_state))
                                {
                                    return false;
                                }
                            }
                        }

                        // after crossing middle of intersection
                        if self.get_x() > center.x && self.get_x() < center.x + 3.0 * lane_width {
                            if car.get_x() < self.get_x() + 1.2 * lane_width
                            && car.get_x() > self.get_x() + 0.6 * car_width
                            && car.get_y() > self.get_y() - 0.6 * lane_width // left hand
                            && car.get_y() < self.get_y() + 0.6 * lane_width // right hand
                            && is_not_self
                            && car.current_direction != Direction::East
                            && car.current_direction != Direction::West
                            {
                                if !(car.get_speed() == 0.0
                                    && self.narrow_crossing_lane_is_clear(global_state))
                                {
                                    return false;
                                }
                            }
                        }
                    }
                    Lane::Middle => {
                        // before crossing middle of intersection
                        if self.get_x() > center.x - 3.0 * lane_width && self.get_x() < center.x {
                            if car.get_x() < self.get_x() + 3.0 * lane_width
                            && car.get_x() > self.get_x() + 0.6 * car_width // front of car
                            && car.get_y() > self.get_y() - 0.6 * lane_width // left hand
                            && car.get_y() < self.get_y() + 0.6 * lane_width // right hand
                            && is_not_self
                            {
                                if !(car.get_speed() == 0.0
                                    && self.narrow_crossing_lane_is_clear(global_state))
                                {
                                    return false;
                                }
                            }
                        }

                        // // after crossing middle of intersection
                        if self.get_x() > center.x && self.get_x() < center.x + 3.0 * lane_width {
                            if car.get_x() < self.get_x() + 1.5 * lane_width
                            && car.get_x() > self.get_x() + 0.6 * car_width // front of car
                            && car.get_y() > self.get_y() - 0.6 * lane_width // left hand
                            && car.get_y() < self.get_y() + 0.7 * lane_width // right hand
                            && is_not_self
                            {
                                if !(car.get_speed() == 0.0
                                    && self.narrow_crossing_lane_is_clear(global_state))
                                {
                                    return false;
                                }
                            }
                        }
                    }
                    Lane::Right => {
                        if car.get_x() < self.get_x() + 1.0 * lane_width
                            && car.get_x() > self.get_x() + 0.5 * car_width
                            && car.get_y() > self.get_y() - 0.5 * lane_width // left hand
                            && car.get_y() < self.get_y() + 0.5 * lane_width // right hand
                            && is_not_self
                        {
                            return false;
                        }
                    }
                },

                _ => unreachable!(),
            }
        }
        return true;
    }

    pub fn narrow_crossing_lane_is_clear(&mut self, global_state: &GlobalState) -> bool {
        let direction = self.get_current_direction();
        let dimensions = global_state.get_intersection().get_dimensions();
        let car_width = dimensions.get_car_width();
        let lane_width = dimensions.get_lane_width();
        let cars = global_state.get_cars();

        for car in cars {
            let is_not_self = car.get_y() != self.get_y() && car.get_x() != self.get_x();
            match direction {
                Direction::South => {
                    if car.get_y() < self.get_y() + 1.8 * lane_width
                        && car.get_y() > self.get_y() + 0.5 * car_width
                        && car.get_x() > self.get_x() - 1.3 * car_width
                        && car.get_x() < self.get_x() + 1.3 * car_width
                        && is_not_self
                    {
                        return false;
                    }
                }
                Direction::West => {
                    if car.get_x() > self.get_x() - 1.5 * lane_width
                        && car.get_x() < self.get_x() - 0.5 * car_width
                        && car.get_y() > self.get_y() - 1.3 * car_width
                        && car.get_y() < self.get_y() + 1.3 * car_width
                        && is_not_self
                    {
                        return false;
                    }
                }
                Direction::North => {
                    if car.get_y() > self.get_y() - 1.4 * lane_width
                        && car.get_y() < self.get_y() - 0.5 * car_width
                        && car.get_x() > self.get_x() - 1.3 * car_width
                        && car.get_x() < self.get_x() + 1.3 * car_width
                        && is_not_self
                    {
                        return false;
                    }
                }
                Direction::East => {
                    if car.get_x() < self.get_x() + 1.6 * lane_width
                        && car.get_x() > self.get_x() + 0.5 * car_width
                        && car.get_y() > self.get_y() - 1.3 * car_width
                        && car.get_y() < self.get_y() + 1.3 * car_width
                        && is_not_self
                    {
                        return false;
                    }
                }

                _ => unreachable!(),
            }
        }
        return true;
    }

    pub fn is_at_intersection(self, global_state: &GlobalState) -> bool {
        let dimensions = global_state.get_intersection().get_dimensions();
        let center = dimensions.get_center();
        let intersection_width = dimensions.get_intersection_width();
        let car_width = dimensions.get_car_width();

        match self.current_direction {
            Direction::South => {
                if self.y < center.y + intersection_width / 2.0 - car_width
                    && self.is_leaving_intersection()
                {
                    return true;
                } else if self.y > center.y - intersection_width / 2.0 - 2.0 * car_width
                    && !self.is_leaving_intersection()
                {
                    return true;
                }
            }
            Direction::West => {
                if self.x > center.x - intersection_width / 2.0 + car_width
                    && self.is_leaving_intersection()
                {
                    return true;
                } else if self.x < center.x + intersection_width / 2.0 + 2.0 * car_width
                    && !self.is_leaving_intersection()
                {
                    return true;
                }
            }
            Direction::North => {
                if self.y > center.y - intersection_width / 2.0 + car_width
                    && self.is_leaving_intersection()
                {
                    return true;
                } else if self.y < center.y + intersection_width / 2.0 + 2.0 * car_width
                    && !self.is_leaving_intersection()
                {
                    return true;
                }
            }
            Direction::East => {
                if self.x < center.x + intersection_width / 2.0 - car_width
                    && self.is_leaving_intersection()
                {
                    return true;
                }

                if self.x > center.x - intersection_width / 2.0 - 2.0 * car_width
                    && !self.is_leaving_intersection()
                {
                    return true;
                }
            }

            _ => unreachable!(),
        }

        return false;
    }

    pub fn draw(&self, global_state: &GlobalState) {
        let car_width = global_state
            .get_intersection()
            .get_dimensions()
            .get_car_width();

        draw_texture_ex(
            global_state.get_car_sprite(),
            match self.current_direction {
                Direction::North | Direction::South => self.x - car_width / 2.0,
                Direction::West | Direction::East => self.x - 0.5 * car_width,
                _ => unreachable!(),
            },
            match self.current_direction {
                Direction::North | Direction::South => self.y - 1.0 * car_width,
                Direction::West | Direction::East => self.y - car_width,
                _ => unreachable!(),
            },
            Color::new(1.0, 1.0, 1.0, 1.0),
            DrawTextureParams {
                dest_size: Some(Vec2 {
                    x: car_width,
                    y: car_width * 2.0,
                }),
                source: Some(Rect {
                    x: self.sprite.0,
                    y: self.sprite.1,
                    w: self.sprite.2,
                    h: self.sprite.3,
                }),
                rotation: self.rotation,
                flip_x: false,
                flip_y: false,
                pivot: None,
            },
        );
    }

    pub fn change_direction(&mut self) {
        let lane = self.get_lane();
        self.set_current_direction(match self.get_current_direction() {
            Direction::North => match lane {
                Lane::Left => Direction::West,
                Lane::Middle => Direction::North,
                Lane::Right => Direction::East,
            },

            Direction::South => match lane {
                Lane::Left => Direction::East,
                Lane::Middle => Direction::South,
                Lane::Right => Direction::West,
            },

            Direction::West => match lane {
                Lane::Left => Direction::South,
                Lane::Middle => Direction::West,
                Lane::Right => Direction::North,
            },
            Direction::East => match lane {
                Lane::Left => Direction::North,
                Lane::Middle => Direction::East,
                Lane::Right => Direction::South,
            },
            _ => unreachable!(),
        });
    }

    //setters
    pub fn set_x(&mut self, new_value: f32) {
        self.x = new_value;
    }

    pub fn set_y(&mut self, new_value: f32) {
        self.y = new_value;
    }

    pub fn set_rotation(&mut self, new_value: f32) {
        //need to think about this one
        self.rotation = new_value;
    }

    pub fn leaving_intersection(&mut self, new_value: bool) {
        self.leaving_intersection = new_value;
    }

    pub fn set_speed(&mut self, new_value: f32) {
        self.speed = new_value;
    }

    pub fn set_target_speed(&mut self, new_value: f32) {
        self.target_speed = new_value;
    }

    pub fn set_current_direction(&mut self, new_value: Direction) {
        self.current_direction = new_value;
    }

    pub fn set_driving_time(&mut self, new_value: f32) {
        self.driving_time = new_value;
    }

    //getters
    pub fn get_x(&self) -> f32 {
        self.x
    }

    pub fn get_y(&self) -> f32 {
        self.y
    }

    pub fn get_rotation(&self) -> f32 {
        //need to think about this one
        self.rotation
    }

    pub fn is_leaving_intersection(&self) -> bool {
        self.leaving_intersection
    }

    pub fn get_speed(&self) -> f32 {
        self.speed
    }
    pub fn get_target_speed(&self) -> f32 {
        self.target_speed
    }

    pub fn get_current_direction(&self) -> Direction {
        self.current_direction
    }

    pub fn get_lane(&self) -> Lane {
        self.lane
    }

    pub fn get_turn_offset(&self) -> f32 {
        self.turn_offset
    }

    pub fn get_driving_time(&self) -> f32 {
        self.driving_time
    }
}

#[derive(Debug, Clone, PartialEq, Copy)]
pub enum Direction {
    North,
    South,
    East,
    West,
    Random,
}

#[derive(Debug, Clone, PartialEq, Copy)]
pub enum Lane {
    Left,
    Middle,
    Right,
}
