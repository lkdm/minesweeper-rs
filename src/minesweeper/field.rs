#[derive(Debug, Clone, Copy)]
pub enum Field {
    // The only way to change a Field's state is to flag, reveal or unflag it.
    Unrevealed,
    Flagged,
    Revealed(u8),
    Mine,
}

impl Field {
    pub fn flag(&mut self) {
        match self {
            Field::Unrevealed => *self = Field::Flagged,
            Field::Flagged => *self = Field::Unrevealed,
            _ => (),
        }
    }
    pub fn reveal(&mut self, adjacent_mines: u8) {
        match self {
            Field::Unrevealed => *self = Field::Revealed(adjacent_mines),
            Field::Flagged => *self = Field::Revealed(adjacent_mines),
            Field::Revealed(_) => (),
            Field::Mine => (),
        }
    }
}
