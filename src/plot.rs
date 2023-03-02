use iced::alignment::Horizontal;
use iced::executor;
use iced::theme::{self, Theme};
use iced::time;
use iced::widget::canvas;
use iced::widget::canvas::event::{self, Event};
use iced::widget::canvas::path::{Arc, Builder};
use iced::widget::canvas::{Cache, Canvas, Cursor, Frame, Geometry, Path, Text};
use iced::widget::{button, checkbox, column, container, pick_list, row, slider, text};
use iced::window;
use iced::{
    Alignment, Application, Color, Command, Element, Length, Point, Rectangle, Settings, Size,
    Subscription, Vector,
};

use crate::Message;

pub struct Plot {
    data: Vec<f32>,
    max_value: f32,
    space: f32,
    corner_radius: f32,
    plot_cache: Cache,
}

impl Plot {
    pub fn new(data: Vec<f32>) -> Self {
        let max_value = data.iter().cloned().fold(0.0, f32::max);
        let default_space = 5.0;
        let default_corner_radius = 15.0;
        Plot {
            data,
            max_value,
            space: default_space,
            corner_radius: default_corner_radius,
            plot_cache: Cache::default(),
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
                &canvas::Path::new(|p| {
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
            &canvas::Path::new(|p| {
                p.rectangle(upper_left, size);
            }),
            // Color::from_rgb(0.05, 0.3, 0.23),
            Color::TRANSPARENT,
        );

        // draw a perimeter around the plot canvas with curved corners
        frame.stroke(
            &canvas::Path::new(|p| {
                p.arc(Arc {
                    center: Point::new(sr, sr),
                    radius,
                    start_angle: std::f32::consts::PI,
                    end_angle: std::f32::consts::FRAC_PI_2 * 3.0,
                });
            }),
            canvas::Stroke {
                style: canvas::Style::Solid(Color::WHITE),
                width: 3.0,
                ..canvas::Stroke::default()
            },
        );

        frame.stroke(
            &canvas::Path::new(|p| {
                p.arc(Arc {
                    center: Point::new(bounds.width - sr, sr),
                    radius,
                    start_angle: std::f32::consts::PI * 3. / 2.,
                    end_angle: std::f32::consts::PI * 2.0,
                });
            }),
            canvas::Stroke {
                style: canvas::Style::Solid(Color::WHITE),
                width: 3.0,
                ..canvas::Stroke::default()
            },
        );

        frame.stroke(
            &canvas::Path::new(|p| {
                p.arc(Arc {
                    center: Point::new(bounds.width - sr, bounds.height - sr),
                    radius,
                    start_angle: 0.0,
                    end_angle: std::f32::consts::FRAC_PI_2,
                });
            }),
            canvas::Stroke {
                style: canvas::Style::Solid(Color::WHITE),
                width: 3.0,
                ..canvas::Stroke::default()
            },
        );

        frame.stroke(
            &canvas::Path::new(|p| {
                p.arc(Arc {
                    center: Point::new(sr, bounds.height - sr),
                    radius,
                    start_angle: std::f32::consts::FRAC_PI_2,
                    end_angle: std::f32::consts::PI,
                });
            }),
            canvas::Stroke {
                style: canvas::Style::Solid(Color::WHITE),
                width: 3.0,
                ..canvas::Stroke::default()
            },
        );

        frame.stroke(
            &canvas::Path::new(|p| {
                p.move_to(Point::new(sr, space));
                p.line_to(Point::new(bounds.width - sr, space));
            }),
            canvas::Stroke {
                style: canvas::Style::Solid(Color::WHITE),
                width: 3.0,
                ..canvas::Stroke::default()
            },
        );

        frame.stroke(
            &canvas::Path::new(|p| {
                p.move_to(Point::new(bounds.width - space, sr));
                p.line_to(Point::new(bounds.width - space, bounds.height - sr));
            }),
            canvas::Stroke {
                style: canvas::Style::Solid(Color::WHITE),
                width: 3.0,
                ..canvas::Stroke::default()
            },
        );

        frame.stroke(
            &canvas::Path::new(|p| {
                p.move_to(Point::new(bounds.width - sr, bounds.height - space));
                p.line_to(Point::new(sr, bounds.height - space));
            }),
            canvas::Stroke {
                style: canvas::Style::Solid(Color::WHITE),
                width: 3.0,
                ..canvas::Stroke::default()
            },
        );

        frame.stroke(
            &canvas::Path::new(|p| {
                p.move_to(Point::new(space, bounds.height - sr));
                p.line_to(Point::new(space, sr));
            }),
            canvas::Stroke {
                style: canvas::Style::Solid(Color::WHITE),
                width: 3.0,
                ..canvas::Stroke::default()
            },
        );

        self.plot_data(&mut frame);

        vec![frame.into_geometry()]
    }
}

pub fn plot<'a>(data: Vec<f32>, width: Length, height: Length) -> Element<'a, Message> {
    let plotter = Plot::new(data);
    // Element::new(Plot::new(data))
    let can = Canvas::new(plotter).width(width).height(height);

    let element = Element::new(can);
    element
}
