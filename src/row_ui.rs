use crate::app::Dash;
use crate::gui::BODY_HEIGHT;
use crate::msg::Message;
use crate::msg::Message::FillHopper;
use iced::widget::{container, image, row, Button, Column, Container, Row, Text, Toggler};
use iced::Length;

use crate::row_ui::Message::ToggleAutoPrime;

pub fn make_row(dash: &Dash, id: usize) -> Container<Message> {
    let col = Column::new()
        .push(row![Text::new(format!("Row {}", id + 1))])
        .push(row![
            gear_icon(dash.priming(id)),
            Button::new("Prime").on_press(FillHopper(id))
        ])
        .push(row![Toggler::new(
            "Auto: ".to_string(),
            dash.auto_prime_on(id),
            move |b| { ToggleAutoPrime(id, b) }
        )]);
    Container::new(col).height(BODY_HEIGHT / 2)
}

fn gear_icon<'a>(running: bool) -> Container<'a, Message> {
    container(if running {
        Text::new("1")
    } else {
        Text::new("0")
    })
    .width(Length::Fill)
    .center_x()
}
