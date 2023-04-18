//! by John Conway. It leverages a `Canvas` together with other widgets.
// mod checkboxes;
mod config;
// mod grid;
mod immi_plot;
mod immitance;
mod legend;
mod partners;
mod plot;
mod preset;
mod tonal_tables;
// mod modal;
// mod partners::modal::*;

use immitance::*;
use partners::modal::Modal;
use partners::{get_all_partners, get_all_succursales, modal, Partner};

use tonal_tables::{
    get_message_fn, identification_language, make_tonal_tables, seuils_vocaux_tables, stap, tympa,
    TableContainerCustomStyle, TableTitleCustomStyle, TonalTable,
};
// use checkboxes::{Transductor, Validity};
// use grid::Grid;
use config::{
    CustomButtonStyle, LegendCustomStyle, TitleContainerCustomStyle,
    DEFAULT_TEXT_INPUT_CONTENT_SIZE, DEFAULT_TEXT_SIZE, LEGEND_BOTTOM_SPACE, LEGEND_WIDTH,
    RADIO_SPACING, RADIO_TITLE_SIZE, SECTION_SEPARATOR_SPACE, SECTION_TITLE_BG_COLOR,
    SECTION_TITLE_TEXT_COLOR, SPACE_BELOW_SECTION_TITLE, TABLE_MISC_SIZE, WINDOW_HEIGHT,
    WINDOW_WIDTH,
};
use immi_plot::im_plot;
use legend::{draw_legend, Legend};
use plot::{plot, EarSide, Plot, Shape};
use preset::Preset;

use chrono;
use chrono::Datelike;

