mod config;

mod immi_plot;

mod legend;
mod partners;
mod plot;
mod preset;
mod tonal_tables;
mod view;
mod util;

use util::*;

use view::view;



use config::{
    WINDOW_HEIGHT, WINDOW_WIDTH
};


use iced::executor;
use iced::keyboard::{self, Modifiers};
use iced::theme::Theme;

use iced::widget::canvas::event;




use iced::{window, widget};
use iced::{
    subscription, Application, Command, Element,
    Settings, Subscription,
};





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
            Message::LegendModifierSelected(_) => {}
            Message::LegendShapeSelected(_) => {}

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
            
            Message::CCPatientChanged(value) => self.cc.patient = value,
            Message::CCAudioProChanged(value) => self.cc.audioprothesiste = value,
            Message::CCFamilyDocChanged(value) => self.cc.family_doctor = value,
            Message::CCORLChanged(value) => self.cc.orl = value,
            Message::CCOtherChanged(value) => self.cc.other = value,
            Message::CCReadapt(value) => self.cc.readapt = value,
            

            Message::PartnerChanged(value) =>  {
                self.partner = value;
                self.succursale_overlay_menu = Modals::Succursale;
            }

            Message::SuccursaleChanged(value) =>  {
                self.partner = value;
                println!("succursale changed : {:?}", self.partner );
                self.succursale_overlay_menu = Modals::None;
            }

            Message::ShowParnerChoices => {
                // self.show_partner_choices = true;
                self.succursale_overlay_menu = Modals::Partner;
                return widget::focus_next();
            }
            Message::ShowSuccursaleChoices => {
                // self.show_partner_choices = true;
                self.succursale_overlay_menu = Modals::Succursale;
                return widget::focus_next();
            }
            Message::HideSuccursaleMenu => {
                self.succursale_overlay_menu = Modals::None;
            }
            Message::CancelSuccursaleChoices => {
                self.succursale_overlay_menu = Modals::None;
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

    fn view(&self)  ->  Element<Message> {
        view(self)
    }

    fn theme(&self) -> Theme {
        Theme::Light
    }
}

