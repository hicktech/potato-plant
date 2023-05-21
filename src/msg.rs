use crate::io::{Cmd, Event};

#[derive(Debug, Clone)]
pub enum Message {
    Halt,
    IncreaseSpacing,
    DecreaseSpacing,
    ToggleAutoPrime(usize, bool),
    FillHopper(usize),
    TabSelected(usize),
    SimulateCmd(Cmd),
    IOEvent(Event),
}
