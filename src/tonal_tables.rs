use crate::plot::EarSide;

use super::config::{
    TABLE_BACKGROUND_COLOR, TABLE_BORDER_COLOR, TABLE_ENTRY_SIZE, TABLE_ENTRY_TITLE_SIZE,
    TABLE_SPACING, TABLE_TEXT_COLOR, TABLE_TITLE_BG_COLOR, TABLE_TITLE_SIZE,
    TABLE_TITLE_TEXT_COLOR, TONAL_TABLE_COL_WIDTH,
};

use super::{AudioRox, Message};

use iced::alignment::Horizontal;
// use iced_native::widget::Container;

use iced::theme::{self, Theme};

use iced::widget::{
    column, container, container::Appearance, horizontal_space, row, text, text_input,
    vertical_space, Row,
};

use iced::{Alignment, Element, Length};

#[derive(Default)]
pub struct TonalTable {
    pub msp: String,
    pub msp4: String,
    pub fletcher: String,
}

pub struct TableContainerCustomStyle;

impl container::StyleSheet for TableContainerCustomStyle {
    type Style = Theme;

    fn appearance(&self, _style: &Self::Style) -> Appearance {
        container::Appearance {
            text_color: None,
            background: None,
            border_radius: 0.0,
            border_width: 2.0,
            border_color: TABLE_BORDER_COLOR,
        }
    }
}

pub struct TableTitleCustomStyle;

impl container::StyleSheet for TableTitleCustomStyle {
    type Style = Theme;

    fn appearance(&self, _style: &Self::Style) -> Appearance {
        container::Appearance {
            text_color: None, //Some(Color::from_rgb(0.05, 0.05, 0.02)),
            background: Some(TABLE_TITLE_BG_COLOR.into()),
            // background: None,
            border_radius: 0.0,
            border_width: 2.0,
            border_color: TABLE_BORDER_COLOR,
        }
    }
}

pub fn get_message_fn(s: &str, side: EarSide) -> impl Fn(String) -> Message {
    match (s, side) {
        ("MSP", EarSide::Right) => Message::MSPRightChanged,
        ("SDP", EarSide::Right) => Message::SDPRightChanged,
        ("MSP4", EarSide::Right) => Message::MSP4RightChanged,
        ("SRP", EarSide::Right) => Message::SRPRightChanged,
        ("FLCH", EarSide::Right) => Message::FLCHRightChanged,

        ("MSP", EarSide::Left) => Message::MSPLeftChanged,
        ("SDP", EarSide::Left) => Message::SDPLeftChanged,
        ("MSP4", EarSide::Left) => Message::MSP4LeftChanged,
        ("SRP", EarSide::Left) => Message::SRPLeftChanged,
        ("FLCH", EarSide::Left) => Message::FLCHLeftChanged,

        ("MSP", EarSide::Free) => Message::MSPFreeChanged,
        ("SDP", EarSide::Free) => Message::SDPFreeChanged,
        ("MSP4", EarSide::Free) => Message::MSP4FreeChanged,
        ("SRP", EarSide::Free) => Message::SRPFreeChanged,
        ("FLCH", EarSide::Free) => Message::FLCHFreeChanged,

        _ => panic!("Not a valid Table message: {}", s),
    }
}

pub fn make_tonal_tables(
    audio_rox: &AudioRox,
) -> (Element<Message>, Element<Message>, Element<Message>) {
    ///////////////////////////////////////////// TONAL TABLE LEFT /////////////////////////////////////////////
    let tonal_table_columns_left = [
        ("MSP", &audio_rox.tonal_table_left.msp),
        ("MSP4", &audio_rox.tonal_table_left.msp4),
        ("FLCH", &audio_rox.tonal_table_left.fletcher),
    ];

    let tonal_table_left = make_one_table(
        EarSide::Left,
        "Moyennes tonales oreille gauche (dB HL)",
        &tonal_table_columns_left,
    );

    ///////////////////////////////////////////// TONAL TABLE LEFT /////////////////////////////////////////////

    ///////////////////////////////////////////// TONAL TABLE RIGHT /////////////////////////////////////////////
    let tonal_table_columns_right = [
        ("MSP", &audio_rox.tonal_table_right.msp),
        ("MSP4", &audio_rox.tonal_table_right.msp4),
        ("FLCH", &audio_rox.tonal_table_right.fletcher),
        // ("N Confor\nparole", &audio_rox),
    ];

    let tonal_table_right = make_one_table(
        EarSide::Right,
        "Moyennes tonales oreille droite (dB HL)",
        &tonal_table_columns_right,
    );

    ///////////////////////////////////////////// TONAL TABLE RIGHT /////////////////////////////////////////////

    ///////////////////////////////////////////// TONAL TABLE FREE SPACE /////////////////////////////////////////////
    let tonal_table_columns_free = [
        ("MSP", &audio_rox.tonal_table_free.msp),
        ("MSP4", &audio_rox.tonal_table_free.msp4),
        ("FLCH", &audio_rox.tonal_table_free.fletcher),
        // ("N Confor\nparole", &audio_rox),
    ];

    let tonal_table_free = make_one_table(
        EarSide::Free,
        "Moyennes tonales champ libre (dB HL)",
        &tonal_table_columns_free,
    );

    ///////////////////////////////////////////// TONAL TABLE FREE SPACE /////////////////////////////////////////////

    (tonal_table_right, tonal_table_left, tonal_table_free)
}

