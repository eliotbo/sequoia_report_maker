//! by John Conway. It leverages a `Canvas` together with other widgets.
// mod checkboxes;
mod config;
// mod grid;
mod legend;
mod plot;
mod preset;
mod tonal_tables;

use tonal_tables::{
    get_message_fn, make_tonal_tables, seuils_vocaux_tables, TableContainerCustomStyle,
    TableTitleCustomStyle, TonalTable,
};
// use checkboxes::{Transductor, Validity};
// use grid::Grid;
use config::{
    DEFAULT_TEXT_INPUT_CONTENT_SIZE, DEFAULT_TEXT_SIZE, LEGEND_BOTTOM_SPACE, LEGEND_WIDTH,
    RADIO_SPACING, RADIO_TITLE_SIZE, SECTION_SEPARATOR_SPACE, SECTION_TITLE_BG_COLOR,
    SECTION_TITLE_TEXT_COLOR, TABLE_MISC_SIZE,
};
use legend::{draw_legend, Legend};
use plot::{plot, EarSide, Plot, Shape};
use preset::Preset;

use iced::alignment::{Horizontal, Vertical};
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
pub enum Lang {
    French,
    English,
    None,
}
impl Default for Lang {
    fn default() -> Self {
        Lang::None
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

pub fn main() -> iced::Result {
    env_logger::builder().format_timestamp(None).init();

    AudioRox::run(Settings {
        antialiasing: true,
        window: window::Settings {
            position: window::Position::Centered,
            size: (1080, 950),
            ..window::Settings::default()
        },
        ..Settings::default()
    })
}

#[derive(Default)]
struct VocalTable {
    sdp: String,
    srp: String,
    misc: String,
}

#[derive(Default)]
pub struct AudioRox {
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
    pub tonal_table_left: TonalTable,
    pub tonal_table_right: TonalTable,
    pub tonal_table_free: TonalTable,

    vocal_table_left: VocalTable,
    vocal_table_right: VocalTable,
    vocal_table_free: VocalTable,
    vocal_table_binaural: VocalTable,

    vocal_misc_right: String,
    vocal_misc_left: String,
    vocal_misc_bin: String,

    anterior_threshold_date: String,
    audiometer_name: String,
    adequate_rest_period: bool,

    vocal_lang: Lang,
}

#[derive(Debug, Clone)]
pub enum Message {
    AdequateRestPeriodChanged(bool),
    AnteriorThresholdDateChanged(String),
    AudiometerNameChanged(String),
    DefaultChecked(bool),
    CustomChecked(bool),
    ValidityChanged(Validity),
    MethodChanged(Method),
    TransductorChanged(Transductor),

    MSPRightChanged(String),
    MSP4RightChanged(String),
    FLCHRightChanged(String),

    MSPLeftChanged(String),
    MSP4LeftChanged(String),
    FLCHLeftChanged(String),

    SDPRightChanged(String),
    SRPRightChanged(String),
    MiscRightChanged(String),

    SDPLeftChanged(String),
    SRPLeftChanged(String),
    MiscLeftChanged(String),

    SDPFreeChanged(String),
    SRPFreeChanged(String),
    MiscBinChanged(String),

    // VocalMiscRightChanged(String),
    // VocalMiscLeftChanged(String),
    // VocalMiscBinChanged(String),
    VocalLangChanged(Lang),

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
            Message::AdequateRestPeriodChanged(value) => self.adequate_rest_period = value,
            Message::AnteriorThresholdDateChanged(value) => self.anterior_threshold_date = value,
            Message::AudiometerNameChanged(value) => self.audiometer_name = value,
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

            Message::MSPRightChanged(new_msp) => self.tonal_table_right.msp = new_msp,
            Message::MSP4RightChanged(new_msp4) => self.tonal_table_right.msp4 = new_msp4,
            Message::FLCHRightChanged(new_fletcher) => {
                self.tonal_table_right.fletcher = new_fletcher
            }

            Message::MSPLeftChanged(new_msp) => self.tonal_table_left.msp = new_msp,
            Message::MSP4LeftChanged(new_msp4) => self.tonal_table_left.msp4 = new_msp4,
            Message::FLCHLeftChanged(new_fletcher) => self.tonal_table_left.fletcher = new_fletcher,

            Message::SDPRightChanged(new_sdp) => self.vocal_table_right.sdp = new_sdp,
            Message::SRPRightChanged(new_srp) => self.vocal_table_right.srp = new_srp,
            Message::SDPLeftChanged(new_sdp) => self.vocal_table_left.sdp = new_sdp,
            Message::SRPLeftChanged(new_srp) => self.vocal_table_left.srp = new_srp,

            Message::SDPFreeChanged(new_sdp) => self.vocal_table_binaural.sdp = new_sdp,
            Message::SRPFreeChanged(new_srp) => self.vocal_table_binaural.srp = new_srp,

            Message::MiscRightChanged(new_pa) => self.vocal_table_right.misc = new_pa,
            Message::MiscLeftChanged(new_pa) => self.vocal_table_left.misc = new_pa,
            Message::MiscBinChanged(new_pa) => self.vocal_table_binaural.misc = new_pa,

            // Message::MiscRightChanged(new_misc) => self.vocal_misc_right = new_misc,
            // Message::MiscLeftChanged(new_misc) => self.vocal_misc_left = new_misc,
            // Message::MiscBinChanged(new_misc) => self.vocal_misc_bin = new_misc,
            Message::VocalLangChanged(new_lang) => self.vocal_lang = new_lang,

            Message::None => {}
        }

        Command::none()
    }

