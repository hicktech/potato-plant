#[derive(Debug, Clone)]
pub enum Message {
    IncreaseSpacing,
    DecreaseSpacing,
    ToggleAutoPrime(usize, bool),
    StartPrime(usize),
}
