use iced::alignment::{Horizontal, Vertical};

use iced::theme::Theme;

use iced::widget::canvas;

use iced::widget::canvas::path::{Arc, Builder};
use iced::widget::canvas::{Cache, Canvas, Cursor, Path, Text};

use iced::{Color, Element, Length, Point, Rectangle, Size, Vector};

use crate::config::{
    self, CORNER_RADIUS, PLOT_DASH_STROKE, PLOT_DOT_SIZE, PLOT_SHAPE_SIZE, PLOT_SHAPE_STROKE,
    PLOT_X_OFFSET, PLOT_Y_OFFSET_END, PLOT_Y_OFFSET_START, SPACE,
};
use crate::Message;

pub enum EarSide {
    Right,
    Left,
}

pub struct Plot {
    data: Vec<f32>,
    ear_side: EarSide,
    shape: Shape,
    space: f32,
    corner_radius: f32,
    plot_cache: Cache,
}

impl Default for Plot {
    fn default() -> Self {
        let data = vec![];
        Self {
            data,
            ear_side: EarSide::Right,
            shape: Shape::None,
            space: SPACE,
            corner_radius: CORNER_RADIUS,
            plot_cache: Cache::default(),
        }
    }
}

impl Plot {
    pub fn new(data: Vec<f32>, shape: Shape) -> Self {
        Plot {
            data,
            shape,
            ..Default::default()
        }
    }

    fn plot_data(&self, frame: &mut canvas::Frame, side: &EarSide) {
        let space = self.space;
        let x_offset = PLOT_X_OFFSET + space;
        let y_offset = PLOT_Y_OFFSET_START + space;

        let plot_width = frame.width() - x_offset * 2.0;
        let plot_height = frame.height() - y_offset * 2.0;
        let x_unit = plot_width / 6.0;
        let y_unit = plot_height / 12.0;

        let mut builder = Builder::new();

        // Draw lines between the points
        for i in 0..(self.data.len() - 1) {
            let x1 = (i + 1) as f32 * x_unit + x_offset;
            let y1 = self.data[i] / 10.0 * y_unit + y_offset;

            let x2 = (i + 2) as f32 * x_unit + x_offset;
            let y2 = self.data[i + 1] / 10.0 * y_unit + y_offset;

            let point1 = Point::new(x1, y1);
            let point2 = Point::new(x2, y2);

            builder.move_to(point1);
            builder.line_to(point2);
        }

        // draw dashed line
        frame.stroke(&builder.build(), PLOT_DASH_STROKE);

        // Draw points
        for (i, value) in self.data.iter().enumerate() {
            let x = (i + 1) as f32 * x_unit + x_offset;
            let y = value / 10.0 * y_unit + y_offset;
            frame.stroke(
                // &Shape::triangle(Point::new(x, y), PLOT_SHAPE_SIZE),
                &self.shape.draw_shape(Point::new(x, y), PLOT_SHAPE_SIZE),
                PLOT_SHAPE_STROKE,
            );
            frame.fill(
                &Shape::circle(Point::new(x, y), PLOT_DOT_SIZE),
                Color::from_rgb8(200, 0, 0),
            );
        }
    }
}

#[derive(Debug, Clone)]
pub enum Interaction {
    None,
}

impl Default for Interaction {
    fn default() -> Self {
        Interaction::None
    }
}

impl canvas::Program<Message> for Plot {
    type State = Interaction;

