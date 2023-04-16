use crate::plot::EarSide;

use super::config::{
    STAP_ENTRY_SIZE, TABLE_BORDER_COLOR, TABLE_ENTRY_SIZE, TABLE_ENTRY_TITLE_SIZE,
    TABLE_TEXT_COLOR, TABLE_TITLE_BG_COLOR, TABLE_TITLE_SIZE, TABLE_TITLE_TEXT_COLOR,
    TONAL_TABLE_COL_WIDTH,
};

use super::{AudioRox, IdLang, Message, Stap, Tympa};

use iced::alignment::Horizontal;
// use iced_native::widget::Container;

use iced::theme::{self, Theme};

use iced::widget::{
    column, container, container::Appearance, horizontal_space, row, text, text_input,
    vertical_space, Column, Row,
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
        ("MSP4", EarSide::Right) => Message::MSP4RightChanged,
        ("FLCH", EarSide::Right) => Message::FLCHRightChanged,

        ("SDP", EarSide::Right) => Message::SDPRightChanged,
        ("SRP", EarSide::Right) => Message::SRPRightChanged,

        ("N confort\nparole", EarSide::Right) => Message::MiscRightChanged,

        ("MSP", EarSide::Left) => Message::MSPLeftChanged,
        ("MSP4", EarSide::Left) => Message::MSP4LeftChanged,
        ("FLCH", EarSide::Left) => Message::FLCHLeftChanged,

        ("SRP", EarSide::Left) => Message::SRPLeftChanged,
        ("SDP", EarSide::Left) => Message::SDPLeftChanged,
        ("N confort\nparole", EarSide::Left) => Message::MiscLeftChanged,

        ("SDP", EarSide::Free) => Message::SDPFreeChanged,
        ("SRP", EarSide::Free) => Message::SRPFreeChanged,
        ("N confort\nparole", EarSide::Free) => Message::MiscBinChanged,

        ("IdParoleRes1", EarSide::Left) => Message::IdLanRes1LeftChanged,
        ("IdParoleRes2", EarSide::Left) => Message::IdLanRes2LeftChanged,
        ("IdParoleLev1", EarSide::Left) => Message::IdLanLev1LeftChanged,
        ("IdParoleLev2", EarSide::Left) => Message::IdLanLev2LeftChanged,

        ("IdParoleRes1", EarSide::Right) => Message::IdLanRes1RightChanged,
        ("IdParoleRes2", EarSide::Right) => Message::IdLanRes2RightChanged,
        ("IdParoleLev1", EarSide::Right) => Message::IdLanLev1RightChanged,
        ("IdParoleLev2", EarSide::Right) => Message::IdLanLev2RightChanged,

        ("IdParoleRes1", EarSide::Free) => Message::IdLanRes1BinChanged,
        ("IdParoleRes2", EarSide::Free) => Message::IdLanRes2BinChanged,
        ("IdParoleLev1", EarSide::Free) => Message::IdLanLev1BinChanged,
        ("IdParoleLev2", EarSide::Free) => Message::IdLanLev2BinChanged,

        ("TympaVolume", EarSide::Left) => Message::TympaVolumeLeftChanged,
        ("TympaVolume", EarSide::Right) => Message::TympaVolumeRightChanged,

        ("TympaPressure", EarSide::Left) => Message::TympaPressureLeftChanged,
        ("TympaPressure", EarSide::Right) => Message::TympaPressureRightChanged,

        ("TympaCompliance", EarSide::Left) => Message::TympaComplianceLeftChanged,
        ("TympaCompliance", EarSide::Right) => Message::TympaComplianceRightChanged,

        ("StapIpsi500", EarSide::Left) => Message::StapIpsi500LeftChanged,
        ("StapIpsi1000", EarSide::Left) => Message::StapIpsi1000LeftChanged,
        ("StapIpsi2000", EarSide::Left) => Message::StapIpsi2000LeftChanged,
        ("StapIpsi4000", EarSide::Left) => Message::StapIpsi4000LeftChanged,

        ("StapIpsi500", EarSide::Right) => Message::StapIpsi500RightChanged,
        ("StapIpsi1000", EarSide::Right) => Message::StapIpsi1000RightChanged,
        ("StapIpsi2000", EarSide::Right) => Message::StapIpsi2000RightChanged,
        ("StapIpsi4000", EarSide::Right) => Message::StapIpsi4000RightChanged,

        ("StapControl500", EarSide::Left) => Message::StapControl500LeftChanged,
        ("StapControl1000", EarSide::Left) => Message::StapControl1000LeftChanged,
        ("StapControl2000", EarSide::Left) => Message::StapControl2000LeftChanged,
        ("StapControl4000", EarSide::Left) => Message::StapControl4000LeftChanged,

        ("StapControl500", EarSide::Right) => Message::StapControl500RightChanged,
        ("StapControl1000", EarSide::Right) => Message::StapControl1000RightChanged,
        ("StapControl2000", EarSide::Right) => Message::StapControl2000RightChanged,
        ("StapControl4000", EarSide::Right) => Message::StapControl4000RightChanged,

        _ => panic!("Not a valid Table message: {}", s),
    }
}

