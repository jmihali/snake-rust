use crate::game::Coordinates;

pub struct Apple {
    coordinates: Coordinates,
}

impl Apple {
    pub fn new(x: u32, y: u32) -> Self {
        Apple {
            coordinates: Coordinates::new(x as i32, y as i32),
        }
    }

    pub fn random_grid_except(max_x: u32, max_y: u32, exceptions: &[Coordinates]) -> Option<Self> {
        let grid_size = (max_x * max_y) as usize;
        let exceptions_size = exceptions.len();

        // If snake occupies entire grid, no space for apple
        if exceptions_size >= grid_size {
            return None;
        }

        const MAX_ATTEMPTS: u32 = 100;
        for _ in 0..MAX_ATTEMPTS {
            let coordinates = Coordinates::random(max_x as i32, max_y as i32);

            if exceptions.iter().all(|segment| *segment != coordinates) {
                return Some(Self { coordinates });
            }
        }

        // Fallback: brute-force search for empty cell
        for x in 0..max_x {
            for y in 0..max_y {
                let coordinates: Coordinates = Coordinates::new(x as i32, y as i32);
                if !exceptions.contains(&coordinates) {
                    return Some(Self { coordinates });
                }
            }
        }

        None
    }

    pub fn get_coordinates(&self) -> &Coordinates {
        &self.coordinates
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::game::Orientation;
    use crate::game::Snake;

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

        let apple = Apple::random_grid_except(5, 5, snake.get_body()).unwrap();

        assert!(snake.get_body().iter().all(|x| *x != apple.coordinates))
    }
}
