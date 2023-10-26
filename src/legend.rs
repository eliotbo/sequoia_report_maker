use iced::alignment::{Horizontal, Vertical};

use iced::widget::canvas;

use iced::widget::canvas::{Canvas, Path, Text};
use iced::widget::canvas::event::{self, Event};



use iced::{mouse, Theme, Renderer, Element, Length, Point, Rectangle, Size, Vector};

use crate::config::{LEGEND_SELECT_STROKE,
     GRAY, LEGEND_BORDER_COLOR, LEGEND_HEIGHT, LEGEND_SYMBOL_STROKE_COLOR,
    LEGEND_TEXT_COLOR, LEGEND_TITLES_COLOR, LEGEND_WIDTH, SPACE, ICON_SIZE,
    LEGEND_SELECT_MODIFIER_STROKE, self
};
use crate::plot::{add_contour, Shape};
use crate::Message;

pub struct Legend {
    space: f32,
    icon_positions: LegendLRPositions,
}

impl Default for Legend {
    fn default() -> Self {
        Self {
            space: SPACE,
            icon_positions: LegendLRPositions::default(),
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub enum Side {
    LeftShape,
    RightShape,
    LeftModifier,
    RightModifier,
}

#[derive(Copy, Clone, Debug)]
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
        let y2 = 207.05;
        let y3 = 250.55;

        let dx = 5.;
        let dy = ICON_SIZE / 2.;

        let vs = 19.;

        LegendLRPositions { 
            l: LegendPos { 
                sa_not_masked: Point { x: xl, y: y0 }, 
                sa_masked: Point { x: xl, y: y0 + vs }, 
                sa_discomfort: Point { x: xl, y: y0 + 2. * vs }, // y: 84.05 }, 
                sa_champs_libre: Point { x: xl, y: y0 + 3. * vs }, //y: 103.05 }, 
                sa_aa: Point { x: xl, y: y0 + 4. * vs }, // y: 122.05 }, 
                so_not_masked: Point { x: xl, y: y1 }, // y: 160.05 }, 
                so_masked: Point { x: xl, y: y1 + vs }, // y: 179.05 }, 
                
                other_no_response: Point { x: xl + dx, y: y2 - dy}, // y: 207.55 }, 
                other_no_vibro: Point { x: xl, y: y2 + vs }, //y: 226.55 }, 
                other_insufficient: Point { x: xl, y: y3 }, // y: y1 + 2. * vs }, // y: 250.55 } 
            }, 
  
            r: LegendPos { 
                sa_not_masked: Point { x: xr, y: y0 }, 
                sa_masked: Point { x: xr, y: y0 + vs }, 
                sa_discomfort: Point { x: xr, y: y0 + 2.*vs }, 
                sa_champs_libre: Point { x: xr, y: y0 + 3.*vs }, 
                sa_aa: Point { x: xr, y: y0 + 4. * vs }, 
                so_masked: Point { x: xr, y: y1}, 
                so_not_masked: Point { x: xr, y: y1 + vs }, 
                other_no_response: Point { x: xr - dx, y: y2 - dy }, 
                other_no_vibro: Point { x: xr, y: y2 + vs }, 
                other_insufficient: Point { x: xr, y: y3 } } 
                
        }


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
    pub fn get_icon_under_cursor(&self, cursor: Point) -> Option<(Side, LegendIcon, Point)> {
     
        let dv  = Vector::new(ICON_SIZE/2. + 3.0, ICON_SIZE/2. + 2.);
        
        let mut lv = vec![
            (self.l.sa_not_masked, Side::LeftShape, LegendIcon::ANonMasque),
            (self.l.sa_masked, Side::LeftShape,  LegendIcon::AMasque),
            (self.l.sa_discomfort, Side::LeftShape,  LegendIcon::Inconfort),
            (self.l.sa_champs_libre, Side::LeftShape,  LegendIcon::ChampLibre),
            (self.l.sa_aa, Side::LeftShape,  LegendIcon::AA),
            (self.l.so_masked, Side::LeftShape,  LegendIcon::OMasque),
            (self.l.so_not_masked, Side::LeftShape,  LegendIcon::ONonMasque),
            
            (self.l.other_no_response + dv, Side::LeftModifier,  LegendIcon::PasDeReponse),
            (self.l.other_no_vibro, Side::LeftModifier,  LegendIcon::Vibrotactile),
            (self.l.other_insufficient, Side::LeftModifier,  LegendIcon::Insufficient),
        ];

        let dv  = Vector::new(ICON_SIZE/2. + 3.0, ICON_SIZE/2. + 2.);
        
        let mut rv = vec![
            (self.r.sa_not_masked, Side::RightShape,LegendIcon::ANonMasque),
            (self.r.sa_masked, Side::RightShape,  LegendIcon::AMasque),
            (self.r.sa_discomfort, Side::RightShape,  LegendIcon::Inconfort),
            (self.r.sa_champs_libre, Side::RightShape, LegendIcon::ChampLibre),
            (self.r.sa_aa, Side::RightShape, LegendIcon::AA),
            (self.r.so_masked, Side::RightShape, LegendIcon::OMasque),
            (self.r.so_not_masked, Side::RightShape, LegendIcon::ONonMasque),
           
            (self.r.other_no_response + dv, Side::RightModifier, LegendIcon::PasDeReponse),
            (self.r.other_no_vibro, Side::RightModifier, LegendIcon::Vibrotactile),
            (self.r.other_insufficient, Side::RightModifier, LegendIcon::Insufficient),
        ];

        lv.append(&mut rv);

        for (pos, side, icon) in lv.iter() {
            // println!("{:?}", pos);
            let top_left = *pos - Vector::new(ICON_SIZE / 2.0, ICON_SIZE / 2.);

            let start_rect = Rectangle::new(top_left, Size::new(ICON_SIZE, ICON_SIZE));

            if start_rect.contains(cursor) {
                return Some((*side, *icon, top_left))
                
            } 
        }

        return None;
    }

}


pub struct Interaction {
    left_shape: Option<(LegendIcon, Point)>,
    right_shape: Option<(LegendIcon, Point)>,
    left_modifier: Option<(LegendIcon, Point)>,
    right_modifier: Option<(LegendIcon, Point)>,
}

impl Default for Interaction {
    fn default() -> Self {
        Interaction {
            left_shape: None,
            right_shape: None,
            left_modifier: None,
            right_modifier : None,      
        }
    }
}

impl canvas::Program<Message> for Legend {
    type State = Interaction;

    fn draw(
        &self,
        state: &Self::State,
        renderer: &Renderer,

        _theme: &Theme,
        bounds: Rectangle,
        _cursor: mouse::Cursor,
    ) -> Vec<canvas::Geometry> {
        let mut frame = canvas::Frame::new(renderer, bounds.size());

        let space = self.space;

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

        let center_h = bounds.width / 2.0;

        let mut v = 9.0;

        let droit = Text {
            content: "DROITE".to_string(),
            color: LEGEND_TEXT_COLOR,
            size: 14.0,
            position: Point::new(space + 4.0, v),
            horizontal_alignment: Horizontal::Left,
            vertical_alignment: Vertical::Center,
            font: config::DEFAULT_FONT,
            ..Text::default()
        };
        frame.fill_text(droit.clone());



        let gauche = Text {
            content: "GAUCHE".to_string(),
            color: LEGEND_TEXT_COLOR,
            size: 14.0,
            position: Point::new(bounds.width - space - 4.0, v),
            horizontal_alignment: Horizontal::Right,
            vertical_alignment: Vertical::Center,
            font: config::DEFAULT_FONT,
            ..Text::default()
        };

        frame.fill_text(gauche.clone());

        v += vs * 0.95;
        let seuil_aerien = Text {
            content: "SEUIL AÉRIEN".to_string(),
            color: LEGEND_TITLES_COLOR,
            size: 14.0,
            position: Point::new(center_h, v),
            horizontal_alignment: Horizontal::Center,
            vertical_alignment: Vertical::Center,
            font: config::DEFAULT_FONT,
            ..Text::default()
        };

        let rect_path_seuil =
            canvas::Path::rectangle(Point::new(0.0, v - vs / 2.0), Size::new(bounds.width, vs));

        frame.fill(
            &rect_path_seuil,
            canvas::fill::Fill {
                style: canvas::Style::Solid(GRAY),
                rule: canvas::fill::Rule::NonZero,
            },
        );

        frame.fill_text(seuil_aerien);

        let legend_text = Text {
            content: "".to_string(),
            color: LEGEND_TEXT_COLOR,
            size: 14.0,
            horizontal_alignment: Horizontal::Center,
            vertical_alignment: Vertical::Center,
            font: config::DEFAULT_FONT,
            ..Text::default()
        };

        v += vs;
        frame.fill_text(Text {
            content: "Non masqué".to_string(),
            position: Point::new(center_h, v),
            ..legend_text
        });

        let rl_pos = LegendLRPositions::default();
        

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
        // let oy = Vector::new(ss * 0.7, -ss);
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
            size: 14.0,
            position: Point::new(center_h, v),
            horizontal_alignment: Horizontal::Center,
            vertical_alignment: Vertical::Center,
            font: config::DEFAULT_FONT,
            ..Text::default()
        };
        let rect_path_seuil =
            canvas::Path::rectangle(Point::new(0.0, v - vs / 2.0), Size::new(bounds.width, vs));

        frame.fill(
            &rect_path_seuil,
            canvas::Fill {
                style: canvas::Style::Solid(GRAY),
                rule: canvas::fill::Rule::NonZero,
            },
        );
        frame.fill_text(seuil_osseux);


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
            font: config::DEFAULT_FONT,
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
                rule: canvas::fill::Rule::NonZero,
            },
        );
        frame.fill_text(seuil_osseux);