pub fn make_tonal_tables(audio_rox: &AudioRox) -> (Element<Message>, Element<Message>) {
    let tonal_table_columns_left = [
        ("MSP", &audio_rox.tonal_table_left.msp),
        ("MSP4", &audio_rox.tonal_table_left.msp4),
        ("FLCH", &audio_rox.tonal_table_left.fletcher),
    ];

    let tonal_table_left = make_one_tonal_table(
        EarSide::Left,
        // "Moyennes tonales oreille gauche (dB HL)",
        "MOYENNES TONALES OREILLE GAUCHE - dB HL",
        &tonal_table_columns_left,
    );

    let tonal_table_columns_right = [
        ("MSP", &audio_rox.tonal_table_right.msp),
        ("MSP4", &audio_rox.tonal_table_right.msp4),
        ("FLCH", &audio_rox.tonal_table_right.fletcher),
        // ("N Confor\nparole", &audio_rox),
    ];

    let tonal_table_right = make_one_tonal_table(
        EarSide::Right,
        // "Moyennes tonales oreille droite (dB HL)",
        "MOYENNES TONALES OREILLE DROITE - dB HL",
        &tonal_table_columns_right,
    );

    (tonal_table_right, tonal_table_left)
}

pub fn seuils_vocaux_tables(
    audio_rox: &AudioRox,
) -> (Element<Message>, Element<Message>, Element<Message>) {
    // vocal tables
    let vocal_input_table_columns_left = [
        ("SDP", &audio_rox.vocal_table_left.sdp),
        ("SRP", &audio_rox.vocal_table_left.srp),
        ("N confort\nparole", &audio_rox.vocal_table_left.misc),
    ];

    let vocal_input_table_columns_right = [
        ("SDP", &audio_rox.vocal_table_right.sdp),
        ("SRP", &audio_rox.vocal_table_right.srp),
        ("N confort\nparole", &audio_rox.vocal_table_right.misc),
        // ("N Confor\nparole", &self),
    ];

    let vocal_input_table_columns_binaural = [
        ("SDP", &audio_rox.vocal_table_binaural.sdp),
        ("SRP", &audio_rox.vocal_table_binaural.srp),
        ("N confort\nparole", &audio_rox.vocal_table_binaural.misc),
        // ("N Confor\nparole", &self),
    ];

    let tonal_table_left = make_one_vocal_table(
        EarSide::Left,
        // "Moyennes tonales oreille gauche (dB HL)",
        "SEUILS VOCAUX OR. GAUCHE - dB HL",
        &vocal_input_table_columns_left,
    );

    let tonal_table_right = make_one_vocal_table(
        EarSide::Right,
        // "Moyennes tonales oreille droite (dB HL)",
        "SEUILS VOCAUX OR. DROITE - dB HL",
        &vocal_input_table_columns_right,
    );

    let tonal_table_bin = make_one_vocal_table(
        EarSide::Free,
        "SEUILS VOCAUX BINAURAL - dB HL",
        &vocal_input_table_columns_binaural,
    );

    (tonal_table_right, tonal_table_left, tonal_table_bin)
}

