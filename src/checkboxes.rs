use iced::theme::{self, Theme};
use iced::widget::{
    checkbox, column, container, radio, row, rule, text, text_input, Column, Row, Rule,
};
use iced::{Element, Length};

use crate::Message;
// use super::quad;

// const ICON_FONT: Font = Font::External {
//     name: "Icons",
//     bytes: include_bytes!("../fonts/icons.ttf"),
// };

#[derive(Default)]
pub struct CheckBex {
    default_checkbox: bool,
    custom_checkbox: bool,
    validity: Validity,
    transductor: Transductor,
    msp: String,
    sdp: String,
    msp4: String,
    srp: String,
}

// #[derive(Debug, Clone)]
// pub enum Message {
//     DefaultChecked(bool),
//     CustomChecked(bool),
//     ValidityChanged(Validity),
//     TransductorChanged(Transductor),
//     MSPChanged(String),
//     SDPChanged(String),
//     MSP4Changed(String),
//     SRPChanged(String),
// }

// pub fn radio<Message, Renderer, V>(
//     label: impl Into<String>,
//     value: V,
//     selected: Option<V>,
//     on_click: impl FnOnce(V) -> Message
// ) -> Radio<Message, Renderer>

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

pub fn get_message_fn(s: &str) -> impl Fn(String) -> Message {
    match s {
        "MSP" => Message::MSPChanged,
        "SDP" => Message::SDPChanged,
        "MSP4" => Message::MSP4Changed,
        "SRP" => Message::SRPChanged,
        _ => Message::SDPChanged,
    }
}

impl CheckBex {
    pub fn view(&self) -> Element<'static, Message> {
        let r_size = 16;
        let t_size = 16;

        ///////////////////////////////////////////// CHECKBOX /////////////////////////////////////////////
        let default_checkbox = checkbox("Default", self.default_checkbox, Message::DefaultChecked)
            .size(r_size)
            .text_size(t_size);

        // let custom_checkbox = checkbox("Custom", self.custom_checkbox, Message::CustomChecked)
        //     .icon(checkbox::Icon {
        //         font: ICON_FONT,
        //         code_point: '\u{e901}',
        //         size: None,
        //     })
        //     .size(r_size)
        //     .text_size(t_size);

        let checkbox_content = column![default_checkbox].spacing(6);

        ///////////////////////////////////////////// VALIDITE /////////////////////////////////////////////
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

        let validity_section: Element<_> = column![good_validity, medium_validity, null_validity]
            .spacing(3)
            .into();

        let validity_title = text("Validité").size(20);

        let validity_content = column![validity_title, validity_section,].spacing(6);
        ///////////////////////////////////////////// VALIDITE /////////////////////////////////////////////

        ///////////////////////////////////////////// TRANSDUCTOR /////////////////////////////////////////////
        let transductor = self.transductor;

        let intra = radio(
            "Intra-auriculaire",
            Transductor::Intra,
            Some(transductor),
            Message::TransductorChanged,
        )
        .size(r_size)
        .text_size(t_size);

        let supra = radio(
            "Supra-auriculaire",
            Transductor::Supra,
            Some(transductor),
            Message::TransductorChanged,
        )
        .size(r_size)
        .text_size(t_size);

        let free = radio(
            "Bonne",
            Transductor::Free,
            Some(transductor),
            Message::TransductorChanged,
        )
        .size(r_size)
        .text_size(t_size);

        let transductor_section: Element<_> = column![intra, supra, free].spacing(3).into();

        let transductor_title = text("Transducteur").size(20);

        let transductor_content = column![transductor_title, transductor_section,].spacing(6);
        ///////////////////////////////////////////// TRANSDUCTOR /////////////////////////////////////////////

        ///////////////////////////////////////////// INPUT TABLE /////////////////////////////////////////////
        ///////////////////////////////////// MSP
        // let msp_text = text("MSP").size(20);

        // let msp_input = text_input(
        //     "---",
        //     &self.msp,
        //     Message::MSPChanged,
        // )
        // .padding(10)
        // .size(16);

        // let msp = column![msp_text, msp_input].width(Length::FillPortion(1));

        // ///////////////////////////////////// SDP
        // let sdp_text = text("SDP").size(20);

        // let sdp_input = text_input(
        //     "---",
        //     &self.sdp,
        //     Message::SDPChanged,
        // )
        // .padding(10)
        // .size(16);

        // let sdp = column![sdp_text, sdp_input].width(Length::FillPortion(1));

        let input_table_columns = [
            ("MSP", &self.msp),
            ("SDP", &self.sdp),
            ("MSP4", &self.msp4),
            ("SRP", &self.srp),
        ];

        let mut input_table = Row::new();
        for (title, variable) in input_table_columns.iter().cloned() {
            let message_fn = get_message_fn(title);
            let text = text(title).size(16);
            let input = text_input("---", &variable, message_fn).padding(3).size(16);
            let col = column![text, input].width(Length::FillPortion(1));
            input_table = input_table.push(col);
            // table = table.push("hello");
        }

        input_table = input_table.width(Length::Fixed(250.));
        ///////////////////////////////////////////// INPUT TABLE /////////////////////////////////////////////

        ///////////////////////////////////////////// TABLE /////////////////////////////////////////////
        // let msp = column!["MSP", ""];
        // let sdp = column!["SDP", ""];
        // let msp4 = column!["MSP4", ""];
        // let srp = column!["SRP", ""];

        let mut table = Row::new();

        for (s, _) in input_table_columns.iter() {
            let entry = column![container(*s).padding(5), container("").padding(5)];
            table = table.push(entry);
            table = table.push(Rule::vertical(1));
        }
        table = table.height(Length::Fixed(150.));

        // let table = column!["TITLE", row![msp,sdp,msp4, srp]];

        ///////////////////////////////////////////// TABLE /////////////////////////////////////////////

        let all_content = column![
            validity_content,
            transductor_content,
            checkbox_content,
            input_table,
            table
        ]
        .spacing(20)
        .width(Length::Fill)
        .height(Length::Fill);

        // all_content

        container(all_content)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .center_y()
            .into()
    }
}
