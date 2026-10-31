#[derive(Debug, Clone, Copy)]
pub enum Neighbor {
    North,
    NorthEast,
    SouthEast,
    South,
    SouthWest,
    NorthWest
}

impl Neighbor {
    pub const fn all() -> [Neighbor; 6] {
        [
            Self::North,
            Self::NorthEast,
            Self::SouthEast,
            Self::South,
            Self::SouthWest,
            Self::NorthWest
        ]
    }
}
