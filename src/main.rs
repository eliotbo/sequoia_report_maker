//! by John Conway. It leverages a `Canvas` together with other widgets.

use serde::{Deserialize, Serialize};
// use serde_json::Result;
use std::fs::File;
use std::io::Write;
// mod checkboxes;
mod config;
// mod grid;
mod immi_plot;
// mod immitance;
mod legend;
mod partners;
mod plot;
mod preset;
mod tonal_tables;
// mod modal;
// mod partners::modal::*;

// use immitance::*;
// use partners::modal::Modal;
use partners::{get_all_partners, get_all_succursales, modal, Partner};

use tonal_tables::{
     identification_language, make_tonal_tables, seuils_vocaux_tables, stap, tympa,
    IsRecorded, Lang,   TonalTable,
};
// use checkboxes::{Transductor, Validity};
// use grid::Grid;
use config::{
    CustomButtonStyle, LegendCustomStyle, TitleContainerCustomStyle,
    DEFAULT_TEXT_INPUT_CONTENT_SIZE, DEFAULT_TEXT_SIZE, IMMIT_CANVAS_WIDTH, LEGEND_BOTTOM_SPACE,
    LEGEND_WIDTH,  RADIO_SIZE, RADIO_SPACING, RADIO_TEXT_SIZE, RADIO_TITLE_SIZE,
    SECTION_SEPARATOR_SPACE,  SECTION_TITLE_HORIZONTAL_SPACE,
     SPACE_BELOW_SECTION_TITLE,  TEXT_LINE_VSPACE,
    WINDOW_HEIGHT, WINDOW_WIDTH
};
use immi_plot::im_plot;
use legend::{draw_legend};
use plot::{plot, EarSide, Plot, PlotInfo, Shape};


// use chrono;
// use chrono::Datelike;



use iced::alignment::{Horizontal, Vertical};
use iced::font::{self, Font};
// use iced::event::{self, Event};
use iced::executor;
use iced::keyboard::{self, Modifiers};
use iced::theme::{self, Theme};

use iced::widget::canvas::event;

use iced::widget::{Column, scrollable,
    self, button, checkbox, column, container, horizontal_space,
    radio, row, text, text_input, vertical_space, Rule,
};
use iced::widget::scrollable::{ Properties};

use iced::window;
use iced::{
    subscription, Alignment, Application, Command, Element, Length,
    Settings, Subscription,
};
// use std::time::{Duration, Instant};



