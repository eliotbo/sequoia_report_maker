use iced::{Color, Size};

use iced::theme::{self, Theme};

use iced::widget::canvas;
use iced::widget::container;

pub const SECTION_TITLE_BG_COLOR: Color = Color::from_rgb(0.7, 0.7, 0.7);
pub const SECTION_TITLE_TEXT_COLOR: Color = Color::from_rgb(0.05, 0.05, 0.02);

pub const SECTION_SEPARATOR_SPACE: f32 = 20.0;

pub const TABLE_ENTRY_TITLE_SIZE: f32 = 16.;
pub const TABLE_ENTRY_SIZE: f32 = 12.;
pub const TABLE_TITLE_SIZE: f32 = 18.;
pub const TABLE_MISC_SIZE: f32 = 16.0;
pub const STAP_ENTRY_SIZE: f32 = 8.0;

pub const IMMIT_CANVAS_WIDTH: f32 = 210.0;

pub const DEFAULT_TEXT_INPUT_CONTENT_SIZE: f32 = 12.0;
pub const DEFAULT_TEXT_SIZE: f32 = 16.0;

pub const TABLE_TEXT_COLOR: Color = Color::from_rgb(0.75, 0.75, 0.7);
// pub const TABLE_TEXT_COLOR: Color = Color::from_rgb(0.05, 0.05, 0.02);
pub const TABLE_SPACING: f32 = 4.0;
pub const TABLE_BACKGROUND_COLOR: Color = Color::from_rgb(0.4, 0.4, 0.4);

pub const TABLE_TITLE_BG_COLOR: Color = Color::from_rgb(0.4, 0.4, 0.4);
pub const TABLE_BORDER_COLOR: Color = Color::from_rgb(0.4, 0.4, 0.4);

pub const TABLE_TITLE_TEXT_COLOR: Color = Color::from_rgb(0.8, 0.82, 0.810);
pub const TONAL_TABLE_COL_WIDTH: f32 = 60.0;

pub const SPACE: f32 = 1.0;
pub const CORNER_RADIUS: f32 = 15.0;
pub const AXIS_LABEL_COLOR: Color = Color::from_rgb(0.75, 0.75, 0.7);
pub const GRID_COLOR: Color = Color::from_rgb(0.4, 0.4, 0.4);

pub const RADIO_SPACING: f32 = 3.0;
pub const RADIO_TITLE_SIZE: f32 = 14.0;

pub const LEGEND_HEIGHT: f32 = 283.0;
pub const LEGEND_WIDTH: f32 = 220.0;
pub const LEGEND_Y_OFFSET_START: f32 = 5.0;
pub const LEGEND_BOTTOM_SPACE: f32 = 5.0;

pub const PLOT_CANVAS_HEIGHT: f32 = 427.0;
pub const PLOT_CANVAS_WIDTH: f32 = 460.0;
pub const PLOT_X_OFFSET_START: f32 = 35.0;
pub const PLOT_X_OFFSET_END: f32 = 35.0;
pub const PLOT_Y_OFFSET_START: f32 = 25.0;
pub const PLOT_Y_OFFSET_END: f32 = 75.0;
pub const PLOT_DOT_SIZE: f32 = 5.0;
pub const PLOT_SHAPE_SIZE: f32 = 10.0;
pub const PLOT_TICK_LABEL_SPACE: f32 = 5.0;
pub const PLOT_TICK_SIZE: f32 = 25.0;
pub const PLOT_CA_CO_Y_SPACE: f32 = 20.0;
pub const PLOT_X_AXIS: [isize; 8] = [0, 125, 250, 500, 1000, 2000, 4000, 8000];
pub const PLOT_Y_AXIS: [isize; 14] = [-10, 0, 10, 20, 30, 40, 50, 60, 70, 80, 90, 100, 110, 120];

pub const IM_PLOT_TICK_SIZE: f32 = 28.0;

pub const PLOT_SHAPE_STROKE: canvas::Stroke = canvas::Stroke {
    style: canvas::Style::Solid(Color {
        r: 0.83,
        g: 0.83,
        b: 0.83,
        a: 0.83,
    }),
    width: 1.25,
    line_cap: canvas::LineCap::Round,
    line_join: canvas::LineJoin::Round,
    line_dash: canvas::LineDash {
        segments: &[],
        offset: 0,
    },
};

pub const PLOT_DASH: canvas::LineDash = canvas::LineDash {
    segments: &[8., 5.],
    offset: 11,
};

pub struct TitleContainerCustomStyle;

impl container::StyleSheet for TitleContainerCustomStyle {
    type Style = Theme;

    fn appearance(&self, _style: &Self::Style) -> container::Appearance {
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

    fn appearance(&self, _style: &Self::Style) -> container::Appearance {
        container::Appearance {
            text_color: Some(Color::from_rgb(0.05, 0.05, 0.02)),
            background: Some(Color::from_rgb(0.3, 0.3, 0.3).into()),
            border_radius: 25.0,
            border_width: 0.0,
            border_color: Color::from_rgb(0.5, 0.25, 0.25),
        }
    }
}
