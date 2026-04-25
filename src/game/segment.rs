use crate::game::Coordinates;
use crate::game::Orientation;

struct Segment {
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
    fn get_tail_coordinates(&self) -> Coordinates {
        self.tail_coordinates.clone()
    }

    fn get_length(&self) -> u32 {
        self.length
    }
}

impl Segment {
    fn set_orientation(&mut self, orientation: Orientation) {
        self.orientation = orientation;
    }
}

impl Segment {
    fn extend_head(&mut self) {
        self.length += 1;
    }

    fn shorten_tail(&mut self) {
        self.length -= 1;
        match self.orientation {
            Orientation::North => self.tail_coordinates.move_north(),
            Orientation::South => self.tail_coordinates.move_south(),
            Orientation::East => self.tail_coordinates.move_east(),
            Orientation::West => self.tail_coordinates.move_west(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn extend_segment_head() {
        let x = 2;
        let y = 3;
        let length = 4;
        let mut segment_north = Segment::new(Coordinates::new(x, y), Orientation::North, length);
        let mut segment_south = Segment::new(Coordinates::new(x, y), Orientation::South, length);
        let mut segment_east = Segment::new(Coordinates::new(x, y), Orientation::East, length);
        let mut segment_west = Segment::new(Coordinates::new(x, y), Orientation::West, length);

        segment_north.extend_head();
        segment_south.extend_head();
        segment_east.extend_head();
        segment_west.extend_head();

        assert_eq!(segment_north.get_tail_coordinates(), Coordinates::new(2, 3));
        assert_eq!(segment_north.get_length(), length + 1);

        assert_eq!(segment_south.get_tail_coordinates(), Coordinates::new(2, 3));
        assert_eq!(segment_south.get_length(), length + 1);

        assert_eq!(segment_east.get_tail_coordinates(), Coordinates::new(2, 3));
        assert_eq!(segment_east.get_length(), length + 1);

        assert_eq!(segment_west.get_tail_coordinates(), Coordinates::new(2, 3));
        assert_eq!(segment_west.get_length(), length + 1);
    }

    #[test]
    fn shorten_segment_tail() {
        let x = 2;
        let y = 3;
        let length = 4;
        let mut segment_north = Segment::new(Coordinates::new(x, y), Orientation::North, length);
        let mut segment_south = Segment::new(Coordinates::new(x, y), Orientation::South, length);
        let mut segment_east = Segment::new(Coordinates::new(x, y), Orientation::East, length);
        let mut segment_west = Segment::new(Coordinates::new(x, y), Orientation::West, length);

        segment_north.shorten_tail();
        segment_south.shorten_tail();
        segment_east.shorten_tail();
        segment_west.shorten_tail();

        assert_eq!(segment_north.get_tail_coordinates(), Coordinates::new(2, 4));
        assert_eq!(segment_north.get_length(), length - 1);

        assert_eq!(segment_south.get_tail_coordinates(), Coordinates::new(2, 2));
        assert_eq!(segment_south.get_length(), length - 1);

        assert_eq!(segment_east.get_tail_coordinates(), Coordinates::new(3, 3));
        assert_eq!(segment_east.get_length(), length - 1);

        assert_eq!(segment_west.get_tail_coordinates(), Coordinates::new(1, 3));
        assert_eq!(segment_west.get_length(), length - 1);
    }
}
