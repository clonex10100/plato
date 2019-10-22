use crate::framebuffer::Framebuffer;
use crate::device::CURRENT_DEVICE;
use super::{View, Event, Hub, Bus, ViewId, THICKNESS_MEDIUM};
use super::icon::Icon;
use super::input_field::InputField;
use super::filler::Filler;
use crate::gesture::GestureEvent;
use crate::input::DeviceEvent;
use crate::color::{TEXT_BUMP_SMALL, SEPARATOR_NORMAL};
use crate::geom::Rectangle;
use crate::app::Context;
use crate::unit::scale_by_dpi;
use crate::font::Fonts;

#[derive(Debug)]
pub struct SearchBar {
    pub rect: Rectangle,
    children: Vec<Box<dyn View>>,
}

impl SearchBar {
    pub fn new(rect: Rectangle, placeholder: &str, text: &str) -> SearchBar {
        let mut children = Vec::new();
        let dpi = CURRENT_DEVICE.dpi;
        let thickness = scale_by_dpi(THICKNESS_MEDIUM, dpi) as i32;
        let side = rect.height() as i32;

        let search_rect = rect![rect.min, rect.min + side];
        let search_icon = Icon::new("search",
                                    search_rect,
                                    Event::ToggleNear(ViewId::SearchMenu, search_rect))
                               .background(TEXT_BUMP_SMALL[0]);

        children.push(Box::new(search_icon) as Box<dyn View>);
        
        let separator = Filler::new(rect![pt!(rect.min.x + side, rect.min.y),
                                          pt!(rect.min.x + side + thickness, rect.max.y)],
                                    SEPARATOR_NORMAL);

        children.push(Box::new(separator) as Box<dyn View>);

        let input_field = InputField::new(rect![pt!(rect.min.x + side + thickness, rect.min.y),
                                                pt!(rect.max.x - side - thickness, rect.max.y)],
                                          ViewId::SearchInput)
                                     .border(false)
                                     .text(text)
                                     .placeholder(placeholder);

        children.push(Box::new(input_field) as Box<dyn View>);

        let separator = Filler::new(rect![pt!(rect.max.x - side - thickness, rect.min.y),
                                          pt!(rect.max.x - side, rect.max.y)],
                                    SEPARATOR_NORMAL);

        children.push(Box::new(separator) as Box<dyn View>);

        let close_icon = Icon::new("close",
                                   rect![pt!(rect.max.x - side, rect.min.y),
                                         pt!(rect.max.x, rect.max.y)],
                                   Event::Close(ViewId::SearchBar))
                              .background(TEXT_BUMP_SMALL[0]);

        children.push(Box::new(close_icon) as Box<dyn View>);

        SearchBar {
            rect,
            children,
        }
    }

    pub fn set_text(&mut self, text: &str, hub: &Hub) {
        if let Some(input_field) = self.children[2].downcast_mut::<InputField>() {
            input_field.set_text(text, true, hub);
        }
    }
}

impl View for SearchBar {
    fn handle_event(&mut self, evt: &Event, _hub: &Hub, _bus: &mut Bus, _context: &mut Context) -> bool {
        match *evt {
            Event::Gesture(GestureEvent::Tap(center)) |
            Event::Gesture(GestureEvent::HoldFingerShort(center, ..)) if self.rect.includes(center) => true,
            Event::Gesture(GestureEvent::Swipe { start, .. }) if self.rect.includes(start) => true,
            Event::Device(DeviceEvent::Finger { position, .. }) if self.rect.includes(position) => true,
            _ => false,
        }
    }

    fn render(&self, _fb: &mut dyn Framebuffer, _rect: Rectangle, _fonts: &mut Fonts) {
    }

    fn resize(&mut self, rect: Rectangle, hub: &Hub, context: &mut Context) {
        let dpi = CURRENT_DEVICE.dpi;
        let thickness = scale_by_dpi(THICKNESS_MEDIUM, dpi) as i32;
        let side = rect.height() as i32;
        self.children[0].resize(rect![rect.min, rect.min + side], hub, context);
        self.children[1].resize(rect![pt!(rect.min.x + side, rect.min.y),
                                      pt!(rect.min.x + side + thickness, rect.max.y)], hub, context);
        self.children[2].resize(rect![pt!(rect.min.x + side + thickness, rect.min.y),
                                      pt!(rect.max.x - side - thickness, rect.max.y)], hub, context);
        self.children[3].resize(rect![pt!(rect.max.x - side - thickness, rect.min.y),
                                      pt!(rect.max.x - side, rect.max.y)], hub, context);
        self.children[4].resize(rect![pt!(rect.max.x - side, rect.min.y),
                                      pt!(rect.max.x, rect.max.y)], hub, context);
        self.rect = rect;
    }

    fn rect(&self) -> &Rectangle {
        &self.rect
    }

    fn rect_mut(&mut self) -> &mut Rectangle {
        &mut self.rect
    }

    fn children(&self) -> &Vec<Box<dyn View>> {
        &self.children
    }

    fn children_mut(&mut self) -> &mut Vec<Box<dyn View>> {
        &mut self.children
    }
}
