//! by John Conway. It leverages a `Canvas` together with other widgets.
// mod checkboxes;
mod config;
// mod grid;
mod legend;
mod plot;
mod preset;

// use checkboxes::{Transductor, Validity};
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
    button, checkbox, column, container, container::Appearance, horizontal_space, pick_list, radio,
    row, slider, text, text_input, vertical_space, Container, Row, Rule,
};
use iced::window;
use iced::{
    Alignment, Application, Color, Command, Element, Length, Point, Rectangle, Settings, Size,
    Subscription,
};
use std::time::{Duration, Instant};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Validity {
    Good,
    Medium,
    Poor,
    None,
}
impl Default for Validity {
    fn default() -> Self {
        Validity::None
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Method {
    Visual,
    Play,
    None,
}
impl Default for Method {
    fn default() -> Self {
        Method::None
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Transductor {
    Intra,
    Supra,
    Free,
    None,
}

impl Default for Transductor {
    fn default() -> Self {
        Transductor::None
    }
}

pub fn get_message_fn(s: &str, is_right: bool) -> impl Fn(String) -> Message {
    match s {
        "MSP" if is_right => Message::MSPRightChanged,
        "SDP" if is_right => Message::SDPRightChanged,
        "MSP4" if is_right => Message::MSP4RightChanged,
        "SRP" if is_right => Message::SRPRightChanged,

        "MSP" if !is_right => Message::MSPLeftChanged,
        "SDP" if !is_right => Message::SDPLeftChanged,
        "MSP4" if !is_right => Message::MSP4LeftChanged,
        "SRP" if !is_right => Message::SRPLeftChanged,
        _ => panic!("Not a valid Table message: {}", s),
    }
}
pub fn main() -> iced::Result {
    env_logger::builder().format_timestamp(None).init();

    AudioRox::run(Settings {
        antialiasing: true,
        window: window::Settings {
            position: window::Position::Centered,
            size: (1080, 750),
            ..window::Settings::default()
        },
        ..Settings::default()
    })
}

#[derive(Default)]
struct Table {
    msp: String,
    sdp: String,
    msp4: String,
    srp: String,
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
    method: Method,
    transductor: Transductor,
    table_left: Table,
    table_right: Table,
}

#[derive(Debug, Clone)]
pub enum Message {
    DefaultChecked(bool),
    CustomChecked(bool),
    ValidityChanged(Validity),
    MethodChanged(Method),
    TransductorChanged(Transductor),
    MSPRightChanged(String),
    SDPRightChanged(String),
    MSP4RightChanged(String),
    SRPRightChanged(String),

    MSPLeftChanged(String),
    SDPLeftChanged(String),
    MSP4LeftChanged(String),
    SRPLeftChanged(String),
    None,
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
            Message::MethodChanged(new_method) => self.method = new_method,
            Message::MSPRightChanged(new_msp) => self.table_right.msp = new_msp,
            Message::SDPRightChanged(new_sdp) => self.table_right.sdp = new_sdp,
            Message::MSP4RightChanged(new_msp4) => self.table_right.msp4 = new_msp4,
            Message::SRPRightChanged(new_srp) => self.table_right.srp = new_srp,

            Message::MSPLeftChanged(new_msp) => self.table_left.msp = new_msp,
            Message::SDPLeftChanged(new_sdp) => self.table_left.sdp = new_sdp,
            Message::MSP4LeftChanged(new_msp4) => self.table_left.msp4 = new_msp4,
            Message::SRPLeftChanged(new_srp) => self.table_left.srp = new_srp,

            Message::None => {}
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

        ///////////////////////////////////////////// METHOD /////////////////////////////////////////////
        let r_size = 16;
        let t_size = 16;
        let method = self.method;

        let visual_method = radio(
            "Visuelle",
            Method::Visual,
            Some(method),
            Message::MethodChanged,
        )
        .size(r_size)
        .text_size(t_size);

        let play_method = radio("Jeu", Method::Play, Some(method), Message::MethodChanged)
            .size(r_size)
            .text_size(t_size);

        let method_section = column![visual_method, play_method]
            .spacing(6)
            .width(Length::Shrink);

        let method_title = text("Méthode").size(20).width(Length::Shrink);

        let audriometer_type = column![
            text("Audiomètre:\nAD629")
                .size(16)
                .horizontal_alignment(Horizontal::Left),
            vertical_space(10),
            text("Normes ANSI\nen vigueur")
                .size(16)
                .horizontal_alignment(Horizontal::Left),
        ]
        .align_items(Alignment::Start);

        let method_content = row![
            horizontal_space(5),
            audriometer_type,
            horizontal_space(45),
            column![method_title, vertical_space(3.0), method_section,],
        ];
        ///////////////////////////////////////////// METHOD /////////////////////////////////////////////

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
        ///
        ///////////////////////////////////////////// TABLE /////////////////////////////////////////////
        let input_table_columns_left = [
            ("MSP", &self.table_left.msp),
            ("SDP", &self.table_left.sdp),
            ("MSP4", &self.table_left.msp4),
            ("SRP", &self.table_left.srp),
        ];

        let input_table_columns_right = [
            ("MSP", &self.table_right.msp),
            ("SDP", &self.table_right.sdp),
            ("MSP4", &self.table_right.msp4),
            ("SRP", &self.table_right.srp),
            // ("N Confor\nparole", &self),
        ];
        // let msp = column!["MSP", ""];
        // let sdp = column!["SDP", ""];
        // let msp4 = column!["MSP4", ""];
        // let srp = column!["SRP", ""];
        let mut table_left = Row::new();
        let col_width = 60.0;

        for (s, variable) in input_table_columns_left.iter() {
            let is_right = false;
            let message_fn = get_message_fn(s, is_right);
            let entry = column![
                container(text(*s))
                    .style(theme::Container::Box)
                    .padding(5)
                    .width(Length::Fixed(col_width))
                    .align_x(Horizontal::Center),
                text_input("", &variable, message_fn)
                    .padding(3)
                    .size(24)
                    .width(Length::Fixed(col_width)),
                // container("sO").style(theme::Container::Box).padding(5)
            ]
            .align_items(Alignment::Center);

            table_left = table_left.push(entry);
            table_left = table_left.push(horizontal_space(Length::Fixed(3.0)));
            // table = table.push(Rule::vertical(10));
            // table_left = table_left.height(Length::Shrink);
        }
        let table_left = table_left.height(Length::Shrink);

        let mut table_right = Row::new();

        for (s, variable) in input_table_columns_right.iter() {
            let is_right = true;
            let message_fn = get_message_fn(s, is_right);
            let entry = column![
                container(text(*s))
                    .style(theme::Container::Box)
                    .padding(5)
                    .width(Length::Fixed(col_width))
                    .align_x(Horizontal::Center),
                text_input("", &variable, message_fn)
                    .padding(3)
                    .size(24)
                    .width(Length::Fixed(col_width)),
                // container("sO").style(theme::Container::Box).padding(5)
            ]
            .align_items(Alignment::Center);

            table_right = table_right.push(entry);
            table_right = table_right.push(horizontal_space(Length::Fixed(3.0)));
            // table = table.push(Rule::vertical(10));
            // table_left = table_left.height(Length::Shrink);
        }
        let table_right = table_right.height(Length::Shrink);

        // let delimited_table = column![table, Rule::horizontal(1)];

        // let table = column!["TITLE", row![msp,sdp,msp4, srp]];

        ///////////////////////////////////////////// TABLE /////////////////////////////////////////////

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
        let audiogram_right = plot(data1.clone(), Shape::Less);
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
            .height(Length::Fill)
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

        let mid_col = column![
            vertical_space(20.0),
            val_and_trans,
            vertical_space(15.0),
            method_content.width(Length::Shrink).height(Length::Shrink),
            // vertical_space(-100.0),
            // audriometer_type,
            legend,
        ]
        .align_items(Alignment::Center)
        .height(Length::Shrink);

        let mid_audiograph = container(mid_col).width(Length::FillPortion(1));

        let tonal_audiogram_title = column![text("AUDIOMÉTRIE TONALE")
            .size(30)
            .horizontal_alignment(Horizontal::Center)]
        .width(Length::Fill)
        .align_items(Alignment::Center);

        let tonal_audiogram_title_container = container(tonal_audiogram_title)
            .width(Length::Fill)
            .style(theme::Container::Custom(Box::new(
                TitleContainerCustomStyle,
            )));

        let vocal_audiogram_title = column![text("AUDIOMÉTRIE VOCALE")
            .size(30)
            .horizontal_alignment(Horizontal::Center)]
        .width(Length::Fill)
        .align_items(Alignment::Center);

        let vocal_audiogram_title_container = container(vocal_audiogram_title)
            .width(Length::Fill)
            .style(theme::Container::Custom(Box::new(
                TitleContainerCustomStyle,
            )));

        let audiograms = column![
            vocal_audiogram_title_container,
            row![audio_right, mid_audiograph, audio_left]
        ];

        let tonal_audiogram_content = column![header, audiograms].height(Length::FillPortion(1));

        // let checkbex = checkboxes::CheckBex::default();
        // let checkbex_element = checkbex.view();
        //
        // let content = column![checkbex_element];

        let tables = row![
            table_left.align_items(Alignment::Center),
            horizontal_space(Length::Fixed(30.0)),
            table_right.align_items(Alignment::Center)
        ]
        .align_items(Alignment::Center);

        let vocal_audiogram_content = column![
            tonal_audiogram_title_container,
            vertical_space(Length::Fixed(15.0)),
            tables
        ]
        .align_items(Alignment::Center);

        let content = column![tonal_audiogram_content, vocal_audiogram_content];

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
