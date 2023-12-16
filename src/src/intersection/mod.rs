pub mod dimensions;
pub mod lanes;

use self::dimensions::*;
use self::lanes::*;
use crate::car::Direction;
use macroquad::{prelude::*, window};

#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct Point {
    pub x: f32,
    pub y: f32,
}

#[derive(Debug, Default, Clone, PartialEq, Copy)]
pub struct Intersection {
    window_width: f32,
    window_height: f32,
    dimensions: Dimensions,
    north: Lanes,
    east: Lanes,
    south: Lanes,
    west: Lanes,
}

impl Intersection {
    pub fn new() -> Self {
        let width = window::screen_width();
        let height = window::screen_height();
        let mut intersection = Intersection {
            window_width: width,
            window_height: height,
            dimensions: Dimensions::new(width, height),
            north: Lanes::default(),
            east: Lanes::default(),
            south: Lanes::default(),
            west: Lanes::default(),
        };
        calculate_axis(&mut intersection);
        intersection
    }

    pub fn draw(&self) {
        let mut i: i8 = 0;
        let center = self.dimensions.get_center();
        let lane_width = self.dimensions.get_lane_width();

        while i < 7 {
            // Left to center
            let x1 = center.x - self.dimensions.get_x_max();
            let y1 = center.y - 3.0 * lane_width + (i as f32 * lane_width);
            let x2 = center.x - 3.0 * lane_width;
            let y2 = y1;
            draw_line(x1, y1, x2, y2, 1.0, WHITE);

            // center to right
            let x1 = center.x + 3.0 * lane_width;
            let y1 = center.y - 3.0 * lane_width + (i as f32 * lane_width);
            let x2 = center.x + self.dimensions.get_x_max();
            let y2 = y1;
            draw_line(x1, y1, x2, y2, 1.0, WHITE);

            // center to top
            let x1 = center.x - 3.0 * lane_width + (i as f32 * lane_width);
            let y1 = center.y - 3.0 * lane_width;
            let x2 = x1;
            let y2 = center.y - self.dimensions.get_y_max();
            draw_line(x1, y1, x2, y2, 1.0, WHITE);

            // center to bottom
            let x1 = center.x - 3.0 * lane_width + (i as f32 * lane_width);
            let y1 = center.y + 3.0 * lane_width;
            let x2 = x1;
            let y2 = center.y + self.dimensions.get_y_max();
            draw_line(x1, y1, x2, y2, 1.0, WHITE);

            i += 1;
        }
    }

    pub fn draw_axis(&self) {
        let lane_width = self.dimensions.get_lane_width();
        let center = self.dimensions.get_center();
        let start_x = center.x - 3.0 * lane_width;
        let start_y = center.y - 3.0 * lane_width;

        for direction in self.iter() {
            let mut i = 0.0;
            if direction == &self.north {
                while i < 6.0 {
                    draw_line(
                        start_x + (0.5 + i) * lane_width,
                        0.0,
                        start_x + (0.5 + i) * lane_width,
                        center.y * 2.0,
                        1.0,
                        RED,
                    );

                    draw_line(
                        0.0,
                        start_y + (0.5 + i) * lane_width,
                        center.x * 2.0,
                        start_y + (0.5 + i) * lane_width,
                        1.0,
                        RED,
                    );

                    i += 1.0
                }
            }
        }
    }

    /// If window is resized, recalculate intersection values
    pub fn has_changed(&self) -> bool {
        if self.window_height != window::screen_height()
            || self.window_width != window::screen_width()
        {
            return true;
        }
        return false;
    }

    pub fn get_window_width(&self) -> f32 {
        self.window_width
    }

    pub fn get_window_height(&self) -> f32 {
        self.window_height
    }

    pub fn get_dimensions(&self) -> Dimensions {
        self.dimensions
    }

    pub fn get_lanes(&self, direction: Direction) -> Lanes {
        match direction {
            Direction::North => self.north,
            Direction::East => self.east,
            Direction::South => self.south,
            Direction::West => self.west,
            _ => unreachable!(),
        }
    }
}

struct Iter<'a> {
    inner: &'a Intersection,
    index: u8,
}

impl Intersection {
    fn iter(&self) -> Iter<'_> {
        Iter {
            inner: self,
            index: 0,
        }
    }
}

impl<'a> Iterator for Iter<'a> {
    type Item = &'a Lanes;

    fn next(&mut self) -> Option<Self::Item> {
        let ret = match self.index {
            0 => &self.inner.north,
            1 => &self.inner.east,
            2 => &self.inner.south,
            3 => &self.inner.west,
            _ => return None,
        };
        self.index += 1;
        Some(ret)
    }
}
