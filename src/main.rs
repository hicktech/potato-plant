use iced::window::Position;
use iced::{window, Application, Settings};
use popl::app::Dash;
use popl::state::Monitor;

fn main() -> iced::Result {
    Dash::run(Settings {
        id: None,
        antialiasing: true,
        exit_on_close_request: true,
        window: window::Settings {
            size: (800, 480),
            position: Position::Centered,
            resizable: false,
            ..window::Settings::default()
        },
        flags: Monitor {
            feet_planted: 19166.4,
            ground_speed_mph: 3.3,
            ..Monitor::default()
        },
        ..Settings::default()
    })
}
