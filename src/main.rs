use iced::window::Position;
use iced::{window, Application, Settings};
use popl::app::Dash;
use popl::io::{IoCfg, IO};
use popl::monitor::Monitor;

fn main() -> iced::Result {
    #[cfg(all(target_arch = "arm"))]
    let io = IO::new(IoCfg::default()).expect("io init error");
    //#[cfg(all(target_arch = "x86_64"))]
    let io = IO::fake(IoCfg::default()).expect("io init error");

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
            seed_wheel_speed_rpm: 100.0,
            auto_prime: [true, true],
            priming: [false, false],
            planter_raised: false,
            io,
        },
        ..Settings::default()
    })
}