#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)] 
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum MethodEval {
    Standard,
    Visual,
    Play,
    None,
}
impl Default for MethodEval {
    fn default() -> Self {
        MethodEval::None
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
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

    let default_font = config::DEFAULT_FONT;

    AudioRox::run(Settings {
        antialiasing: true,

        default_font,

        window: window::Settings {
            position: window::Position::Centered,
            size: (WINDOW_WIDTH, WINDOW_HEIGHT),
            ..window::Settings::default()
        },
        ..Settings::default()
    })
}

// fn get_window_image() -> ImageBuffer<Rgba<u8>, Vec<u8>> {
//     // Create a new window
//     let window = AudioRox::new(Settings {
//         antialiasing: true,
//         window: window::Settings {
//             position: window::Position::Centered,
//             size: (WINDOW_WIDTH, WINDOW_HEIGHT),
//             ..window::Settings::default()
//         },
//         ..Settings::default()
//     })
//     .0;

//     // Draw the window contents
//     let rox_default = AudioRox::default();
//     let mut renderer = iced::Renderer::new(rox_default);
//     let mut buffer = vec![0; (WINDOW_WIDTH * WINDOW_HEIGHT * 4) as usize];
//     let viewport =
//         iced_graphics::Viewport::with_physical_size(Size::new(WINDOW_WIDTH, WINDOW_HEIGHT), 1.0);
//     renderer.backend_mut().set_viewport(viewport);
//     renderer.draw(&mut buffer, &window);

//     // Convert the buffer to an image
//     ImageBuffer::<Rgba<u8>, Vec<u8>>::from_raw(WINDOW_WIDTH, WINDOW_HEIGHT, buffer).unwrap()
// }

#[derive(Default, Serialize, Deserialize)]
struct VocalTable {
    sdp: String,
    srp: String,
    list: String,
    misc: String,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct IdLang {
    pub result1: String,
    pub level1: String,
    pub list1: String,
    pub result2: String,
    pub level2: String,
    pub list2: String,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct Tympa {
    pub volume: String,
    pub pressure: String,
    pub compliance: String,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct KHzList {
    pub khz_500: String,
    pub khz_1000: String,
    pub khz_2000: String,
    pub khz_4000: String,
}
#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct Stap {
    pub ipsi: KHzList,
    pub control: KHzList,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct CC {
    pub patient: bool,
    pub audioprothesiste: bool,
    pub family_doctor: bool,
    pub orl: bool,
    pub other: bool,
    pub readapt: bool,
}

impl Default for CC {
    fn default() -> Self {
        Self {
            patient: false,
            audioprothesiste: false,
            family_doctor: false,
            orl: false,
            other: false,
            readapt: false,
        }
    }
}

#[derive(Default, Serialize, Deserialize)]
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
    method: MethodEval,
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
    tympanometer_name: String,
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

    cc: CC,

    plot_right: PlotInfo,
    plot_left: PlotInfo,
}

impl AudioRox {
    fn hide_partner_choices(&mut self) {
        self.show_partner_choices = false;
        // self.email.clear();
        // self.password.clear();
    }

    fn save_to_file(&self, filename: &str) -> std::io::Result<()> {
        let data = self;

        let json = serde_json::to_string(&data).unwrap();
        let mut file = File::create(filename)?;
        file.write_all(json.as_bytes())
    }

    fn load_from_file(
        &self,
        filename: &str,
    ) -> std::result::Result<Self, Box<dyn std::error::Error>> {
        let file_contents = std::fs::read_to_string(filename)?;
        let data: Self = serde_json::from_str(&file_contents)?;
        Ok(data)
    }
}

fn load_fonts() -> iced::Command<Message> {
    iced::Command::batch([
        iced::font::load(include_bytes!("../fonts/Roboto-Medium.ttf").as_slice())
            .map(Message::FontLoaded),
            font::load(include_bytes!("../fonts/FiraSans-Light.ttf").as_slice())
            .map(Message::FontLoaded),
            font::load(include_bytes!("../fonts/Lato-Bold.ttf").as_slice())
            .map(Message::FontLoaded),
    ])
}

#[derive(Debug, Clone)]
pub enum Message {
    FontLoaded(Result<(), font::Error>),

    LegendShapeSelected(Shape),
    LegendModifierSelected(Shape),

    SaveFile,
    LoadFile,

    ShowParnerChoices,
    HidePartnerChoices,

    CCPatientChanged(bool),
    CCAudioProChanged(bool),
    CCFamilyDocChanged(bool),
    CCORLChanged(bool),
    CCOtherChanged(bool),
    CCReadapt(bool),

    PartnerChanged(Partner),
    AdequateRestPeriodChanged(bool),
    AnteriorThresholdDateChanged(String),
    AudiometerNameChanged(String),
    TympanometerNameChanged(String),
    DefaultChecked(bool),
    CustomChecked(bool),
    ValidityChanged(Validity),
    MethodChanged(MethodEval),
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
    IdLanList1LeftChanged(String),
    IdLanList2LeftChanged(String),

    IdLanRes1RightChanged(String),
    IdLanRes2RightChanged(String),
    IdLanLev1RightChanged(String),
    IdLanLev2RightChanged(String),
    IdLanList1RightChanged(String),
    IdLanList2RightChanged(String),

    IdLanRes1BinChanged(String),
    IdLanRes2BinChanged(String),
    IdLanLev1BinChanged(String),
    IdLanLev2BinChanged(String),
    IdLanList1BinChanged(String),
    IdLanList2BinChanged(String),

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
            
            load_fonts(),
        )
    }

