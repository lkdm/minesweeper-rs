// Field enum and FieldData struct and their implementation are a simple finite state machine.

#[derive(Debug, Clone, Copy)]
pub struct FieldData {
    pub has_mine: bool,
    pub adjacent_mines: u8,
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
    // If the field is revealed, do nothing and return None. Else, reveal the field, and return the field data.
    pub fn reveal(&mut self) {
        match self {
            Field::Unrevealed(fdata) => *self = Field::Revealed(*fdata),
            Field::Flagged(fdata) => *self = Field::Revealed(*fdata),
            _ => {},
        }
    }

    pub fn has_mine(&self) -> bool {
        match self {
            Field::Revealed(fdata) => fdata.has_mine,
            _ => false,
        }
    }
}