    fn draw(
        &self,
        _interaction: &Interaction,
        _theme: &Theme,
        bounds: Rectangle,
        _cursor: Cursor,
    ) -> Vec<canvas::Geometry> {
        let mut frame = canvas::Frame::new(bounds.size());

        let space = self.space;

        let y_offset0 = PLOT_Y_OFFSET_START;
        let y_offset1 = PLOT_Y_OFFSET_END;
        let x_offset = PLOT_X_OFFSET;
        let plot_width = frame.width() - x_offset * 2.0 - space * 2.0;
        let plot_height = frame.height() - y_offset0 - y_offset1 - space * 2.0;

        let space = self.space;
        let radius = self.corner_radius;
        // let sr = space + radius;

        let upper_left = Point::new(space, space);
        let size = Size::new(bounds.width - 2. * space, bounds.height - 2. * space);

        let num_y_ticks = 13;
        let y_unit = plot_height / (num_y_ticks as f32 - 1.0);
        let x_unit = plot_width / 6.0;
        frame.fill(
            &Path::new(|p| {
                p.rectangle(upper_left, size);
            }),
            // Color::from_rgb(0.05, 0.3, 0.23),
            Color::TRANSPARENT,
        );

        let legend_text = Text {
            color: config::AXIS_LABEL_COLOR,
            size: 16.0,
            ..Text::default()
        };

        let y_axis = [0, 10, 20, 30, 40, 50, 60, 70, 80, 90, 100, 110, 120];
        let x_axis = [0, 250, 500, 1000, 2000, 4000, 8000];

        let mut y = 0.;
        let y_stroke = canvas::Stroke {
            style: canvas::Style::Solid(config::GRID_COLOR),
            width: 1.0,
            line_cap: canvas::LineCap::Round,
            line_join: canvas::LineJoin::Round,
            ..canvas::Stroke::default()
        };
        // add grid to the plot frame
        for y_usize in 0..num_y_ticks {
            y = y_unit * y_usize as f32 + y_offset0 + space;

            frame.stroke(
                &Path::new(|p| {
                    p.move_to(Point::new(space + x_offset, y));
                    p.line_to(Point::new(bounds.width - space, y));
                }),
                y_stroke.clone(),
            );

            let units = format!("{}", y_axis[y_usize]);

            frame.fill_text(Text {
                content: units,
                horizontal_alignment: Horizontal::Right,
                vertical_alignment: Vertical::Center,
                position: Point::new(space + x_offset - 5.0, y),
                ..legend_text
            });
        }

        // frame.stroke(
        //     &Path::new(|p| {
        //         p.move_to(Point::new(space + x_offset, y));
        //         p.line_to(Point::new(bounds.width - space, bounds.height - space));
        //     }),
        //     y_stroke,
        // );

        frame.fill_text(Text {
            content: "dB HL".to_string(),
            horizontal_alignment: Horizontal::Left,
            vertical_alignment: Vertical::Top,
            position: Point::new(space, space + 2.0),
            ..legend_text
        });

        let x_stroke = canvas::Stroke {
            style: canvas::Style::Solid(config::GRID_COLOR),
            width: 1.0,
            line_cap: canvas::LineCap::Round,
            line_join: canvas::LineJoin::Round,
            ..canvas::Stroke::default()
        };

        frame.fill_text(Text {
            content: "Hz".to_string(),
            horizontal_alignment: Horizontal::Right,
            vertical_alignment: Vertical::Top,
            // position: Point::new(space + 15.0, (num_y_ticks as f32) * y_unit + 0.0 + y_offset),
            position: Point::new(frame.width() - space, space + 2.0),
            ..legend_text
        });

        for x_usize in 0..7 {
            let x = x_unit * x_usize as f32 + PLOT_X_OFFSET;

            frame.stroke(
                &Path::new(|p| {
                    p.move_to(Point::new(x, y_offset0 + space));
                    p.line_to(Point::new(x, plot_height + y_offset0 + space));
                }),
                x_stroke.clone(),
            );
            let content = format!("{}", x_axis[x_usize] as f32 / 1.0);

            // let units = if x_usize == 1 {
            //     format!("{} Hz", x_axis[x_usize - 1])
            // } else {
            //     format!("{}", x_axis[x_usize - 1])
            // };

            if x_usize == 0 {
                continue;
            }

            frame.fill_text(Text {
                content,
                horizontal_alignment: Horizontal::Center,
                vertical_alignment: Vertical::Top,
                // position: Point::new(x + 2.0, bounds.height - 5.0),
                position: Point::new(x, space + 2.0),
                ..legend_text
            });
        }

        frame.stroke(
            &Path::new(|p| {
                p.move_to(Point::new(frame.width() - space, y_offset0 + space));
                p.line_to(Point::new(
                    frame.width() - space,
                    plot_height + y_offset0 + space,
                ));
            }),
            x_stroke.clone(),
        );

        for x_usize in 2..6 {
            // let x = plot_width * (x_usize as f32 + 0.5) / 7.0 + PLOT_X_OFFSET;
            let x = x_unit * (x_usize as f32 + 0.5) + PLOT_X_OFFSET;
            let y1 = plot_height + y_offset0 + space;

            frame.stroke(
                &Path::new(|p| {
                    p.move_to(Point::new(x, space + y_offset0));
                    p.line_to(Point::new(x, y1));
                }),
                canvas::Stroke {
                    style: canvas::Style::Solid(config::GRID_COLOR),
                    width: 1.0,
                    line_cap: canvas::LineCap::Round,
                    line_join: canvas::LineJoin::Round,
                    line_dash: canvas::LineDash {
                        segments: &[3., 3.],
                        offset: 0,
                    },
                },
            );
        }

        let stroke = PLOT_SHAPE_STROKE;

        let dot_color = Color::from_rgb8(200, 0, 0);
        let ss = PLOT_SHAPE_SIZE; // shape size
        let ds = PLOT_DOT_SIZE;

        let y = 350.0;
        let mut x = 50.0;

        // draw small square as a data point example
        x = x + 25.0;
        frame.stroke(&Shape::circle(Point::new(x, y), ss), stroke.clone());
        frame.fill(&Shape::circle(Point::new(x, y), ds), dot_color);

        // draw an x as a data point example

        x = x + 25.0;
        frame.stroke(&Shape::x(Point::new(x, y), ss), stroke.clone());
        frame.fill(&Shape::circle(Point::new(x, y), ds), dot_color);

        // draw small triangle as a data point example
        x = x + 25.0;
        frame.stroke(&Shape::triangle(Point::new(x, y), 12.0), stroke.clone());
        frame.fill(&Shape::circle(Point::new(x, y), ds), dot_color);
        frame.stroke(
            &Shape::bottom_left_arrow(Point::new(x, y), ss),
            stroke.clone(),
        );

        // draw small circle as a data point example
        x = x + 25.0;
        frame.stroke(&Shape::square(Point::new(x, y), ss), stroke.clone());
        frame.fill(&Shape::circle(Point::new(x, y), ds), dot_color);

        // draw U  symbol as a data point example
        x = x + 25.0;
        frame.stroke(&Shape::u(Point::new(x, y), ss), stroke.clone());
        frame.fill(&Shape::circle(Point::new(x, y), ds), dot_color);

        // draw greater than symbol as a data point example
        x = x + 25.0;
        frame.stroke(&Shape::greater_than(Point::new(x, y), ss), stroke.clone());
        frame.fill(&Shape::circle(Point::new(x, y), ds), dot_color);

        // draw less than symbol as a data point example
        x = x + 25.0;
        frame.stroke(&Shape::less_than(Point::new(x, y), ss), stroke.clone());
        frame.fill(&Shape::circle(Point::new(x, y), ds), dot_color);

        // draw a small bracket ([) as a data point example
        x = x + 25.0;
        frame.stroke(&Shape::left_bracket(Point::new(x, y), ss), stroke.clone());
        frame.fill(&Shape::circle(Point::new(x, y), ds), dot_color);
        // draw a small bottom left arrow as a data point example
        frame.stroke(
            &Shape::bottom_left_arrow(Point::new(x, y), ss),
            stroke.clone(),
        );

        // draw the corresponding closing bracket (]) as a data point example
        x = x + 25.0;
        frame.stroke(&Shape::right_bracket(Point::new(x, y), ss), stroke.clone());
        frame.fill(&Shape::circle(Point::new(x, y), ds), dot_color);
        // draw a small bottom right arrow as a data point example
        frame.stroke(
            &Shape::bottom_right_arrow(Point::new(x, y), ss),
            stroke.clone(),
        );

        // draw VT as a data point example
        x = x + 25.0;
        frame.stroke(&Shape::vt(Point::new(x, y), ss), stroke.clone());
        frame.fill(&Shape::circle(Point::new(x, y), ds), dot_color);

        self.plot_data(&mut frame, &self.ear_side);

        // add_contour(&mut frame, rectangle, radius, space, 2.0, Color::WHITE);

        vec![frame.into_geometry()]
    }
}

