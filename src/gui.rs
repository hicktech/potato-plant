use crate::app::{Dash, Page};
use crate::io::Cmd::{LowerPlanter, RaisePlanter, SeedBeltControl};
use crate::io::Event::{GroundSpeed, PlanterLowered, PlanterRaised, SeedWheelSpeed};
use crate::msg::Message;
use crate::msg::Message::{IOEvent, SimulateCmd};
use crate::row_ui::make_row;
use crate::util::{fps_to_sps, mph_to_fps, row_feet_to_acres, rpm_to_seed_per_second};
use iced::widget::{
    horizontal_space, row, slider, Button, Column, Container, Row, Slider, Space, Text, Toggler,
};
use iced::{alignment, Alignment, Length, Renderer, Theme};
use iced_aw::graphics::IconText;
use iced_aw::{Icon, TabBar, TabLabel};

const SCREEN_WIDTH: u16 = 800;
const SCREEN_HEIGHT: u16 = 480;
const HEAD_HEIGHT: u16 = 35;
const FOOT_HEIGHT: u16 = 65;
const TAB_HEIGHT: u16 = 25;
pub const BODY_HEIGHT: u16 = SCREEN_HEIGHT - TAB_HEIGHT - HEAD_HEIGHT - FOOT_HEIGHT;

pub fn make_tabs(dash: &Dash) -> Container<Message> {
    let mut tabs = TabBar::new(0, Message::TabSelected);
    tabs = tabs.push(TabLabel::Text("monitor".to_string()));
    tabs = tabs.push(TabLabel::Text("io".to_string()));
    Container::new(tabs)
}

pub fn make_dash_page(dash: &Dash) -> Container<Message> {
    let body = Column::new()
        .push(make_tabs(dash))
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
    let target_sps = fps_to_sps(fps, dash.in_between_seed);
    let actual_sps = rpm_to_seed_per_second(dash.seed_wheel_speed_rpm());

    let row = Row::new()
        .push(Text::new(format!("Acres: {acres:<.2} | Rows: {rowft}'")))
        .push(Space::new(Length::Fill, Length::Fill))
        .push(if dash.planter_raised() {
            IconText::new(Icon::ArrowUp)
        } else {
            IconText::new(Icon::ArrowDown)
        })
        .push(Space::new(Length::Fill, Length::Fill))
        .push(Text::new(format!(
            "{mph:<.1} MPH  |  {fps:<.1} FPS  |  {target_sps:<.1} SPS | {actual_sps}"
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
        )
        .push(
            Button::new("X")
                .height(FOOT_HEIGHT)
                .width(FOOT_HEIGHT)
                .on_press(Message::Halt),
        );
    Container::new(row).width(Length::Fill)
}

pub fn make_io_page(dash: &Dash) -> Container<Message> {
    let body = Column::new()
        .push(make_tabs(dash))
        .push(Toggler::new(
            "Planter Raised:".to_string(),
            dash.planter_raised(),
            move |b| SimulateCmd(if b { RaisePlanter } else { LowerPlanter }),
        ))
        .push(row![
            Text::new("Ground Speed"),
            slider(0.0..=10.0, dash.ground_speed_mph(), |v| {
                IOEvent(GroundSpeed(v))
            })
            .step(0.1)
        ])
        .push(Toggler::new(
            "Hopper 1 fill switch:".to_string(),
            dash.priming(0),
            move |b| SimulateCmd(SeedBeltControl(0, b)),
        ))
        .push(Toggler::new(
            "Hopper 2 fill switch:".to_string(),
            dash.priming(1),
            move |b| SimulateCmd(SeedBeltControl(1, b)),
        ))
        .push(row![
            Text::new("Seed wheel speed"),
            slider(0.0..=50.0, dash.seed_wheel_speed_rpm(), |v| {
                IOEvent(SeedWheelSpeed(v))
            })
            .step(0.1)
        ]);

    Container::new(body).width(Length::Fill)
}
