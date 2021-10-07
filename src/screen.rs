use crate::pixel::Pixel;

use grid::{self, Grid};
use iced::{executor, Application, Column, Command, Container, Text};

pub struct ScreenOptions {
    title: String,
    dimensions: (usize, usize),
    speed: usize,
}

impl ScreenOptions {
    pub fn new(
        title: Option<&str>,
        dimensions: Option<(usize, usize)>,
        speed: Option<usize>,
    ) -> Self {
        Self {
            title: match title {
                Some(title) => title.to_string(),
                None => String::new(),
            },
            dimensions: dimensions.unwrap_or((32, 24)),
            speed: speed.unwrap_or(5),
        }
    }
}

impl Default for ScreenOptions {
    fn default() -> Self {
        Self::new(None, None, None)
    }
}

pub struct Screen {
    title: String,
    grid: Grid<Pixel>,
    queued_ticks: usize,
    speed: usize,
    next_speed: Option<usize>,
}

impl Default for Screen {
    fn default() -> Self {
        Self {
            grid: Grid::new(32, 24),
            queued_ticks: 0,
            speed: 5,
            next_speed: None,
            title: String::new(),
        }
    }
}

#[derive(Debug, Clone)]
pub enum Message {
    // Grid(grid::Message, usize),
}

impl Application for Screen {
    type Message = Message;
    type Executor = executor::Default;
    type Flags = ScreenOptions;

    fn new(flags: Self::Flags) -> (Self, iced::Command<Self::Message>) {
        let (rows, cols) = flags.dimensions;
        let grid = Grid::new(rows, cols);
        let speed = flags.speed;
        let title = flags.title;

        (
            Self {
                title,
                grid,
                speed,
                ..Self::default()
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        self.title.clone()
    }

    fn update(
        &mut self,
        _message: Self::Message,
        _clipboard: &mut iced::Clipboard,
    ) -> Command<Self::Message> {
        todo!();
        // Command::none()
    }

    fn view(&mut self) -> iced::Element<'_, Self::Message> {
        // let content = Column::new().push(
        //     self.grid
        //         .view()
        //         .map(move |message| Message::Grid(message, version)),
        // );

        // Container::new(content)
        //     .width(Length::Fill)
        //     .height(Length::Fill)
        //     .style(style::Container)
        //     .into()

        Text::new("hello world").into()
    }
}
