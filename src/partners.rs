use iced::widget::{
    column, container,  radio, vertical_space, Rule,
};
use iced::{
Element
};

use iced::widget::{button, text,  Container};
use iced::Length;
use iced::theme::{self, Theme};


// use iced::{event, mouse, overlay, Color, Point, Rectangle, Size};
// use iced_native;

use super::Message;

use crate::config::{CustomButtonStyle, TEXT_LINE_VSPACE};

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PartnerAndSuccursale {
    Harmonie(Harmonie),
    Bois(Bois),
    Prevost(Prevost),
    Autres(Autres),
    None,
}

impl Default for PartnerAndSuccursale {
    fn default() -> Self {
        PartnerAndSuccursale::None
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Bois {
    Montmagny,
    Levy,
    None,
}

impl Default for Bois {
    fn default() -> Self {
        Bois::None
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Harmonie {
    JeanGauvin,
    None,
}

impl Default for Harmonie {
    fn default() -> Self {
        Harmonie::None
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Prevost {
    Quebec,
    Malbaie,
    BaieStPaul,
    None,
}

impl Default for Prevost {
    fn default() -> Self {
        Prevost::None
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Autres {
    Quebec,
    Beaupre,
    BaieStPaul,
    None,
}

impl Default for Autres {
    fn default() -> Self {
        Autres::None
    }
}

pub fn get_all_partners(partner: &PartnerAndSuccursale) -> Element<Message> {
    let size = 16;
    let harmonie = radio(
        "Harmonie",
        PartnerAndSuccursale::Harmonie(Harmonie::None),
        Some(*partner),
        Message::PartnerChanged,
    )
    .size(size)
    .text_size(size);

    let bois = radio(
        "Bois",
        PartnerAndSuccursale::Bois(Bois::None),
        Some(*partner),
        Message::PartnerChanged,
    )
    .size(size)
    .text_size(size);

    let prevost = radio(
        "Prévost",
        PartnerAndSuccursale::Prevost(Prevost::None),
        Some(*partner),
        Message::PartnerChanged,
    )
    .size(size)
    .text_size(size);

    let aures = radio(
        "Autres",
        PartnerAndSuccursale::Autres(Autres::None),
        Some(*partner),
        Message::PartnerChanged,
    )
    .size(size)
    .text_size(size);

    column![harmonie, bois, prevost, aures].spacing(2.0).into()
}

pub fn get_chosen_succursale(partner: &PartnerAndSuccursale) -> (String, String) {
    let vspace = 1.5;
    // let clinic_vspace = 1.5;
    // let clinic_name_size = 14.;
    let text_size = 15;

     match partner {
        PartnerAndSuccursale::Bois(succursale) => {
            let clinic = "Clinique de l'audition Bois et Associés audioprothésistes";

            let succursale_string = match succursale {
                Bois::Montmagny => "83 Bd Taché O, Montmagny, QC G5V 3A6, 418-248-7077",
                Bois::Levy => "5500 Bd Guillaume-Couture suite 111, Lévis, QC G6V 4Z2, 418-837-3626",
                Bois::None => "",
            };

            (clinic.into(), succursale_string.into())
            
        }
        PartnerAndSuccursale::Harmonie(succursale) => {
            let clinic = "Harmonie Audition - Myriam Brunel Audioprothésiste";

            let succursale_string = match succursale {
                Harmonie::JeanGauvin => "790 Rte Jean-Gauvin local 230, Québec City, Quebec G1X 0B6. (418) 476-1455",
                _ => "",
            };

            (clinic.into(), succursale_string.into())
        }
        PartnerAndSuccursale::Prevost(succursale) => {
            let clinic = "Prévost Audioprothésistes";

            let succursale_string = match succursale {
                Prevost::Quebec => "1000 Ch Ste-Foy bureau 201, Québec City, Quebec G1S 2L6. (418) 688-1430",
                Prevost::Malbaie => "342 Rue St Étienne, La Malbaie, QC G5A 1M7. 1 (800) 363-5617",
                Prevost::BaieStPaul => "5 Rue Boivin bureau 208, Baie-Saint-Paul, QC. 1 (800) 363-5617",
                Prevost::None => "",
            };

            (clinic.into(), succursale_string.into())
        }

        _ => {
            (
                "".into(),
                "".into()
            )
        }
    }


}



pub fn make_succursale_element(partner: &PartnerAndSuccursale) -> Container<'_,Message> {
    let (mut clinic, succursale) = get_chosen_succursale(partner);
    let mut suc_element: Element<'_,Message> = text(succursale).to_owned().size(14).into();

    if clinic == "" {

        let vspace = TEXT_LINE_VSPACE;
        suc_element = 
            column![
                vertical_space(vspace),
                Rule::horizontal(0),
                vertical_space(vspace),
                Rule::horizontal(0),
                vertical_space(vspace),
                Rule::horizontal(1),
                vertical_space(vspace),
                Rule::horizontal(1)
            ].into()
        
    }



    let location = container(column![
        button(text(&("Lieu de l'évaluation : ".to_owned() + &clinic)).size(18.))
            .on_press(Message::ShowParnerChoices)
            .padding(0.)
            .style(theme::Button::Custom(Box::new(CustomButtonStyle))),
        // text("Lieu de l'évaluation: Clinique de l'Audition Bois & Associés audioprothésistes").size(14),
        // text(succursale).to_owned().size(14)
        suc_element
    ])
    .height(Length::Fixed(120. + 60.));

    location.into()
}


pub fn get_all_succursales(partner: &PartnerAndSuccursale) -> (String, Element<Message>) {
    let vspace = 1.5;
    // let clinic_vspace = 1.5;
    // let clinic_name_size = 14.;
    let text_size = 15;

    let (clinic, succursales) = match partner {
        PartnerAndSuccursale::Bois(_) => {

            let (clinic, montmagny) = get_chosen_succursale(&PartnerAndSuccursale::Bois(Bois::Montmagny));


            let suc1 = radio(
                montmagny,
                PartnerAndSuccursale::Bois(Bois::Montmagny),
                Some(*partner),
                Message::SuccursaleChanged,
            )
            .size(12)
            .text_size(text_size);

            let (_, levis) = get_chosen_succursale(&PartnerAndSuccursale::Bois(Bois::Montmagny));

            let suc2 = radio(
                levis,
                PartnerAndSuccursale::Bois(Bois::Levy),
                Some(*partner),
                Message::SuccursaleChanged,
            )
            .size(12)
            .text_size(text_size);

            (clinic, column![suc1, vertical_space(vspace), suc2])
        }
        PartnerAndSuccursale::Harmonie(_) => {
            let (clinic, jean_gauving) = get_chosen_succursale(&PartnerAndSuccursale::Harmonie(Harmonie::JeanGauvin));


            let suc1 = radio(
                jean_gauving,
                PartnerAndSuccursale::Harmonie(Harmonie::JeanGauvin),
                Some(*partner),
                Message::SuccursaleChanged,
            )
            .size(12)
            .text_size(text_size);

            (clinic, column![suc1])
        }
        PartnerAndSuccursale::Prevost(_) => {
            let (clinic, quebec) = get_chosen_succursale(&PartnerAndSuccursale::Prevost(Prevost::Quebec));

            let suc1 = radio(
                quebec,
                PartnerAndSuccursale::Prevost(Prevost::Quebec),

                Some(*partner),
                Message::SuccursaleChanged,
            )
            .size(12)
            .text_size(text_size);

            let (_, malbaie) = get_chosen_succursale(&PartnerAndSuccursale::Prevost(Prevost::Malbaie));

            let suc2 = radio(
                malbaie,
                PartnerAndSuccursale::Prevost(Prevost::Malbaie),
                Some(*partner),
                Message::SuccursaleChanged,
            )
            .size(12)
            .text_size(text_size);

            let (_, baie_st_paul) = get_chosen_succursale(&PartnerAndSuccursale::Prevost(Prevost::BaieStPaul));

            let suc3 = radio(
                baie_st_paul,
                PartnerAndSuccursale::Prevost(Prevost::BaieStPaul),
                Some(*partner),
                Message::SuccursaleChanged,
            )
            .size(12)
            .text_size(text_size);

            (
                clinic,
                column![
                    suc1,
                    vertical_space(vspace),
                    suc2,
                    vertical_space(vspace),
                    suc3
                ],
            )
        }

        // Partner::Aures(_) => {
        //     let quebec = radio(
        //         "1363 Av. Maguire Bureau 202, Québec, QC G1T 1Z2. (581) 491-6363",
        //         // Aures::Quebec,
        //         Partner::Aures(Aures::Quebec),
        //         Some(*partner),
        //         Message::PartnerChanged,
        //     )
        //     .size(12)
        //     .text_size(14);

        //     let beaupre = radio(
        //         "175 Bd du Beau Pré, Beaupré, QC G0A 1E0. (418) 702-1721",
        //         // Aures::Beaupre,
        //         Partner::Aures(Aures::Beaupre),
        //         Some(*partner),
        //         Message::PartnerChanged,
        //     )
        //     .size(12)
        //     .text_size(14);

        //     let baie_st_paul = radio(
        //         "4 Rue du Moulin Local 101, Baie-Saint-Paul, QC G3Z 2R8. (418) 760 - 8521",
        //         // Aures::BaieStPaul,
        //         Partner::Aures(Aures::BaieStPaul),
        //         Some(*partner),
        //         Message::PartnerChanged,
        //     )
        //     .size(12)
        //     .text_size(14);

        //     column![quebec, beaupre, baie_st_paul]
        // }
        _ => {
            let vspace = TEXT_LINE_VSPACE;
            (
                "".into(),
                column![
                    vertical_space(vspace),
                    Rule::horizontal(0),
                    vertical_space(vspace),
                    Rule::horizontal(0),
                    vertical_space(vspace),
                    Rule::horizontal(1),
                    vertical_space(vspace),
                    Rule::horizontal(1)
                ],
            )
        }
    };

    return (clinic.into(), container(succursales).into());
}

pub mod modal {
    // use iced_native::alignment::Alignment;
    // use iced_native::widget::{self, Tree};
    // use iced_native::{
    //     event, layout, mouse, overlay, renderer, Clipboard, Color, Element, Event, Layout, Length,
    //     Point, Rectangle, Shell, Size, Widget,
    // };

    use iced::advanced::layout::{self, Layout};
    use iced::advanced::overlay;
    use iced::advanced::renderer;
    use iced::advanced::widget::{self, Widget};
    use iced::advanced::{self, Clipboard, Shell};
    use iced::alignment::Alignment;
    use iced::event;
    use iced::mouse;
    use iced::{
        BorderRadius, Color, Element, Event, Length, Point, Rectangle, Size,
    };

    /// A widget that centers a modal element over some base element
    pub struct Modal<'a, Message, Renderer> {
        base: Element<'a, Message, Renderer>,
        modal: Element<'a, Message, Renderer>,
        on_blur: Option<Message>,
    }

    impl<'a, Message, Renderer> Modal<'a, Message, Renderer> {
        /// Returns a new [`Modal`]
        pub fn new(
            base: impl Into<Element<'a, Message, Renderer>>,
            modal: impl Into<Element<'a, Message, Renderer>>,
        ) -> Self {
            Self {
                base: base.into(),
                modal: modal.into(),
                on_blur: None,
            }
        }

        /// Sets the message that will be produces when the background
        /// of the [`Modal`] is pressed
        pub fn on_blur(self, on_blur: Message) -> Self {
            Self {
                on_blur: Some(on_blur),
                ..self
            }
        }
    }

    impl<'a, Message, Renderer> Widget<Message, Renderer> for Modal<'a, Message, Renderer>
    where
        Renderer: advanced::Renderer,
        Message: Clone,
    {
        fn children(&self) -> Vec<widget::Tree> {
            vec![widget::Tree::new(&self.base), widget::Tree::new(&self.modal)]
        }

        fn diff(&self, tree: &mut widget::Tree) {
            tree.diff_children(&[&self.base, &self.modal]);
        }

        fn width(&self) -> Length {
            self.base.as_widget().width()
        }

        fn height(&self) -> Length {
            self.base.as_widget().height()
        }

        fn layout(
            &self,
            renderer: &Renderer,
            limits: &layout::Limits,
        ) -> layout::Node {
            self.base.as_widget().layout(renderer, limits)
        }

        fn on_event(
            &mut self,
            state: &mut widget::Tree,
            event: Event,
            layout: Layout<'_>,
            cursor: mouse::Cursor,
            renderer: &Renderer,
            clipboard: &mut dyn Clipboard,
            shell: &mut Shell<'_, Message>,
            viewport: &Rectangle,
        ) -> event::Status {
            self.base.as_widget_mut().on_event(
                &mut state.children[0],
                event,
                layout,
                cursor,
                renderer,
                clipboard,
                shell,
                viewport,
            )
        }

        fn draw(
            &self,
            state: &widget::Tree,
            renderer: &mut Renderer,
            theme: &<Renderer as advanced::Renderer>::Theme,
            style: &renderer::Style,
            layout: Layout<'_>,
            cursor: mouse::Cursor,
            viewport: &Rectangle,
        ) {
            self.base.as_widget().draw(
                &state.children[0],
                renderer,
                theme,
                style,
                layout,
                cursor,
                viewport,
            );
        }

        fn overlay<'b>(
            &'b mut self,
            state: &'b mut widget::Tree,
            layout: Layout<'_>,
            _renderer: &Renderer,
        ) -> Option<overlay::Element<'b, Message, Renderer>> {
            Some(overlay::Element::new(
                layout.position(),
                Box::new(Overlay {
                    content: &mut self.modal,
                    tree: &mut state.children[1],
                    size: layout.bounds().size(),
                    on_blur: self.on_blur.clone(),
                }),
            ))
        }


        fn mouse_interaction(
            &self,
            state: &widget::Tree,
            layout: Layout<'_>,
            cursor: mouse::Cursor,
            viewport: &Rectangle,
            renderer: &Renderer,
        ) -> mouse::Interaction {
            self.base.as_widget().mouse_interaction(
                &state.children[0],
                layout,
                cursor,
                viewport,
                renderer,
            )
        }

        fn operate(
            &self,
            state: &mut widget::Tree,
            layout: Layout<'_>,
            renderer: &Renderer,
            operation: &mut dyn widget::Operation<Message>,
        ) {
            self.base
                .as_widget()
                .operate(&mut state.children[0], layout, renderer, operation);
        }
    }

    struct Overlay<'a, 'b, Message, Renderer> {
        content: &'b mut Element<'a, Message, Renderer>,
        tree: &'b mut widget::Tree,
        size: Size,
        on_blur: Option<Message>,
    }

    impl<'a, 'b, Message, Renderer> overlay::Overlay<Message, Renderer>
        for Overlay<'a, 'b, Message, Renderer>
    where
        Renderer: advanced::Renderer,
        Message: Clone,
    {
        fn layout(&self, renderer: &Renderer, _bounds: Size, position: Point) -> layout::Node {
            let limits = layout::Limits::new(Size::ZERO, self.size)
                .width(Length::Fill)
                .height(Length::Fill);

            let mut child = self.content.as_widget().layout(renderer, &limits);
            child.align(Alignment::Center, Alignment::Center, limits.max());

            let mut node = layout::Node::with_children(self.size, vec![child]);

            node.move_to(position);

            node
        }

        fn on_event(
            &mut self,
            event: Event,
            layout: Layout<'_>,
            cursor: mouse::Cursor,
            renderer: &Renderer,
            clipboard: &mut dyn Clipboard,
            shell: &mut Shell<'_, Message>,
        ) -> event::Status {
            let content_bounds = layout.children().next().unwrap().bounds();

            if let Some(message) = self.on_blur.as_ref() {
                if let Event::Mouse(mouse::Event::ButtonPressed(mouse::Button::Left)) = &event {
                    if let Some(pos) = cursor.position() {
                    if !content_bounds.contains(pos) {
                        shell.publish(message.clone());
                        return event::Status::Captured;
                    }
                }
            }}

            self.content.as_widget_mut().on_event(
                self.tree,
                event,
                layout.children().next().unwrap(),
                cursor,
                renderer,
                clipboard,
                shell,
                &layout.bounds(),
            )
        }

        fn draw(
            &self,
            renderer: &mut Renderer,
            theme: &Renderer::Theme,
            style: &renderer::Style,
            layout: Layout<'_>,
            cursor: mouse::Cursor,
        ) {
            renderer.fill_quad(
                renderer::Quad {
                    bounds: layout.bounds(),
                    border_radius: Default::default(),
                    border_width: 0.0,
                    border_color: Color::TRANSPARENT,
                },
                Color {
                    a: 0.80,
                    ..Color::BLACK
                },
            );

            self.content.as_widget().draw(
                self.tree,
                renderer,
                theme,
                style,
                layout.children().next().unwrap(),
                cursor,
                &layout.bounds(),
            );
        }

        fn operate(
            &mut self,
            layout: Layout<'_>,
            renderer: &Renderer,
            operation: &mut dyn widget::Operation<Message>,
        ) {
            self.content.as_widget().operate(
                self.tree,
                layout.children().next().unwrap(),
                renderer,
                operation,
            );
        }

        fn mouse_interaction(
            &self,
            layout: Layout<'_>,
            cursor: mouse::Cursor,
            viewport: &Rectangle,
            renderer: &Renderer,
        ) -> mouse::Interaction {
            self.content.as_widget().mouse_interaction(
                self.tree,
                layout.children().next().unwrap(),
                cursor,
                viewport,
                renderer,
            )
        }

        fn overlay<'c>(
            &'c mut self,
            layout: Layout<'_>,
            renderer: &Renderer,
        ) -> Option<overlay::Element<'c, Message, Renderer>> {
            self.content.as_widget_mut().overlay(
                self.tree,
                layout.children().next().unwrap(),
                renderer,
            )
        }
    }

    impl<'a, Message, Renderer> From<Modal<'a, Message, Renderer>> for Element<'a, Message, Renderer>
    where
        Renderer: 'a + advanced::Renderer,
        Message: 'a + Clone,
    {
        fn from(modal: Modal<'a, Message, Renderer>) -> Self {
            Element::new(modal)
        }
    }
}
