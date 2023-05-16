use crate::gui::make_page;
use crate::msg::Message;
use crate::state::Monitor;
use iced::{executor, Application, Command, Element, Renderer, Theme};

/// Potato planting dashboard
pub struct Dash {
    monitor: Monitor,
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
            FillHopper(id) => self.monitor.priming[id] = !self.monitor.priming[id],
            _ => {}
        };
        Command::none()
    }

    fn view(&self) -> Element<'_, Self::Message, Renderer<Self::Theme>> {
        make_page(self).into()
    }
}