pub fn add_contour(
    frame: &mut canvas::Frame,
    rect: Rectangle,
    radius: f32,
    space: f32,
    width: f32,
    color: Color,
) {
    let top_left = Point::new(rect.x, rect.y);
    let bottom_right = Point::new(rect.x + rect.width, rect.y + rect.height);
    let top_right = Point::new(bottom_right.x, top_left.y);
    let bottom_left = Point::new(top_left.x, bottom_right.y);

    let sr = space + radius;

    // draw a perimeter around the plot canvas with curved corners
    if radius > 0.0 {
        frame.stroke(
            &Path::new(|p| {
                p.arc(Arc {
                    center: top_left + Vector::new(sr, sr),
                    radius,
                    start_angle: std::f32::consts::PI,
                    end_angle: std::f32::consts::FRAC_PI_2 * 3.0,
                });
            }),
            canvas::Stroke {
                style: canvas::Style::Solid(color),
                width,
                ..canvas::Stroke::default()
            },
        );

        frame.stroke(
            &Path::new(|p| {
                p.arc(Arc {
                    center: top_right + Vector::new(-sr, sr),
                    radius,
                    start_angle: std::f32::consts::PI * 3. / 2.,
                    end_angle: std::f32::consts::PI * 2.0,
                });
            }),
            canvas::Stroke {
                style: canvas::Style::Solid(color),
                width,
                ..canvas::Stroke::default()
            },
        );

        frame.stroke(
            &Path::new(|p| {
                p.arc(Arc {
                    center: bottom_right + Vector::new(-sr, -sr),
                    radius,
                    start_angle: 0.0,
                    end_angle: std::f32::consts::FRAC_PI_2,
                });
            }),
            canvas::Stroke {
                style: canvas::Style::Solid(color),
                width,
                ..canvas::Stroke::default()
            },
        );

        frame.stroke(
            &Path::new(|p| {
                p.arc(Arc {
                    center: bottom_left + Vector::new(sr, -sr),
                    radius,
                    start_angle: std::f32::consts::FRAC_PI_2,
                    end_angle: std::f32::consts::PI,
                });
            }),
            canvas::Stroke {
                style: canvas::Style::Solid(color),
                width,
                ..canvas::Stroke::default()
            },
        );
    }

    frame.stroke(
        &Path::new(|p| {
            p.move_to(top_left + Vector::new(sr, space));
            p.line_to(top_right + Vector::new(-sr, space));
        }),
        canvas::Stroke {
            style: canvas::Style::Solid(color),
            width,
            ..canvas::Stroke::default()
        },
    );

    frame.stroke(
        &Path::new(|p| {
            p.move_to(top_right + Vector::new(-space, sr));
            p.line_to(bottom_right + Vector::new(-space, -sr));
        }),
        canvas::Stroke {
            style: canvas::Style::Solid(color),
            width,
            ..canvas::Stroke::default()
        },
    );

    frame.stroke(
        &Path::new(|p| {
            p.move_to(bottom_right + Vector::new(-sr, -space));
            p.line_to(bottom_left + Vector::new(sr, -space));
        }),
        canvas::Stroke {
            style: canvas::Style::Solid(color),
            width,
            ..canvas::Stroke::default()
        },
    );

    frame.stroke(
        &Path::new(|p| {
            p.move_to(bottom_left + Vector::new(space, -sr));
            p.line_to(top_left + Vector::new(space, sr));
        }),
        canvas::Stroke {
            style: canvas::Style::Solid(color),
            width,
            ..canvas::Stroke::default()
        },
    );
}