use iced::alignment::{Horizontal, Vertical};
use iced::executor;
use iced::theme::{self, Theme};
use iced::time;
use iced::widget::canvas;
use iced::widget::canvas::event::{self, Event};
use iced::widget::canvas::path::Builder;
use iced::widget::canvas::{Cache, Canvas, Cursor, Frame, Geometry, Path, Text};
use iced::widget::{
    self, button, checkbox, column, container, container::Appearance, horizontal_space, pick_list,
    radio, row, slider, text, text_input, vertical_space, Container, Row, Rule,
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
}
impl Default for Lang {
    fn default() -> Self {
        Lang::French
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
            size: (WINDOW_WIDTH, WINDOW_HEIGHT),
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

#[derive(Default, Debug, Clone)]
pub struct IdLang {
    pub result1: String,
    pub level1: String,
    pub result2: String,
    pub level2: String,
}

#[derive(Default, Debug, Clone)]
pub struct Tympa {
    pub volume: String,
    pub pressure: String,
    pub compliance: String,
}

#[derive(Default, Debug, Clone)]
pub struct KHzList {
    pub khz_500: String,
    pub khz_1000: String,
    pub khz_2000: String,
    pub khz_4000: String,
}
#[derive(Default, Debug, Clone)]
pub struct Stap {
    pub ipsi: KHzList,
    pub control: KHzList,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IsRecorded {
    Yes,
    No,
}

impl Default for IsRecorded {
    fn default() -> Self {
        IsRecorded::No
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Succursale {
    Montmagny,
    Levy,
    None,
}

impl Default for Succursale {
    fn default() -> Self {
        Succursale::None
    }
}

#[derive(Default)]
pub struct AudioRox {
    show_partner_choices: bool,
    is_playing: bool,
    queued_ticks: usize,
    speed: usize,
    next_speed: Option<usize>,
    version: usize,

    partner: Partner,

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

    id_lang_left: IdLang,
    id_lang_right: IdLang,
    id_lang_bin: IdLang,

    tympa_left: Tympa,
    tympa_right: Tympa,

    stap_left: Stap,
    stap_right: Stap,

    vocal_lang: Lang,
    is_recorded: IsRecorded,
}

impl AudioRox {
    fn hide_partner_choices(&mut self) {
        self.show_partner_choices = false;
        // self.email.clear();
        // self.password.clear();
    }
}

#[derive(Debug, Clone)]
pub enum Message {
    ShowParnerChoices,
    HidePartnerChoices,
    PartnerChanged(Partner),
    AdequateRestPeriodChanged(bool),
    AnteriorThresholdDateChanged(String),
    AudiometerNameChanged(String),
    DefaultChecked(bool),
    CustomChecked(bool),
    ValidityChanged(Validity),
    MethodChanged(Method),
    TransductorChanged(Transductor),

    // SuccursaleChanged(Succursale),
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
    IsRecordedChanged(IsRecorded),

    IdLanRes1LeftChanged(String),
    IdLanRes2LeftChanged(String),
    IdLanLev1LeftChanged(String),
    IdLanLev2LeftChanged(String),

    IdLanRes1RightChanged(String),
    IdLanRes2RightChanged(String),
    IdLanLev1RightChanged(String),
    IdLanLev2RightChanged(String),

    IdLanRes1BinChanged(String),
    IdLanRes2BinChanged(String),
    IdLanLev1BinChanged(String),
    IdLanLev2BinChanged(String),

    TympaVolumeLeftChanged(String),
    TympaPressureLeftChanged(String),
    TympaComplianceLeftChanged(String),

    TympaVolumeRightChanged(String),
    TympaPressureRightChanged(String),
    TympaComplianceRightChanged(String),

    StapIpsi500LeftChanged(String),
    StapIpsi1000LeftChanged(String),
    StapIpsi2000LeftChanged(String),
    StapIpsi4000LeftChanged(String),

    StapControl500LeftChanged(String),
    StapControl1000LeftChanged(String),
    StapControl2000LeftChanged(String),
    StapControl4000LeftChanged(String),

    StapIpsi500RightChanged(String),
    StapIpsi1000RightChanged(String),
    StapIpsi2000RightChanged(String),
    StapIpsi4000RightChanged(String),

    StapControl500RightChanged(String),
    StapControl1000RightChanged(String),
    StapControl2000RightChanged(String),
    StapControl4000RightChanged(String),

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
            Message::PartnerChanged(value) => self.partner = value,

            Message::ShowParnerChoices => {
                self.show_partner_choices = true;
                return widget::focus_next();
            }
            Message::HidePartnerChoices => {
                self.hide_partner_choices();
            }

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
            // Message::SuccursaleChanged(new_succursale) => self.succursale = new_succursale,
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
            Message::IsRecordedChanged(new) => self.is_recorded = new,

            Message::IdLanRes1LeftChanged(new_lang) => self.id_lang_left.result1 = new_lang,
            Message::IdLanRes2LeftChanged(new_lang) => self.id_lang_left.result2 = new_lang,
            Message::IdLanLev1LeftChanged(new_lang) => self.id_lang_left.level1 = new_lang,
            Message::IdLanLev2LeftChanged(new_lang) => self.id_lang_left.level2 = new_lang,

            Message::IdLanRes1RightChanged(new_lang) => self.id_lang_right.result1 = new_lang,
            Message::IdLanRes2RightChanged(new_lang) => self.id_lang_right.result2 = new_lang,
            Message::IdLanLev1RightChanged(new_lang) => self.id_lang_right.level1 = new_lang,
            Message::IdLanLev2RightChanged(new_lang) => self.id_lang_right.level2 = new_lang,

            Message::IdLanRes1BinChanged(new_lang) => self.id_lang_bin.result1 = new_lang,
            Message::IdLanRes2BinChanged(new_lang) => self.id_lang_bin.result2 = new_lang,
            Message::IdLanLev1BinChanged(new_lang) => self.id_lang_bin.level1 = new_lang,
            Message::IdLanLev2BinChanged(new_lang) => self.id_lang_bin.level2 = new_lang,

            Message::TympaVolumeLeftChanged(new_tympa_num) => {
                self.tympa_left.volume = new_tympa_num
            }
            Message::TympaVolumeRightChanged(new_tympa_num) => {
                self.tympa_right.volume = new_tympa_num
            }

            Message::TympaPressureLeftChanged(new_tympa_num) => {
                self.tympa_left.pressure = new_tympa_num
            }
            Message::TympaPressureRightChanged(new_tympa_num) => {
                self.tympa_right.pressure = new_tympa_num
            }

            Message::TympaComplianceLeftChanged(new_tympa_num) => {
                self.tympa_left.compliance = new_tympa_num
            }
            Message::TympaComplianceRightChanged(new_tympa_num) => {
                self.tympa_right.compliance = new_tympa_num
            }

            Message::StapIpsi500LeftChanged(new) => self.stap_left.ipsi.khz_500 = new,
            Message::StapIpsi1000LeftChanged(new) => self.stap_left.ipsi.khz_1000 = new,
            Message::StapIpsi2000LeftChanged(new) => self.stap_left.ipsi.khz_2000 = new,
            Message::StapIpsi4000LeftChanged(new) => self.stap_left.ipsi.khz_4000 = new,

            Message::StapControl500LeftChanged(new) => self.stap_left.control.khz_500 = new,
            Message::StapControl1000LeftChanged(new) => self.stap_left.control.khz_1000 = new,
            Message::StapControl2000LeftChanged(new) => self.stap_left.control.khz_2000 = new,
            Message::StapControl4000LeftChanged(new) => self.stap_left.control.khz_4000 = new,

            Message::StapIpsi500RightChanged(new) => self.stap_right.ipsi.khz_500 = new,
            Message::StapIpsi1000RightChanged(new) => self.stap_right.ipsi.khz_1000 = new,
            Message::StapIpsi2000RightChanged(new) => self.stap_right.ipsi.khz_2000 = new,
            Message::StapIpsi4000RightChanged(new) => self.stap_right.ipsi.khz_4000 = new,

            Message::StapControl500RightChanged(new) => self.stap_right.control.khz_500 = new,
            Message::StapControl1000RightChanged(new) => self.stap_right.control.khz_1000 = new,
            Message::StapControl2000RightChanged(new) => self.stap_right.control.khz_2000 = new,
            Message::StapControl4000RightChanged(new) => self.stap_right.control.khz_4000 = new,

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
                // Message::AudiometerNameChanged
            )
            .on_input(Message::AudiometerNameChanged)
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
                // Message::AnteriorThresholdDateChanged
            )
            .on_input(Message::AnteriorThresholdDateChanged)
            .size(DEFAULT_TEXT_INPUT_CONTENT_SIZE)
            .width(Length::Fill)
        ]
        .align_items(Alignment::Center);

        // a checkbox for adequate rest period
        let adequate_rest_period = checkbox(
            "Durée de repos sonore adéquate (> 16h)",
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

        let transductor_title = text("ÉCOUTEURS")
            .size(RADIO_TITLE_SIZE)
            .width(Length::Shrink);

        let transductor_content = column![transductor_title, transductor_section,].spacing(3);
        ///////////////////////////////////////////// TRANSDUCTOR /////////////////////////////////////////////

        let (tonal_table_right, tonal_table_left) = make_tonal_tables(&self);
        let (vocal_table_right, vocal_table_left, vocal_table_bin) = seuils_vocaux_tables(&self);
        let (id_lang_table_right, id_lang_table_left, id_lang_table_bin) =
            identification_language(&self);
        let (tympa_table_right, tympa_table_left) = tympa(&self);
        let (stap_table_right, stap_table_left) = stap(&self);

        // create a header with two columns of text: on the left and one on the right
        let text_vspace = 20.0;

        // let montmagny = radio(
        //     "83 Bd Taché O, Montmagny, QC G5V 3A6, 418-248-7077",
        //     Succursale::Montmagny,
        //     Some(self.succursale),
        //     Message::SuccursaleChanged,
        // )
        // .size(12)
        // .text_size(14);

        // let levy = radio(
        //     "5500 Bd Guillaume-Couture suite 111, Lévis, QC G6V 4Z2, 418-837-3626",
        //     Succursale::Levy,
        //     Some(self.succursale),
        //     Message::SuccursaleChanged,
        // )
        // .size(12)
        // .text_size(14);

        let current_date = chrono::Utc::now().date_naive();
        // println!("{}", current_date.year());
        let year = current_date.year();
        let day = current_date.day();
        let month = current_date.month();

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
                row![
                    text(format!("Date de l'évaluation:")).size(14),
                    // text(format!("Date de l'évaluation: {day}/{month}/{year}")).size(14),
                    // vertical_space(Length::Fixed(text_vspace)),
                    Rule::horizontal(text_vspace * 1.3),
                ],
                // modal
                // Rule::horizontal(1),
                button("Lieu de l'évaluation: ")
                    .on_press(Message::ShowParnerChoices)
                    .style(theme::Button::Custom(Box::new(CustomButtonStyle))),
                // text("Lieu de l'évaluation: Clinique de l'Audition Bois & Associés audioprothésistes").size(14),
                get_all_succursales(&self.partner),
                // montmagny,
                // levy,
                vertical_space(Length::Fixed(2.)),
                // text("Clinique de l'Audition Bois & Associés audioprothésistes,\n83 Bd Taché O, Montmagny, QC G5V 3A6").size(15),
                // vertical_space(Length::Fixed(text_vspace)),
                // Rule::horizontal(1),
                // vertical_space(Length::Fixed(text_vspace)),
                // Rule::horizontal(1),
                // vertical_space(Length::Fixed(text_vspace)),
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

        let immit_graph = container(im_plot()).align_x(Horizontal::Center);
        // .style(theme::Container::Custom(Box::new(
        //     TableContainerCustomStyle,
        // )));

        // let audio_right_title = text("OREILLE DROITE")
        //     .size(26)
        //     .horizontal_alignment(Horizontal::Center);

        let audio_right = container(
            column![
                // audio_right_title,
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
        // let audio_left_title = text("OREILLE GAUCHE")
        //     .size(26)
        //     .horizontal_alignment(Horizontal::Center);

        let audio_left = column![
            // audio_left_title,
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
            horizontal_space(3.0),
            validity_content
                .width(Length::Shrink)
                .height(Length::Shrink),
            horizontal_space(6.0),
            transductor_content
                .width(Length::Shrink)
                .height(Length::Shrink),
            horizontal_space(2.0),
            // method_radio.width(Length::Shrink).height(Length::Shrink),
        ]
        .spacing(5)
        .align_items(Alignment::Start);

        let mid_col = column![
            vertical_space(5.0),
            legend,
            vertical_space(LEGEND_BOTTOM_SPACE),
            container(column![
                val_and_trans.width(Length::Fixed(250.)),
                vertical_space(6.0),
                text("Normes en vigueur : ANSI série 3 ").size(DEFAULT_TEXT_SIZE),
                vertical_space(6.0),
                standard_container
            ])
            .padding(5.0)
            .style(theme::Container::Custom(Box::new(LegendCustomStyle,))) // .style(LegendCustomStyle),
        ]
        .align_items(Alignment::Center)
        .height(Length::Shrink)
        .width(Length::Fixed(LEGEND_WIDTH));

        let mid_audiograph = container(mid_col).width(Length::Shrink);

        let tonal_audiogram_title = column![row![
            container(
                text("OREILLE DROITE")
                    .size(22)
                    .horizontal_alignment(Horizontal::Center) // .vertical_alignment(Vertical::Bottom)
            )
            .width(Length::FillPortion(1))
            .height(Length::Fill)
            .align_x(Horizontal::Center)
            .align_y(Vertical::Center),
            container(
                text("AUDIOMÉTRIE TONALE")
                    .size(30)
                    .horizontal_alignment(Horizontal::Center)
            )
            .width(Length::FillPortion(1))
            .align_x(Horizontal::Center)
            .align_y(Vertical::Bottom),
            container(
                text("OREILLE GAUCHE")
                    .size(22)
                    .horizontal_alignment(Horizontal::Center)
            )
            .width(Length::FillPortion(1))
            .height(Length::Fill)
            .align_x(Horizontal::Center)
            .align_y(Vertical::Center),
        ]
        .align_items(Alignment::Center)]
        .width(Length::Fill)
        .height(Length::Fixed(32.0))
        .align_items(Alignment::End);

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

        let immitance_title = column![text("IMMITANCEMÉTRIE")
            .size(30)
            .horizontal_alignment(Horizontal::Center)]
        .width(Length::Fill)
        .align_items(Alignment::Center);

        let vocal_audiogram_title_container = container(vocal_audiogram_title)
            .width(Length::Fill)
            .style(theme::Container::Custom(Box::new(
                TitleContainerCustomStyle,
            )));

        let immitance_title_container =
            container(immitance_title)
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
                IsRecorded::No,
                Some(self.is_recorded),
                Message::IsRecordedChanged
            )
            .spacing(RADIO_SPACING)
            .size(r_size)
            .text_size(t_size),
            radio(
                "Enregis.",
                IsRecorded::Yes,
                Some(self.is_recorded),
                Message::IsRecordedChanged
            )
            .spacing(RADIO_SPACING)
            .size(r_size)
            .text_size(t_size),
        ]
        .spacing(6);

        let open_text_input_right = text_input("", &self.vocal_misc_right)
            .on_input(Message::MiscRightChanged)
            .size(TABLE_MISC_SIZE)
            .width(Length::Fill);

        let open_text_input_left = text_input("", &self.vocal_misc_left)
            .on_input(Message::MiscLeftChanged)
            .size(TABLE_MISC_SIZE)
            .width(Length::Fill);

        let open_text_input_bin = text_input("", &self.vocal_misc_bin)
            .on_input(Message::MiscBinChanged)
            .size(TABLE_MISC_SIZE)
            .width(Length::Fill);

        let vocal_tables = row![
            horizontal_space(10),
            column![vocal_table_right, vertical_space(5), open_text_input_right]
                .width(Length::FillPortion(2)),
            horizontal_space(10),
            vocal_lang,
            horizontal_space(10),
            column![vocal_table_bin, vertical_space(5), open_text_input_bin]
                .width(Length::FillPortion(2)),
            horizontal_space(10),
            voice,
            horizontal_space(10),
            column![vocal_table_left, vertical_space(5), open_text_input_left]
                .width(Length::FillPortion(2)),
            horizontal_space(10),
        ]
        .width(Length::Shrink)
        .align_items(Alignment::Center);

        let id_lang_tables = row![
            horizontal_space(10),
            id_lang_table_right,
            horizontal_space(10),
            id_lang_table_bin,
            horizontal_space(10),
            id_lang_table_left,
            horizontal_space(10),
        ]
        .width(Length::Shrink)
        .align_items(Alignment::Center);

        let vocal_audiogram_content = column![
            vocal_audiogram_title_container,
            vertical_space(Length::Fixed(SPACE_BELOW_SECTION_TITLE)),
            vocal_tables,
            // vertical_space(Length::Fixed(10.0)),
            // vocal_misc
        ]
        .align_items(Alignment::Center);

        let tympa_content = row![
            horizontal_space(10),
            column![
                tympa_table_right,
                vertical_space(SPACE_BELOW_SECTION_TITLE),
                stap_table_right
            ]
            .width(Length::FillPortion(2)),
            horizontal_space(2.0),
            immit_graph.width(Length::FillPortion(1)),
            horizontal_space(2.0),
            // horizontal_space(LEGEND_WIDTH * 1.15),
            column![
                tympa_table_left,
                vertical_space(SPACE_BELOW_SECTION_TITLE),
                stap_table_left
            ]
            .width(Length::FillPortion(2)),
            horizontal_space(10),
        ];

        let immitance_content = column![
            immitance_title_container,
            vertical_space(Length::Fixed(SPACE_BELOW_SECTION_TITLE)),
            tympa_content
        ];

        // (stap_table_right, stap_table_left)

        // let stap_content = row![
        //     horizontal_space(10),
        //     stap_table_right,
        //     horizontal_space(LEGEND_WIDTH * 1.15),
        //     stap_table_left,
        //     horizontal_space(10),
        // ];

        //

        let content = column![
            tonal_audiogram_content,
            vertical_space(SECTION_SEPARATOR_SPACE),
            vocal_audiogram_content,
            vertical_space(8),
            id_lang_tables,
            vertical_space(SECTION_SEPARATOR_SPACE),
            immitance_content
        ];

        let final_content = container(content.align_items(Alignment::Center))
            .width(Length::Fill)
            // .height(Length::Fill)
            ;

        if self.show_partner_choices {
            let modal_content = container(
                column![
                    text("Sign Up").size(24),
                    column![
                        // column![text("Email").size(12),].spacing(5),
                        // column![text("Password").size(12),].spacing(5),
                        get_all_partners(&self.partner),
                        button(text("Submit")).on_press(Message::HidePartnerChoices),
                    ]
                    .spacing(10)
                ]
                .spacing(20),
            )
            .width(300)
            .padding(10)
            .style(theme::Container::Box);

            modal::Modal::new(final_content, modal_content)
                .on_blur(Message::HidePartnerChoices)
                .into()
        } else {
            final_content.into()
        }
    }

    fn theme(&self) -> Theme {
        Theme::Dark
    }
}
