use iced::alignment::{Horizontal, Vertical};

// use iced::theme::Theme;

use iced::widget::{canvas};

// use iced::widget::canvas::path::{Arc, Builder};
use iced::widget::canvas::{ Canvas, Path, Text};
use iced::mouse;

use iced::{ Element, Length, Point, Rectangle, Renderer, Theme};

use crate::config::{
    self, IMMIT_CANVAS_HEIGHT, IMMIT_CANVAS_WIDTH, IM_PLOT_TICK_SIZE,
    IM_PLOT_X_OFFSET,  
     PLOT_TICK_LABEL_SPACE, 
    PLOT_Y_OFFSET_START,

};
use crate::Message;


pub struct ImmitPlot;

// impl Default for ImmitPlot {
//     fn default() -> Self {
//         Self {
//             v1: 0.0,
//             v2: 0.0,
//             w1: 0.0,
//             w2: 0.0,
//         }
//     }
// }

// impl ImmitPlot {
//     pub fn new() -> Self {
//         Self::default()
//     }
// }

#[derive(Debug, Clone)]
pub enum Interaction {
    None,
}

impl Default for Interaction {
    fn default() -> Self {
        Interaction::None
    }
}

impl canvas::Program<Message, Renderer> for ImmitPlot {
    type State = Interaction;

    fn draw(
        &self,
        // _interaction: &Interaction,
        _state: &Self::State,
        renderer: &Renderer,
        _theme: &Theme,
        bounds: Rectangle,
        _cursor: mouse::Cursor,
    ) -> Vec<canvas::Geometry> {
        let mut frame = canvas::Frame::new(renderer, bounds.size());

        let space = 2.0;

        let y_offset0 = PLOT_Y_OFFSET_START;

        let y_axis = [2.5, 2.0, 1.5, 1.0, 0.5, 0.0];
        let x_axis = [-300, -200, -100, 0, 100, 200];

        let x_offset = IM_PLOT_X_OFFSET;

        let plot_width = (x_axis.len() - 1) as f32 * IM_PLOT_TICK_SIZE * 1.0;
        let plot_height =
            PLOT_Y_OFFSET_START + space + (y_axis.len() - 1) as f32 * IM_PLOT_TICK_SIZE;

        let y_unit = IM_PLOT_TICK_SIZE;
        let x_unit = IM_PLOT_TICK_SIZE;

        let legend_text = Text {
            color: config::AXIS_LABEL_COLOR,
            size: 13.0,
            font: config::DEFAULT_FONT,
            ..Text::default()
        };

        let mut y: f32 ;
        let y_stroke = canvas::Stroke {
            style: canvas::Style::Solid(config::GRID_COLOR),
            width: 1.0,
            line_cap: canvas::LineCap::Round,
            line_join: canvas::LineJoin::Round,
            ..canvas::Stroke::default()
        };

        let first_x = x_offset + space;
        let last_x = first_x + plot_width;

        // let db_x_position = first_x - PLOT_TICK_LABEL_SPACE + 10.0;
        // let db_halign = Horizontal::Right;

        // let hz_x_position = last_x;
        // let hz_halign = Horizontal::Right;

        let y_tick_x_pos = first_x - PLOT_TICK_LABEL_SPACE;
        let y_tick_h_align = Horizontal::Right;

        // let y1 = plot_height + PLOT_CA_CO_Y_SPACE;

        // frame.fill_text(Text {
        //     content: "ml".to_string(),
        //     horizontal_alignment: db_halign,
        //     vertical_alignment: Vertical::Bottom,
        //     position: Point::new(db_x_position - 10.0, space + y_offset0 - 15.0),
        //     ..legend_text
        // });

        // frame.fill_text(Text {
        //     content: "Hz".to_string(),
        //     horizontal_alignment: hz_halign,
        //     vertical_alignment: Vertical::Bottom,
        //     // position: Point::new(space + 15.0, (num_y_ticks as f32) * y_unit + 0.0 + y_offset),
        //     position: Point::new(
        //         hz_x_position,
        //         space + PLOT_Y_OFFSET_START - PLOT_TICK_LABEL_SPACE,
        //     ),
        //     ..legend_text
        // });

        // add grid to the plot frame
        for y_usize in 0..(y_axis.len()) {
            y = y_unit * y_usize as f32 + y_offset0 + space;

            frame.stroke(
                &Path::new(|p| {
                    p.move_to(Point::new(first_x, y));
                    p.line_to(Point::new(last_x, y));
                }),
                y_stroke.clone(),
            );

            let mut units = format!("{}", y_axis[y_usize]);

            if units == "0" {
                units = "0 ml".into();
            }
            frame.fill_text(Text {
                content: units,
                horizontal_alignment: y_tick_h_align,
                vertical_alignment: Vertical::Center,
                position: Point::new(y_tick_x_pos, y),
                ..legend_text
            });
        }

        let x_stroke = canvas::Stroke {
            style: canvas::Style::Solid(config::GRID_COLOR),
            width: 1.0,
            line_cap: canvas::LineCap::Round,
            line_join: canvas::LineJoin::Round,
            ..canvas::Stroke::default()
        };

        for x_usize in 0..(x_axis.len()) {
            let x = x_unit * x_usize as f32 + first_x;

            frame.stroke(
                &Path::new(|p| {
                    p.move_to(Point::new(x, y_offset0 + space));
                    p.line_to(Point::new(x, plot_height));
                }),
                x_stroke.clone(),
            );
            let content = format!("{}", x_axis[x_usize] as f32 / 1.0);

            if x_axis[x_usize] == 200 {
                frame.fill_text(Text {
                    content: "daPa".to_string(),
                    horizontal_alignment: Horizontal::Left,
                    vertical_alignment: Vertical::Bottom,
                    position: Point::new(
                        x + 14.0,
                        space - PLOT_TICK_LABEL_SPACE + PLOT_Y_OFFSET_START,
                    ),
                    // size: 10.0,
                    ..legend_text
                });
            }

            if x_usize == 0 {
                continue;
            }

            // if x_axis[x_usize] == 200 {
            //     content = "200".into();
            //     println!("here");
            // }

            frame.fill_text(Text {
                content,
                horizontal_alignment: Horizontal::Center,
                vertical_alignment: Vertical::Bottom,
                position: Point::new(x, space - PLOT_TICK_LABEL_SPACE + PLOT_Y_OFFSET_START),
                ..legend_text
            });
        }

        vec![frame.into_geometry()]
    }
}

pub fn im_plot<'a>() -> Element<'a, Message> {
    let plotter = ImmitPlot;
    // Element::new(Plot::new(data))
    let can = Canvas::new(plotter)
        // .width(Length::Fill)
        .width(Length::Fixed(IMMIT_CANVAS_WIDTH))
        .height(Length::Fixed(IMMIT_CANVAS_HEIGHT));

    let element = Element::new(can);
    // let element = container(can).into();
    element
}