    fn title(&self) -> String {
        String::from("audiometry")
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::FontLoaded(_) => (),
            Message::LegendModifierSelected(value) => {}
            Message::LegendShapeSelected(value) => {}

            Message::LoadFile => {
                // Perform your save operation here
                // println!("S key is pressed");
                println!("{}", serde_json::to_string(&self).unwrap());
                match self.load_from_file("data.json") {
                    Ok(data) => {
                        println!("Data saved successfully");
                        *self = data;
                    }
                    Err(e) => println!("Failed to save data: {}", e),
                }
                // ...
            }

            Message::SaveFile => {
                // Perform your save operation here
                // println!("S key is pressed");
                println!("{}", serde_json::to_string(&self).unwrap());
                match self.save_to_file("data.json") {
                    Ok(_) => println!("Data loaded successfully"),
                    Err(e) => println!("Failed to load data: {}", e),
                }
                // ...
            }

            // Message::CCChanged(value) => {}
            Message::CCPatientChanged(value) => self.cc.patient = value,
            Message::CCAudioProChanged(value) => self.cc.audioprothesiste = value,
            Message::CCFamilyDocChanged(value) => self.cc.family_doctor = value,
            Message::CCORLChanged(value) => self.cc.orl = value,
            Message::CCOtherChanged(value) => self.cc.other = value,
            Message::CCReadapt(value) => self.cc.readapt = value,
            

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
            Message::TympanometerNameChanged(value) => self.tympanometer_name = value,
            Message::DefaultChecked(value) => {
                self.default_checkbox = value;
            }
            Message::CustomChecked(value) => self.custom_checkbox = value,
            Message::ValidityChanged(new_validity) => self.validity = new_validity,
            Message::TransductorChanged(new_transductor) => {
                self.transductor = new_transductor;
            }
            Message::MethodChanged(new_method) => self.method = new_method,
            // Message::SuccursaleChanged(new_succursale) => self.succursale = new_succursale,
            Message::MSPRightChanged(value) => self.tonal_table_right.msp = value,
            Message::MSP4RightChanged(value) => self.tonal_table_right.msp4 = value,
            Message::FLCHRightChanged(value) => self.tonal_table_right.fletcher = value,

            Message::MSPLeftChanged(value) => self.tonal_table_left.msp = value,
            Message::MSP4LeftChanged(value) => self.tonal_table_left.msp4 = value,
            Message::FLCHLeftChanged(value) => self.tonal_table_left.fletcher = value,

            Message::SDPRightChanged(value) => self.vocal_table_right.sdp = value,
            Message::SRPRightChanged(value) => self.vocal_table_right.srp = value,

            // Message::ListRightChanged(value) => self.vocal_table_right.list = value,
            Message::SDPLeftChanged(value) => self.vocal_table_left.sdp = value,
            Message::SRPLeftChanged(value) => self.vocal_table_left.srp = value,

            // Message::ListLeftChanged(value) => self.vocal_table_left.list = value,
            Message::SDPFreeChanged(value) => self.vocal_table_binaural.sdp = value,
            Message::SRPFreeChanged(value) => self.vocal_table_binaural.srp = value,

            Message::MiscRightChanged(value) => self.vocal_table_right.misc = value,
            Message::MiscLeftChanged(value) => self.vocal_table_left.misc = value,
            Message::MiscBinChanged(value) => self.vocal_table_binaural.misc = value,

            // Message::MiscRightChanged(new_misc) => self.vocal_misc_right = new_misc,
            // Message::MiscLeftChanged(new_misc) => self.vocal_misc_left = new_misc,
            // Message::MiscBinChanged(new_misc) => self.vocal_misc_bin = new_misc,
            Message::VocalLangChanged(value) => self.vocal_lang = value,
            Message::IsRecordedChanged(value) => self.is_recorded = value,

            Message::IdLanRes1LeftChanged(value) => self.id_lang_left.result1 = value,
            Message::IdLanRes2LeftChanged(value) => self.id_lang_left.result2 = value,
            Message::IdLanLev1LeftChanged(value) => self.id_lang_left.level1 = value,
            Message::IdLanLev2LeftChanged(value) => self.id_lang_left.level2 = value,
            Message::IdLanList1LeftChanged(value) => self.id_lang_left.list1 = value,
            Message::IdLanList2LeftChanged(value) => self.id_lang_left.list2 = value,

            Message::IdLanRes1RightChanged(value) => self.id_lang_right.result1 = value,
            Message::IdLanRes2RightChanged(value) => self.id_lang_right.result2 = value,
            Message::IdLanLev1RightChanged(value) => self.id_lang_right.level1 = value,
            Message::IdLanLev2RightChanged(value) => self.id_lang_right.level2 = value,
            Message::IdLanList1RightChanged(value) => self.id_lang_right.list1 = value,
            Message::IdLanList2RightChanged(value) => self.id_lang_right.list2 = value,

            Message::IdLanRes1BinChanged(value) => self.id_lang_bin.result1 = value,
            Message::IdLanRes2BinChanged(value) => self.id_lang_bin.result2 = value,
            Message::IdLanLev1BinChanged(value) => self.id_lang_bin.level1 = value,
            Message::IdLanLev2BinChanged(value) => self.id_lang_bin.level2 = value,
            Message::IdLanList1BinChanged(value) => self.id_lang_bin.list1 = value,
            Message::IdLanList2BinChanged(value) => self.id_lang_bin.list2 = value,

            Message::TympaVolumeLeftChanged(value) => self.tympa_left.volume = value,
            Message::TympaVolumeRightChanged(value) => self.tympa_right.volume = value,

            Message::TympaPressureLeftChanged(value) => self.tympa_left.pressure = value,
            Message::TympaPressureRightChanged(value) => self.tympa_right.pressure = value,

            Message::TympaComplianceLeftChanged(value) => self.tympa_left.compliance = value,
            Message::TympaComplianceRightChanged(value) => self.tympa_right.compliance = value,

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

            Message::None => {} // _ => {}
        }

