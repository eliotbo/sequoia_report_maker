
use serde::{Deserialize, Serialize};

use std::fs::File;
use std::io::Write;

use super::partners:: PartnerAndSuccursale;

use super::tonal_tables::{IsRecorded, Lang,   TonalTable,};

use super::plot::{ PlotInfo, Shape};

use iced::font;


#[derive(Debug, Clone)]
pub enum Message {
    FontLoaded(Result<(), font::Error>),

    LegendShapeSelected(Shape),
    LegendModifierSelected(Shape),

    SaveFile,
    LoadFile,

    ShowParnerChoices,
    ShowSuccursaleChoices,
    HideSuccursaleMenu,
    CancelSuccursaleChoices,


    CCPatientChanged(bool),
    CCAudioProChanged(bool),
    CCFamilyDocChanged(bool),
    CCORLChanged(bool),
    CCOtherChanged(bool),
    CCReadapt(bool),

    PartnerChanged(PartnerAndSuccursale),
    SuccursaleChanged(PartnerAndSuccursale),
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


#[derive(Default, Serialize, Deserialize)]
pub struct VocalTable {
    pub sdp: String,
    pub srp: String,
    pub list: String,
    pub misc: String,
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

#[derive( Serialize, Deserialize)]
pub enum Modals {
    None,
    Partner,
    Succursale,
}

impl Default for Modals {
    fn default() -> Self {
        Modals::None
    }
}

#[derive(Default, Serialize, Deserialize)]
pub struct AudioRox {
    pub show_partner_choices: bool,
    #[serde(skip_serializing, skip_deserializing)]
    pub succursale_overlay_menu: Modals,
    pub is_playing: bool,
    pub queued_ticks: usize,
    pub speed: usize,
    pub next_speed: Option<usize>,
    pub version: usize,

    pub partner: PartnerAndSuccursale,
    pub default_checkbox: bool,
    pub custom_checkbox: bool,
    pub validity: Validity, // TODO: these three should be somewhere else
    pub method: MethodEval, // 
    pub transductor: Transductor, // Maybe this whole struct (AudioRox should be in a data related file)
    pub tonal_table_left: TonalTable,
    pub tonal_table_right: TonalTable,
    pub tonal_table_free: TonalTable,

    pub vocal_table_left: VocalTable,
    pub vocal_table_right: VocalTable,
    pub vocal_table_free: VocalTable,
    pub vocal_table_binaural: VocalTable,

    pub vocal_misc_right: String,
    pub vocal_misc_left: String,
    pub vocal_misc_bin: String,

    pub anterior_threshold_date: String,
    pub audiometer_name: String,
    pub tympanometer_name: String,
    pub adequate_rest_period: bool,

    pub id_lang_left: IdLang,
    pub id_lang_right: IdLang,
    pub id_lang_bin: IdLang,

    pub tympa_left: Tympa,
    pub tympa_right: Tympa,
    pub stap_left: Stap,
    pub stap_right: Stap,
    pub vocal_lang: Lang,
    pub is_recorded: IsRecorded,
    pub cc: CC,

    #[serde(skip_serializing, skip_deserializing)]
    pub _plot_right: PlotInfo,
    #[serde(skip_serializing, skip_deserializing)]
    pub _plot_left: PlotInfo,
}

impl AudioRox {


    pub fn save_to_file(&self, filename: &str) -> std::io::Result<()> {
        let data = self;

        let json = serde_json::to_string(&data).unwrap();
        let mut file = File::create(filename)?;
        file.write_all(json.as_bytes())
    }

    pub fn load_from_file(
        &self,
        filename: &str,
    ) -> std::result::Result<Self, Box<dyn std::error::Error>> {
        let file_contents = std::fs::read_to_string(filename)?;
        let data: Self = serde_json::from_str(&file_contents)?;
        Ok(data)
    }
}

pub fn load_fonts() -> iced::Command<Message> {
    iced::Command::batch([
        iced::font::load(include_bytes!("../fonts/Roboto-Medium.ttf").as_slice())
            .map(Message::FontLoaded),
            font::load(include_bytes!("../fonts/FiraSans-Light.ttf").as_slice())
            .map(Message::FontLoaded),
            font::load(include_bytes!("../fonts/Lato-Bold.ttf").as_slice())
            .map(Message::FontLoaded),
    ])
}

