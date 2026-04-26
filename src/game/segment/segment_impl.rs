use crate::game::coordinates::Coordinates;
use crate::game::orientation::Orientation;
use crate::game::segment::error::{Error, Result};

pub struct Segment {
    tail_coordinates: Coordinates,
    orientation: Orientation,
    length: u32,
}

impl Segment {
    pub fn new(tail_coordinates: Coordinates, orientation: Orientation, length: u32) -> Self {
        Self {
            tail_coordinates,
            orientation,
            length,
        }
    }
}

impl Segment {
    pub fn get_tail_coordinates(&self) -> Coordinates {
        self.tail_coordinates.clone()
    }

    pub fn get_length(&self) -> u32 {
        self.length
    }

    pub fn get_orientation(&self) -> Orientation {
        self.orientation.clone()
    }

    pub fn get_head_coordinates(&self) -> Coordinates {
        self.tail_coordinates
            .get_relative_coordinates(&self.orientation, self.length)
    }
}

impl Segment {
    pub fn set_orientation(&mut self, orientation: Orientation) {
        self.orientation = orientation;
    }
}

impl Segment {
    pub fn extend_head(&mut self) {
        self.length += 1;
    }

    pub fn shorten_tail(&mut self) -> Result<()> {
        if self.length == 0 {
            return Err(Error::SegmentAlreadyZero);
        }
        self.length -= 1;
        self.tail_coordinates.move_coordinates(&self.orientation, 1);
        Ok(())
    }
}