    fn subscription(&self) -> Subscription<Message> {
        Subscription::none()
    }

    fn view(&self) -> Element<Message> {
        //
        let r_size = 16;
        let t_size = 16;
        // let RADIO_TITLE_SIZE = 18;

        ///////////////////////////////////////////// VALIDITE /////////////////////////////////////////////
        let validity = self.validity;

        let good_validity = radio(
            "Bonne",
            Validity::Good,
            Some(validity),
            Message::ValidityChanged,
        )
        .spacing(RADIO_SPACING)
        .size(r_size)
        .text_size(t_size);

        let medium_validity = radio(
            "Moyenne",
            Validity::Medium,
            Some(validity),
            Message::ValidityChanged,
        )
        .spacing(RADIO_SPACING)
        .size(r_size)
        .text_size(t_size);

        let null_validity = radio(
            "Nulle",
            Validity::Poor,
            Some(validity),
            Message::ValidityChanged,
        )
        .spacing(RADIO_SPACING)
        .size(r_size)
        .text_size(t_size);

        let validity_section = column![good_validity, medium_validity, null_validity]
            .spacing(6)
            .width(Length::Shrink);

        let validity_title = text("VALIDITÉ")
            .size(RADIO_TITLE_SIZE)
            .width(Length::Shrink);

        let validity_content = column![validity_title, validity_section,].spacing(3);
        ///////////////////////////////////////////// VALIDITE /////////////////////////////////////////////

        ///////////////////////////////////////////// METHOD /////////////////////////////////////////////
        // let method = self.method;

        // let visual_method = radio(
        //     "Visuelle",
        //     Method::Visual,
        //     Some(method),
        //     Message::MethodChanged,
        // )
        // .size(r_size)
        // .spacing(RADIO_SPACING)
        // .text_size(t_size);

        // let play_method = radio("Jeu", Method::Play, Some(method), Message::MethodChanged)
        //     .size(r_size)
        //     .text_size(t_size)
        //     .spacing(RADIO_SPACING);

        // let method_section = column![visual_method, play_method]
        //     .spacing(6)
        //     .width(Length::Shrink);

        // let method_title = text("MÉTHODE").size(RADIO_TITLE_SIZE).width(Length::Shrink);

        // let method_radio = column![method_title, method_section].spacing(3);

        // let method_radio_and_audiotype =
        //     row![method_radio, horizontal_space(45.0), audriometer_type];

        ///////////////////////////////////////////// METHOD /////////////////////////////////////////////

        ///////////////////////////////////////////// standard /////////////////////////////////////////////
        // text_input for audiometer name
        let audiometer_type = row![
            text("Audiomètre: ")
                .size(16)
                .horizontal_alignment(Horizontal::Left),
            text_input(
                "AD629",
                &self.audiometer_name,
                Message::AudiometerNameChanged
            )
            .size(DEFAULT_TEXT_INPUT_CONTENT_SIZE)
            .width(Length::Fill)
        ]
        .align_items(Alignment::Center);

        let anterior_thresholds_date = row![
            text("Date des seuils antérieurs : ")
                .size(16)
                .horizontal_alignment(Horizontal::Left),
            text_input(
                "",
                &self.anterior_threshold_date,
                Message::AnteriorThresholdDateChanged
            )
            .size(DEFAULT_TEXT_INPUT_CONTENT_SIZE)
            .width(Length::Fill)
        ]
        .align_items(Alignment::Center);

        // a checkbox for adequate rest period
        let adequate_rest_period = checkbox(
            "Durée de repos sonore adéquate",
            self.adequate_rest_period,
            Message::AdequateRestPeriodChanged,
        )
        .spacing(RADIO_SPACING)
        .text_size(16);

        // self.audiometer_name;
        let standard = column![
            vertical_space(5.),
            // .vertical_alignment(Vertical::Center),
            audiometer_type,
            vertical_space(5.),
            // text("Date des seuils antérieurs : _______").size(DEFAULT_TEXT_SIZE),
            anterior_thresholds_date,
            vertical_space(5.),
            adequate_rest_period,
            // text("   Période de repos sonore adéquate").size(DEFAULT_TEXT_SIZE),
        ];
        let standard_container = container(column![standard,].align_items(Alignment::Start));
        ///////////////////////////////////////////// standard /////////////////////////////////////////////

        ///////////////////////////////////////////// TRANSDUCTOR /////////////////////////////////////////////
        let transductor = self.transductor;

        let intra = radio(
            "Intra",
            Transductor::Intra,
            Some(transductor),
            Message::TransductorChanged,
        )
        .spacing(RADIO_SPACING)
        .size(r_size)
        .text_size(t_size);

        let supra = radio(
            "Supra",
            Transductor::Supra,
            Some(transductor),
            Message::TransductorChanged,
        )
        .spacing(RADIO_SPACING)
        .size(r_size)
        .text_size(t_size);

        let free = radio(
            "Haut-parleurs",
            Transductor::Free,
            Some(transductor),
            Message::TransductorChanged,
        )
        .spacing(RADIO_SPACING)
        .size(r_size)
        .text_size(t_size);

        let transductor_section = column![intra, supra, free].spacing(6).width(Length::Shrink);

        let transductor_title = text("SOURCE").size(RADIO_TITLE_SIZE).width(Length::Shrink);

        let transductor_content = column![transductor_title, transductor_section,].spacing(3);
        ///////////////////////////////////////////// TRANSDUCTOR /////////////////////////////////////////////

        let (tonal_table_right, tonal_table_left) = make_tonal_tables(&self);
        let (vocal_table_right, vocal_table_left, vocal_table_bin) = seuils_vocaux_tables(&self);

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
            .width(Length::FillPortion(1)),
            // .align(Alignment::Start),
            horizontal_space(1),
            text("ÉVALUATION\nAUDIOLOGIQUE")
                .size(30)
                .horizontal_alignment(Horizontal::Center)
                .width(Length::Fill),
            horizontal_space(1),
            // .align(Alignment::Start),
            column![
                row![text("DATE"), vertical_space(Length::Fixed(text_vspace))],
                Rule::horizontal(1),
                vertical_space(Length::Fixed(text_vspace)),
                Rule::horizontal(1),
                vertical_space(Length::Fixed(text_vspace)),
                Rule::horizontal(1),
                vertical_space(Length::Fixed(text_vspace)),
            ]
            // .width(Length::Fixed(400.0))
            .width(Length::FillPortion(1))
        ]
        .padding([0, 5, 0, 5])
        .width(Length::Fill);