        v += vs / 2.0;
        let dxy = Vector::new(ss * 0.7, -ss - 4.0);

        frame.fill_text(Text {
            content: "Pas de réponse".to_string(),
            position: Point::new(center_h, v),
            ..legend_text
        });
        frame.stroke(
            &Shape::bottom_left_arrow(rl_pos.l.other_no_response, ss),
            symbol_stroke.clone(),
        );
        frame.stroke(
            &Shape::bottom_right_arrow(rl_pos.r.other_no_response, ss),
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


        v += vs + 0.0;
        let oy = Vector::new(ss * 0.7, -ss * 1.6);
        frame.fill_text(Text {
            content: "Surassourdissement".to_string(),
            position: Point::new(center_h, v),
            horizontal_alignment: Horizontal::Center,
            ..legend_text
        });
        frame.fill_text(Text {
            content: "ou masque insuffisant".to_string(),
            position: Point::new(center_h, v + 15.0),
            horizontal_alignment: Horizontal::Center,
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



        let legend_rect_size = Size::new(bounds.width, bounds.height);
        let rectangle = Rectangle::new(Point::new(0., 0.), legend_rect_size);
        add_contour(&mut frame, rectangle, 6.0, space, 2.0, LEGEND_BORDER_COLOR);

        let b = ICON_SIZE / 2.;
        let size = Size::new(2. * b, 2. * b);
        if let Some((_, pos)) = state.left_shape {
            frame.stroke(
                &Path::rectangle(pos, size),
                LEGEND_SELECT_STROKE.clone(),
            );
        }
        if let Some((_, pos)) = state.right_shape {
            frame.stroke(
                &Path::rectangle(pos, size),
                LEGEND_SELECT_STROKE.clone(),
            );
        }
        if let Some((_, pos)) = state.left_modifier {
            frame.stroke(
                &Path::rectangle(pos, size),
                LEGEND_SELECT_MODIFIER_STROKE.clone(),
            );
        }
        if let Some((_, pos)) = state.right_modifier {
            frame.stroke(
                &Path::rectangle(pos, size),
                LEGEND_SELECT_MODIFIER_STROKE.clone(),
            );
        }
        

        vec![frame.into_geometry()]
    }

    fn update(
        &self,
        state: &mut Interaction,
        event: Event, 
        bounds: Rectangle,
        cursor: mouse::Cursor,
    ) -> (event::Status, Option<Message>) {
        let cursor_in_bounds: bool = cursor.is_over(bounds);


        // a click or a scroll outside the track window has not effect
        let cursor_position = if let Some(pos) = cursor.position_from(bounds.position()) {
            pos
        } else {
            return (event::Status::Ignored, None);
        };


        if !cursor_in_bounds {
            match event {
                Event::Mouse(mouse::Event::ButtonPressed(_))
                | Event::Mouse(mouse::Event::WheelScrolled { .. }) => {
                    return (event::Status::Ignored, None);
                }
                _ => {}
            }
        }

        match event {
            Event::Mouse(mouse::Event::ButtonPressed(_)) => {
                if let Some((side, icon, pos)) = self.icon_positions.get_icon_under_cursor(cursor_position) {
                    println!("{:?}, {:?}", side, icon);
                    match side {
                        Side::LeftShape => {
                            state.left_shape = Some((icon, pos));
                        }
                        Side::RightShape => {
                            state.right_shape = Some((icon, pos));
                        }
                        Side::LeftModifier => {
                            state.left_modifier = Some((icon, pos));
                        }
                        Side::RightModifier => {
                            state.right_modifier = Some((icon, pos));
                        }
                    }
                    
                }
            }
            _ => {}
        }

        return (event::Status::Ignored, None);
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