        Command::none()
    }

    fn subscription(&self) -> Subscription<Message> {
        subscription::events_with(|event, status| match (event, status) {
            (
                iced::event::Event::Keyboard(keyboard::Event::KeyPressed {
                    key_code: keyboard::KeyCode::S,
                    modifiers: Modifiers::CTRL,
                    ..
                }),
                event::Status::Ignored,
            ) => Some(Message::SaveFile),
            (
                iced::event::Event::Keyboard(keyboard::Event::KeyPressed {
                    key_code: keyboard::KeyCode::L,
                    modifiers: Modifiers::CTRL,
                    ..
                }),
                event::Status::Ignored,
            ) => Some(Message::LoadFile),
            _ => None,
        })
    }

    fn view(&self) -> Element<Message> {
        //
        let r_size = RADIO_SIZE;
        let t_size = RADIO_TEXT_SIZE;
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
            .spacing(3)
            .width(Length::Shrink);

        let validity_title = text("VALIDITÉ")
            .size(RADIO_TITLE_SIZE)
            .width(Length::Shrink);

        let validity_content = column![validity_title, validity_section,].spacing(2);
        ///////////////////////////////////////////// VALIDITE /////////////////////////////////////////////

        ///////////////////////////////////////////// METHOD /////////////////////////////////////////////
        let cond_standard = radio(
            "Standard (Hughson-Westlake)",
            MethodEval::Standard,
            Some(self.method),
            Message::MethodChanged,
        )
        .spacing(RADIO_SPACING)
        .size(RADIO_SIZE)
        .text_size(RADIO_TEXT_SIZE);

        let cond_play = radio(
            "Jeu",
            MethodEval::Play,
            Some(self.method),
            Message::MethodChanged,
        )
        .spacing(RADIO_SPACING)
        .size(RADIO_SIZE)
        .text_size(RADIO_TEXT_SIZE);

        let cond_visual = radio(
            "Visuel",
            MethodEval::Visual,
            Some(self.method),
            Message::MethodChanged,
        )
        .spacing(RADIO_SPACING)
        .size(RADIO_SIZE)
        .text_size(RADIO_TEXT_SIZE);

        let method_eval = column![
            text("MÉTHODE D'ÉVALUATION : \nCONDITIONNEMENT").size(RADIO_TITLE_SIZE),
            // vertical_space(2.0),
            cond_standard,
            cond_play,
            cond_visual,
        ]
        .spacing(3);

        ///////////////////////////////////////////// METHOD /////////////////////////////////////////////

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
            text("Date seuils antérieurs (•) : ")
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
            "Repos sonore inadéquat (< 16h)",
            self.adequate_rest_period,
            Message::AdequateRestPeriodChanged,
        )
        .spacing(RADIO_SPACING)
        .size(12)
        .text_size(16);