        // let data1 = vec![1.0, 2.0, 3.0, 4.0, 0.0, 8.0, 7.0, 8.0, 9.0, 10.0];
        // let data1 = vec![10.0, 20.0, 30.0, 40.0, 0.0, 80.0, 70.0, 80.0, 90.0, 100.0];
        let data1 = vec![10.0, 20.0, 30.0, 10.0, 60.0, 65.0];
        let data2 = data1.iter().map(|x| x + 10.0).collect::<Vec<f32>>();
        // let data2 = vec![1.0, 2.0, 3.0, 4.0, 3.0, 3.0, 1.0];
        // let data3 = vec![1.0, 2.0, 3.0, 4.0, 3.0, 4.0, 2.0, 2.5, 2.0, 1.0];
        let audiogram_right =
            container(plot(data1.clone(), Shape::Less, EarSide::Right)).align_x(Horizontal::Center);
        // .style(theme::Container::Custom(Box::new(
        //     TableContainerCustomStyle,
        // )));

        let audio_right_title = text("OREILLE DROITE")
            .size(26)
            .horizontal_alignment(Horizontal::Center);

        let audio_right = container(
            column![
                audio_right_title,
                audiogram_right,
                vertical_space(7.0),
                row![
                    horizontal_space(10.0),
                    tonal_table_right,
                    horizontal_space(10.0),
                ]
            ]
            // .width(Length::FillPortion(2))
            .align_items(Alignment::Center),
        )
        // .style(theme::Container::Custom(Box::new(
        //     TableContainerCustomStyle,
        // )))
        .center_x();