pub fn identification_language(
    audio_rox: &AudioRox,
) -> (Element<Message>, Element<Message>, Element<Message>) {
    let id_table_left = make_one_id_language_table(
        EarSide::Left,
        "IDENTIFICATION PAROLE OR. GAUCHE",
        &audio_rox.id_lang_left,
    );

    let id_table_right = make_one_id_language_table(
        EarSide::Right,
        "IDENTIFICATION PAROLE OR. DROITE",
        &audio_rox.id_lang_right,
    );

    let id_table_bin = make_one_id_language_table(
        EarSide::Free,
        "IDENTIFICATION PAROLE BINAURAL",
        &audio_rox.id_lang_bin,
    );

    (id_table_right, id_table_left, id_table_bin)
}

pub fn make_one_tonal_table(
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

pub fn make_one_vocal_table(
    ear_side: EarSide,
    table_name: &str,
    table_columns: &[(&str, &String)],
) -> Element<'static, Message> {
    let mut table = Row::new();
    table = table.push(horizontal_space(Length::Fixed(2.0)));

    for (s, variable) in table_columns.iter() {
        let message_fn = get_message_fn(s, ear_side);
        // let txt = if *s == "" {
        //     ""
        // } else {
        //     ""
        // };
        let entry = row![
            container(
                text(*s)
                    .style(TABLE_TEXT_COLOR)
                    .size(TABLE_ENTRY_TITLE_SIZE)
                    .horizontal_alignment(Horizontal::Right)
            ),
            horizontal_space(3.0),
            // .padding(3)
            // .width(Length::Fixed(TONAL_TABLE_COL_WIDTH)),
            // .align_x(Horizontal::Center),
            text_input("", &variable, message_fn)
                .size(TABLE_ENTRY_SIZE)
                .width(Length::Fixed(TONAL_TABLE_COL_WIDTH)),
            horizontal_space(2.0)
        ]
        .align_items(Alignment::Center);

        table = table.push(entry);
        table = table.push(horizontal_space(Length::Fixed(2.0)));
        // table = table.push(Rule::vertical(10));
        // table_left = table_left.height(Length::Shrink);
    }
    table = table.push(horizontal_space(Length::Fixed(2.0)));
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

pub fn make_one_id_language_table(
    ear_side: EarSide,
    table_name: &str,
    table_columns: &IdLang,
) -> Element<'static, Message> {
    let mut table = Column::new();
    // table = table.push(horizontal_space(Length::Fixed(2.0)));

    // let message_fn = get_message_fn("IdParole", ear_side);

    let row_title_len = 60.0;
    let col1 = row![
        // vertical_space(2.0),
        container(
            text("Résultat")
                .style(TABLE_TEXT_COLOR)
                .size(TABLE_ENTRY_TITLE_SIZE) // .horizontal_alignment(Horizontal::Right)
        )
        .align_x(Horizontal::Right)
        .width(Length::Fixed(row_title_len)),
        // .width(Length::Shrink),
        // .padding(3),
        // container()
        horizontal_space(2.0),
        // .padding(3)
        // .width(Length::Fixed(TONAL_TABLE_COL_WIDTH)),
        // .align_x(Horizontal::Center),
        // vertical_space(3.0),
        container(
            text_input(
                "",
                &table_columns.result1,
                get_message_fn("IdParoleRes1", ear_side)
            )
            .size(TABLE_ENTRY_SIZE)
            .width(Length::Fixed(TONAL_TABLE_COL_WIDTH)),
        ),
        // vertical_space(1.0),
        horizontal_space(15.0),
        container(
            text_input(
                "",
                &table_columns.result2,
                get_message_fn("IdParoleRes2", ear_side)
            )
            .size(TABLE_ENTRY_SIZE)
            .width(Length::Fixed(TONAL_TABLE_COL_WIDTH)),
        ),
        horizontal_space(2.0),
        text("%")
            .style(TABLE_TEXT_COLOR)
            .size(TABLE_ENTRY_TITLE_SIZE)
            .horizontal_alignment(Horizontal::Right)
    ]
    .spacing(3)
    .align_items(Alignment::Center);

    let col2 = row![
        // vertical_space(2.0),
        container(
            text("  Niveau")
                .style(TABLE_TEXT_COLOR)
                .size(TABLE_ENTRY_TITLE_SIZE) // .horizontal_alignment(Horizontal::Right)
        )
        .align_x(Horizontal::Right)
        .width(Length::Fixed(row_title_len)),
        // .width(Length::Shrink),
        horizontal_space(2.0),
        // vertical_space(3.0),
        container(
            text_input(
                "",
                &table_columns.level1,
                get_message_fn("IdParoleLev1", ear_side)
            )
            .size(TABLE_ENTRY_SIZE)
            .width(Length::Fixed(TONAL_TABLE_COL_WIDTH)),
        ),
        // vertical_space(1.0),
        horizontal_space(15.0),
        container(
            text_input(
                "",
                &table_columns.level2,
                get_message_fn("IdParoleLev2", ear_side)
            )
            .size(TABLE_ENTRY_SIZE)
            .width(Length::Fixed(TONAL_TABLE_COL_WIDTH)),
        ),
        horizontal_space(2.0),
        text("db HL")
            .style(TABLE_TEXT_COLOR)
            .size(TABLE_ENTRY_TITLE_SIZE)
            .horizontal_alignment(Horizontal::Right)
    ]
    .spacing(3)
    .align_items(Alignment::Center);

    table = table.push(col1);
    // table = table.push(horizontal_space(Length::Fixed(10.0)));
    table = table.push(vertical_space(Length::Fixed(3.0)));
    table = table.push(col2);

    //
    // table = table.push(Rule::vertical(10));
    // table_left = table_left.height(Length::Shrink);

    // table = table.push(horizontal_space(Length::Fixed(2.0)));
    // let table = table.height(Length::Shrink);

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

pub fn tympa(audio_rox: &AudioRox) -> (Element<Message>, Element<Message>) {
    let tympa_table_left = make_one_tympa_table(
        EarSide::Left,
        "TYMPANOMÉTRIE OR. GAUCHE",
        &audio_rox.tympa_left,
    );

    let tympa_table_right = make_one_tympa_table(
        EarSide::Right,
        "TYMPANOMÉTRIE OR. DROITE",
        &audio_rox.tympa_right,
    );

    (tympa_table_right, tympa_table_left)
}

pub fn make_one_tympa_table(
    ear_side: EarSide,
    table_name: &str,
    table_columns: &Tympa,
) -> Element<'static, Message> {
    let mut table = Column::new();
    // table = table.push(horizontal_space(Length::Fixed(2.0)));

    // let message_fn = get_message_fn("IdParole", ear_side);

    // let row_title_len = 60.0;
    // let units_len = 30.0;
    let col1 = row![
        // vertical_space(2.0),
        horizontal_space(2.0),
        container(
            column![
                text("Volume")
                    .style(TABLE_TEXT_COLOR)
                    .size(TABLE_ENTRY_TITLE_SIZE)
                    .horizontal_alignment(Horizontal::Center),
                text("ml")
                    .style(TABLE_TEXT_COLOR)
                    .size(TABLE_ENTRY_TITLE_SIZE * 0.7)
                    .horizontal_alignment(Horizontal::Center)
            ]
            .align_items(Alignment::Center)
        )
        .align_x(Horizontal::Right)
        // .width(Length::Fixed(row_title_len)),
        .width(Length::Shrink),
        //
        container(
            text_input(
                "",
                &table_columns.volume,
                get_message_fn("TympaVolume", ear_side)
            )
            .size(TABLE_ENTRY_SIZE)
            .width(Length::Fixed(TONAL_TABLE_COL_WIDTH)),
        ),
        // horizontal_space(2.0),
        // container(
        //     text("ml")
        //         .style(TABLE_TEXT_COLOR)
        //         .size(TABLE_ENTRY_TITLE_SIZE) // .horizontal_alignment(Horizontal::Right)
        // )
        // .align_x(Horizontal::Left)
        // .width(Length::Fixed(units_len)),
        horizontal_space(10.0),
        //
        //
        container(
            column![
                text("Pression")
                    .style(TABLE_TEXT_COLOR)
                    .size(TABLE_ENTRY_TITLE_SIZE)
                    .horizontal_alignment(Horizontal::Center),
                text("daPa")
                    .style(TABLE_TEXT_COLOR)
                    .size(TABLE_ENTRY_TITLE_SIZE * 0.7)
                    .horizontal_alignment(Horizontal::Center)
            ]
            .align_items(Alignment::Center)
        )
        .align_x(Horizontal::Right)
        .width(Length::Shrink),
        //
        container(
            text_input(
                "",
                &table_columns.pressure,
                get_message_fn("TympaPressure", ear_side)
            )
            .size(TABLE_ENTRY_SIZE)
            .width(Length::Fixed(TONAL_TABLE_COL_WIDTH)),
        ),
        //
        horizontal_space(10.0),
        //
        //
        container(
            column![
                text("Compliance")
                    .style(TABLE_TEXT_COLOR)
                    .size(TABLE_ENTRY_TITLE_SIZE)
                    .horizontal_alignment(Horizontal::Center),
                text("ml")
                    .style(TABLE_TEXT_COLOR)
                    .size(TABLE_ENTRY_TITLE_SIZE * 0.7)
                    .horizontal_alignment(Horizontal::Center)
            ]
            .align_items(Alignment::Center)
        )
        .align_x(Horizontal::Right)
        // .width(Length::Fixed(row_title_len)),
        .width(Length::Shrink),
        //
        container(
            text_input(
                "",
                &table_columns.compliance,
                get_message_fn("TympaCompliance", ear_side)
            )
            .size(TABLE_ENTRY_SIZE)
            .width(Length::Fixed(TONAL_TABLE_COL_WIDTH)),
        ),
        //
        horizontal_space(2.0),
        //
        //
    ]
    .spacing(3)
    .align_items(Alignment::Center);
    table = table.push(col1);

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

pub fn stap(audio_rox: &AudioRox) -> (Element<Message>, Element<Message>) {
    let stap_table_left = make_one_stap_table(
        EarSide::Left,
        "RÉFLEXE STAPÉDIEN OR. GAUCHE - dB HL",
        &audio_rox.stap_left,
    );

    let stap_table_right = make_one_stap_table(
        EarSide::Right,
        "RÉFLEXE STAPÉDIEN OR. DROITE - dB HL",
        &audio_rox.stap_right,
    );

    (stap_table_right, stap_table_left)
}

pub fn make_one_stap_table(
    ear_side: EarSide,
    table_name: &str,
    table_columns: &Stap,
) -> Element<'static, Message> {
    let mut table = Column::new();
    // table = table.push(horizontal_space(Length::Fixed(2.0)));

    // let message_fn = get_message_fn("IdParole", ear_side);

    let row_title_len = 60.0;
    let units_len = 30.0;

    let first_col_width = 80.0;
    let col_width = 60.0;
    let text_input_width = col_width * 0.92;

    let top_row = row![
        container(
            text("Stimulation")
                .style(TABLE_TEXT_COLOR)
                .size(TABLE_ENTRY_TITLE_SIZE)
                .horizontal_alignment(Horizontal::Center),
        )
        .width(first_col_width)
        .align_x(Horizontal::Right),
        container(
            text("500Hz")
                .style(TABLE_TEXT_COLOR)
                .size(TABLE_ENTRY_TITLE_SIZE)
                .horizontal_alignment(Horizontal::Center),
        )
        .width(col_width)
        .align_x(Horizontal::Center),
        container(
            text("1kHz")
                .style(TABLE_TEXT_COLOR)
                .size(TABLE_ENTRY_TITLE_SIZE)
                .horizontal_alignment(Horizontal::Center),
        )
        .width(col_width)
        .align_x(Horizontal::Center),
        container(
            text("2kHz")
                .style(TABLE_TEXT_COLOR)
                .size(TABLE_ENTRY_TITLE_SIZE)
                .horizontal_alignment(Horizontal::Center),
        )
        .width(col_width)
        .align_x(Horizontal::Center),
        container(
            text("4kHz")
                .style(TABLE_TEXT_COLOR)
                .size(TABLE_ENTRY_TITLE_SIZE)
                .horizontal_alignment(Horizontal::Center),
        )
        .width(col_width)
        .align_x(Horizontal::Center),
    ]
    .spacing(3)
    .align_items(Alignment::Center);

    // Ipsilatéral (Seuil H.L.)

    let second_row = row![
        container(
            text("Ipsilatéral")
                .style(TABLE_TEXT_COLOR)
                .size(TABLE_ENTRY_TITLE_SIZE)
                .horizontal_alignment(Horizontal::Center),
        )
        .width(first_col_width)
        .align_x(Horizontal::Right),
        container(
            text_input(
                "",
                &table_columns.ipsi.khz_500,
                get_message_fn("StapIpsi500", ear_side)
            )
            .size(TABLE_ENTRY_SIZE)
            .width(Length::Fixed(text_input_width)),
        )
        .width(col_width)
        .align_x(Horizontal::Center),
        container(
            text_input(
                "",
                &table_columns.ipsi.khz_1000,
                get_message_fn("StapIpsi1000", ear_side)
            )
            .size(TABLE_ENTRY_SIZE)
            .width(Length::Fixed(text_input_width)),
        )
        .width(col_width)
        .align_x(Horizontal::Center),
        container(
            text_input(
                "",
                &table_columns.ipsi.khz_2000,
                get_message_fn("StapIpsi2000", ear_side)
            )
            .size(TABLE_ENTRY_SIZE)
            .width(Length::Fixed(text_input_width)),
        )
        .width(col_width)
        .align_x(Horizontal::Center),
        container(
            text_input(
                "",
                &table_columns.ipsi.khz_4000,
                get_message_fn("StapIpsi4000", ear_side)
            )
            .size(TABLE_ENTRY_SIZE)
            .width(Length::Fixed(text_input_width)),
        )
        .width(col_width)
        .align_x(Horizontal::Center),
    ]
    .spacing(3)
    .align_items(Alignment::Center);

    let third_row = row![
        container(
            text("Controlatéral")
                .style(TABLE_TEXT_COLOR)
                .size(TABLE_ENTRY_TITLE_SIZE)
                .horizontal_alignment(Horizontal::Center),
        )
        .width(first_col_width)
        .align_x(Horizontal::Right),
        container(
            text_input(
                "",
                &table_columns.control.khz_500,
                get_message_fn("StapControl500", ear_side)
            )
            .size(TABLE_ENTRY_SIZE)
            .width(Length::Fixed(text_input_width)),
        )
        .width(col_width)
        .align_x(Horizontal::Center),
        container(
            text_input(
                "",
                &table_columns.control.khz_1000,
                get_message_fn("StapControl1000", ear_side)
            )
            .size(TABLE_ENTRY_SIZE)
            .width(Length::Fixed(text_input_width)),
        )
        .width(col_width)
        .align_x(Horizontal::Center),
        container(
            text_input(
                "",
                &table_columns.control.khz_2000,
                get_message_fn("StapControl2000", ear_side)
            )
            .size(TABLE_ENTRY_SIZE)
            .width(Length::Fixed(text_input_width)),
        )
        .width(col_width)
        .align_x(Horizontal::Center),
        container(
            text_input(
                "",
                &table_columns.control.khz_4000,
                get_message_fn("StapControl4000", ear_side)
            )
            .size(TABLE_ENTRY_SIZE)
            .width(Length::Fixed(text_input_width)),
        )
        .width(col_width)
        .align_x(Horizontal::Center),
    ]
    .spacing(3)
    .align_items(Alignment::Center);

    table = table.push(top_row);
    table = table.push(vertical_space(2.0));
    table = table.push(second_row);
    table = table.push(vertical_space(4.0));
    table = table.push(third_row);

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
