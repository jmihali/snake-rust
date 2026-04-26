use std::collections::{VecDeque, vec_deque};

use crate::game::coordinates::Coordinates;
use crate::game::orientation::{self, Orientation};
use crate::game::segment::Segment;

pub struct Snake {
    // in the body, the first segment is the tail,
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

    fn shorten_tail_segment(&mut self) {
        self.body.front_mut().unwrap().shorten_tail();
        // todo: do not use unwrap
    }

    fn get_tail_length(&self) -> u32 {
        self.body.front().unwrap().get_length()
        // todo: do not use unwrap
    }

    fn extend_head_segment(&mut self) {
        self.body.back_mut().unwrap().extend_head();
        // todo: do not use unwrap
    }

    pub fn get_head_orientation(&self) -> Orientation {
        self.body.back().unwrap().get_orientation()
        // todo: do not use unwrap
    }

    pub fn set_head_orientation(&mut self, orientation: Orientation) {
        if orientation != self.get_head_orientation() {
            self.add_head_segment(orientation);
        }
    }

    pub fn advance(&mut self, grow: bool) {
        self.extend_head_segment();

        if !grow {
            self.shorten_tail_segment();
            if self.get_tail_length() == 0 {
                self.remove_tail_segment();
            }
        }
    }
}