pub fn plot<'a>(data: Vec<f32>, shape: Shape) -> Element<'a, Message> {
    let plotter = Plot::new(data, shape);
    // Element::new(Plot::new(data))
    let can = Canvas::new(plotter)
        .width(Length::Fill)
        .height(Length::Fill);

    let element = Element::new(can);
    element
}

#[derive(Debug, Clone, Copy)]
pub enum Shape {
    None,
    Circle,
    Square,
    Triangle,
    U,
    X,
    VT,
    Greater,
    Less,
    RightArrow,
    LeftArrow,
    LeftBracket,
    RightBracket,
}

impl Shape {
    // method that takes a string and returns the
    fn draw_shape(&self, position: Point, size: f32) -> Path {
        match self {
            Shape::Triangle => Shape::triangle(position, size),
            Shape::Circle => Shape::circle(position, size),
            Shape::Square => Shape::square(position, size),
            Shape::U => Shape::u(position, size),
            Shape::X => Shape::x(position, size),
            Shape::VT => Shape::vt(position, size),
            Shape::Greater => Shape::greater_than(position, size),
            Shape::Less => Shape::less_than(position, size),
            Shape::RightArrow => Shape::bottom_right_arrow(position, size),
            Shape::LeftArrow => Shape::bottom_left_arrow(position, size),
            Shape::LeftBracket => Shape::left_bracket(position, size),
            Shape::RightBracket => Shape::right_bracket(position, size),

            _ => Path::new(|_| {}),
        }
    }

    pub fn square(position: Point, size: f32) -> Path {
        Path::new(|p| {
            let s = size / 2.0;
            p.move_to(position + Vector::new(-s, -s));
            p.line_to(position + Vector::new(-s, s));
            p.line_to(position + Vector::new(s, s));
            p.line_to(position + Vector::new(s, -s));
            p.close();
        })
    }

    pub fn circle(position: Point, size: f32) -> Path {
        Path::new(|p| {
            let radius = size * 0.5;
            p.circle(position, radius);
        })
    }

    //triangle
    pub fn triangle(position: Point, size: f32) -> Path {
        Path::new(|p| {
            let s = size * 0.6;
            p.move_to(position + Vector::new(0., -s * 0.9));
            p.line_to(position + Vector::new(s, s * 0.9));
            p.line_to(position + Vector::new(-s, s * 0.9));
            p.close();
        })
    }

