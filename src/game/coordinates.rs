use crate::game::Orientation;
use rand;

#[derive(Debug, Clone, Default, PartialEq)]
pub struct Coordinates {
    x: u32,
    y: u32,
}

impl Coordinates {
    pub fn new(x: u32, y: u32) -> Self {
        Self { x, y }
    }

    pub fn random(max_x: u32, may_y: u32) -> Self {
        Self {
            x: rand::random_range(0..=max_x),
            y: rand::random_range(0..=may_y),
        }
    }
}

impl Coordinates {
    pub fn get_x(&self) -> u32 {
        return self.x;
    }

    pub fn get_y(&self) -> u32 {
        return self.y;
    }
}

impl Coordinates {
    pub fn move_coordinates(&mut self, orientation: &Orientation, distance: u32) {
        match orientation {
            Orientation::North => {
                self.y += distance;
            }
            Orientation::South => {
                self.y -= distance;
            }
            Orientation::East => {
                self.x += distance;
            }
            Orientation::West => {
                self.x -= distance;
            }
        }
    }

    pub fn get_relative_coordinates(
        &self,
        orientation: &Orientation,
        distance: u32,
    ) -> Coordinates {
        match orientation {
            Orientation::North => Coordinates {
                x: self.x,
                y: self.y + distance,
            },
            Orientation::South => Coordinates {
                x: self.x,
                y: self.y - distance,
            },
            Orientation::East => Coordinates {
                x: self.x + distance,
                y: self.y,
            },
            Orientation::West => Coordinates {
                x: self.x - distance,
                y: self.y,
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn coordinates_new() {
        let x = 4;
        let y = 10;
        let coordinates = Coordinates::new(x, y);

        assert!(coordinates.get_x() == x);
        assert!(coordinates.get_y() == y);
    }

    #[test]
    fn coordinates_random() {
        let max_x = 4;
        let max_y = 10;
        let coordinates = Coordinates::random(max_x, max_y);

        assert!(coordinates.get_x() <= max_x);
        assert!(coordinates.get_y() <= max_y);
    }
}