        // self.audiometer_name;
        let standard = column![
            audiometer_type,
            vertical_space(2.),
            anterior_thresholds_date,
            vertical_space(2.),
            adequate_rest_period,
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

        let transductor_section = column![intra, supra, free].spacing(3).width(Length::Shrink);

        let transductor_title = text("ÉCOUTEURS")
            .size(RADIO_TITLE_SIZE)
            .width(Length::Shrink);

        let transductor_content = column![transductor_title, transductor_section,].spacing(3);
        ///////////////////////////////////////////// TRANSDUCTOR /////////////////////////////////////////////

        let (tonal_table_right, tonal_table_left) = make_tonal_tables(&self);
        let (vocal_table_right, vocal_table_left, vocal_lang, voice) = seuils_vocaux_tables(&self);
        let (id_lang_table_right, id_lang_table_left, id_lang_table_bin) =
            identification_language(&self);
        let (tympa_table_right, tympa_table_left) = tympa(&self);
        let (stap_table_right, stap_table_left) = stap(&self);

        // create a header with two columns of text: on the left and one on the right
        let text_vspace = TEXT_LINE_VSPACE;

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

        // let current_date = chrono::Utc::now().date_naive();
        // println!("{}", current_date.year());
        // let year = current_date.year();
        // let day = current_date.day();
        // let month = current_date.month();

        let (clinic, succursales) = get_all_succursales(&self.partner);



        let header = row![
            horizontal_space(50.0),
            container(
                column![
                    // vertical_space(10.0),
                    // container(image::Image::new("images/logo.PNG").width(128))
                    //     .width(Length::Fixed(96.)),
                    text("Roxanne Bolduc")
                        .font(config::FIRA)
                        .size(30)
                        .horizontal_alignment(Horizontal::Left),
                    text("Audiologiste")
                        .size(20)
                        .horizontal_alignment(Horizontal::Left),
                ]
                .align_items(Alignment::Center)
                
            ).width(Length::FillPortion(2))
            .align_x(Horizontal::Left),
            // .alignment(Alignment::Start),
            horizontal_space(45),
            container(column![
                text("ÉVALUATION AUDIOLOGIQUE")
                    .size(27)
                    .horizontal_alignment(Horizontal::Center)
                    .width(Length::Fill),
                vertical_space(15.)
            ])
            .height(Length::Fill)
            .align_x(Horizontal::Center)
            .align_y(Vertical::Bottom)
            .width(Length::FillPortion(3)),
            horizontal_space(1),
            // .align(Alignment::Start),
            column![
                vertical_space(Length::Fixed(13.)),
                container(row![
                    text("Date de l'évaluation : ")
                        .size(18.0)
                        .vertical_alignment(Vertical::Center),
                    // text(format!("Date de l'évaluation: {day}/{month}/{year}")).size(14),
                    // vertical_space(Length::Fixed(text_vspace)),
                    column![vertical_space(text_vspace), Rule::horizontal(1.0),]
                ])
                // .align_y(Vertical::Bottom)
                .height(Length::Fixed(20.0)),
                vertical_space(Length::Fixed(20.)),
                // modal
                // Rule::horizontal(1),
                container(column![
                    button(text(&("Lieu de l'évaluation : ".to_owned() + &clinic)).size(18.))
                        .on_press(Message::ShowParnerChoices)
                        .padding(0.)
                        .style(theme::Button::Custom(Box::new(CustomButtonStyle))),
                    // text("Lieu de l'évaluation: Clinique de l'Audition Bois & Associés audioprothésistes").size(14),
                    succursales
                ])
                .height(Length::Fixed(120. + 60.)),
                
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
            .width(Length::FillPortion(2))
        ]
        .align_items(Alignment::Center)
        .padding([0, 5, 0, 5])
        .height(Length::Fixed(120. + 60.))
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

        let audio_right = column![
            // audio_right_title,
            audiogram_right,
            vertical_space(15.0),
            row![
                horizontal_space(10.0),
                tonal_table_right,
                horizontal_space(10.0),
            ]
        ]
        // .width(Length::FillPortion(2))
        .align_items(Alignment::Center);

        // .style(theme::Container::Custom(Box::new(
        //     TableContainerCustomStyle,
        // )))
        // .center_x();

        let audiorgam_left = plot(data2.clone(), Shape::X, EarSide::Left);
        // let audio_left_title = text("OREILLE GAUCHE")
        //     .size(26)
        //     .horizontal_alignment(Horizontal::Center);

        let audio_left = column![
            // audio_left_title,
            audiorgam_left,
            vertical_space(15.0),
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
                vertical_space(5.0),
                row![horizontal_space(8.0), method_eval],
                vertical_space(2.0),
                text("Normes ANSI S3 en vigueur").size(DEFAULT_TEXT_SIZE),
                vertical_space(2.0),
                standard_container,
            ])
            .padding(5.0)
            .style(theme::Container::Custom(Box::new(LegendCustomStyle,))), // .style(LegendCustomStyle),
        ]
        .align_items(Alignment::Center)
        .height(Length::Shrink)
        .width(Length::Fixed(LEGEND_WIDTH));

        let mid_audiograph = container(mid_col).width(Length::Shrink);

        let tonal_audiogram_title = make_title("AUDIOMÉTRIE TONALE");

        let tonal_audiogram_title_container = container(tonal_audiogram_title)
            .width(Length::Fill)
            .style(theme::Container::Custom(Box::new(
                TitleContainerCustomStyle,
            )));

        let vocal_audiogram_title = make_title("AUDIOMÉTRIE VOCALE");
        let immitance_title = make_title("IMMITANCEMÉTRIE");

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

        // let method_eval = row![
        //     text("Méthode d'évaluation : Conditionnement ").size(15),
        //     horizontal_space(10.0),
        //     radio(
        //         "Standard (Hughson-Westlake)",
        //         MethodEval::Standard,
        //         Some(self.method),
        //         Message::MethodChanged
        //     )
        //     .spacing(RADIO_SPACING)
        //     .size(12)
        //     .text_size(14),
        //     horizontal_space(10.0),
        //     radio(
        //         "Jeu",
        //         MethodEval::Play,
        //         Some(self.method),
        //         Message::MethodChanged
        //     )
        //     .spacing(RADIO_SPACING)
        //     .size(12)
        //     .text_size(14),
        //     horizontal_space(10.0),
        //     radio(
        //         "Visuel",
        //         MethodEval::Visual,
        //         Some(self.method),
        //         Message::MethodChanged
        //     )
        //     .spacing(RADIO_SPACING)
        //     .size(12)
        //     .text_size(14),
        // ];

