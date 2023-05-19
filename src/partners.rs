use iced::widget::{
    self, button, column, container, horizontal_space, radio, row, text, vertical_space, Rule,
};
use iced::{
    executor, keyboard, subscription, theme, Alignment, Application, Command, Element, Event,
    Length, Settings, Subscription,
};

use iced::{event, mouse, overlay, Color, Point, Rectangle, Size};
use iced_native;

use super::Message;

use crate::config::TEXT_LINE_VSPACE;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Partner {
    Harmonie(Harmonie),
    Bois(Bois),
    Prevost(Prevost),
    Aures(Aures),
    None,
}

impl Default for Partner {
    fn default() -> Self {
        Partner::None
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
pub enum Aures {
    Quebec,
    Beaupre,
    BaieStPaul,
    None,
}

impl Default for Aures {
    fn default() -> Self {
        Aures::None
    }
}

pub fn get_all_partners(partner: &Partner) -> Element<Message> {
    let size = 16;
    let harmonie = radio(
        "Harmonie",
        Partner::Harmonie(Harmonie::None),
        Some(*partner),
        Message::PartnerChanged,
    )
    .size(size)
    .text_size(size);

    let bois = radio(
        "Bois",
        Partner::Bois(Bois::None),
        Some(*partner),
        Message::PartnerChanged,
    )
    .size(size)
    .text_size(size);

    let prevost = radio(
        "Prévost",
        Partner::Prevost(Prevost::None),
        Some(*partner),
        Message::PartnerChanged,
    )
    .size(size)
    .text_size(size);

    let aures = radio(
        "Aures",
        Partner::Aures(Aures::None),
        Some(*partner),
        Message::PartnerChanged,
    )
    .size(size)
    .text_size(size);

    column![harmonie, bois, prevost, aures].spacing(2.0).into()
}

pub fn get_all_succursales(partner: &Partner) -> (String, Element<Message>) {
    let vspace = 1.5;
    let clinic_vspace = 1.5;
    let clinic_name_size = 14.;
    let text_size = 15;

    let (clinic, succursales) = match partner {
        Partner::Bois(_) => {
            // let clinic = text("Clinique de l'audition Bois et Associés audioprothésistes")
            //     .size(clinic_name_size);
            let clinic = "Clinique de l'audition Bois et Associés audioprothésistes";

            let montmagny = radio(
                "83 Bd Taché O, Montmagny, QC G5V 3A6, 418-248-7077",
                Partner::Bois(Bois::Montmagny),
                Some(*partner),
                Message::PartnerChanged,
            )
            .size(12)
            .text_size(text_size);

            let levis = radio(
                "5500 Bd Guillaume-Couture suite 111, Lévis, QC G6V 4Z2, 418-837-3626",
                Partner::Bois(Bois::Levy),
                Some(*partner),
                Message::PartnerChanged,
            )
            .size(12)
            .text_size(text_size);

            (clinic, column![montmagny, vertical_space(vspace), levis])
        }
        Partner::Harmonie(_) => {
            // let clinic =
            //     text("Harmonie Audition - Myriam Brunel Audioprothésiste").size(clinic_name_size);
            let clinic = "Harmonie Audition - Myriam Brunel Audioprothésiste";

            let jean_gauvin = radio(
                "790 Rte Jean-Gauvin local 230, Québec City, Quebec G1X 0B6. (418) 476-1455",
                Partner::Harmonie(Harmonie::JeanGauvin),
                // Harmonie::JeanGauvin,
                Some(*partner),
                Message::PartnerChanged,
            )
            .size(12)
            .text_size(text_size);

            (clinic, column![jean_gauvin])
        }
        Partner::Prevost(_) => {
            // let clinic = text("Prévost Audioprothésistes").size(clinic_name_size);
            let clinic = "Prévost Audioprothésistes";
            let quebec = radio(
                "1000 Ch Ste-Foy bureau 201, Québec City, Quebec G1S 2L6. (418) 688-1430",
                Partner::Prevost(Prevost::Quebec),
                // Prevost::Quebec,
                Some(*partner),
                Message::PartnerChanged,
            )
            .size(12)
            .text_size(text_size);

            let malbaie = radio(
                "342 Rue St Étienne, La Malbaie, QC G5A 1M7. 1 (800) 363-5617",
                // Prevost::Malbaie,
                Partner::Prevost(Prevost::Malbaie),
                Some(*partner),
                Message::PartnerChanged,
            )
            .size(12)
            .text_size(text_size);

            let baie_st_paul = radio(
                "5 Rue Boivin bureau 208, Baie-Saint-Paul, QC. 1 (800) 363-5617",
                // Prevost::BaieStPaul,
                Partner::Prevost(Prevost::BaieStPaul),
                Some(*partner),
                Message::PartnerChanged,
            )
            .size(12)
            .text_size(text_size);

            (
                clinic,
                column![
                    quebec,
                    vertical_space(vspace),
                    malbaie,
                    vertical_space(vspace),
                    baie_st_paul
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
                "",
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
    use iced_native::alignment::Alignment;
    use iced_native::widget::{self, Tree};
    use iced_native::{
        event, layout, mouse, overlay, renderer, Clipboard, Color, Element, Event, Layout, Length,
        Point, Rectangle, Shell, Size, Vector, Widget,
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
        Renderer: iced_native::Renderer,
        Message: Clone,
    {
        fn children(&self) -> Vec<Tree> {
            vec![Tree::new(&self.base), Tree::new(&self.modal)]
        }

        fn diff(&self, tree: &mut Tree) {
            tree.diff_children(&[&self.base, &self.modal]);
        }

        fn width(&self) -> Length {
            self.base.as_widget().width()
        }

        fn height(&self) -> Length {
            self.base.as_widget().height()
        }

        fn layout(&self, renderer: &Renderer, limits: &layout::Limits) -> layout::Node {
            self.base.as_widget().layout(renderer, limits)
        }

        fn on_event(
            &mut self,
            state: &mut Tree,
            event: Event,
            layout: Layout<'_>,
            cursor_position: Point,
            renderer: &Renderer,
            clipboard: &mut dyn Clipboard,
            shell: &mut Shell<'_, Message>,
        ) -> event::Status {
            self.base.as_widget_mut().on_event(
                &mut state.children[0],
                event,
                layout,
                cursor_position,
                renderer,
                clipboard,
                shell,
            )
        }

        fn draw(
            &self,
            state: &Tree,
            renderer: &mut Renderer,
            theme: &<Renderer as iced_native::Renderer>::Theme,
            style: &renderer::Style,
            layout: Layout<'_>,
            cursor_position: Point,
            viewport: &Rectangle,
        ) {
            self.base.as_widget().draw(
                &state.children[0],
                renderer,
                theme,
                style,
                layout,
                cursor_position,
                viewport,
            );
        }

        fn overlay<'b>(
            &'b mut self,
            state: &'b mut Tree,
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
            state: &Tree,
            layout: Layout<'_>,
            cursor_position: Point,
            viewport: &Rectangle,
            renderer: &Renderer,
        ) -> mouse::Interaction {
            self.base.as_widget().mouse_interaction(
                &state.children[0],
                layout,
                cursor_position,
                viewport,
                renderer,
            )
        }

        fn operate(
            &self,
            state: &mut Tree,
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
        tree: &'b mut Tree,
        size: Size,
        on_blur: Option<Message>,
    }

    impl<'a, 'b, Message, Renderer> overlay::Overlay<Message, Renderer>
        for Overlay<'a, 'b, Message, Renderer>
    where
        Renderer: iced_native::Renderer,
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
            cursor_position: Point,
            renderer: &Renderer,
            clipboard: &mut dyn Clipboard,
            shell: &mut Shell<'_, Message>,
        ) -> event::Status {
            let content_bounds = layout.children().next().unwrap().bounds();

            if let Some(message) = self.on_blur.as_ref() {
                if let Event::Mouse(mouse::Event::ButtonPressed(mouse::Button::Left)) = &event {
                    if !content_bounds.contains(cursor_position) {
                        shell.publish(message.clone());
                        return event::Status::Captured;
                    }
                }
            }

            self.content.as_widget_mut().on_event(
                self.tree,
                event,
                layout.children().next().unwrap(),
                cursor_position,
                renderer,
                clipboard,
                shell,
            )
        }

        fn draw(
            &self,
            renderer: &mut Renderer,
            theme: &Renderer::Theme,
            style: &renderer::Style,
            layout: Layout<'_>,
            cursor_position: Point,
        ) {
            renderer.fill_quad(
                renderer::Quad {
                    bounds: layout.bounds(),
                    border_radius: renderer::BorderRadius::from(0.0),
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
                cursor_position,
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
            cursor_position: Point,
            viewport: &Rectangle,
            renderer: &Renderer,
        ) -> mouse::Interaction {
            self.content.as_widget().mouse_interaction(
                self.tree,
                layout.children().next().unwrap(),
                cursor_position,
                viewport,
                renderer,
            )
        }
    }

    impl<'a, Message, Renderer> From<Modal<'a, Message, Renderer>> for Element<'a, Message, Renderer>
    where
        Renderer: 'a + iced_native::Renderer,
        Message: 'a + Clone,
    {
        fn from(modal: Modal<'a, Message, Renderer>) -> Self {
            Element::new(modal)
        }
    }
}