        let audiorgam_left = plot(data2.clone(), Shape::X, EarSide::Left);
        let audio_left_title = text("OREILLE GAUCHE")
            .size(26)
            .horizontal_alignment(Horizontal::Center);

        let audio_left = column![
            audio_left_title,
            audiorgam_left,
            vertical_space(7.0),
            row![
                // horizontal_space(10.0),
                tonal_table_left,
                horizontal_space(10.0),
            ]
        ]
        // .width(Length::FillPortion(2))
        .align_items(Alignment::Center);

        let legend = container(draw_legend())
            // .style(theme::Container::Custom(Box::new(LegendCustomStyle)))
            // .height(Length::Fill)
            .width(Length::Shrink);

        // let legend_title = text(" ").size(13).horizontal_alignment(Horizontal::Center);

        let val_and_trans = row![
            horizontal_space(10.0),
            validity_content
                .width(Length::Shrink)
                .height(Length::Shrink),
            horizontal_space(10.0),
            transductor_content
                .width(Length::Shrink)
                .height(Length::Shrink),
            horizontal_space(2.0),
            // method_radio.width(Length::Shrink).height(Length::Shrink),
        ]
        .spacing(5)
        .align_items(Alignment::Start);

        let mid_col = column![
            // vertical_space(2.0),
            legend,
            vertical_space(LEGEND_BOTTOM_SPACE),
            val_and_trans.width(Length::Fixed(250.)),
            vertical_space(15.0),
            text("Normes en vigueur : ANSI série 3 ").size(DEFAULT_TEXT_SIZE),
            vertical_space(15.0),
            standard_container
        ]
        .align_items(Alignment::Center)
        .height(Length::Shrink)
        .width(Length::Fixed(LEGEND_WIDTH));

        let mid_audiograph = container(mid_col).width(Length::Shrink);

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

        // ///////////////////////////////////// TONAL TABLES /////////////////////////////////////
        // let tonal_tables = row![
        //     horizontal_space(10),
        //     tonal_table_right,
        //     horizontal_space(10),
        //     tonal_table_left,
        //     horizontal_space(10),
        // ]
        // .width(Length::Shrink)
        // .align_items(Alignment::Center);
        // ///////////////////////////////////// TONAL TABLES /////////////////////////////////////

        let audiograms = column![
            tonal_audiogram_title_container,
            // row![audio_right, mid_audiograph, horizontal_space(6), audio_left] // .align_items(Alignment::Center)
            row![
                audio_right.width(Length::FillPortion(1)),
                mid_audiograph.width(Length::Shrink),
                horizontal_space(6),
                audio_left.width(Length::FillPortion(1)) // .align(Alignment::Center)
            ], // .align_items(Alignment::Center)
               // vertical_space(5.0),
               // tonal_tables
        ];

