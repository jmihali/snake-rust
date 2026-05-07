use crate::game::coordinates::Coordinates;
use crate::game::orientation::Orientation;
use crate::game::snake::{Error, Result};

#[derive(Debug, PartialEq)]
pub struct Snake {
    body: Vec<Coordinates>, // body[0] = head
    head_orientation: Orientation,
}

// constructors
impl Snake {
    pub fn new(coordinates: Coordinates, orientation: Orientation) -> Self {
        Self {
            body: vec![coordinates],
            head_orientation: orientation,
        }
    }

    pub fn from_body(body: Vec<Coordinates>, head_orientation: Orientation) -> Result<Self> {
        // Use .iter() and .zip() to simulate windows without cloning or make_contiguous
        let is_contiguous = body
            .iter()
            .zip(body.iter().skip(1))
            .all(|(current, next)| current.is_contiguous_to(next));

        if !is_contiguous {
            return Err(Error::SnakeBodyNotContiguous);
        }

        // todo: handle case when head orientation is opposite to the body
        Ok(Self {
            body,
            head_orientation,
        })
    }
}

// helper methods
impl Snake {
    fn get_head_orientation(&self) -> &Orientation {
        &self.head_orientation
    }

    fn shorten_tail(&mut self) -> Result<()> {
        if let Some(_) = self.body.pop() {
            return Ok(());
        }
        Err(Error::SnakeBodyEmpty)
    }

    fn extend_head(&mut self) -> Result<()> {
        if let Some(current_head_coordinates) = self.body.get(0) {
            let new_head_coordinates = match self.head_orientation {
                Orientation::North => Coordinates::new(
                    current_head_coordinates.get_x(),
                    current_head_coordinates.get_y() + 1,
                ),
                Orientation::South => Coordinates::new(
                    current_head_coordinates.get_x(),
                    current_head_coordinates.get_y() - 1,
                ),
                Orientation::East => Coordinates::new(
                    current_head_coordinates.get_x() + 1,
                    current_head_coordinates.get_y(),
                ),
                Orientation::West => Coordinates::new(
                    current_head_coordinates.get_x() - 1,
                    current_head_coordinates.get_y(),
                ),
            };

            self.body.insert(0, new_head_coordinates);
            Ok(())
        } else {
            return Err(Error::SnakeBodyEmpty);
        }
    }
}

// public methods
impl Snake {
    pub fn set_head_orientation(&mut self, orientation: Orientation) -> Result<()> {
        if orientation == self.head_orientation {
            return Ok(());
        }

        match self.head_orientation {
            Orientation::North => {
                if orientation == Orientation::South {
                    return Ok(());
                }
            }
            Orientation::South => {
                if orientation == Orientation::North {
                    return Ok(());
                }
            }
            Orientation::East => {
                if orientation == Orientation::West {
                    return Ok(());
                }
            }
            Orientation::West => {
                if orientation == Orientation::East {
                    return Ok(());
                }
            }
        }

        self.head_orientation = orientation;

        Ok(())
    }

    pub fn advance(&mut self, grow: bool) -> Result<()> {
        self.extend_head()?;
        if !grow {
            self.shorten_tail()?;
        }
        Ok(())
    }

    pub fn get_body(&self) -> &Vec<Coordinates> {
        &self.body
    }
}
