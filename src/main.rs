//! by John Conway. It leverages a `Canvas` together with other widgets.
mod checkboxes;
mod config;
// mod grid;
mod legend;
mod plot;
mod preset;

use checkboxes::{Transductor, Validity};
// use grid::Grid;
use legend::{draw_legend, Legend};
use plot::{plot, Plot, Shape};
use preset::Preset;

use iced::alignment::Horizontal;
use iced::executor;
use iced::theme::{self, Theme};
use iced::time;
use iced::widget::canvas;
use iced::widget::canvas::event::{self, Event};
use iced::widget::canvas::path::Builder;
use iced::widget::canvas::{Cache, Canvas, Cursor, Frame, Geometry, Path, Text};
use iced::widget::{
    button, checkbox, column, container, container::Appearance, pick_list, radio, row, slider,
    text, vertical_space, Container, Rule,
};
use iced::window;
use iced::{
    Alignment, Application, Color, Command, Element, Length, Point, Rectangle, Settings, Size,
    Subscription,
};
use std::time::{Duration, Instant};

pub fn main() -> iced::Result {
    env_logger::builder().format_timestamp(None).init();

    AudioRox::run(Settings {
        antialiasing: true,
        window: window::Settings {
            position: window::Position::Centered,
            size: (1080, 550),
            ..window::Settings::default()
        },
        ..Settings::default()
    })
}

#[derive(Default)]
struct AudioRox {
    is_playing: bool,
    queued_ticks: usize,
    speed: usize,
    next_speed: Option<usize>,
    version: usize,

    default_checkbox: bool,
    custom_checkbox: bool,
    validity: Validity,
    transductor: Transductor,
    msp: String,
    sdp: String,
    msp4: String,
    srp: String,
}

#[derive(Debug, Clone)]
pub enum Message {
    DefaultChecked(bool),
    CustomChecked(bool),
    ValidityChanged(Validity),
    TransductorChanged(Transductor),
    MSPChanged(String),
    SDPChanged(String),
    MSP4Changed(String),
    SRPChanged(String),
}

