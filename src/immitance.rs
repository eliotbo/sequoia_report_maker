use crate::plot::EarSide;

use super::config::{
    TABLE_BORDER_COLOR, TABLE_ENTRY_SIZE, TABLE_ENTRY_TITLE_SIZE, TABLE_TEXT_COLOR,
    TABLE_TITLE_BG_COLOR, TABLE_TITLE_SIZE, TABLE_TITLE_TEXT_COLOR, TONAL_TABLE_COL_WIDTH,
};

use super::{AudioRox, IdLang, Message};

use iced::alignment::Horizontal;
// use iced_native::widget::Container;

use iced::theme::{self, Theme};

use iced::widget::{
    column, container, container::Appearance, horizontal_space, row, text, text_input,
    vertical_space, Column, Row,
};

use iced::{Alignment, Element, Length};
