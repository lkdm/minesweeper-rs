// Field enum and FieldData struct and their implementation are a simple finite state machine.

#[derive(Debug, Clone, Copy)]
struct FieldData {
    has_mine: bool,
    adjacent_mines: u8,
}

#[derive(Debug, Clone, Copy)]
pub enum Field {
    Unrevealed(FieldData),
    Revealed(FieldData),
    Flagged(FieldData)
}

impl Field {
    pub fn new(has_mine: bool, adjacent_mines: u8) -> Field {
        Field::Unrevealed(FieldData {
            has_mine,
            adjacent_mines,
        })
    }
    pub fn flag(&mut self) {
        match self {
            Field::Unrevealed(fdata) => *self = Field::Flagged(*fdata),
            Field::Flagged(fdata) => *self = Field::Unrevealed(*fdata),
            _ => {}
        }
    }
    pub fn reveal(&mut self) {
        match self {
            Field::Unrevealed(fdata) => *self = Field::Revealed(*fdata),
            _ => {}
        }
    }
}
