use crate::app::Dash;
use crate::msg::Message;
use crate::row_ui::make_row;
use crate::util::{fps_to_sps, mph_to_fps, row_feet_to_acres};
use iced::widget::{Button, Column, Container, Row, Space, Text};
use iced::Length;

const SCREEN_WIDTH: u16 = 800;
const SCREEN_HEIGHT: u16 = 480;
const HEAD_HEIGHT: u16 = 40;
const FOOT_HEIGHT: u16 = 80;
pub const BODY_HEIGHT: u16 = SCREEN_HEIGHT - HEAD_HEIGHT - FOOT_HEIGHT;

pub fn make_page(dash: &Dash) -> Container<Message> {
    let body = Column::new()
        .push(header(dash).height(HEAD_HEIGHT))
        .push(body(dash).height(BODY_HEIGHT))
        .push(footer(dash).height(FOOT_HEIGHT));

    Container::new(body).height(Length::Fill)
}

fn body(dash: &Dash) -> Container<Message> {
    let col = Column::new()
        .push(make_row(dash, 0))
        .push(make_row(dash, 1));
    Container::new(col).height(BODY_HEIGHT)
}

fn header(dash: &Dash) -> Container<Message> {
    let rowft = dash.row_feet_planted();
    let acres = row_feet_to_acres(rowft);
    let mph = dash.ground_speed_mph();
    let fps = mph_to_fps(mph);
    let sps = fps_to_sps(fps, dash.in_between_seed);

    let row = Row::new()
        .push(Text::new(format!("Acres: {acres:<.2} | Rows: {rowft}'")))
        .push(Space::new(Length::Fill, Length::Fill))
        .push(Text::new(format!(
            "{mph:<.1} MPH  |  {fps:<.1} FPS  |  {sps:<.1} SPS"
        )));
    Container::new(row).width(Length::Fill)
}

fn footer(dash: &Dash) -> Container<Message> {
    let row = Row::new()
        .push(
            Button::new("+")
                .height(FOOT_HEIGHT)
                .width(FOOT_HEIGHT)
                .on_press(Message::IncreaseSpacing),
        )
        .push(Text::new("10").size(24))
        .push(
            Button::new("-")
                .height(FOOT_HEIGHT)
                .width(FOOT_HEIGHT)
                .on_press(Message::DecreaseSpacing),
        );
    Container::new(row).width(Length::Fill)
}
