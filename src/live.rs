//! Live-value helpers for the validation pages.
//!
//! Each page wires a widget's output (button click, slider drag, etc.) into
//! a shared `Rc<RefCell<T>>`; a `LiveText` renders the current value every
//! repaint, so toggling a checkbox or dragging a slider visibly moves the
//! number on screen — proving the widget isn't just decoration.

use std::cell::RefCell;
use std::rc::Rc;

use hayate_ui::render::{Renderer, TextEngine};
use hayate_ui::scroll::delegate::ItemRect;
use hayate_ui::widget::core::{Constraints, EventResponse, Size, Widget, WidgetEvent};

/// Widget that renders `label + formatter(state)` every frame.
///
/// Returns dirty=true when the formatted output differs from the last paint,
/// so the App only schedules a repaint when the value actually changes.
pub struct LiveText<T: 'static> {
    state: Rc<RefCell<T>>,
    format: Box<dyn Fn(&T) -> String>,
    font_size: f32,
    color: (u8, u8, u8),
    engine: Option<Rc<RefCell<TextEngine>>>,
    last_rendered: RefCell<String>,
    last_rect: ItemRect,
    width_hint: f32,
}

impl<T: 'static> LiveText<T> {
    pub fn new(
        state: Rc<RefCell<T>>,
        format: impl Fn(&T) -> String + 'static,
        font_size: f32,
    ) -> Self {
        Self {
            state,
            format: Box::new(format),
            font_size,
            color: (220, 220, 220),
            engine: None,
            last_rendered: RefCell::new(String::new()),
            last_rect: ItemRect::new(0.0, 0.0, 0.0, 0.0),
            width_hint: 220.0,
        }
    }

    pub fn with_engine(mut self, engine: Rc<RefCell<TextEngine>>) -> Self {
        self.engine = Some(engine);
        self
    }

    pub fn with_color(mut self, r: u8, g: u8, b: u8) -> Self {
        self.color = (r, g, b);
        self
    }

    pub fn with_width(mut self, w: f32) -> Self {
        self.width_hint = w;
        self
    }

    fn current(&self) -> String {
        let s = self.state.borrow();
        (self.format)(&*s)
    }
}

impl<T: 'static> Widget for LiveText<T> {
    fn layout(&mut self, constraints: &Constraints) -> Size {
        let w = self.width_hint.min(constraints.max_width);
        let h = self.font_size * 1.4;
        constraints.constrain(Size::new(w, h))
    }

    fn paint(&mut self, renderer: &mut Renderer, rect: ItemRect) {
        self.last_rect = rect;
        let text = self.current();
        if let Some(ref engine) = self.engine {
            let mut e = engine.borrow_mut();
            let (r, g, b) = self.color;
            renderer.draw_text(
                &mut e, &text, rect.x, rect.y,
                self.font_size, self.font_size * 1.4,
                r, g, b, rect.width,
            );
        }
        *self.last_rendered.borrow_mut() = text;
    }

    fn event(&mut self, _event: &WidgetEvent) -> EventResponse {
        EventResponse::Ignored
    }

    fn dirty(&self) -> bool {
        self.current() != *self.last_rendered.borrow()
    }

    fn clear_dirty(&mut self) {}

    fn inject_engine(&mut self, engine: Rc<RefCell<TextEngine>>) {
        self.engine = Some(engine);
    }
}
