


use super::util::*;
use super::partners::{get_all_partners, get_all_succursales, modal, };

use super::tonal_tables::{
     identification_language, make_tonal_tables, seuils_vocaux_tables, stap, tympa,

};

use super::config::{
     LegendCustomStyle, TitleContainerCustomStyle,
    DEFAULT_TEXT_INPUT_CONTENT_SIZE, IMMIT_CANVAS_WIDTH, LEGEND_BOTTOM_SPACE,
    LEGEND_WIDTH,  RADIO_SIZE, RADIO_SPACING, RADIO_TEXT_SIZE, RADIO_TITLE_SIZE,
    SECTION_SEPARATOR_SPACE,  SECTION_TITLE_HORIZONTAL_SPACE,
     SPACE_BELOW_SECTION_TITLE,  TEXT_LINE_VSPACE,
};
use super::immi_plot::im_plot;
use super::legend::draw_legend;
use super::plot::{plot, EarSide,   Shape};

// use super::{AudioRox, Message, };
use iced::alignment::{Horizontal, Vertical};

use iced::theme;

use iced::widget::{Column, scrollable,
     button, checkbox, column, container, horizontal_space,
    radio, row, text, text_input, vertical_space, Rule,
};

use iced::{
    Alignment,  Element, Length,
};



pub fn view(audiorox: &AudioRox) ->  Element<Message> {
    //
    let r_size = RADIO_SIZE;
    let t_size = RADIO_TEXT_SIZE;

    ///////////////////////////////////////////// VALIDITE /////////////////////////////////////////////
    let validity = audiorox.validity;

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
        Some(audiorox.method),
        Message::MethodChanged,
    )
    .spacing(RADIO_SPACING)
    .size(RADIO_SIZE)
    .text_size(13.7);

    let cond_play = radio(
        "Jeu",
        MethodEval::Play,
        Some(audiorox.method),
        Message::MethodChanged,
    )
    .spacing(RADIO_SPACING)
    .size(RADIO_SIZE)
    .text_size(RADIO_TEXT_SIZE);

    let cond_visual = radio(
        "Visuel",
        MethodEval::Visual,
        Some(audiorox.method),
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



    ///////////////////////////////////////////// standard /////////////////////////////////////////////
    // text_input for audiometer name
    let audiometer_type = row![
        text("Audiomètre: ")
            .size(14)
            .horizontal_alignment(Horizontal::Left),
        text_input(
            "AD629",
            &audiorox.audiometer_name,
            // Message::AudiometerNameChanged
        )
        .on_input(Message::AudiometerNameChanged)
        .size(DEFAULT_TEXT_INPUT_CONTENT_SIZE)
        .width(Length::Fill)
    ]
    .align_items(Alignment::Center);

    let anterior_thresholds_date = row![
        text("Date seuils antérieurs (•) : ")
            .size(14)
            .horizontal_alignment(Horizontal::Left),
        text_input(
            "",
            &audiorox.anterior_threshold_date,
        )
        .on_input(Message::AnteriorThresholdDateChanged)
        .size(DEFAULT_TEXT_INPUT_CONTENT_SIZE)
        .width(Length::Fill)
    ]
    .align_items(Alignment::Center);

    // a checkbox for adequate rest period
    let adequate_rest_period = checkbox(
        "Repos sonore inadéquat (<16h)",
        audiorox.adequate_rest_period,
        Message::AdequateRestPeriodChanged,
    )
    .spacing(RADIO_SPACING)
    .size(14)
    .text_size(14);

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
    let transductor = audiorox.transductor;

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

    let (tonal_table_right, tonal_table_left) = make_tonal_tables(&audiorox);
    let (vocal_table_right, vocal_table_left, vocal_lang, voice) = seuils_vocaux_tables(&audiorox);
    let (id_lang_table_right, id_lang_table_left, id_lang_table_bin) =
        identification_language(&audiorox);
    let (tympa_table_right, tympa_table_left) = tympa(&audiorox);
    let (stap_table_right, stap_table_left) = stap(&audiorox);

    // create a header with two columns of text: on the left and one on the right
    let text_vspace = TEXT_LINE_VSPACE;




    let succursale = super::partners::make_succursale_element(&audiorox.partner);



    let header = row![
        horizontal_space(50.0),
        container(
            column![
                text("Roxanne Bolduc")
                    .font(super::config::FIRA)
                    .size(30)
                    .horizontal_alignment(Horizontal::Left),
                text("Audiologiste")
                    .size(20)
                    .horizontal_alignment(Horizontal::Left),
            ]
            .align_items(Alignment::Center)
            
        ).width(Length::FillPortion(2))
        .align_x(Horizontal::Left),
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

        column![
            vertical_space(Length::Fixed(13.)),
            container(row![
                text("Date de l'évaluation : ")
                    .size(18.0)
                    .vertical_alignment(Vertical::Center),

                column![vertical_space(text_vspace), Rule::horizontal(1.0),]
            ])
            .height(Length::Fixed(20.0)),
            vertical_space(Length::Fixed(20.)),

            succursale
            .height(Length::Fixed(120. + 60.)),
            vertical_space(Length::Fixed(2.)),

        ]

        .width(Length::FillPortion(2))
    ]
    .align_items(Alignment::Center)
    .padding([0, 5, 0, 5])
    .height(Length::Fixed(120. + 60.))
    .width(Length::Fill);
    let data1 = vec![10.0, 20.0, 30.0, 10.0, 60.0, 65.0];
    let data2 = data1.iter().map(|x| x + 10.0).collect::<Vec<f32>>();
    let audiogram_right =
        container(plot(data1.clone(), Shape::Less, EarSide::Right)).align_x(Horizontal::Center);

    let immit_graph = container(im_plot()).align_x(Horizontal::Center);
    // .style(theme::Container::Custom(Box::new(
    //     TableContainerCustomStyle,
    // )));



    let audio_right = column![
        audiogram_right,
        vertical_space(15.0),
        row![
            horizontal_space(10.0),
            tonal_table_right,
            horizontal_space(10.0),
        ]
    ]
    .align_items(Alignment::Center);

    // .style(theme::Container::Custom(Box::new(
    //     TableContainerCustomStyle,
    // )));

    let audiorgam_left = plot(data2.clone(), Shape::X, EarSide::Left);


    let audio_left = column![
        audiorgam_left,
        vertical_space(15.0),
        row![
            tonal_table_left,
            horizontal_space(10.0),
        ]
    ]
    .align_items(Alignment::Center);

    let legend = container(draw_legend())
        .width(Length::Shrink);

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
            text("Normes ANSI S3 en vigueur").size(RADIO_TEXT_SIZE),
            vertical_space(2.0),
            standard_container,
        ])
        .padding(5.0)
        .style(theme::Container::Custom(Box::new(LegendCustomStyle,))), 
        // .style(LegendCustomStyle),
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



    let audiograms = column![
        row![
            horizontal_space(SECTION_TITLE_HORIZONTAL_SPACE),
            tonal_audiogram_title_container,
            horizontal_space(SECTION_TITLE_HORIZONTAL_SPACE),
        ],
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
                .align_x(Horizontal::Left) 
        ], 
    ]
    .align_items(Alignment::Center);

    let tonal_audiogram_content = column![header, audiograms];



    let vocal_tables = row![
        horizontal_space(10),
        container(vocal_table_right).width(Length::FillPortion(4)),
        horizontal_space(10),
        row![
            container(voice).width(Length::FillPortion(1)),
            horizontal_space(10),
            container(vocal_lang).width(Length::FillPortion(1)),
        ]
        .width(Length::FillPortion(4)),
        horizontal_space(10),
        container(vocal_table_left).width(Length::FillPortion(4)),
        horizontal_space(10),
    ]
    .width(Length::Shrink)
    .align_items(Alignment::Center);

    let id_lang_tables = row![
        horizontal_space(10),
        container(id_lang_table_right).width(Length::FillPortion(4)),

        horizontal_space(10),
        container(id_lang_table_bin).width(Length::FillPortion(4)),

        horizontal_space(10),
        container(id_lang_table_left).width(Length::FillPortion(4)),
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

        vertical_space(Length::Fixed(SPACE_BELOW_SECTION_TITLE)),
        vocal_tables,

    ]
    .align_items(Alignment::Center);

    let tympanometer_type = row![
        text("Tympanomètre: ")
            .size(16)
            .horizontal_alignment(Horizontal::Left),
        text_input(
            "",
            &audiorox.tympanometer_name,
        )
        .on_input(Message::TympanometerNameChanged)
        .size(DEFAULT_TEXT_INPUT_CONTENT_SIZE) 
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

    ])
    .align_y(Vertical::Top);


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
        checkbox("Patient", audiorox.cc.patient, Message::CCPatientChanged)
            .size(note_vspace)
            .text_size(note_vspace),
        checkbox(
            "Audioprothésiste",
            audiorox.cc.audioprothesiste,
            Message::CCAudioProChanged
        )
        .size(note_vspace)
        .text_size(note_vspace),
        checkbox(
            "Médecin de famille",
            audiorox.cc.family_doctor,
            Message::CCFamilyDocChanged
        )
        .size(note_vspace)
        .text_size(note_vspace),
        checkbox(
            "Centre de réadaptation",
            audiorox.cc.readapt,
            Message::CCReadapt
        ).size(note_vspace)
        .text_size(note_vspace),
        checkbox("ORL", audiorox.cc.orl, Message::CCORLChanged)
            .size(note_vspace)
            .text_size(note_vspace),
        checkbox(
            "_____________________",
            audiorox.cc.other,
            Message::CCOtherChanged
        )
        .size(note_vspace)
        .text_size(note_vspace),
    ]
    .spacing(2);

    let logo_ordre = column![
        vertical_space(1.0),
        container(
            iced::widget::image::Image::new("images/ordre256.jpg").width(150) 
        )
        .width(Length::Fixed(150.))
    ];

    let signature = row![

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
        bottom_content // .style(theme::Container::Custom(Box::new(TableTitleCustomStyle,)))
    ];

    let final_content = container(content.align_items(Alignment::Center))
        .width(Length::Fill)
        // .height(Length::Fill)
        ;

    if let Modals::Partner = audiorox.succursale_overlay_menu {
        
        let modal_content = container(
            column![
                text("Partenaire").size(24),
                column![
                    get_all_partners(&audiorox.partner),
                    button(text("OK")).on_press(Message::HideSuccursaleMenu),
                ]
                .spacing(15)
            ]
            .spacing(5),
        )
        .width(300)
        .padding(10)
        .style(theme::Container::Box);

        modal::Modal::new(final_content, modal_content)
            .on_blur(Message::HideSuccursaleMenu)
            .into()
    } else if let Modals::Succursale = audiorox.succursale_overlay_menu {
        let modal_content = container(
            column![
                text("Succursale").size(24),
                column![
                    get_all_succursales(&audiorox.partner).1,
                    button(text("OK")).on_press(Message::CancelSuccursaleChoices),
                ]
                .spacing(15)
            ]
            .spacing(5),
        )
        .width(300)
        .padding(10)
        .style(theme::Container::Box);

        modal::Modal::new(final_content, modal_content)
            .on_blur(Message::HideSuccursaleMenu)
            .into()
    } else {
        let final_element: Column<Message> = column![final_content];

        scrollable(final_element).into()
    }
}

pub fn make_title(title: &str) -> Element<Message> {
    let title_bar = column![row![
        container(
            text("OREILLE DROITE")
                .size(22)
                .horizontal_alignment(Horizontal::Center) 
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