        let audiograms = column![
            row![
                horizontal_space(SECTION_TITLE_HORIZONTAL_SPACE),
                tonal_audiogram_title_container,
                horizontal_space(SECTION_TITLE_HORIZONTAL_SPACE),
            ],
            // row![audio_right, mid_audiograph, horizontal_space(6), audio_left] // .align_items(Alignment::Center)
            row![
                container(audio_right)
                    .width(Length::FillPortion(1))
                    .align_x(Horizontal::Right),
                container(mid_audiograph)
                    .width(Length::Shrink)
                    .align_x(Horizontal::Center),
                horizontal_space(6),
                container(audio_left)
                    .width(Length::FillPortion(1))
                    .align_x(Horizontal::Left) // .align(Alignment::Center)
            ], // .align_items(Alignment::Center)
               // vertical_space(5.0),
               // tonal_tables
               // vertical_space(2.0),
               // method_eval
        ]
        .align_items(Alignment::Center);

        let tonal_audiogram_content = column![header, audiograms];

        // let checkbex = checkboxes::CheckBex::default();
        // let checkbex_element = checkbex.view();
        //
        // let content = column![checkbex_element];

        ///////////////////////////////////// VOCAL TABLES /////////////////////////////////////

        // // a column of two checkboxes for "FR" and "ANG"
        // let vocal_lang = row![
        //     text("LANGUE: ").size(RADIO_TITLE_SIZE),
        //     horizontal_space(5.),
        //     radio(
        //         "Fr.",
        //         Lang::French,
        //         Some(self.vocal_lang),
        //         Message::VocalLangChanged
        //     )
        //     .spacing(RADIO_SPACING)
        //     .size(r_size)
        //     .text_size(t_size),
        //     radio(
        //         "Ang.",
        //         Lang::English,
        //         Some(self.vocal_lang),
        //         Message::VocalLangChanged
        //     )
        //     .spacing(RADIO_SPACING)
        //     .size(r_size)
        //     .text_size(t_size),
        // ]
        // .spacing(6);

        // let voice = row![
        //     text("VOIX: ").size(RADIO_TITLE_SIZE),
        //     horizontal_space(5.),
        //     radio(
        //         "Nue",
        //         IsRecorded::No,
        //         Some(self.is_recorded),
        //         Message::IsRecordedChanged
        //     )
        //     .spacing(RADIO_SPACING)
        //     .size(r_size)
        //     .text_size(t_size),
        //     radio(
        //         "Enregistrée",
        //         IsRecorded::Yes,
        //         Some(self.is_recorded),
        //         Message::IsRecordedChanged
        //     )
        //     .spacing(RADIO_SPACING)
        //     .size(r_size)
        //     .text_size(t_size),
        // ]
        // .spacing(6);

        // let open_text_input_right = text_input("", &self.vocal_misc_right)
        //     .on_input(Message::MiscRightChanged)
        //     .size(TABLE_MISC_SIZE)
        //     .width(Length::Fill);

        // let open_text_input_left = text_input("", &self.vocal_misc_left)
        //     .on_input(Message::MiscLeftChanged)
        //     .size(TABLE_MISC_SIZE)
        //     .width(Length::Fill);

        // let open_text_input_bin = text_input("", &self.vocal_misc_bin)
        //     .on_input(Message::MiscBinChanged)
        //     .size(TABLE_MISC_SIZE)
        //     .width(Length::Fill);

        // let vocal_tables = row![
        //     horizontal_space(10),
        //     column![vocal_table_right, vertical_space(5), open_text_input_right]
        //         .width(Length::FillPortion(2)),
        //     horizontal_space(10),
        //     vocal_lang,
        //     horizontal_space(10),
        //     column![vocal_table_bin, vertical_space(5), open_text_input_bin]
        //         .width(Length::FillPortion(2)),
        //     horizontal_space(10),
        //     voice,
        //     horizontal_space(10),
        //     column![vocal_table_left, vertical_space(5), open_text_input_left]
        //         .width(Length::FillPortion(2)),
        //     horizontal_space(10),
        // ]
        // .width(Length::Shrink)
        // .align_items(Alignment::Center);

