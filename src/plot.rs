use iced::alignment::{Horizontal, Vertical};

use iced::theme::Theme;

use iced::widget::{canvas, container};

use iced::widget::canvas::path::{Arc, Builder};
use iced::widget::canvas::{Cache, Canvas, Cursor, Path, Text};

use iced::{Color, Element, Length, Point, Rectangle, Size, Vector};

use crate::config::{
    self, CORNER_RADIUS, PLOT_CANVAS_HEIGHT, PLOT_CANVAS_WIDTH, PLOT_CA_CO_Y_SPACE, PLOT_DASH,
    PLOT_DOT_SIZE, PLOT_SHAPE_SIZE, PLOT_SHAPE_STROKE, PLOT_TICK_LABEL_SPACE, PLOT_TICK_SIZE,
    PLOT_X_AXIS, PLOT_X_OFFSET_END, PLOT_X_OFFSET_START, PLOT_Y_AXIS, PLOT_Y_OFFSET_END,
    PLOT_Y_OFFSET_START, SPACE,
};
use crate::Message;

const NUM_X_TICKS: usize = 7;
const NUM_Y_TICKS: usize = 14;

#[derive(Debug, Clone, Copy)]
pub enum EarSide {
    Right,
    Left,
    Free,
}

enum Conduction {
    Bone,
    Air,
}

pub struct Plot {
    data: Vec<f32>,
    ear_side: EarSide,
    shape: Shape,
    space: f32,

    top_left: Point,
    size: Size,

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
            top_left: Point::new(0.0, 0.0),
            size: Size::new(0.0, 0.0),
        }
    }
}

impl Plot {
    pub fn new(data: Vec<f32>, shape: Shape, ear_side: EarSide) -> Self {
        let space = SPACE;

        let mut first_x = PLOT_X_OFFSET_START + space;
        let plot_width = (PLOT_X_AXIS.len()) as f32 * PLOT_TICK_SIZE * 2.0;
        let mut last_x = first_x + plot_width - PLOT_TICK_SIZE * 0.6;

        let y_size = (PLOT_Y_AXIS.len() - 1) as f32 * PLOT_TICK_SIZE;

        if let EarSide::Left = ear_side {
            first_x = space;
            last_x = first_x + plot_width - PLOT_TICK_SIZE;
        }

        let top_left = Point::new(first_x, PLOT_Y_OFFSET_START + space);
        let size = Size::new(last_x - first_x, y_size);

        Plot {
            data,
            shape,
            ear_side,
            top_left,
            size,
            space,
            ..Default::default()
        }
    }

    fn get_conduction(&self) -> Conduction {
        match self.shape {
            Shape::LeftBracket | Shape::RightBracket | Shape::Greater | Shape::Less => {
                Conduction::Bone
            }
            _ => Conduction::Air,
        }
    }

