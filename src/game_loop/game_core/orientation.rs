#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Default)]
pub enum Orientation {
    #[default]
    North,
    South,
    East,
    West,
}

impl Orientation {
    pub fn is_opposite(&self, other: Orientation) -> bool {
        matches!(
            (*self, other),
            (Orientation::North, Orientation::South)
                | (Orientation::South, Orientation::North)
                | (Orientation::East, Orientation::West)
                | (Orientation::West, Orientation::East)
        )
    }

    pub fn get_delta(&self) -> (i32, i32) {
        match self {
            Orientation::North => (0, -1),
            Orientation::South => (0, 1),
            Orientation::East => (1, 0),
            Orientation::West => (-1, 0),
        }
    }
}