    // bottom left arrow
    pub fn bottom_left_arrow(pos: Point, size: f32) -> Path {
        Path::new(|p| {
            let arror_len = size * 1.2;
            let tail_len = 0.55 * size;
            let oy = Vector::new(0., size * 0.6);
            p.move_to(pos + oy);
            p.line_to(pos + Vector::new(-arror_len, arror_len) + oy);
            p.line_to(pos + Vector::new(-arror_len + tail_len, arror_len) + oy);
            p.move_to(pos + Vector::new(-arror_len, arror_len) + oy);
            p.line_to(pos + Vector::new(-arror_len, arror_len - tail_len) + oy);
        })
    }

    // botton right arrow
    pub fn bottom_right_arrow(pos: Point, size: f32) -> Path {
        Path::new(|p| {
            let arror_len = size * 1.2;
            let tail_len = 0.55 * size;
            let oy = Vector::new(0., size * 0.6);
            p.move_to(pos + oy);
            p.line_to(pos + Vector::new(arror_len, arror_len) + oy);
            p.line_to(pos + Vector::new(arror_len - tail_len, arror_len) + oy);
            p.move_to(pos + Vector::new(arror_len, arror_len) + oy);
            p.line_to(pos + Vector::new(arror_len, arror_len - tail_len) + oy);
        })
    }

    // left bracket
    pub fn left_bracket(pos: Point, size: f32) -> Path {
        Path::new(|p| {
            let s = size * 0.4;
            let v = size * 1.2;
            // let oy = Vector::new(0., v / 2.0);
            let pos = pos + Vector::new(0., v / 2.0);
            p.move_to(pos);
            p.line_to(pos + Vector::new(-s, 0.0));
            p.line_to(pos + Vector::new(-s, -v));
            p.line_to(pos + Vector::new(0.0, -v));
            // p.close();
        })
    }

    // right bracket
    pub fn right_bracket(pos: Point, size: f32) -> Path {
        Path::new(|p| {
            let s = size * 0.4;
            let v = size * 1.2;
            // let oy = Vector::new(0., v / 2.0);
            let pos = pos + Vector::new(0., v / 2.0);
            p.move_to(pos);
            p.line_to(pos + Vector::new(s, 0.0));
            p.line_to(pos + Vector::new(s, -v));
            p.line_to(pos + Vector::new(0.0, -v));
            // p.close();
        })
    }

    // x
    pub fn x(pos: Point, size: f32) -> Path {
        Path::new(|p| {
            let s = size * 0.5;
            p.move_to(pos + Vector::new(s, -s));
            p.line_to(pos - Vector::new(s, -s));
            p.move_to(pos + Vector::new(-s, -s));
            p.line_to(pos - Vector::new(-s, -s));
        })
    }

    // greater_than
    pub fn greater_than(pos: Point, size: f32) -> Path {
        Path::new(|p| {
            let s = size * 0.5;
            p.move_to(pos + Vector::new(-s, -s));
            p.line_to(pos + Vector::new(s, 0.0));
            p.line_to(pos + Vector::new(-s, s));
        })
    }

    // less than
    pub fn less_than(pos: Point, size: f32) -> Path {
        Path::new(|p| {
            let s = size * 0.5;
            p.move_to(pos + Vector::new(s, -s));
            p.line_to(pos + Vector::new(-s, 0.0));
            p.line_to(pos + Vector::new(s, s));
        })
    }

    // U shape
    pub fn u(pos: Point, size: f32) -> Path {
        Path::new(|p| {
            let radius = size * 0.5;
            p.arc(Arc {
                center: pos,
                radius,
                start_angle: 0.0,
                end_angle: std::f32::consts::PI,
            });
            p.line_to(pos + Vector::new(-radius, -size * 0.65));
            p.move_to(pos + Vector::new(radius, 0.0));
            p.line_to(pos + Vector::new(radius, -size * 0.65));
        })
    }

    // VT
    pub fn vt(pos: Point, size: f32) -> Path {
        Path::new(|p| {
            let s = size;
            let oy = Vector::new(0., s / 2.0);
            let pos = pos + oy;

            p.move_to(pos + Vector::new(-s / 2.0, 0.0));
            p.line_to(pos + Vector::new(0.0, -s));
            p.move_to(pos + Vector::new(-s / 2.0, 0.0));
            p.line_to(pos + Vector::new(-s, -s));

            p.move_to(pos + Vector::new(s / 2.0, 0.0));

            p.line_to(pos + Vector::new(s / 2.0, -s));
            p.move_to(pos + Vector::new(0.0, -s));
            p.line_to(pos + Vector::new(s, -s));
        })
    }
}
