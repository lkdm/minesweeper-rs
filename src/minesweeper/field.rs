// Field enum and FieldData struct and their implementation are a simple finite state machine.

#[derive(Debug, Clone, Copy)]
pub enum FieldState {
    Closed,
    Open,
    Flagged
}

#[derive(Debug, Clone, Copy)]
pub enum Field {
    Number {
        state: FieldState,
        count: u8,
    },
    Mine {
        state: FieldState,
    }
}


impl Field {
    pub fn new(has_mine: bool, adjacent_mines: u8) -> Field {
        if has_mine {
            Field::Mine {
                state: FieldState::Closed,
            }
        } else {
            Field::Number {
                state: FieldState::Closed,
                count: adjacent_mines,
            }
        }
    }

    pub fn flag(&mut self) {
        match self {
            Field::Number { state, .. } | Field::Mine { state } => {
                match state {
                    FieldState::Closed => *state = FieldState::Flagged,
                    FieldState::Flagged => *state = FieldState::Closed,
                    FieldState::Open => {}
                }
            }
        }
    }

    pub fn open(&mut self) {
        match self {
            Field::Number { state, .. } | Field::Mine { state } => {
                match state {
                    FieldState::Closed => *state = FieldState::Open,
                    FieldState::Flagged => {}
                    FieldState::Open => {}
                }
            }
        }
    }

}
