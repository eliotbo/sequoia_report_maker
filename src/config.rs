use iced::{Color, Size};

use iced::widget::canvas;

pub const SPACE: f32 = 1.0;
pub const CORNER_RADIUS: f32 = 15.0;
pub const AXIS_LABEL_COLOR: Color = Color::from_rgb(0.75, 0.75, 0.7);
pub const GRID_COLOR: Color = Color::from_rgb(0.4, 0.4, 0.4);

pub const LEGEND_HEIGHT: f32 = 245.0;
pub const LEGEND_Y_OFFSET_START: f32 = 13.0;

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
