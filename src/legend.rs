use iced::alignment::{Horizontal, Vertical};
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
use crate::plot::{add_contour, Shape};
use crate::Message;

pub struct Legend {
    space: f32,
    corner_radius: f32,
}

impl Default for Legend {
    fn default() -> Self {
        Self {
            space: SPACE,
            corner_radius: CORNER_RADIUS,
        }
    }
}

#[derive(Default)]
struct LegendShape {
    theme: Theme,
}

pub struct LegendState;

impl canvas::Program<Message> for Legend {
    type State = ();

    fn draw(
        &self,
        _interaction: &(),
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

        // shape size
        let ss = 10.0;

        // right side
        let rx = bounds.width - space - 2.0 * ss;

        // left side
        let lx = space + 2.0 * ss;

        // vertical space
        let vs = 2. * ss + 1.0;

        // draw small square as a data point example

        let center_h = bounds.width / 2.0;
        let audiogram_text = Text {
            content: "AUDIOGRAMME".to_string(),
            color: Color::WHITE,
            size: 30.0,
            position: Point::new(center_h, space),
            horizontal_alignment: Horizontal::Center,
            vertical_alignment: Vertical::Top,
            ..Text::default()
        };

        frame.fill_text(audiogram_text);

        let v = 2.5 * vs;
        let seuil_aerien = Text {
            content: "Seuil aérien".to_string(),
            color: Color::WHITE,
            size: 26.0,
            position: Point::new(center_h, v),
            horizontal_alignment: Horizontal::Center,
            vertical_alignment: Vertical::Center,
            ..Text::default()
        };

        frame.fill_text(seuil_aerien);

        let legend_text = Text {
            content: "".to_string(),
            color: Color::WHITE,
            size: 20.0,
            horizontal_alignment: Horizontal::Center,
            vertical_alignment: Vertical::Center,
            ..Text::default()
        };

        let v = 4. * vs;
        frame.fill_text(Text {
            content: "Non masqué".to_string(),
            position: Point::new(center_h, v),
            ..legend_text
        });
        frame.stroke(&Shape::circle(Point::new(lx, v), ss), stroke.clone());
        frame.stroke(&Shape::x(Point::new(rx, v), ss), stroke.clone());

        let v = 5. * vs;
        frame.fill_text(Text {
            content: "Masqué".to_string(),
            position: Point::new(center_h, v),
            ..legend_text
        });
        frame.stroke(&Shape::square(Point::new(rx, v), ss), stroke.clone());
        frame.stroke(&Shape::triangle(Point::new(lx, v), ss), stroke.clone());

        let v = 6. * vs;
        frame.fill_text(Text {
            content: "Inconfort".to_string(),
            position: Point::new(center_h, v),
            ..legend_text
        });
        frame.fill_text(Text {
            content: "U".to_string(),
            position: Point::new(lx + 0., v),
            ..legend_text
        });
        frame.fill_text(Text {
            content: "U".to_string(),
            position: Point::new(rx + 0.3, v),
            ..legend_text
        });

        let v = 7.5 * vs;
        let seuil_osseux = Text {
            content: "Seuil osseux".to_string(),
            color: Color::WHITE,
            size: 26.0,
            position: Point::new(center_h, v),
            horizontal_alignment: Horizontal::Center,
            vertical_alignment: Vertical::Center,
            ..Text::default()
        };
        frame.fill_text(seuil_osseux);

        let v = 9.0 * vs;
        frame.fill_text(Text {
            content: "Non masqué".to_string(),
            position: Point::new(center_h, v),
            ..legend_text
        });
        frame.stroke(&Shape::less_than(Point::new(lx, v), ss), stroke.clone());
        frame.stroke(&Shape::greater_than(Point::new(rx, v), ss), stroke.clone());

        let v = 10.0 * vs;
        frame.fill_text(Text {
            content: "Masqué".to_string(),
            position: Point::new(center_h, v),
            ..legend_text
        });
        frame.stroke(
            &Shape::left_bracket(Point::new(lx, v + 0.6 * ss), ss),
            stroke.clone(),
        );
        frame.stroke(
            &Shape::right_bracket(Point::new(rx, v + 0.6 * ss), ss),
            stroke.clone(),
        );
        add_contour(&mut frame, bounds, 0.0, space, 2.0, Color::WHITE);

        vec![frame.into_geometry()]
    }
}

pub fn draw_legend(width: Length, height: Length) -> Element<'static, Message> {
    // let plotter = Plot::new(data);
    let legend = Legend::default();
    // Element::new(Plot::new(data))
    let can = Canvas::new(legend).width(width).height(height);

    let element = Element::new(can);
    element
}
