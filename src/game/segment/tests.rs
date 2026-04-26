#[cfg(test)]
mod tests {
    use crate::game::Orientation;
    use crate::game::coordinates::*;
    use crate::game::segment::*;

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

        let _ = segment_north.shorten_tail();
        let _ = segment_south.shorten_tail();
        let _ = segment_east.shorten_tail();
        let _ = segment_west.shorten_tail();

        assert_eq!(segment_north.get_tail_coordinates(), Coordinates::new(2, 4));
        assert_eq!(segment_north.get_length(), length - 1);

        assert_eq!(segment_south.get_tail_coordinates(), Coordinates::new(2, 2));
        assert_eq!(segment_south.get_length(), length - 1);

        assert_eq!(segment_east.get_tail_coordinates(), Coordinates::new(3, 3));
        assert_eq!(segment_east.get_length(), length - 1);

        assert_eq!(segment_west.get_tail_coordinates(), Coordinates::new(1, 3));
        assert_eq!(segment_west.get_length(), length - 1);
    }

    #[test]
    fn get_head_coordinates() {
        let x = 7;
        let y = 8;
        let length = 4;
        let segment_north = Segment::new(Coordinates::new(x, y), Orientation::North, length);
        let segment_south = Segment::new(Coordinates::new(x, y), Orientation::South, length);
        let segment_east = Segment::new(Coordinates::new(x, y), Orientation::East, length);
        let segment_west = Segment::new(Coordinates::new(x, y), Orientation::West, length);

        assert_eq!(
            segment_north.get_head_coordinates(),
            Coordinates::new(x, y + length)
        );
        assert_eq!(
            segment_south.get_head_coordinates(),
            Coordinates::new(x, y - length)
        );
        assert_eq!(
            segment_east.get_head_coordinates(),
            Coordinates::new(x + length, y)
        );
        assert_eq!(
            segment_west.get_head_coordinates(),
            Coordinates::new(x - length, y)
        );
    }

    #[test]
    fn shorten_tail_zero_segment() {
        let mut segment = Segment::new(Coordinates::new(3, 4), Orientation::South, 0);

        assert_eq!(segment.shorten_tail(), Err(Error::SegmentAlreadyZero));
    }
}
