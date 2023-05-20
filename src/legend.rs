use iced::alignment::{Horizontal, Vertical};

use iced::theme::Theme;

use iced::widget::canvas;

use iced::widget::canvas::{Canvas, Cursor, Text};

use iced::{Color, Element, Length, Point, Rectangle, Size, Vector};

use crate::config::{
    CORNER_RADIUS, GRAY, LEGEND_BORDER_COLOR, LEGEND_HEIGHT, LEGEND_SYMBOL_STROKE_COLOR,
    LEGEND_TEXT_COLOR, LEGEND_TITLES_COLOR, LEGEND_WIDTH, LEGEND_Y_OFFSET_START, SPACE,
};
use crate::plot::{add_contour, Shape};
use crate::Message;

pub struct Legend {
    space: f32,
    corner_radius: f32,
    icon_positions: LegendLRPositions,
}

impl Default for Legend {
    fn default() -> Self {
        Self {
            space: SPACE,
            corner_radius: CORNER_RADIUS,
            icon_positions: LegendLRPositions::default(),
        }
    }
}

pub enum Side {
    Left,
    Right,
}


pub enum LegendIcon {
    ANonMasque,
    AMasque,
    OMasque,
    ONonMasque,
    Inconfort,
    ChampLibre,
    AA,
    PasDeReponse,
    Vibrotactile,
    Insufficient,
}


#[derive( Debug)]
pub struct LegendLRPositions {
    l: LegendPos,
    r: LegendPos,
}

impl Default for LegendLRPositions {
    fn default() -> Self {

        let xl = 21.0;
        let xr = 197.0;

        let y0 = 46.05;
        let y1 = 160.05;
        let y2 = 250.55;
        let vs = 19.;

        // v += vs / 2.0;
        // let z = 7.0;
        // let zz = Point::new(xl + 0.5 * 10., v - 0.4 * 10. - z);
        LegendLRPositions { 
            l: LegendPos { 
                sa_not_masked: Point { x: xl, y: y0 }, 
                sa_masked: Point { x: xl, y: y0 + vs }, 
                sa_discomfort: Point { x: xl, y: y0 + 2. * vs }, // y: 84.05 }, 
                sa_champs_libre: Point { x: xl, y: y0 + 3. * vs }, //y: 103.05 }, 
                sa_aa: Point { x: xl, y: y0 + 4. * vs }, // y: 122.05 }, 
                so_not_masked: Point { x: xl, y: y1 }, // y: 160.05 }, 
                so_masked: Point { x: xl, y: y1 + vs }, // y: 179.05 }, 
                
                other_no_response: Point { x: xl, y: y1 + 2. * vs }, // y: 207.55 }, 
                other_no_vibro: Point { x: xl, y: y1 + 3. * vs }, //y: 226.55 }, 
                other_insufficient: Point { x: xl, y: y2 }, // y: y1 + 2. * vs }, // y: 250.55 } 
            }, 
  
            r: LegendPos { 
                sa_not_masked: Point { x: xr, y: y0 }, 
                sa_masked: Point { x: xr, y: y0 + vs }, 
                sa_discomfort: Point { x: xr, y: y0 + 2.*vs }, 
                sa_champs_libre: Point { x: xr, y: y0 + 3.*vs }, 
                sa_aa: Point { x: xr, y: y0 + 4. * vs }, 
                so_masked: Point { x: xr, y: y1}, 
                so_not_masked: Point { x: xr, y: y1 + vs }, 
                other_no_response: Point { x: xr, y: y1 + 2. * vs }, 
                other_no_vibro: Point { x: xr, y: y1 + 3. * vs }, 
                other_insufficient: Point { x: xr, y: y2 } } 
                
        }
        //     r: LegendPos { 
        //         sa_not_masked: Point { x: xr, y: 46.05 }, 
        //         sa_masked: Point { x: xr, y: 65.05 }, 
        //         sa_discomfort: Point { x: xr, y: 84.05 }, 
        //         sa_champs_libre: Point { x: xr, y: 103.05 }, 
        //         sa_aa: Point { x: xr, y: 122.05 }, 
        //         so_masked: Point { x: xr, y: 179.05 }, 
        //         so_not_masked: Point { x: xr, y: 160.05 }, 
        //         other_no_response: Point { x: xr, y: 207.55 }, 
        //         other_no_vibro: Point { x: xr, y: 226.55 }, 
        //         other_insufficient: Point { x: xr, y: 250.55 } } 
                
        // }

    }
}