pub fn make_vocal_tables(audio_rox: &AudioRox) -> (Element<Message>, Element<Message>) {
    ///////////////////////////////////////////// VOCAL TABLES /////////////////////////////////////////////
    // vocal tables
    let vocal_input_table_columns_left = [
        ("SDP", &audio_rox.vocal_table_left.sdp),
        ("SRP", &audio_rox.vocal_table_left.srp),
    ];

    let vocal_input_table_columns_right = [
        ("SDP", &audio_rox.vocal_table_right.sdp),
        ("SRP", &audio_rox.vocal_table_right.srp),
        // ("N Confor\nparole", &self),
    ];
    let mut vocal_table_left = Row::new();

    for (s, variable) in vocal_input_table_columns_left.iter() {
        let ear_side = EarSide::Left;
        let message_fn = get_message_fn(s, ear_side);
        let entry = column![
            container(text(*s).style(TABLE_TEXT_COLOR))
                .style(theme::Container::Box)
                .padding(5)
                .width(Length::Fixed(TONAL_TABLE_COL_WIDTH))
                .align_x(Horizontal::Center),
            text_input("", &variable, message_fn)
                .padding(3)
                .size(TABLE_ENTRY_SIZE)
                .width(Length::Fixed(TONAL_TABLE_COL_WIDTH)),
            // container("sO").style(theme::Container::Box).padding(5)
        ]
        .align_items(Alignment::Center);

        vocal_table_left = vocal_table_left.push(entry);
        vocal_table_left = vocal_table_left.push(horizontal_space(Length::Fixed(3.0)));
        // table = table.push(Rule::vertical(10));
        // table_left = table_left.height(Length::Shrink);
    }

    let vocal_table_left = vocal_table_left.height(Length::Shrink);

    let vocal_table_left = column![
        container(text("Moyennes vocales (dB HL)"))
            .style(theme::Container::Box)
            .padding(5)
            .width(Length::Shrink)
            .align_x(Horizontal::Center),
        vocal_table_left
    ]
    .spacing(TABLE_SPACING);
    let vocal_table_left = vocal_table_left.height(Length::Shrink);
    // .spacing(TABLE_SPACING);

    let mut vocal_table_right = Row::new();

    for (s, variable) in vocal_input_table_columns_right.iter() {
        let ear_side = EarSide::Right;
        let message_fn = get_message_fn(s, ear_side);
        let entry = column![
            container(text(*s).style(TABLE_TEXT_COLOR))
                .style(theme::Container::Box)
                .padding(5)
                .width(Length::Fixed(TONAL_TABLE_COL_WIDTH))
                .align_x(Horizontal::Center),
            text_input("", &variable, message_fn)
                .padding(3)
                .size(TABLE_ENTRY_SIZE)
                .width(Length::Fixed(TONAL_TABLE_COL_WIDTH)),
            // container("sO").style(theme::Container::Box).padding(5)
        ]
        .align_items(Alignment::Center);

        vocal_table_right = vocal_table_right.push(entry);
        vocal_table_right = vocal_table_right.push(horizontal_space(Length::Fixed(3.0)));
        // table = table.push(Rule::vertical(10));
        // table_left = table_left.height(Length::Shrink);
    }
    let vocal_table_right = vocal_table_right.height(Length::Shrink);

    (
        vocal_table_right.align_items(Alignment::Center).into(),
        vocal_table_left.align_items(Alignment::Center).into(),
    )
    ///////////////////////////////////////////// VOCAL TABLES /////////////////////////////////////////////
}

pub fn make_one_table(
    ear_side: EarSide,
    table_name: &str,
    table_columns: &[(&str, &String)],
) -> Element<'static, Message> {
    let mut table = Row::new();

    for (s, variable) in table_columns.iter() {
        let message_fn = get_message_fn(s, ear_side);

        let entry = row![
            container(
                text(*s)
                    .style(TABLE_TEXT_COLOR)
                    .size(TABLE_ENTRY_TITLE_SIZE)
            ),
            horizontal_space(2.0),
            // .padding(3)
            // .width(Length::Fixed(TONAL_TABLE_COL_WIDTH)),
            // .align_x(Horizontal::Center),
            text_input("", &variable, message_fn)
                .size(TABLE_ENTRY_SIZE)
                .width(Length::Fixed(TONAL_TABLE_COL_WIDTH)),
            horizontal_space(6.0)
        ]
        .align_items(Alignment::Center);

        table = table.push(entry);
        table = table.push(horizontal_space(Length::Fixed(3.0)));
        // table = table.push(Rule::vertical(10));
        // table_left = table_left.height(Length::Shrink);
    }
    // let table = table.height(Length::Shrink);

    let table = table
        .spacing(3)
        .height(Length::Shrink)
        .align_items(Alignment::Center);

    let table = container(
        column![
            container(
                container(
                    text(table_name)
                        .size(TABLE_TITLE_SIZE)
                        .style(TABLE_TITLE_TEXT_COLOR)
                        .horizontal_alignment(Horizontal::Center)
                )
                .padding(3)
            )
            // .align_x(Ho::Center)
            .align_x(Horizontal::Center)
            .width(Length::Fill)
            .style(theme::Container::Custom(Box::new(TableTitleCustomStyle,))),
            vertical_space(6.),
            table,
            vertical_space(7.5),
        ]
        .align_items(Alignment::Center),
    )
    .style(theme::Container::Custom(Box::new(
        TableContainerCustomStyle,
    )))
    .align_x(Horizontal::Center)
    .width(Length::FillPortion(2));

    table.into()
}