impl Application for AudioRox {
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
        String::from("Rapport d'audiométrie")
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::DefaultChecked(value) => {
                println!("high");
                self.default_checkbox = value;
            }
            Message::CustomChecked(value) => self.custom_checkbox = value,
            Message::ValidityChanged(new_validity) => self.validity = new_validity,
            Message::TransductorChanged(new_transductor) => {
                println!("trans");
                self.transductor = new_transductor;
            }
            Message::MSPChanged(new_msp) => self.msp = new_msp,
            Message::SDPChanged(new_sdp) => self.sdp = new_sdp,
            Message::MSP4Changed(new_msp4) => self.msp4 = new_msp4,
            Message::SRPChanged(new_srp) => self.srp = new_srp,
        }

        Command::none()
    }

    fn subscription(&self) -> Subscription<Message> {
        Subscription::none()
    }

    fn view(&self) -> Element<Message> {
        ///////////////////////////////////////////// VALIDITE /////////////////////////////////////////////
        let r_size = 16;
        let t_size = 16;
        let validity = self.validity;

        let good_validity = radio(
            "Bonne",
            Validity::Good,
            Some(validity),
            Message::ValidityChanged,
        )
        .size(r_size)
        .text_size(t_size);

        let medium_validity = radio(
            "Moyenne",
            Validity::Medium,
            Some(validity),
            Message::ValidityChanged,
        )
        .size(r_size)
        .text_size(t_size);

        let null_validity = radio(
            "Nulle",
            Validity::Poor,
            Some(validity),
            Message::ValidityChanged,
        )
        .size(r_size)
        .text_size(t_size);

        let validity_section = column![good_validity, medium_validity, null_validity]
            .spacing(6)
            .width(Length::Shrink);

        let validity_title = text("Validité").size(20).width(Length::Shrink);

        let validity_content = column![validity_title, validity_section,].spacing(3);
        ///////////////////////////////////////////// VALIDITE /////////////////////////////////////////////

        ///////////////////////////////////////////// TRANSDUCTOR /////////////////////////////////////////////
        let transductor = self.transductor;

        let intra = radio(
            "Intra",
            Transductor::Intra,
            Some(transductor),
            Message::TransductorChanged,
        )
        .size(r_size)
        .text_size(t_size);

        let supra = radio(
            "Supra",
            Transductor::Supra,
            Some(transductor),
            Message::TransductorChanged,
        )
        .size(r_size)
        .text_size(t_size);

        let free = radio(
            "Circum",
            Transductor::Free,
            Some(transductor),
            Message::TransductorChanged,
        )
        .size(r_size)
        .text_size(t_size);

        let transductor_section = column![intra, supra, free].spacing(6).width(Length::Shrink);

        let transductor_title = text("Écouteurs").size(20).width(Length::Shrink);

        let transductor_content = column![transductor_title, transductor_section,].spacing(3);
        ///////////////////////////////////////////// TRANSDUCTOR /////////////////////////////////////////////

        // create a header with two columns of text: on the left and one on the right
        let text_vspace = 20.0;
        let header = row![
            column![
                text("Roxanne Bolduc,")
                    .size(26)
                    .horizontal_alignment(Horizontal::Left),
                text("Audiologiste M.P.A., OOAQ #4182")
                    .size(20)
                    .horizontal_alignment(Horizontal::Left),
            ]
            .width(Length::Fill),
            column![
                vertical_space(Length::Fixed(text_vspace)),
                Rule::horizontal(1),
                vertical_space(Length::Fixed(text_vspace)),
                Rule::horizontal(1),
                vertical_space(Length::Fixed(text_vspace)),
                Rule::horizontal(1),
                vertical_space(Length::Fixed(text_vspace)),
            ]
            .width(Length::Fixed(400.0))
        ]
        .padding([0, 5, 0, 5])
        .width(Length::Fill);

        // let data1 = vec![1.0, 2.0, 3.0, 4.0, 0.0, 8.0, 7.0, 8.0, 9.0, 10.0];
        // let data1 = vec![10.0, 20.0, 30.0, 40.0, 0.0, 80.0, 70.0, 80.0, 90.0, 100.0];
        let data1 = vec![10.0, 20.0, 30.0, 10.0, 60.0, 65.0];
        let data2 = data1.iter().map(|x| x + 10.0).collect::<Vec<f32>>();
        // let data2 = vec![1.0, 2.0, 3.0, 4.0, 3.0, 3.0, 1.0];
        // let data3 = vec![1.0, 2.0, 3.0, 4.0, 3.0, 4.0, 2.0, 2.5, 2.0, 1.0];
        let audiogram_right = plot(data1.clone(), Shape::Triangle);
        let audio_right_title = text("OREILLE DROITE")
            .size(26)
            .horizontal_alignment(Horizontal::Center);

        let audio_right = column![audio_right_title, audiogram_right]
            .width(Length::FillPortion(2))
            .align_items(Alignment::Center);

        let audiorgam_left = plot(data2.clone(), Shape::X);
        let audio_left_title = text("OREILLE GAUCHE")
            .size(26)
            .horizontal_alignment(Horizontal::Center);
        let audio_left = column![audio_left_title, audiorgam_left]
            .width(Length::FillPortion(2))
            .align_items(Alignment::Center);

        let legend = container(draw_legend())
            // .style(theme::Container::Custom(Box::new(LegendCustomStyle)))
            .height(Length::FillPortion(2))
            .width(Length::Fill);

        // let legend_title = text(" ").size(13).horizontal_alignment(Horizontal::Center);

        let val_and_trans = row![
            validity_content
                .width(Length::Shrink)
                .height(Length::Shrink),
            transductor_content
                .width(Length::Shrink)
                .height(Length::Shrink),
        ]
        .spacing(40)
        .align_items(Alignment::Start);
        let mid_col = column![vertical_space(20.0), val_and_trans, legend,]
            .align_items(Alignment::Center)
            .height(Length::Shrink);

        let mid_audiograph = container(mid_col).width(Length::FillPortion(1));

        let audiogram_title = column![text("AUDIOMÉTRIE TONALE")
            .size(30)
            .horizontal_alignment(Horizontal::Center)]
        .width(Length::Fill)
        .align_items(Alignment::Center);

        let audiogram_title_container =
            container(audiogram_title)
                .width(Length::Fill)
                .style(theme::Container::Custom(Box::new(
                    TitleContainerCustomStyle,
                )));

        let audiograms = column![
            audiogram_title_container,
            row![audio_right, mid_audiograph, audio_left]
        ];

        let audiogram_content = column![header, audiograms].height(Length::FillPortion(1));

        let checkbex = checkboxes::CheckBex::default();
        let checkbex_element = checkbex.view();
        //
        // let content = column![checkbex_element];

        let content = column![audiogram_content];

        container(content.align_items(Alignment::Center))
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
    }

    fn theme(&self) -> Theme {
        Theme::Dark
    }
}

struct TitleContainerCustomStyle;

impl container::StyleSheet for TitleContainerCustomStyle {
    type Style = Theme;

    fn appearance(&self, _style: &Self::Style) -> Appearance {
        container::Appearance {
            text_color: Some(Color::from_rgb(0.05, 0.05, 0.02)),
            background: Some(Color::from_rgb(0.7, 0.7, 0.7).into()),
            border_radius: 25.0,
            border_width: 0.0,
            border_color: Color::from_rgb(0.5, 0.25, 0.25),
        }
    }
}
struct LegendCustomStyle;
impl container::StyleSheet for LegendCustomStyle {
    type Style = Theme;

    fn appearance(&self, _style: &Self::Style) -> Appearance {
        container::Appearance {
            text_color: Some(Color::from_rgb(0.05, 0.05, 0.02)),
            background: Some(Color::from_rgb(0.3, 0.3, 0.3).into()),
            border_radius: 25.0,
            border_width: 0.0,
            border_color: Color::from_rgb(0.5, 0.25, 0.25),
        }
    }
}
