use crate::intersection::Intersection;

#[derive(Debug, Default, PartialEq, Clone, Copy)]
pub struct Lanes {
    pub left_axis: f32,
    pub middle_axis: f32,
    pub right_axis: f32,
}


pub fn calculate_axis(intersection: &mut Intersection) {
    // Calculate axis for each lane.

    let center = intersection.get_dimensions().get_center();
    let lane_width = intersection.get_dimensions().get_lane_width();

    intersection.south.right_axis = center.x - 2.5 * lane_width;
    intersection.south.middle_axis = center.x - 1.5 * lane_width;
    intersection.south.left_axis = center.x - 0.5 * lane_width;

    intersection.north.left_axis = center.x + 0.5 * lane_width;
    intersection.north.middle_axis = center.x + 1.5 * lane_width;
    intersection.north.right_axis = center.x + 2.5 * lane_width;

    intersection.west.right_axis = center.y - 2.5 * lane_width;
    intersection.west.middle_axis = center.y - 1.5 * lane_width;
    intersection.west.left_axis = center.y - 0.5 * lane_width;

    intersection.east.left_axis = center.y + 0.5 * lane_width;
    intersection.east.middle_axis = center.y + 1.5 * lane_width;
    intersection.east.right_axis = center.y + 2.5 * lane_width;
}
