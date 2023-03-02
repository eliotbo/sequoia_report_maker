//! by John Conway. It leverages a `Canvas` together with other widgets.
mod grid;
mod plot;
mod preset;

use grid::Grid;
use plot::{plot, Plot};
use preset::Preset;

use iced::alignment::Horizontal;
use iced::executor;
use iced::theme::{self, Theme};
use iced::time;
use iced::widget::canvas;
use iced::widget::canvas::event::{self, Event};
use iced::widget::canvas::path::Builder;
use iced::widget::canvas::{Cache, Canvas, Cursor, Frame, Geometry, Path, Text};
use iced::widget::{button, checkbox, column, container, pick_list, row, slider, text};
use iced::window;
use iced::{
    Alignment, Application, Color, Command, Element, Length, Point, Rectangle, Settings, Size,
    Subscription,
};
use std::time::{Duration, Instant};

pub fn main() -> iced::Result {
    env_logger::builder().format_timestamp(None).init();

    GameOfLife::run(Settings {
        antialiasing: true,
        window: window::Settings {
            position: window::Position::Centered,
            ..window::Settings::default()
        },
        ..Settings::default()
    })
}

#[derive(Default)]
struct GameOfLife {
    grid: Grid,
    is_playing: bool,
    queued_ticks: usize,
    speed: usize,
    next_speed: Option<usize>,
    version: usize,
}

#[derive(Debug, Clone)]
pub enum Message {
    Grid(grid::Message, usize),
    Tick(Instant),
    TogglePlayback,
    ToggleGrid(bool),
    Next,
    Clear,
    SpeedChanged(f32),
    PresetPicked(Preset),
}

impl Application for GameOfLife {
    type Message = Message;
    type Theme = Theme;
    type Executor = executor::Default;
    type Flags = ();

    fn new(_flags: ()) -> (Self, Command<Message>) {
        (
            Self {
                speed: 5,
                ..Self::default()
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        String::from("Game of Life - Iced")
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::Grid(message, version) => {
                if version == self.version {
                    self.grid.update(message);
                }
            }
            Message::Tick(_) | Message::Next => {
                self.queued_ticks = (self.queued_ticks + 1).min(self.speed);

                if let Some(task) = self.grid.tick(self.queued_ticks) {
                    if let Some(speed) = self.next_speed.take() {
                        self.speed = speed;
                    }

                    self.queued_ticks = 0;

                    let version = self.version;

                    return Command::perform(task, move |message| Message::Grid(message, version));
                }
            }
            Message::TogglePlayback => {
                self.is_playing = !self.is_playing;
            }
            Message::ToggleGrid(show_grid_lines) => {
                self.grid.toggle_lines(show_grid_lines);
            }
            Message::Clear => {
                self.grid.clear();
                self.version += 1;
            }
            Message::SpeedChanged(speed) => {
                if self.is_playing {
                    self.next_speed = Some(speed.round() as usize);
                } else {
                    self.speed = speed.round() as usize;
                }
            }
            Message::PresetPicked(new_preset) => {
                self.grid = Grid::from_preset(new_preset);
                self.version += 1;
            }
        }

        Command::none()
    }

    fn subscription(&self) -> Subscription<Message> {
        if self.is_playing {
            time::every(Duration::from_millis(1000 / self.speed as u64)).map(Message::Tick)
        } else {
            Subscription::none()
        }
    }

    fn view(&self) -> Element<Message> {
        let version = self.version;
        let selected_speed = self.next_speed.unwrap_or(self.speed);
        let controls = view_controls(
            self.is_playing,
            self.grid.are_lines_visible(),
            selected_speed,
            self.grid.preset(),
        );

        // create a header with two columns of text: on the left and one on the right
        let header = row![
            text("Game of Life").size(50).width(Length::Fill),
            text("by John Conway\nyay")
                .size(20)
                .horizontal_alignment(Horizontal::Right),
        ]
        .padding([0, 100, 0, 50])
        .width(Length::Fill);

        let data1 = vec![1.0, 2.0, 3.0, 4.0, 0.0, 8.0, 7.0, 8.0, 9.0, 10.0];
        let data2 = vec![1.0, 2.0, 3.0, 4.0, 3.0, 3.0, 1.0];
        let data3 = vec![1.0, 2.0, 3.0, 4.0, 3.0, 4.0, 2.0, 2.5, 2.0, 1.0];
        let element1 = plot(data1, Length::FillPortion(2), Length::Fill);

        let element3 = plot(data3, Length::FillPortion(2), Length::Fill);

        let mid1 = plot(data2.clone(), Length::Fill, Length::FillPortion(2));
        let mid2 = plot(data2, Length::Fill, Length::FillPortion(1));

        let mid_col = column![mid1, mid2];

        let mid_audiograph = container(mid_col).width(Length::FillPortion(1));

        let audiograms = row![element1, mid_audiograph, element3];

        let content = column![header, audiograms, controls];

        container(content.align_items(Alignment::Center))
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
    }

    fn theme(&self) -> Theme {
        Theme::Dark
    }
}

fn view_controls<'a>(
    is_playing: bool,
    is_grid_enabled: bool,
    speed: usize,
    preset: Preset,
) -> Element<'a, Message> {
    let playback_controls = row![
        button(if is_playing { "Pause" } else { "Play" }).on_press(Message::TogglePlayback),
        button("Next")
            .on_press(Message::Next)
            .style(theme::Button::Secondary),
    ]
    .spacing(10);

    let speed_controls = row![
        slider(1.0..=1000.0, speed as f32, Message::SpeedChanged),
        text(format!("x{speed}")).size(16),
    ]
    .width(Length::Fill)
    .align_items(Alignment::Center)
    .spacing(10);

    row![
        playback_controls,
        speed_controls,
        checkbox("Grid", is_grid_enabled, Message::ToggleGrid)
            .size(16)
            .spacing(5)
            .text_size(16),
        pick_list(preset::ALL, Some(preset), Message::PresetPicked)
            .padding(8)
            .text_size(16),
        button("Clear")
            .on_press(Message::Clear)
            .style(theme::Button::Destructive),
    ]
    .padding(10)
    .spacing(20)
    .align_items(Alignment::Center)
    .into()
}
