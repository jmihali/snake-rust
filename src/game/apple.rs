use crate::Coordinates;
use crate::Snake;

pub struct Apple {
    coordinates: Coordinates,
}

impl Apple {
    pub fn random(max_x: i32, may_y: i32, snake: &Snake) -> Self {
        loop {
            let coordinates = Coordinates::random(max_x, may_y);

            if snake.get_body().iter().all(|x| *x != coordinates) {
                return Self { coordinates };
            }
        }
    }

    pub fn get_coordinates(&self) -> &Coordinates {
        &self.coordinates
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::game::Orientation;

    #[test]
    fn apple_new() {
        let snake = Snake::from_body(
            vec![
                Coordinates::new(0, 0),
                Coordinates::new(1, 0),
                Coordinates::new(2, 0),
                Coordinates::new(2, 1),
                Coordinates::new(2, 2),
                Coordinates::new(3, 2),
            ],
            Orientation::East,
        )
        .unwrap();

        let apple = Apple::random(5, 5, &snake);

        assert!(snake.get_body().iter().all(|x| *x != apple.coordinates))
    }
}
