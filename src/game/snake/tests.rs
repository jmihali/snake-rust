#[cfg(test)]
mod tests {
    use std::collections::VecDeque;

    use crate::game::Orientation;
    use crate::game::Segment;
    use crate::game::coordinates::*;
    use crate::game::snake::*;

    #[test]
    fn constructor_from_contiguous_body() {
        let body = VecDeque::from([
            Segment::new(Coordinates::new(0, 0), Orientation::East, 4),
            Segment::new(Coordinates::new(4, 0), Orientation::North, 3),
            Segment::new(Coordinates::new(4, 3), Orientation::West, 3),
        ]);

        assert!(Snake::from_body(body).is_ok());
    }

    #[test]
    fn constructor_from_non_contiguous_body() {
        let body = VecDeque::from([
            Segment::new(Coordinates::new(0, 0), Orientation::East, 4),
            Segment::new(Coordinates::new(4, 0), Orientation::North, 2),
            Segment::new(Coordinates::new(4, 3), Orientation::West, 3),
        ]);

        assert!(Snake::from_body(body).is_err());
    }
}
