use crate::gui::{make_dash_page, make_io_page};
use crate::monitor::Monitor;
use crate::msg::Message;
use iced::{executor, Application, Command, Element, Renderer, Subscription, Theme};
use rppal::gpio::Gpio;

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
}