        let vocal_tables = row![
            horizontal_space(10),
            container(vocal_table_right).width(Length::FillPortion(4)),
            // horizontal_space(10),
            // container(vocal_lang).width(Length::FillPortion(1)),
            horizontal_space(10),
            row![
                container(voice).width(Length::FillPortion(1)),
                horizontal_space(10),
                container(vocal_lang).width(Length::FillPortion(1)),
            ]
            .width(Length::FillPortion(4)),
            // horizontal_space(10),
            // container(voice).width(Length::FillPortion(1)),
            horizontal_space(10),
            container(vocal_table_left).width(Length::FillPortion(4)),
            horizontal_space(10),
        ]
        .width(Length::Shrink)
        .align_items(Alignment::Center);

        let id_lang_tables = row![
            horizontal_space(10),
            container(id_lang_table_right).width(Length::FillPortion(4)),
            // column![
            //     id_lang_table_right,
            //     vertical_space(5),
            //     open_text_input_right
            // ]
            // .width(Length::FillPortion(4)),
            horizontal_space(10),
            container(id_lang_table_bin).width(Length::FillPortion(4)),
            // column![id_lang_table_bin, vertical_space(5), open_text_input_bin]
            //     .width(Length::FillPortion(6)),
            horizontal_space(10),
            container(id_lang_table_left).width(Length::FillPortion(4)),
            // column![id_lang_table_left, vertical_space(5), open_text_input_left]
            //     .width(Length::FillPortion(4)),
            horizontal_space(10),
        ]
        .width(Length::Shrink)
        .align_items(Alignment::Center);

        let vocal_audiogram_content = column![
            row![
                horizontal_space(SECTION_TITLE_HORIZONTAL_SPACE),
                vocal_audiogram_title_container,
                horizontal_space(SECTION_TITLE_HORIZONTAL_SPACE),
            ],
            // vertical_space(Length::Fixed(SPACE_BELOW_SECTION_TITLE)),
            // row![vocal_lang, horizontal_space(50.), voice],
            vertical_space(Length::Fixed(SPACE_BELOW_SECTION_TITLE)),
            vocal_tables,
            // vertical_space(Length::Fixed(10.0)),
            // vocal_misc
        ]
        .align_items(Alignment::Center);

        let tympanometer_type = row![
            text("Tympanomètre: ")
                .size(16)
                .horizontal_alignment(Horizontal::Left),
            text_input(
                "",
                &self.tympanometer_name,
                // Message::AudiometerNameChanged
            )
            .on_input(Message::TympanometerNameChanged)
            .size(DEFAULT_TEXT_INPUT_CONTENT_SIZE) // .width(Length::Fill)
        ]
        .width(Length::Fixed(IMMIT_CANVAS_WIDTH - 50.0))
        .align_items(Alignment::Center);

        let tympa_content = row![
            horizontal_space(10),
            column![
                tympa_table_right,
                vertical_space(SPACE_BELOW_SECTION_TITLE),
                stap_table_right
            ]
            .width(Length::FillPortion(5)),
            horizontal_space(2.0),
            column![immit_graph, vertical_space(5.0), tympanometer_type]
                .width(Length::FillPortion(3))
                .align_items(Alignment::Center),
            horizontal_space(2.0),
            // horizontal_space(LEGEND_WIDTH * 1.15),
            column![
                tympa_table_left,
                vertical_space(SPACE_BELOW_SECTION_TITLE),
                stap_table_left
            ]
            .width(Length::FillPortion(5)),
            horizontal_space(10),
        ]
        .height(Length::Shrink);

        let immitance_content = column![
            row![
                horizontal_space(SECTION_TITLE_HORIZONTAL_SPACE),
                immitance_title_container,
                horizontal_space(SECTION_TITLE_HORIZONTAL_SPACE),
            ],
            vertical_space(Length::Fixed(SPACE_BELOW_SECTION_TITLE)),
            tympa_content
        ];

        //
        let note_vspace = 16.0;
        let note_line = container(row![
            text("Notes: ")
                .size(note_vspace)
                .vertical_alignment(Vertical::Center),
            // text(format!("Date de l'évaluation: {day}/{month}/{year}")).size(14),
            // vertical_space(Length::Fixed(text_vspace)),
            // column![vertical_space(note_vspace), Rule::horizontal(1.0),]
        ])
        .align_y(Vertical::Top);
        // .height(Length::Fixed(30.0));