    fn plot_data(&self, frame: &mut canvas::Frame, side: &EarSide) {
        let space = self.space;
        let x_offset = PLOT_X_OFFSET_START + space;
        let y_offset = PLOT_Y_OFFSET_START + space;

        let plot_width = frame.width() - PLOT_X_OFFSET_START * 2.0;
        let plot_height = frame.height() - y_offset * 2.0;

        let x_unit = PLOT_TICK_SIZE; //plot_width / NUM_X_TICKS as f32;
        let y_unit = PLOT_TICK_SIZE; //plot_height / 12.0;

        // println!("shape is {:?}", self.shape);
        let line_dash = if let Conduction::Bone = self.get_conduction() {
            PLOT_DASH
        } else {
            canvas::LineDash::default()
        };

        let mut builder = Builder::new();

        // Draw lines between the points
        for i in 0..(self.data.len() - 1) {
            let x1 = 2.0 * (i + 1) as f32 * x_unit + self.top_left.x;
            let y1 = self.data[i] / 10.0 * y_unit + self.top_left.y;

            let x2 = 2.0 * (i + 2) as f32 * x_unit + self.top_left.x;
            let y2 = self.data[i + 1] / 10.0 * y_unit + self.top_left.y;

            let point1 = Point::new(x1, y1);
            let point2 = Point::new(x2, y2);

            builder.move_to(point1);
            builder.line_to(point2);
        }

        // draw line (either dashed or not)
        frame.stroke(
            &builder.build(),
            canvas::Stroke {
                line_dash,
                ..PLOT_SHAPE_STROKE
            },
        );

        // Draw points
        for (i, value) in self.data.iter().enumerate() {
            let x = 2.0 * (i + 1) as f32 * x_unit + self.top_left.x;
            let y = value / 10.0 * y_unit + self.top_left.y;
            frame.stroke(
                // &Shape::triangle(Point::new(x, y), PLOT_SHAPE_SIZE),
                &self.shape.draw_shape(Point::new(x, y), PLOT_SHAPE_SIZE),
                PLOT_SHAPE_STROKE,
            );
            // frame.fill(
            //     &Shape::circle(Point::new(x, y), PLOT_DOT_SIZE),
            //     Color::from_rgb8(200, 0, 0),
            // );
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
        // let y_offset1 = PLOT_Y_OFFSET_END;

        let y_axis = [-10, 0, 10, 20, 30, 40, 50, 60, 70, 80, 90, 100, 110, 120];
        let x_axis = [0, 125, 250, 500, 1000, 2000, 4000, 8000];

        let x_offset = PLOT_X_OFFSET_START;

        // if let EarSide::Right = self.ear_side {
        //     PLOT_X_OFFSET_START
        // } else {
        //     PLOT_X_OFFSET_START
        // };

        let plot_width = (x_axis.len()) as f32 * PLOT_TICK_SIZE * 2.0;
        let plot_height = PLOT_Y_OFFSET_START + space + (y_axis.len() - 1) as f32 * PLOT_TICK_SIZE;

        let y_unit = PLOT_TICK_SIZE;
        let x_unit = PLOT_TICK_SIZE;

        let legend_text = Text {
            color: config::AXIS_LABEL_COLOR,
            size: 16.0,
            ..Text::default()
        };

        let mut y = 0.;
        let y_stroke = canvas::Stroke {
            style: canvas::Style::Solid(config::GRID_COLOR),
            width: 1.0,
            line_cap: canvas::LineCap::Round,
            line_join: canvas::LineJoin::Round,
            ..canvas::Stroke::default()
        };

        let mut first_x = x_offset + space;
        let mut last_x = first_x + plot_width - x_unit * 0.6;

        let mut db_x_position = first_x - PLOT_TICK_LABEL_SPACE + 10.0;
        let mut db_halign = Horizontal::Right;

        let mut hz_x_position = last_x;
        let mut hz_halign = Horizontal::Right;

        let mut y_tick_x_pos = first_x - PLOT_TICK_LABEL_SPACE;
        let mut y_tick_h_align = Horizontal::Right;

        // let mut ca_co_x_position = last_x;
        let y1 = plot_height + PLOT_CA_CO_Y_SPACE;
        let mut ca_upper_left = Point::new(first_x, y1);
        let mut ca_h_align = Horizontal::Right;

        let mut ca_label_x = first_x - PLOT_TICK_LABEL_SPACE;

        if let EarSide::Left = self.ear_side {
            first_x = space;
            last_x = first_x + plot_width - x_unit;

            y_tick_x_pos = last_x + PLOT_TICK_LABEL_SPACE;
            y_tick_h_align = Horizontal::Left;

            db_x_position = last_x + PLOT_TICK_LABEL_SPACE;
            db_halign = Horizontal::Left;

            hz_x_position = space;
            hz_halign = Horizontal::Left;

            ca_upper_left = Point::new(first_x, y1);
            ca_h_align = Horizontal::Left;
            ca_label_x = last_x + PLOT_TICK_LABEL_SPACE - x_unit * 0.5;
        };

        frame.fill_text(Text {
            content: "dB HL".to_string(),
            horizontal_alignment: db_halign,
            vertical_alignment: Vertical::Bottom,
            position: Point::new(db_x_position, space + y_offset0 - 10.0),
            ..legend_text
        });

        frame.fill_text(Text {
            content: "Hz".to_string(),
            horizontal_alignment: hz_halign,
            vertical_alignment: Vertical::Bottom,
            // position: Point::new(space + 15.0, (num_y_ticks as f32) * y_unit + 0.0 + y_offset),
            position: Point::new(
                hz_x_position,
                space + PLOT_Y_OFFSET_START - PLOT_TICK_LABEL_SPACE,
            ),
            ..legend_text
        });

        // add grid to the plot frame
        for y_usize in 0..NUM_Y_TICKS {
            y = y_unit * y_usize as f32 + y_offset0 + space;

            frame.stroke(
                &Path::new(|p| {
                    p.move_to(Point::new(first_x, y));
                    p.line_to(Point::new(last_x, y));
                }),
                y_stroke.clone(),
            );

            let units = format!("{}", y_axis[y_usize]);

            frame.fill_text(Text {
                content: units,
                horizontal_alignment: y_tick_h_align,
                vertical_alignment: Vertical::Center,
                position: Point::new(y_tick_x_pos, y),
                ..legend_text
            });

            // if let EarSide::Right = self.ear_side {

            // } else {
            //     frame.fill_text(Text {
            //         content: units,
            //         horizontal_alignment: Horizontal::Left,
            //         vertical_alignment: Vertical::Center,
            //         position: Point::new(space + x_offset + plot_width + 3.0, y),
            //         ..legend_text
            //     });
            // };
        }

        // frame.stroke(
        //     &Path::new(|p| {
        //         p.move_to(Point::new(space + x_offset, y));
        //         p.line_to(Point::new(bounds.width - space, bounds.height - space));
        //     }),
        //     y_stroke,
        // );

        let x_stroke = canvas::Stroke {
            style: canvas::Style::Solid(config::GRID_COLOR),
            width: 1.0,
            line_cap: canvas::LineCap::Round,
            line_join: canvas::LineJoin::Round,
            ..canvas::Stroke::default()
        };

        for x_usize in 0..8 {
            let x = x_unit * 2.0 * x_usize as f32 + first_x;

            frame.stroke(
                &Path::new(|p| {
                    p.move_to(Point::new(x, y_offset0 + space));
                    p.line_to(Point::new(x, plot_height));
                }),
                x_stroke.clone(),
            );
            let content = format!("{}", x_axis[x_usize] as f32 / 1.0);

            if x_usize == 0 {
                continue;
            }

            frame.fill_text(Text {
                content,
                horizontal_alignment: Horizontal::Center,
                vertical_alignment: Vertical::Bottom,
                // position: Point::new(x + 2.0, bounds.height - 5.0),
                position: Point::new(x, space - PLOT_TICK_LABEL_SPACE + PLOT_Y_OFFSET_START),
                ..legend_text
            });
        }

        // last vertical stroke
        frame.stroke(
            &Path::new(|p| {
                p.move_to(Point::new(last_x, y_offset0 + space));
                p.line_to(Point::new(last_x, plot_height));
            }),
            x_stroke.clone(),
        );

        for x_usize in 2..NUM_X_TICKS {
            let x = x_unit * 2.0 * (x_usize as f32 + 0.5) + first_x;
            // let y1 = plot_height + y_offset0 + space;

            frame.stroke(
                &Path::new(|p| {
                    p.move_to(Point::new(x, space + y_offset0));
                    p.line_to(Point::new(x, plot_height));
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
            )
        }

        //////////////////////////////// bottom CA CO table //////////////////////////////////////
        // y = y_unit * NUM_X_TICKS as f32 + y_offset0 + space;

        // let max_x_units = 13;
        // let max_x = x_unit * 0.5 * (max_x_units as f32 + 1.0) + x_offset;

        let max_x = x_unit * ((2 * NUM_X_TICKS) as f32 + 0.5);
        let size = Size::new(max_x, PLOT_TICK_SIZE * 2.0);

        let caco_stroke = canvas::Stroke {
            style: canvas::Style::Solid(config::GRID_COLOR),
            width: 1.0,
            line_cap: canvas::LineCap::Round,
            line_join: canvas::LineJoin::Round,
            ..Default::default()
        };
        frame.stroke(
            &Path::new(|p| {
                p.rectangle(ca_upper_left, size);
            }),
            caco_stroke.clone(),
        );

        frame.fill(
            &Path::new(|p| {
                p.rectangle(ca_upper_left, Size::new(x_unit * 3.5, size.height));
            }),
            config::GRID_COLOR,
        );

        frame.stroke(
            &Path::new(|p| {
                p.move_to(ca_upper_left + Vector::new(0.0, size.height / 2.0));
                p.line_to(ca_upper_left + Vector::new(size.width, size.height / 2.0));
            }),
            caco_stroke.clone(),
        );

        for x in 1..(2 * NUM_X_TICKS + 1) {
            let x = x_unit * (x as f32 + 0.5) + ca_upper_left.x;
            // let y1 = plot_height + y_offset0 + space;

            frame.stroke(
                &Path::new(|p| {
                    p.move_to(Point::new(x, ca_upper_left.y));
                    p.line_to(Point::new(x, ca_upper_left.y + size.height));
                }),
                canvas::Stroke {
                    style: canvas::Style::Solid(config::GRID_COLOR),
                    width: 1.0,
                    line_cap: canvas::LineCap::Round,
                    line_join: canvas::LineJoin::Round,
                    ..Default::default()
                },
            )
        }

        frame.fill(
            &Path::new(|p| {
                p.rectangle(
                    ca_upper_left + Vector::new(x_unit * 4.5, 0.0),
                    Size::new(x_unit, size.height),
                );
            }),
            config::GRID_COLOR,
        );

        frame.fill(
            &Path::new(|p| {
                p.rectangle(
                    ca_upper_left + Vector::new(x_unit * 12.5, size.height / 2.0),
                    Size::new(x_unit * 2.0, size.height / 2.0),
                );
            }),
            config::GRID_COLOR,
        );

        frame.fill_text(Text {
            content: "CA".to_string(),
            horizontal_alignment: ca_h_align,
            vertical_alignment: Vertical::Center,
            position: Point::new(ca_label_x, y1 + size.height / 4.0),
            ..legend_text
        });

        frame.fill_text(Text {
            content: "CO".to_string(),
            horizontal_alignment: ca_h_align,
            vertical_alignment: Vertical::Center,
            position: Point::new(ca_label_x, y1 + 3.0 * size.height / 4.0),
            ..legend_text
        });

        //////////////////////////////// bottom CA CO table //////////////////////////////////////

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
        frame.stroke(&Shape::vt(Point::new(x, y), ss), stroke.clone());
        // draw a small bottom right arrow as a data point example
        frame.stroke(
            &Shape::bottom_right_arrow(Point::new(x, y), ss),
            stroke.clone(),
        );

        // draw asterisk as a data point example
        x = x + 25.0;
        frame.stroke(&Shape::asterisk(Point::new(x, y), ss), stroke.clone());
        frame.fill(&Shape::circle(Point::new(x, y), ds), dot_color);

        self.plot_data(&mut frame, &self.ear_side);

        // draw the A symbol as a data point example
        x = x + 25.0;
        frame.stroke(&Shape::a(Point::new(x, y), ss), stroke.clone());
        frame.fill(&Shape::circle(Point::new(x, y), ds * 0.5), dot_color);

        // draw the Z symbol as a data point example
        x = x + 25.0;
        frame.stroke(&Shape::z(Point::new(x, y), ss), stroke.clone());
        frame.fill(&Shape::circle(Point::new(x, y), ds * 0.5), dot_color);

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

pub fn plot<'a>(data: Vec<f32>, shape: Shape, ear_side: EarSide) -> Element<'a, Message> {
    let plotter = Plot::new(data, shape, ear_side);
    // Element::new(Plot::new(data))
    let can = Canvas::new(plotter)
        // .width(Length::Fill)
        .width(Length::Fixed(PLOT_CANVAS_WIDTH))
        .height(Length::Fixed(PLOT_CANVAS_HEIGHT));

    let element = Element::new(can);
    // let element = container(can).into();
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
            let s = size * 0.7;
            let arror_len = s * 1.2;
            let tail_len = 0.55 * s;
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
            let s = size * 0.7;
            let arror_len = s * 1.2;
            let tail_len = 0.55 * s;
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
            let s = size * 0.5;
            let oy = Vector::new(size * 0.7, -size);
            let pos = pos + oy;
            let xt = s / 3.0;

            p.move_to(pos + Vector::new(-s / 2.0, 0.0));
            p.line_to(pos + Vector::new(0.0, -s));
            p.move_to(pos + Vector::new(-s / 2.0, 0.0));
            p.line_to(pos + Vector::new(-s, -s));

            p.move_to(pos + Vector::new(s / 2.0 + xt, 0.0));

            p.line_to(pos + Vector::new(s / 2.0 + xt, -s));
            p.move_to(pos + Vector::new(xt, -s));
            p.line_to(pos + Vector::new(s + xt, -s));
        })
    }

    // asterisk
    pub fn asterisk(pos: Point, size: f32) -> Path {
        Path::new(|p| {
            // make the asterisk shape with 3 lines
            let s = size * 0.35;
            let oy = Vector::new(size * 0.7, -size);
            let pos = pos + oy;

            let theta = 30.0_f32.to_radians();
            let x = Vector::new(0., s);
            let y = Vector::new(s, 0.);
            p.move_to(pos + x * theta.cos() + y * theta.sin());
            p.line_to(pos - x * theta.cos() - y * theta.sin());

            p.move_to(pos - x * theta.cos() + y * theta.sin());
            p.line_to(pos + x * theta.cos() - y * theta.sin());

            p.move_to(pos + y);
            p.line_to(pos - y);
        })
    }

    // the symbol for the letter A
    pub fn a(pos: Point, size: f32) -> Path {
        Path::new(|p| {
            let s = size * 0.5;
            // let oy = Vector::new(size * 0.7, -size);
            let oy = Vector::new(0., 0.);
            let pos = pos + oy;
            let a = 0.75;

            p.move_to(pos + Vector::new(-s * a, s));
            p.line_to(pos + Vector::new(0.0, -s));
            p.line_to(pos + Vector::new(s * a, s));

            p.move_to(pos + Vector::new(-s * a / 2.0, s * 0.2));
            p.line_to(pos + Vector::new(s * a / 2.0, s * 0.2));
            // p.line_to(pos + Vector::new(-s, 0.0));
            // p.move_to(pos + Vector::new(0.0, -s));
            // p.line_to(pos + Vector::new(0.0, -s * 2.0));
        })
    }

    // the symbol for the letter Z
    pub fn z(pos: Point, size: f32) -> Path {
        Path::new(|p| {
            let s = size * 0.5;
            // let oy = Vector::new(size * 0.7, -size);
            let oy = Vector::new(0., 0.);
            let pos = pos + oy;
            let a = 0.75;

            p.move_to(pos + Vector::new(-s * a, s));
            p.line_to(pos + Vector::new(s * a, s));
            p.line_to(pos + Vector::new(-s * a, -s));
            p.line_to(pos + Vector::new(s * a, -s));
        })
    }
}
