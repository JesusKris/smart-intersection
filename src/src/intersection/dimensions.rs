use crate::intersection::Point;

#[derive(Debug, Default, Clone, PartialEq, Copy)]
pub struct Dimensions {
    center: Point,
    car_width: f32,
    lane_width: f32,
    safety_distance: f32,
    intersection_width: f32,
    x_max: f32,
    y_max: f32,
    speed_unit: f32,
}

impl Dimensions {
    pub fn new(width: f32, height: f32) -> Self {
        let x_max: f32;
        let y_max: f32;

        // aspect ratio >= 16:9
        if width / height >= (16.0 / 9.0) {
            y_max = height / 2.0;
            x_max = y_max * (16.0 / 9.0);
        } else {
            x_max = width / 2.0;
            y_max = x_max / (16.0 / 9.0);
        };

        let center = Point {
            x: width / 2.0,
            y: height / 2.0,
        };
        let lane_width = x_max / 15.0;
        let car_width = lane_width / 4.0;
        Dimensions {
            center,
            car_width,
            lane_width,
            safety_distance: car_width * 3.0,
            intersection_width: lane_width * 6.0,
            x_max,
            y_max,
            speed_unit: car_width / 10.0,
        }
    }

    //getters
    pub fn get_center(&self) -> Point {
        self.center
    }

    pub fn get_car_width(&self) -> f32 {
        self.car_width
    }

    pub fn get_lane_width(&self) -> f32 {
        self.lane_width
    }

    pub fn get_intersection_width(&self) -> f32 {
        self.intersection_width
    }

    pub fn get_x_max(&self) -> f32 {
        self.x_max
    }

    pub fn get_y_max(&self) -> f32 {
        self.y_max
    }

    pub fn get_speed_unit(&self) -> f32 {
        self.speed_unit
    }

    pub fn get_safety_distance(&self) -> f32 {
        self.safety_distance
    }
}