#[derive(Default, Debug)]
pub struct LegendPos {
    sa_not_masked: Point,
    sa_masked: Point,
    sa_discomfort: Point,
    sa_champs_libre: Point,
    sa_aa: Point, 
    so_masked: Point,
    so_not_masked: Point,
    other_no_response: Point,
    other_no_vibro: Point,
    other_insufficient: Point,
}

 impl LegendLRPositions {
    pub fn get_icon_under_cursor(&self, cursor: Point) -> Option<bool> {
     
        let mut lv = vec![
            (self.l.sa_not_masked, Side::Left, LegendIcon::ANonMasque),
            (self.l.sa_masked, Side::Left,  LegendIcon::AMasque),
            (self.l.sa_discomfort, Side::Left,  LegendIcon::Inconfort),
            (self.l.sa_champs_libre, Side::Left,  LegendIcon::ChampLibre),
            (self.l.sa_aa, Side::Left,  LegendIcon::AA),
            (self.l.so_masked, Side::Left,  LegendIcon::OMasque),
            (self.l.so_not_masked, Side::Left,  LegendIcon::ONonMasque),
            (self.l.other_insufficient, Side::Left,  LegendIcon::Insufficient),
            (self.l.other_no_response, Side::Left,  LegendIcon::PasDeReponse),
            (self.l.other_no_vibro, Side::Left,  LegendIcon::Vibrotactile),
        ];

        let mut rv = vec![
            (self.r.sa_not_masked, Side::Right,LegendIcon::ANonMasque),
            (self.r.sa_masked, Side::Right,  LegendIcon::AMasque),
            (self.r.sa_discomfort, Side::Right,  LegendIcon::Inconfort),
            (self.r.sa_champs_libre, Side::Right, LegendIcon::ChampLibre),
            (self.r.sa_aa, Side::Right, LegendIcon::AA),
            (self.r.so_masked, Side::Right, LegendIcon::OMasque),
            (self.r.so_not_masked, Side::Right, LegendIcon::ONonMasque),
            (self.r.other_insufficient, Side::Right, LegendIcon::Insufficient),
            (self.r.other_no_response, Side::Right, LegendIcon::PasDeReponse),
            (self.r.other_no_vibro, Side::Right, LegendIcon::Vibrotactile),
        ];

        lv.append(&mut rv);

        return None;
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

        // let legend_text_color = Color::from_rgb(0.05, 0.05, 0.05);

        let stroke = canvas::Stroke {
            style: canvas::Style::Solid(LEGEND_TEXT_COLOR),
            width: 2.0,
            line_cap: canvas::LineCap::Round,
            line_join: canvas::LineJoin::Round,
            ..canvas::Stroke::default()
        };

        let symbol_stroke = canvas::Stroke {
            style: canvas::Style::Solid(LEGEND_SYMBOL_STROKE_COLOR),
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
        // let legend_rect_start = Point::new(0.0, LEGEND_Y_OFFSET_START);
        // let legend_rect_size = Size::new(bounds.width, bounds.height - PLOT_Y_OFFSET_START  );

        // let rect_path = canvas::Path::rectangle(legend_rect_start, legend_rect_size);

        // frame.fill(
        //     &rect_path,
        //     canvas::Fill {
        //         style: canvas::Style::Solid(Color::from_rgb(0.7, 0.7, 0.7)),
        //         rule: canvas::FillRule::NonZero,
        //     },
        // );
        // here
        // frame.stroke(&Shape::circle(Point::new(lx, v), ss), stroke.clone());

        let center_h = bounds.width / 2.0;

        let mut v = 9.0;

        let droit = Text {
            content: "DROITE".to_string(),
            color: LEGEND_TEXT_COLOR,
            size: 16.0,
            position: Point::new(space + 4.0, v),
            horizontal_alignment: Horizontal::Left,
            vertical_alignment: Vertical::Center,
            ..Text::default()
        };
        frame.fill_text(droit.clone());



        let gauche = Text {
            content: "GAUCHE".to_string(),
            color: LEGEND_TEXT_COLOR,
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
            color: LEGEND_TITLES_COLOR,
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
                style: canvas::Style::Solid(GRAY),
                rule: canvas::FillRule::NonZero,
            },
        );

        frame.fill_text(seuil_aerien);

        let legend_text = Text {
            content: "".to_string(),
            color: LEGEND_TEXT_COLOR,
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

        let mut rl_pos = LegendLRPositions::default();
        

        frame.stroke(&Shape::circle(rl_pos.l.sa_not_masked, ss), symbol_stroke.clone());
        frame.stroke(&Shape::x(rl_pos.r.sa_not_masked, ss), symbol_stroke.clone());
        


        v += vs;
        
        frame.fill_text(Text {
            content: "Masqué".to_string(),
            position: Point::new(center_h, v),
            ..legend_text
        });

        
        frame.stroke(&Shape::square(rl_pos.r.sa_masked, ss), symbol_stroke.clone());
        frame.stroke(
            &Shape::triangle(rl_pos.l.sa_masked, ss),
            symbol_stroke.clone(),
        );


        v += vs;
        frame.fill_text(Text {
            content: "Inconfort".to_string(),
            position: Point::new(center_h, v),
            ..legend_text
        });
        frame.stroke(&Shape::u(rl_pos.r.sa_discomfort, ss), symbol_stroke.clone());
        frame.stroke(&Shape::u(rl_pos.l.sa_discomfort, ss), symbol_stroke.clone());



        v += vs;
        // let oy = Vector::new(ss * 0.7, -ss);
        frame.fill_text(Text {
            content: "Champ libre".to_string(),
            position: Point::new(center_h, v),
            ..legend_text
        });
        frame.stroke(&Shape::s(rl_pos.l.sa_champs_libre, ss), symbol_stroke.clone());
        frame.stroke(&Shape::s(rl_pos.r.sa_champs_libre, ss), symbol_stroke.clone());




        v += vs;
        let oy = Vector::new(ss * 0.7, -ss);
        frame.fill_text(Text {
            content: "Avec appareil auditif".to_string(),
            position: Point::new(center_h, v),
            ..legend_text
        });
        frame.stroke(&Shape::a(rl_pos.l.sa_aa, ss), symbol_stroke.clone());
        frame.stroke(&Shape::a(rl_pos.r.sa_aa, ss), symbol_stroke.clone());



        v += 1.0 * vs;
        let seuil_osseux = Text {
            content: "SEUIL OSSEUX".to_string(),
            color: LEGEND_TITLES_COLOR,
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
                style: canvas::Style::Solid(GRAY),
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
        frame.stroke(
            &Shape::less_than(rl_pos.l.so_not_masked, ss),
            symbol_stroke.clone(),
        );
        frame.stroke(
            &Shape::greater_than(rl_pos.r.so_not_masked, ss),
            symbol_stroke.clone(),
        );




        v += vs;
        frame.fill_text(Text {
            content: "Masqué".to_string(),
            position: Point::new(center_h, v),
            ..legend_text
        });
        frame.stroke(
            &Shape::left_bracket(rl_pos.l.so_masked, ss),
            symbol_stroke.clone(),
        );
        frame.stroke(
            &Shape::right_bracket(rl_pos.r.so_masked, ss),
            symbol_stroke.clone(),
        );


        v += 1.0 * vs;
        let seuil_osseux = Text {
            content: " ".to_string(),
            color: LEGEND_TEXT_COLOR,
            size: 16.0,
            position: Point::new(center_h, v),
            horizontal_alignment: Horizontal::Center,
            vertical_alignment: Vertical::Center,
            ..Text::default()
        };
        let rect_path_seuil = canvas::Path::rectangle(
            Point::new(0.0, v - vs / 2.0),
            Size::new(bounds.width, vs / 2.0),
        );

        frame.fill(
            &rect_path_seuil,
            canvas::Fill {
                style: canvas::Style::Solid(GRAY),
                rule: canvas::FillRule::NonZero,
            },
        );
        frame.fill_text(seuil_osseux);

        // // v += vs;
        // droit.position = Point::new(space + 4.0, v);
        // gauche.position = Point::new(bounds.width - space - 4.0, v);
        // frame.fill_text(droit);
        // frame.fill_text(gauche);

        v += vs / 2.0;
        let z = 7.0;
        let dxy = Vector::new(ss * 0.7, -ss - 4.0);

        frame.fill_text(Text {
            content: "Pas de réponse".to_string(),
            position: Point::new(center_h, v),
            ..legend_text
        });
        frame.stroke(
            &Shape::bottom_left_arrow(Point::new(lx + 0.5 * ss, v - 0.4 * ss - z), ss),
            symbol_stroke.clone(),
        );
        frame.stroke(
            &&Shape::bottom_right_arrow(Point::new(rx - 0.5 * ss, v - 0.4 * ss - z), ss),
            symbol_stroke.clone(),
        );


        v += vs;
        frame.fill_text(Text {
            content: "Vibrotactile".to_string(),
            position: Point::new(center_h, v),
            ..legend_text
        });

        
        frame.stroke(
            &Shape::vt(Point::new(lx - 0.0, v) - dxy, ss),
            symbol_stroke.clone(),
        );
        frame.stroke(
            &&Shape::vt(Point::new(rx + 0.3, v) - dxy, ss),
            symbol_stroke.clone(),
        );


        v += vs + 5.0;
        let oy = Vector::new(ss * 0.7, -ss);
        frame.fill_text(Text {
            content: "Surassourdissement ou\nmasque insuffisant".to_string(),
            position: Point::new(center_h, v),
            ..legend_text
        });
        frame.stroke(
            &Shape::asterisk(Point::new(lx, v) - oy, ss),
            symbol_stroke.clone(),
        );
        frame.stroke(
            &Shape::asterisk(Point::new(rx, v) - oy, ss),
            symbol_stroke.clone(),
        );




        // frame.stroke(
        //     &Shape::asterisk(Point::new(lx - 2.0, v) - dxy, ss),
        //     symbol_stroke.clone(),
        // );
        // frame.stroke(
        //     &&Shape::asterisk(Point::new(rx + 2.3, v) - dxy, ss),
        //     symbol_stroke.clone(),
        // );

        // v += vs;
        // println!("v = {}", v);

        let legend_rect_size = Size::new(bounds.width, bounds.height);
        let rectangle = Rectangle::new(Point::new(0., 0.), legend_rect_size);
        add_contour(&mut frame, rectangle, 6.0, space, 2.0, LEGEND_BORDER_COLOR);

        vec![frame.into_geometry()]
    }
}

pub fn draw_legend() -> Element<'static, Message> {
    // let plotter = Plot::new(data);
    let legend = Legend::default();
    // Element::new(Plot::new(data))
    let can = Canvas::new(legend)
        .width(Length::Fixed(LEGEND_WIDTH))
        .height(Length::Fixed(LEGEND_HEIGHT));

    let element = Element::new(can);
    element
}