        let tonal_audiogram_content = column![header, audiograms];

        // let checkbex = checkboxes::CheckBex::default();
        // let checkbex_element = checkbex.view();
        //
        // let content = column![checkbex_element];

        ///////////////////////////////////// VOCAL TABLES /////////////////////////////////////
        ///
        // a column of two checkboxes for "FR" and "ANG"
        let vocal_lang = column![
            text("LANGUE").size(RADIO_TITLE_SIZE),
            radio(
                "Fr.",
                Lang::French,
                Some(self.vocal_lang),
                Message::VocalLangChanged
            )
            .spacing(RADIO_SPACING)
            .size(r_size)
            .text_size(t_size),
            radio(
                "Ang.",
                Lang::English,
                Some(self.vocal_lang),
                Message::VocalLangChanged
            )
            .spacing(RADIO_SPACING)
            .size(r_size)
            .text_size(t_size),
        ]
        .spacing(6);

        let voice = column![
            text("VOIX").size(RADIO_TITLE_SIZE),
            radio(
                "Nue",
                Lang::French,
                Some(self.vocal_lang),
                Message::VocalLangChanged
            )
            .spacing(RADIO_SPACING)
            .size(r_size)
            .text_size(t_size),
            radio(
                "Enregis.",
                Lang::English,
                Some(self.vocal_lang),
                Message::VocalLangChanged
            )
            .spacing(RADIO_SPACING)
            .size(r_size)
            .text_size(t_size),
        ]
        .spacing(6);

        let open_text_input_right =
            text_input("", &self.vocal_misc_right, Message::MiscRightChanged)
                .size(TABLE_MISC_SIZE)
                .width(Length::Fill);

        let open_text_input_left = text_input("", &self.vocal_misc_left, Message::MiscLeftChanged)
            .size(TABLE_MISC_SIZE)
            .width(Length::Fill);

        let open_text_input_bin = text_input("", &self.vocal_misc_bin, Message::MiscBinChanged)
            .size(TABLE_MISC_SIZE)
            .width(Length::Fill);

        let vocal_tables = row![
            horizontal_space(10),
            column![vocal_table_right, vertical_space(10), open_text_input_right]
                .width(Length::FillPortion(2)),
            horizontal_space(10),
            vocal_lang,
            horizontal_space(10),
            column![vocal_table_bin, vertical_space(10), open_text_input_bin]
                .width(Length::FillPortion(2)),
            horizontal_space(10),
            voice,
            horizontal_space(10),
            column![vocal_table_left, vertical_space(10), open_text_input_left]
                .width(Length::FillPortion(2)),
            horizontal_space(10),
        ]
        .width(Length::Shrink)
        .align_items(Alignment::Center);

        // let vocal_tables = row![
        //     vocal_table_left,
        //     horizontal_space(Length::Fixed(30.0)),
        //     vocal_table_right
        // ]
        // .align_items(Alignment::Center);

        ///////////////////////////////////// VOCAL TABLES /////////////////////////////////////
        // let vocal_misc = row![
        //     horizontal_space(10),
        //     text_input("", &self.vocal_misc_right, Message::VocalMiscRightChanged)
        //         .size(TABLE_MISC_SIZE)
        //         .width(Length::Fill),
        //     horizontal_space(10),
        // ];

        let vocal_audiogram_content = column![
            vocal_audiogram_title_container,
            vertical_space(Length::Fixed(15.0)),
            vocal_tables,
            // vertical_space(Length::Fixed(10.0)),
            // vocal_misc
        ]
        .align_items(Alignment::Center);

        let content = column![
            tonal_audiogram_content,
            vertical_space(SECTION_SEPARATOR_SPACE),
            vocal_audiogram_content
        ];

        container(content.align_items(Alignment::Center))
            .width(Length::Fill)
            // .height(Length::Fill)
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
            text_color: Some(SECTION_TITLE_TEXT_COLOR),
            background: Some(SECTION_TITLE_BG_COLOR.into()),
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
