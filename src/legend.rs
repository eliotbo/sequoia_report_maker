use iced::alignment::{Horizontal, Vertical};

use iced::theme::Theme;

use iced::widget::canvas;

use iced::widget::canvas::{Canvas, Cursor, Text};

use iced::{Color, Element, Length, Point, Rectangle, Size, Vector};

use crate::config::{CORNER_RADIUS, LEGEND_HEIGHT, LEGEND_Y_OFFSET_START, SPACE};
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

        let legend_text_color = Color::from_rgb(0.5, 0.5, 0.45);
        let stroke = canvas::Stroke {
            style: canvas::Style::Solid(legend_text_color),
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
        let vs = 2. * ss - 1.0;

        // let legend_rect = Point::new(0.0, config::PLOT_Y_OFFSET_START + config::SPACE);
        // let rectangle = Rectangle::new(Point::ORIGIN, bounds.size());

        // let rect_path = canvas::Path::rectangle(Point::ORIGIN, bounds.size());

        // frame.fill(
        //     &rect_path,
        //     canvas::Fill {
        //         style: canvas::Style::Solid(Color::from_rgb(0.05, 0.05, 0.045)),
        //         rule: canvas::FillRule::NonZero,
        //     },
        // );
        let legend_rect_start = Point::new(0.0, LEGEND_Y_OFFSET_START);
        // let legend_rect_size = Size::new(bounds.width, bounds.height - PLOT_Y_OFFSET_START  );
        let legend_rect_size = Size::new(bounds.width, 273.0);
        // let rectangle = Rectangle::new(legend_rect_start, legend_rect_size);

        let rect_path = canvas::Path::rectangle(legend_rect_start, legend_rect_size);

        frame.fill(
            &rect_path,
            canvas::Fill {
                style: canvas::Style::Solid(Color::from_rgb(0.05, 0.05, 0.045)),
                rule: canvas::FillRule::NonZero,
            },
        );

        // add_contour(
        //     &mut frame,
        //     rectangle,
        //     0.0,
        //     space,
        //     2.0,
        //     Color::from_rgb(1.0, 1.0, 1.0),
        // );

        let center_h = bounds.width / 2.0;

        let mut v = LEGEND_Y_OFFSET_START + 9.0;

        let droit = Text {
            content: "DROITE".to_string(),
            color: legend_text_color,
            size: 16.0,
            position: Point::new(space + 4.0, v),
            horizontal_alignment: Horizontal::Left,
            vertical_alignment: Vertical::Center,
            ..Text::default()
        };
        frame.fill_text(droit.clone());

        let gauche = Text {
            content: "GAUCHE".to_string(),
            color: legend_text_color,
            size: 16.0,
            position: Point::new(bounds.width - space - 4.0, v),
            horizontal_alignment: Horizontal::Right,
            vertical_alignment: Vertical::Center,
            ..Text::default()
        };

        frame.fill_text(gauche.clone());

        v += vs * 0.95;
        let seuil_aerien = Text {
            content: "SEUIL AÉRIEN".to_string(),
            color: legend_text_color,
            size: 16.0,
            position: Point::new(center_h, v),
            horizontal_alignment: Horizontal::Center,
            vertical_alignment: Vertical::Center,
            ..Text::default()
        };

        let rect_path_seuil =
            canvas::Path::rectangle(Point::new(0.0, v - vs / 2.0), Size::new(bounds.width, vs));

        frame.fill(
            &rect_path_seuil,
            canvas::Fill {
                style: canvas::Style::Solid(Color::from_rgb(0.12, 0.12, 0.085)),
                rule: canvas::FillRule::NonZero,
            },
        );

        frame.fill_text(seuil_aerien);

        let legend_text = Text {
            content: "".to_string(),
            color: legend_text_color,
            size: 16.0,
            horizontal_alignment: Horizontal::Center,
            vertical_alignment: Vertical::Center,
            ..Text::default()
        };

        v += vs;
        frame.fill_text(Text {
            content: "Non masqué".to_string(),
            position: Point::new(center_h, v),
            ..legend_text
        });
        frame.stroke(&Shape::circle(Point::new(lx, v), ss), stroke.clone());
        frame.stroke(&Shape::x(Point::new(rx, v), ss), stroke.clone());

        v += vs;
        frame.fill_text(Text {
            content: "Masqué".to_string(),
            position: Point::new(center_h, v),
            ..legend_text
        });
        frame.stroke(&Shape::square(Point::new(rx, v), ss), stroke.clone());
        frame.stroke(&Shape::triangle(Point::new(lx, v), ss), stroke.clone());

        v += vs;
        frame.fill_text(Text {
            content: "Inconfort".to_string(),
            position: Point::new(center_h, v),
            ..legend_text
        });
        frame.stroke(&Shape::u(Point::new(rx, v), ss), stroke.clone());
        frame.stroke(&Shape::u(Point::new(lx, v), ss), stroke.clone());

        v += 1.0 * vs;
        let seuil_osseux = Text {
            content: "SEUIL OSSEUX".to_string(),
            color: legend_text_color,
            size: 16.0,
            position: Point::new(center_h, v),
            horizontal_alignment: Horizontal::Center,
            vertical_alignment: Vertical::Center,
            ..Text::default()
        };
        let rect_path_seuil =
            canvas::Path::rectangle(Point::new(0.0, v - vs / 2.0), Size::new(bounds.width, vs));

        frame.fill(
            &rect_path_seuil,
            canvas::Fill {
                style: canvas::Style::Solid(Color::from_rgb(0.12, 0.12, 0.085)),
                rule: canvas::FillRule::NonZero,
            },
        );
        frame.fill_text(seuil_osseux);

        // // v += vs;
        // droit.position = Point::new(space + 4.0, v);
        // gauche.position = Point::new(bounds.width - space - 4.0, v);
        // frame.fill_text(droit.clone());
        // frame.fill_text(gauche.clone());

        v += vs;
        frame.fill_text(Text {
            content: "Non masqué".to_string(),
            position: Point::new(center_h, v),
            ..legend_text
        });
        frame.stroke(&Shape::less_than(Point::new(lx, v), ss), stroke.clone());
        frame.stroke(&Shape::greater_than(Point::new(rx, v), ss), stroke.clone());

        v += vs;
        frame.fill_text(Text {
            content: "Masqué".to_string(),
            position: Point::new(center_h, v),
            ..legend_text
        });
        frame.stroke(&Shape::left_bracket(Point::new(lx, v), ss), stroke.clone());
        frame.stroke(&Shape::right_bracket(Point::new(rx, v), ss), stroke.clone());

        v += 1.0 * vs;
        let seuil_osseux = Text {
            content: "DIVERS".to_string(),
            color: legend_text_color,
            size: 16.0,
            position: Point::new(center_h, v),
            horizontal_alignment: Horizontal::Center,
            vertical_alignment: Vertical::Center,
            ..Text::default()
        };
        let rect_path_seuil =
            canvas::Path::rectangle(Point::new(0.0, v - vs / 2.0), Size::new(bounds.width, vs));

        frame.fill(
            &rect_path_seuil,
            canvas::Fill {
                style: canvas::Style::Solid(Color::from_rgb(0.12, 0.12, 0.085)),
                rule: canvas::FillRule::NonZero,
            },
        );
        frame.fill_text(seuil_osseux);

        // // v += vs;
        // droit.position = Point::new(space + 4.0, v);
        // gauche.position = Point::new(bounds.width - space - 4.0, v);
        // frame.fill_text(droit);
        // frame.fill_text(gauche);

        v += vs;
        let z = 7.0;
        frame.fill_text(Text {
            content: "Pas de réponse".to_string(),
            position: Point::new(center_h, v),
            ..legend_text
        });
        frame.stroke(
            &Shape::bottom_left_arrow(Point::new(lx + 0.5 * ss, v - 0.4 * ss - z), ss),
            stroke.clone(),
        );
        frame.stroke(
            &&Shape::bottom_right_arrow(Point::new(rx - 0.5 * ss, v - 0.4 * ss - z), ss),
            stroke.clone(),
        );

        v += vs;
        frame.fill_text(Text {
            content: "Vibrotactile".to_string(),
            position: Point::new(center_h, v),
            ..legend_text
        });

        let dxy = Vector::new(ss * 0.7, -ss - 4.0);
        frame.stroke(
            &Shape::vt(Point::new(lx - 0.0, v) - dxy, ss),
            stroke.clone(),
        );
        frame.stroke(
            &&Shape::vt(Point::new(rx + 0.3, v) - dxy, ss),
            stroke.clone(),
        );

        v += vs;
        let oy = Vector::new(ss * 0.7, -ss);
        frame.fill_text(Text {
            content: "Non-valide".to_string(),
            position: Point::new(center_h, v),
            ..legend_text
        });
        frame.stroke(&Shape::asterisk(Point::new(lx, v) - oy, ss), stroke.clone());
        frame.stroke(&Shape::asterisk(Point::new(rx, v) - oy, ss), stroke.clone());

        // frame.stroke(
        //     &Shape::asterisk(Point::new(lx - 2.0, v) - dxy, ss),
        //     stroke.clone(),
        // );
        // frame.stroke(
        //     &&Shape::asterisk(Point::new(rx + 2.3, v) - dxy, ss),
        //     stroke.clone(),
        // );

        // v += vs;
        // println!("v = {}", v);

        vec![frame.into_geometry()]
    }
}

pub fn draw_legend() -> Element<'static, Message> {
    // let plotter = Plot::new(data);
    let legend = Legend::default();
    // Element::new(Plot::new(data))
    let can = Canvas::new(legend)
        .width(Length::Fill)
        .height(Length::Fixed(LEGEND_HEIGHT));

    let element = Element::new(can);
    element
}
