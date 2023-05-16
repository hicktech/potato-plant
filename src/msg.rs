#[derive(Debug, Clone)]
pub enum Message {
    Halt,
    IncreaseSpacing,
    DecreaseSpacing,
    ToggleAutoPrime(usize, bool),
    FillHopper(usize),
}
