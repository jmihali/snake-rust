#[cfg(test)]
mod tests {
    use crate::game::Orientation;
    use crate::game::coordinates::*;
    use crate::game::snake::*;

    #[test]
    fn constructor_from_contiguous_body() {
        let body = vec![
            Coordinates::new(0, 0),
            Coordinates::new(1, 0),
            Coordinates::new(2, 0),
            Coordinates::new(2, 1),
            Coordinates::new(2, 2),
            Coordinates::new(3, 2),
        ];

        assert!(Snake::from_body(body, Orientation::East).is_ok());
    }

    #[test]
    fn constructor_from_non_contiguous_body() {
        let body = vec![
            Coordinates::new(0, 0),
            Coordinates::new(1, 0),
            Coordinates::new(2, 0),
            Coordinates::new(2, 2),
            Coordinates::new(3, 2),
        ];

        assert!(Snake::from_body(body, Orientation::East).is_err());
    }

    #[test]
    fn advance_snake_nogrow() {
        let mut snake = Snake::from_body(
            vec![
                Coordinates::new(3, 2),
                Coordinates::new(2, 2),
                Coordinates::new(2, 1),
                Coordinates::new(2, 0),
                Coordinates::new(1, 0),
                Coordinates::new(0, 0),
            ],
            Orientation::East,
        )
        .unwrap();

        let snake_after = Snake::from_body(
            vec![
                Coordinates::new(4, 2),
                Coordinates::new(3, 2),
                Coordinates::new(2, 2),
                Coordinates::new(2, 1),
                Coordinates::new(2, 0),
                Coordinates::new(1, 0),
            ],
            Orientation::East,
        )
        .unwrap();

        snake.advance(false).unwrap();

        assert_eq!(snake, snake_after);
    }

    #[test]
    fn advance_snake_grow() {
        let mut snake = Snake::from_body(
            vec![
                Coordinates::new(3, 2),
                Coordinates::new(2, 2),
                Coordinates::new(2, 1),
                Coordinates::new(2, 0),
                Coordinates::new(1, 0),
                Coordinates::new(0, 0),
            ],
            Orientation::East,
        )
        .unwrap();

        let snake_after = Snake::from_body(
            vec![
                Coordinates::new(4, 2),
                Coordinates::new(3, 2),
                Coordinates::new(2, 2),
                Coordinates::new(2, 1),
                Coordinates::new(2, 0),
                Coordinates::new(1, 0),
                Coordinates::new(0, 0),
            ],
            Orientation::East,
        )
        .unwrap();

        snake.advance(true).unwrap();

        assert_eq!(snake, snake_after);
    }

    #[test]
    fn advance_snake_turn_north() {
        let mut snake = Snake::from_body(
            vec![
                Coordinates::new(3, 2),
                Coordinates::new(2, 2),
                Coordinates::new(2, 1),
                Coordinates::new(2, 0),
                Coordinates::new(1, 0),
                Coordinates::new(0, 0),
            ],
            Orientation::East,
        )
        .unwrap();

        let snake_after = Snake::from_body(
            vec![
                Coordinates::new(3, 5),
                Coordinates::new(3, 4),
                Coordinates::new(3, 3),
                Coordinates::new(3, 2),
                Coordinates::new(2, 2),
                Coordinates::new(2, 1),
                Coordinates::new(2, 0),
                Coordinates::new(1, 0),
            ],
            Orientation::North,
        )
        .unwrap();

        snake.set_head_orientation(Orientation::North).unwrap();

        snake.advance(true).unwrap();
        snake.advance(true).unwrap();
        snake.advance(false).unwrap();

        assert_eq!(snake, snake_after);
    }
}
