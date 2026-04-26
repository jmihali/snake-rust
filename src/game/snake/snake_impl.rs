use std::collections::{VecDeque, vec_deque};

use crate::game::coordinates::Coordinates;
use crate::game::orientation::{self, Orientation};
use crate::game::segment::Segment;
use crate::game::snake::{Error, Result};

pub struct Snake {
    // in the body, the first segment is the tail,
    // and the last segment is the head
    body: VecDeque<Segment>,
}

// constructors
impl Snake {
    pub fn new(tail_coordinates: Coordinates, orientation: Orientation, length: u32) -> Self {
        let body = VecDeque::from([Segment::new(tail_coordinates, orientation, length)]);
        Self { body: body }
    }

    pub fn from_body<I>(body: I) -> Result<Self>
    where
        I: Into<VecDeque<Segment>>,
    {
        let body = body.into();

        // Use .iter() and .zip() to simulate windows without cloning or make_contiguous
        let is_contiguous = body
            .iter()
            .zip(body.iter().skip(1))
            .all(|(current, next)| current.get_head_coordinates() == next.get_tail_coordinates());

        if !is_contiguous {
            return Err(Error::BodyNotContiguous);
        }

        Ok(Self { body })
    }
}

// helper methods
impl Snake {
    fn remove_tail_segment(&mut self) -> Result<()> {
        let tail = self.body.pop_front();
        if tail.is_none() {
            return Err(Error::SnakeBodyEmpty);
        }
        Ok(())
    }

    fn add_head_segment(&mut self, orientation: Orientation) -> Result<()> {
        let current_head_segment = self.body.back();

        match current_head_segment {
            Some(head) => {
                let new_head_segment_tail = head.get_head_coordinates();
                let new_head_segment = Segment::new(new_head_segment_tail, orientation, 0);
                self.body.push_back(new_head_segment);
                Ok(())
            }
            None => Err(Error::SnakeBodyEmpty),
        }
    }

    fn shorten_tail_segment(&mut self) -> Result<()> {
        match self.body.front_mut() {
            Some(tail) => {
                tail.shorten_tail();
                Ok(())
            }
            None => Err(Error::SnakeBodyEmpty),
        }
    }

    fn get_tail_length(&self) -> Result<u32> {
        match self.body.front() {
            Some(tail) => Ok(tail.get_length()),
            None => Err(Error::SnakeBodyEmpty),
        }
    }

    fn extend_head_segment(&mut self) -> Result<()> {
        match self.body.back_mut() {
            Some(head) => Ok(head.extend_head()),
            None => Err(Error::SnakeBodyEmpty),
        }
    }
}

// public methods
impl Snake {
    pub fn get_head_orientation(&self) -> Result<Orientation> {
        match self.body.back() {
            Some(head) => Ok(head.get_orientation()),
            None => Err(Error::SnakeBodyEmpty),
        }
    }

    pub fn set_head_orientation(&mut self, orientation: Orientation) -> Result<()> {
        if orientation == self.get_head_orientation()? {
            return Ok(());
        }

        match self.get_head_orientation()? {
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

        self.add_head_segment(orientation)?;

        Ok(())
    }

    pub fn advance(&mut self, grow: bool) -> Result<()> {
        self.extend_head_segment()?;
        if !grow {
            self.shorten_tail_segment()?;
            if self.get_tail_length()? == 0 {
                self.remove_tail_segment()?;
            }
        }
        Ok(())
    }
}
