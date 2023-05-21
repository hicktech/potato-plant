use iced::{executor, subscription, Application, Command, Element, Renderer, Subscription, Theme};
use std::time::Duration;

use crate::gui::{make_dash_page, make_io_page};
use crate::monitor::Monitor;
use crate::msg::Message;

pub enum Page {
    Dashboard,
    SoftIO,
}

/// Potato planting dashboard
pub struct Dash {
    monitor: Monitor,
    pub page: Page,
    pub in_between_seed: f32,
}

impl Dash {
    pub fn planter_raised(&self) -> bool {
        self.monitor.planter_raised
    }

    pub fn auto_prime_on(&self, id: usize) -> bool {
        self.monitor.auto_prime[id]
    }

    pub fn priming(&self, id: usize) -> bool {
        self.monitor.priming[id]
    }

    pub fn row_feet_planted(&self) -> f32 {
        self.monitor.feet_planted
    }

    pub fn ground_speed_mph(&self) -> f32 {
        self.monitor.ground_speed_mph
    }

    pub fn seed_wheel_speed_rpm(&self) -> f32 {
        self.monitor.seed_wheel_speed_rpm
    }
}

impl Application for Dash {
    type Executor = executor::Default;
    type Message = Message;
    type Theme = Theme;
    type Flags = Monitor;

    fn new(flags: Self::Flags) -> (Self, Command<Self::Message>) {
        (
            Dash {
                monitor: flags,
                page: Page::Dashboard,
                in_between_seed: 10.0,
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        String::from("Monitor")
    }

    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        use Message::*;
        match message {
            ToggleAutoPrime(id, v) => self.monitor.auto_prime[id] = v,
            FillHopper(id) => {
                self.monitor.priming[id] = !self.monitor.priming[id];
                self.monitor.enable_seed_belt(id, self.monitor.priming[id]);
            }
            Halt => self.monitor.halt(),
            TabSelected(i) if i == 0 => self.page = Page::Dashboard,
            TabSelected(i) if i == 1 => self.page = Page::SoftIO,
            IOEvent(e) => self.monitor.handle_event(e),
            SimulateCmd(cmd) => self.monitor.io.tx.send(cmd).unwrap(),
            _ => {}
        };
        Command::none()
    }

    fn view(&self) -> Element<'_, Self::Message, Renderer<Self::Theme>> {
        match self.page {
            Page::Dashboard => make_dash_page(self).into(),
            Page::SoftIO => make_io_page(self).into(),
        }
    }

    fn subscription(&self) -> Subscription<Self::Message> {
        let rx = self.monitor.io.rx.clone();
        subscription::unfold("foo", (rx), move |(rx)| async {
            match rx.recv_timeout(Duration::from_secs(1)) {
                Ok(e) => (Message::IOEvent(e), (rx)),
                _ => (Message::Halt, (rx)),
            }
        })
    }
}
