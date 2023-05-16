use crate::app::Dash;
use crate::gui::BODY_HEIGHT;
use crate::msg::Message;
use crate::msg::Message::StartPrime;
use iced::widget::{row, Button, Column, Container, Row, Text, Toggler};

use crate::row_ui::Message::ToggleAutoPrime;

pub fn make_row(dash: &Dash, id: usize) -> Container<Message> {
    let col = Column::new()
        .push(row![Text::new(format!("Row {}", id + 1))])
        .push(row![Button::new("Prime").on_press(StartPrime(id))])
        .push(row![Toggler::new(
            "Auto: ".to_string(),
            dash.auto_prime_on(id),
            move |b| { ToggleAutoPrime(id, b) }
        )]);
    Container::new(col).height(BODY_HEIGHT / 2)
}
