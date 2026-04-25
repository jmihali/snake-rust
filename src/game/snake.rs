use std::collections::{VecDeque, vec_deque};

use crate::game::coordinates::Coordinates;
use crate::game::orientation::{self, Orientation};
use crate::game::segment::Segment;

pub struct Snake {
    // in the vec, the first segment is the tail,
    // and the last segment is the head
    body: VecDeque<Segment>,
}

impl Snake {
    pub fn new(tail_coordinates: Coordinates, orientation: Orientation, length: u32) -> Self {
        let body = VecDeque::from([Segment::new(tail_coordinates, orientation, length)]);
        Snake { body: body }
    }
}

impl Snake {
    fn remove_tail_segment(&mut self) {
        self.body.pop_front();
    }

    fn add_head_segment(&mut self, orientation: Orientation) {
        let current_head_segment = self.body.back().expect("Failed to get back segment");
        let new_head_segment_tail = current_head_segment.get_head_coordinates();
        let new_head_segment = Segment::new(new_head_segment_tail, orientation, 0);
        self.body.push_back(new_head_segment);
        todo!("Do not use expect");
    }
}
