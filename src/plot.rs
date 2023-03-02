use iced::alignment::Horizontal;
use iced::executor;
use iced::theme::{self, Theme};
use iced::time;
use iced::widget::canvas;
use iced::widget::canvas::event::{self, Event};
use iced::widget::canvas::path::{Arc, Builder};
use iced::widget::canvas::{Cache, Canvas, Cursor, Frame, Geometry, Path, Stroke, Text};
use iced::widget::{button, checkbox, column, container, pick_list, row, slider, text};
use iced::window;
use iced::{
    Alignment, Application, Color, Command, Element, Length, Point, Rectangle, Settings, Size,
    Subscription, Vector,
};

use crate::config::{CORNER_RADIUS, SPACE};
use crate::Message;

pub struct Plot {
    data: Vec<f32>,
    max_value: f32,
    min_value: f32,
    space: f32,
    corner_radius: f32,
    plot_cache: Cache,
}

impl Default for Plot {
    fn default() -> Self {
        let data = vec![];
        Self {
            data,
            max_value: 0.0,
            min_value: 0.0,
            space: SPACE,
            corner_radius: CORNER_RADIUS,
            plot_cache: Cache::default(),
        }
    }
}

impl Plot {
    pub fn new(data: Vec<f32>) -> Self {
        let max_value = data.iter().cloned().fold(0.0, f32::max);
        let min_value = data.iter().cloned().fold(0.0, f32::min);

        Plot {
            data,
            max_value,
            min_value,
            ..Default::default()
        }
    }

    fn plot_data(&self, frame: &mut canvas::Frame) {
        let plot_width = frame.width() - 20.0;
        let plot_height = frame.height() - 20.0;

        let mut builder = Builder::new();

        // Draw lines between the points
        for i in 0..(self.data.len() - 1) {
            let x1 = i as f32 / self.data.len() as f32 * plot_width + 10.0;
            let y1 = plot_height - self.data[i] / self.max_value * plot_height + 10.0;
            let x2 = (i + 1) as f32 / self.data.len() as f32 * plot_width + 10.0;
            let y2 = plot_height - self.data[i + 1] / self.max_value * plot_height + 10.0;

            let point1 = Point::new(x1, y1);
            let point2 = Point::new(x2, y2);

            builder.move_to(point1);
            builder.line_to(point2);
        }
        // draw dashed line
        frame.stroke(
            &builder.build(),
            canvas::Stroke {
                // color: Color::BLACK,
                style: canvas::Style::Solid(Color::BLACK),
                width: 5.0,
                line_cap: canvas::LineCap::Round,
                line_join: canvas::LineJoin::Round,
                line_dash: canvas::LineDash {
                    segments: &[20., 20.],
                    offset: 11,
                },
                ..canvas::Stroke::default()
            },
        );

        // Draw points
        for (i, value) in self.data.iter().enumerate() {
            let x = i as f32 / self.data.len() as f32 * plot_width + 10.0;
            let y = plot_height - value / self.max_value * plot_height + 10.0;

            frame.fill(
                &Path::new(|p| {
                    p.circle(Point::new(x, y), 4.0);
                }),
                Color::from_rgb8(200, 0, 0),
            );
        }
    }
}

#[derive(Default)]
struct PlotPlot {
    theme: Theme,
}

#[derive(Debug, Clone)]
pub enum Interaction {
    None,
    Drawing,
    Erasing,
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
        let radius = self.corner_radius;
        let sr = space + radius;

        let upper_left = Point::new(space, space);
        let size = Size::new(bounds.width - 2. * space, bounds.height - 2. * space);

        frame.fill(
            &Path::new(|p| {
                p.rectangle(upper_left, size);
            }),
            // Color::from_rgb(0.05, 0.3, 0.23),
            Color::TRANSPARENT,
        );
        // let stroke = Stroke {
        //     style: canvas::Style::Solid(Color::WHITE),
        //     width: 2.0,
        //     ..canvas::Stroke::default()
        // };

        let stroke = canvas::Stroke {
            style: canvas::Style::Solid(Color::WHITE),
            width: 2.0,
            line_cap: canvas::LineCap::Round,
            line_join: canvas::LineJoin::Round,
            ..canvas::Stroke::default()
        };

        let dot_color = Color::from_rgb8(200, 0, 0);

        // draw small square as a data point example
        frame.stroke(
            &Shape::square(Point::new(100.0, 100.0), 10.0),
            stroke.clone(),
        );
        frame.fill(&Shape::circle(Point::new(100.0, 100.0), 5.0), dot_color);

        // draw greater than symbol as a data point example
        frame.stroke(
            &Shape::greater_than(Point::new(200.0, 100.0), 10.0),
            stroke.clone(),
        );
        frame.fill(&Shape::circle(Point::new(200.0, 100.0), 5.0), dot_color);

