#[cfg(test)]
mod tests {
    use std::collections::VecDeque;

    use crate::game::Orientation;
    use crate::game::Segment;
    use crate::game::coordinates::*;
    use crate::game::snake;
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

    #[test]
    fn advance_snake_nogrow() {
        let mut snake = Snake::from_body(VecDeque::from([
            Segment::new(Coordinates::new(0, 0), Orientation::East, 4),
            Segment::new(Coordinates::new(4, 0), Orientation::North, 3),
            Segment::new(Coordinates::new(4, 3), Orientation::West, 3),
        ]))
        .unwrap();

        let snake_after = Snake::from_body(VecDeque::from([
            Segment::new(Coordinates::new(1, 0), Orientation::East, 3),
            Segment::new(Coordinates::new(4, 0), Orientation::North, 3),
            Segment::new(Coordinates::new(4, 3), Orientation::West, 4),
        ]))
        .unwrap();

        snake.advance(false).unwrap();

        assert_eq!(snake, snake_after);
    }

    #[test]
    fn advance_snake_grow() {
        let mut snake = Snake::from_body(VecDeque::from([
            Segment::new(Coordinates::new(0, 0), Orientation::East, 4),
            Segment::new(Coordinates::new(4, 0), Orientation::North, 3),
            Segment::new(Coordinates::new(4, 3), Orientation::West, 3),
        ]))
        .unwrap();

        let snake_after = Snake::from_body(VecDeque::from([
            Segment::new(Coordinates::new(0, 0), Orientation::East, 4),
            Segment::new(Coordinates::new(4, 0), Orientation::North, 3),
            Segment::new(Coordinates::new(4, 3), Orientation::West, 4),
        ]))
        .unwrap();

        snake.advance(true).unwrap();

        assert_eq!(snake, snake_after);
    }

    #[test]
    fn advance_snake_turn_north() {
        let mut snake = Snake::from_body(VecDeque::from([
            Segment::new(Coordinates::new(0, 0), Orientation::East, 4),
            Segment::new(Coordinates::new(4, 0), Orientation::North, 3),
            Segment::new(Coordinates::new(4, 3), Orientation::West, 3),
        ]))
        .unwrap();

        let snake_after = Snake::from_body(VecDeque::from([
            Segment::new(Coordinates::new(2, 0), Orientation::East, 2),
            Segment::new(Coordinates::new(4, 0), Orientation::North, 3),
            Segment::new(Coordinates::new(4, 3), Orientation::West, 3),
            Segment::new(Coordinates::new(1, 3), Orientation::North, 2),
        ]))
        .unwrap();

        snake.set_head_orientation(Orientation::North).unwrap();
        snake.advance(false).unwrap();
        snake.advance(false).unwrap();

        assert_eq!(snake, snake_after);
    }

    #[test]
    fn advance_snake_tail_disappears() {
        let mut snake = Snake::from_body(VecDeque::from([
            Segment::new(Coordinates::new(0, 0), Orientation::East, 4),
            Segment::new(Coordinates::new(4, 0), Orientation::North, 3),
            Segment::new(Coordinates::new(4, 3), Orientation::West, 3),
        ]))
        .unwrap();

        let snake_after = Snake::from_body(VecDeque::from([
            Segment::new(Coordinates::new(4, 0), Orientation::North, 3),
            Segment::new(Coordinates::new(4, 3), Orientation::West, 7),
        ]))
        .unwrap();

        snake.advance(false).unwrap();
        snake.advance(false).unwrap();
        snake.advance(false).unwrap();
        snake.advance(false).unwrap();

        assert_eq!(snake, snake_after);
    }
}