        let note = column![
            note_line,
            vertical_space(Length::Fixed(TEXT_LINE_VSPACE - 10.0)),
            Rule::horizontal(1.),
            vertical_space(Length::Fixed(TEXT_LINE_VSPACE)),
            Rule::horizontal(1.),
            vertical_space(Length::Fixed(TEXT_LINE_VSPACE)),
            Rule::horizontal(1.),
            vertical_space(Length::Fixed(10.0)),
            text("Voir rapport audiologique complet ci-joint.").size(note_vspace),
            text("Évaluation globale des besoins faite.").size(note_vspace),
        ]
        .width(Length::Fixed(450.0));

        let cc = column![
            text("CC").size(note_vspace),
            vertical_space(5.0),
            checkbox("Patient", self.cc.patient, Message::CCPatientChanged)
                .size(note_vspace)
                .text_size(note_vspace),
            checkbox(
                "Audioprothésiste",
                self.cc.audioprothesiste,
                Message::CCAudioProChanged
            )
            .size(note_vspace)
            .text_size(note_vspace),
            checkbox(
                "Médecin de famille",
                self.cc.family_doctor,
                Message::CCFamilyDocChanged
            )
            .size(note_vspace)
            .text_size(note_vspace),
            checkbox(
                "Centre de réadaptation",
                self.cc.readapt,
                Message::CCReadapt
            ).size(note_vspace)
            .text_size(note_vspace),
            checkbox("ORL", self.cc.orl, Message::CCORLChanged)
                .size(note_vspace)
                .text_size(note_vspace),
            checkbox(
                "_____________________",
                self.cc.other,
                Message::CCOtherChanged
            )
            .size(note_vspace)
            .text_size(note_vspace),
        ]
        .spacing(2);

        let logo_ordre = column![
            vertical_space(1.0),
            container(
                iced::widget::image::Image::new("images/ordre256.jpg").width(150) // iced::widget::image::Image::new("images/ordre.png").width(95)
            )
            .width(Length::Fixed(150.))
        ];

        let signature = row![
            // column![
            //     vertical_space(10.0),
            //     container(image::Image::new("images/logo.PNG").width(128))
            //         .width(Length::Fixed(96.)),
            // ],
            column![
                vertical_space(40.0),
                Rule::horizontal(1.),
                row![
                    text("Roxanne Bolduc  MPA,\nAudiologiste OOAQ #4182").size(22),
                    horizontal_space(25.)
                ],
            ]
            .width(Length::Fill)
            .align_items(Alignment::End)
        ];

        let bottom_content = row![
            // method_eval,
            horizontal_space(3.0),
            note,
            horizontal_space(60.0),
            cc,
            horizontal_space(10.0),
            logo_ordre,
            horizontal_space(10.0),
            signature
        ]
        .align_items(Alignment::End);

        let content = column![
            tonal_audiogram_content,
            vertical_space(SECTION_SEPARATOR_SPACE),
            vocal_audiogram_content,
            vertical_space(8),
            id_lang_tables,
            vertical_space(SECTION_SEPARATOR_SPACE),
            immitance_content,
            // vertical_space(SECTION_SEPARATOR_SPACE),
            // vertical_space(SECTION_SEPARATOR_SPACE * 0.9),
            // .style(theme::Container::Custom(Box::new(TableTitleCustomStyle,))),
            // vertical_space(SECTION_SEPARATOR_SPACE),
            // method_eval,
            bottom_content // .style(theme::Container::Custom(Box::new(TableTitleCustomStyle,)))
        ];

        let final_content = container(content.align_items(Alignment::Center))
            .width(Length::Fill)
            // .height(Length::Fill)
            ;

        if self.show_partner_choices {
            let modal_content = container(
                column![
                    text("Partenaire").size(24),
                    column![
                        get_all_partners(&self.partner),
                        button(text("OK")).on_press(Message::HidePartnerChoices),
                    ]
                    .spacing(15)
                ]
                .spacing(5),
            )
            .width(300)
            .padding(10)
            .style(theme::Container::Box);

            modal::Modal::new(final_content, modal_content)
                .on_blur(Message::HidePartnerChoices)
                .into()
        } else {
            // let final_element: Element<Message> = column![final_content];
            let final_element: Column<Message> = column![final_content];

            scrollable(final_element).into()
            // .vertical_scroll( Properties::new()
            // .width(10)
            // .margin(0)
            // .scroller_width(10),
            // ).into()
        }
    }

    fn theme(&self) -> Theme {
        Theme::Light
    }
}

pub fn make_title(title: &str) -> Element<Message> {
    let title_bar = column![row![
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
            text(title)
                .size(24)
                .horizontal_alignment(Horizontal::Center)
        )
        .width(Length::FillPortion(1))
        .align_x(Horizontal::Center)
        .align_y(Vertical::Top),
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

    return title_bar.into();
}