        // draw less than symbol as a data point example
        frame.stroke(
            &Shape::less_than(Point::new(250.0, 150.0), 10.0),
            stroke.clone(),
        );
        frame.fill(&Shape::circle(Point::new(250.0, 150.0), 5.0), dot_color);

        // draw small circle as a data point example
        frame.stroke(
            &Shape::circle(Point::new(150.0, 150.0), 6.0),
            stroke.clone(),
        );
        frame.fill(&Shape::circle(Point::new(150.0, 150.0), 5.0), dot_color);

        // draw small triangle as a data point example
        frame.stroke(
            &Shape::triangle(Point::new(200.0, 200.0), 12.0),
            stroke.clone(),
        );
        frame.fill(&Shape::circle(Point::new(200.0, 200.0), 5.0), dot_color);

        // draw a small arrow as a data point example
        frame.stroke(
            &Shape::arrow(Point::new(250.0, 250.0), 10.0),
            stroke.clone(),
        );
        frame.fill(&Shape::circle(Point::new(250.0, 250.0), 5.0), dot_color);

        // draw a small bracket ([) as a data point example
        frame.stroke(
            &Shape::left_bracket(Point::new(300.0, 300.0), 10.0),
            stroke.clone(),
        );
        frame.fill(&Shape::circle(Point::new(300.0, 300.0), 5.0), dot_color);

        // draw the corresponding closing bracket (]) as a data point example
        frame.stroke(
            &Shape::right_bracket(Point::new(350.0, 350.0), 10.0),
            stroke.clone(),
        );
        frame.fill(&Shape::circle(Point::new(350.0, 350.0), 5.0), dot_color);

        // draw an x as a data point example
        frame.stroke(&Shape::x(Point::new(400.0, 400.0), 10.0), stroke.clone());
        frame.fill(&Shape::circle(Point::new(400.0, 400.0), 5.0), dot_color);

        add_contour(&mut frame, bounds, radius, space, 2.0, Color::WHITE);

        self.plot_data(&mut frame);

        vec![frame.into_geometry()]
    }
}

pub fn add_contour(
    frame: &mut canvas::Frame,
    bounds: Rectangle,
    radius: f32,
    space: f32,
    width: f32,
    color: Color,
) {
    let sr = space + radius;

    // draw a perimeter around the plot canvas with curved corners
    if radius > 0.0 {
        frame.stroke(
            &Path::new(|p| {
                p.arc(Arc {
                    center: Point::new(sr, sr),
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
                    center: Point::new(bounds.width - sr, sr),
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
                    center: Point::new(bounds.width - sr, bounds.height - sr),
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
                    center: Point::new(sr, bounds.height - sr),
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
            p.move_to(Point::new(sr, space));
            p.line_to(Point::new(bounds.width - sr, space));
        }),
        canvas::Stroke {
            style: canvas::Style::Solid(color),
            width,
            ..canvas::Stroke::default()
        },
    );

    frame.stroke(
        &Path::new(|p| {
            p.move_to(Point::new(bounds.width - space, sr));
            p.line_to(Point::new(bounds.width - space, bounds.height - sr));
        }),
        canvas::Stroke {
            style: canvas::Style::Solid(color),
            width,
            ..canvas::Stroke::default()
        },
    );

    frame.stroke(
        &Path::new(|p| {
            p.move_to(Point::new(bounds.width - sr, bounds.height - space));
            p.line_to(Point::new(sr, bounds.height - space));
        }),
        canvas::Stroke {
            style: canvas::Style::Solid(color),
            width,
            ..canvas::Stroke::default()
        },
    );

    frame.stroke(
        &Path::new(|p| {
            p.move_to(Point::new(space, bounds.height - sr));
            p.line_to(Point::new(space, sr));
        }),
        canvas::Stroke {
            style: canvas::Style::Solid(color),
            width,
            ..canvas::Stroke::default()
        },
    );
}

pub fn plot<'a>(data: Vec<f32>, width: Length, height: Length) -> Element<'a, Message> {
    let plotter = Plot::new(data);
    // Element::new(Plot::new(data))
    let can = Canvas::new(plotter).width(width).height(height);

    let element = Element::new(can);
    element
}

pub struct Shape;

impl Shape {
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
            let radius = size / 2.0;
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

    // arrow
    pub fn arrow(pos: Point, size: f32) -> Path {
        Path::new(|p| {
            let arror_len = size;
            let tail_len = 0.4 * size;
            p.move_to(pos);
            p.line_to(pos + Vector::new(0.0, arror_len));
            p.line_to(pos + Vector::new(tail_len, arror_len - tail_len * 0.9));
            // p.line_to(pos + Vector::new(0.0, 10.0));
            p.move_to(pos + Vector::new(0.0, arror_len));
            p.line_to(pos + Vector::new(-tail_len, arror_len - tail_len));
        })
    }

    // left bracket
    pub fn left_bracket(pos: Point, size: f32) -> Path {
        Path::new(|p| {
            let s = size * 0.4;
            let v = size * 1.2;
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
}
