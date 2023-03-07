use iced::Color;

use iced::widget::canvas;

pub const SPACE: f32 = 1.0;
pub const CORNER_RADIUS: f32 = 15.0;
pub const AXIS_LABEL_COLOR: Color = Color::from_rgb(0.75, 0.75, 0.7);
pub const GRID_COLOR: Color = Color::from_rgb(0.5, 0.5, 0.5);
pub const PLOT_X_OFFSET: f32 = 35.0;
pub const PLOT_Y_OFFSET_START: f32 = 25.0;
pub const PLOT_Y_OFFSET_END: f32 = 25.0;
pub const PLOT_DOT_SIZE: f32 = 5.0;
pub const PLOT_SHAPE_SIZE: f32 = 10.0;

pub const PLOT_SHAPE_STROKE: canvas::Stroke = canvas::Stroke {
    style: canvas::Style::Solid(Color {
        r: 0.95,
        g: 0.95,
        b: 0.95,
        a: 0.95,
    }),
    width: 2.0,
    line_cap: canvas::LineCap::Round,
    line_join: canvas::LineJoin::Round,
    line_dash: canvas::LineDash {
        segments: &[],
        offset: 0,
    },
};

pub const PLOT_DASH_STROKE: canvas::Stroke = canvas::Stroke {
    style: canvas::Style::Solid(Color {
        r: 0.85,
        g: 0.85,
        b: 0.85,
        a: 0.85,
    }),
    width: 2.0,
    line_cap: canvas::LineCap::Round,
    line_join: canvas::LineJoin::Round,
    line_dash: canvas::LineDash {
        segments: &[8., 5.],
        offset: 11,
    },
};
