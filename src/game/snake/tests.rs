#[cfg(test)]
mod tests {
    use crate::game::Orientation;
    use crate::game::apple;
    use crate::game::apple::Apple;
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
                Coordinates::new(3, 0),
                Coordinates::new(3, 1),
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
        snake.advance(false).unwrap();

        assert_eq!(snake, snake_after);
    }

    #[test]
    fn snake_has_not_collided_with_itself() {
        let snake = Snake::from_body(
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

        assert!(!snake.has_collided_with_itself().unwrap());
    }

    #[test]
    fn snake_has_collided_with_itself() {
        let snake = Snake::from_body(
            vec![
                Coordinates::new(1, 0),
                Coordinates::new(1, 1),
                Coordinates::new(2, 1),
                Coordinates::new(2, 0),
                Coordinates::new(1, 0),
                Coordinates::new(0, 0),
            ],
            Orientation::East,
        )
        .unwrap();

        assert!(snake.has_collided_with_itself().unwrap());
    }

    #[test]
    fn snake_has_reached_apple() {
        let snake = Snake::from_body(
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

        let apple = Apple::new(3, 2);

        assert!(snake.has_reached_apple(&apple).unwrap());
    }

    #[test]
    fn snake_has_not_reached_apple() {
        let snake = Snake::from_body(
            vec![
                Coordinates::new(2, 2),
                Coordinates::new(2, 1),
                Coordinates::new(2, 0),
                Coordinates::new(1, 0),
                Coordinates::new(0, 0),
            ],
            Orientation::East,
        )
        .unwrap();

        let apple = Apple::new(3, 2);

        assert!(!snake.has_reached_apple(&apple).unwrap());
    }
}
