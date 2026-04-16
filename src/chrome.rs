//! Win95-style chrome wrappers for the gallery root.
//!
//! The App's CSD path draws the title bar at y=0..csd_h and hands the root
//! widget a rect starting at y=csd_h. `BevelFrame` paints a 2-stage raised
//! bevel around that rect so the window has proper Win95 chrome below the
//! title. `StatusBar` draws a sunken 1-pixel bevel + right-aligned label
//! for the "Ready" footer.

use std::cell::RefCell;
use std::rc::Rc;

use hayate_ui::render::{Renderer, TextEngine};
use hayate_ui::scroll::delegate::ItemRect;
use hayate_ui::widget::core::{Constraints, EventResponse, Size, Widget, WidgetEvent};

/// Wraps `child` in a 2-stage raised bevel + a sunken status bar at the bottom.
pub struct BevelFrame {
    child: Box<dyn Widget>,
    status: String,
    engine: Option<Rc<RefCell<TextEngine>>>,
    body_rect: ItemRect,
    // 2-stage bevel colors: outer TL/BR, inner TL/BR
    bevel: (u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8),
    // (otl_r, otl_g, otl_b, obr_r, obr_g, obr_b, itl_r, itl_g, itl_b, ibr_r, ibr_g, ibr_b)
    body_bg: (u8, u8, u8),
    status_height: f32,
    status_bg: (u8, u8, u8),
    status_fg: (u8, u8, u8),
    dirty: bool,
}

impl BevelFrame {
    /// Build a Win95-style chrome: white/light outer TL, black/dark shadow BR,
    /// grey body, 20-pixel sunken status bar.
    pub fn win95(child: Box<dyn Widget>, status: impl Into<String>) -> Self {
        Self {
            child,
            status: status.into(),
            engine: None,
            body_rect: ItemRect::new(0.0, 0.0, 0.0, 0.0),
            bevel: (
                255, 255, 255,  // outer TL: white
                128, 128, 128,  // outer BR: shadow
                223, 223, 223,  // inner TL: light grey
                128, 128, 128,  // inner BR: shadow
            ),
            body_bg: (192, 192, 192),
            status_height: 20.0,
            status_bg: (192, 192, 192),
            status_fg: (0, 0, 0),
            dirty: true,
        }
    }
}

impl Widget for BevelFrame {
    fn layout(&mut self, constraints: &Constraints) -> Size {
        // Outer bevel is 2px; status bar reserves `status_height` at the bottom.
        let inset = 2.0;
        let child_w = (constraints.max_width - inset * 2.0).max(0.0);
        let child_h = (constraints.max_height - inset * 2.0 - self.status_height).max(0.0);
        let cc = Constraints {
            min_width: 0.0, max_width: child_w,
            min_height: 0.0, max_height: child_h,
        };
        self.child.layout(&cc);
        constraints.constrain(Size::new(constraints.max_width, constraints.max_height))
    }

    fn paint(&mut self, renderer: &mut Renderer, rect: ItemRect) {
        self.body_rect = rect;
        let (br_r, br_g, br_b) = self.body_bg;

        // Fill body background
        renderer.fill_rect(&rect, br_r, br_g, br_b, 255);

        // 2-stage raised bevel
        let (x, y, w, h) = (rect.x, rect.y, rect.width, rect.height);
        let (otl_r, otl_g, otl_b, obr_r, obr_g, obr_b,
             itl_r, itl_g, itl_b, ibr_r, ibr_g, ibr_b) = self.bevel;
        // Outer
        renderer.fill_rect(&ItemRect::new(x, y, w, 1.0), otl_r, otl_g, otl_b, 255);
        renderer.fill_rect(&ItemRect::new(x, y, 1.0, h), otl_r, otl_g, otl_b, 255);
        renderer.fill_rect(&ItemRect::new(x, y + h - 1.0, w, 1.0), obr_r, obr_g, obr_b, 255);
        renderer.fill_rect(&ItemRect::new(x + w - 1.0, y, 1.0, h), obr_r, obr_g, obr_b, 255);
        // Inner
        renderer.fill_rect(&ItemRect::new(x + 1.0, y + 1.0, w - 2.0, 1.0), itl_r, itl_g, itl_b, 255);
        renderer.fill_rect(&ItemRect::new(x + 1.0, y + 1.0, 1.0, h - 2.0), itl_r, itl_g, itl_b, 255);
        renderer.fill_rect(&ItemRect::new(x + 1.0, y + h - 2.0, w - 2.0, 1.0), ibr_r, ibr_g, ibr_b, 255);
        renderer.fill_rect(&ItemRect::new(x + w - 2.0, y + 1.0, 1.0, h - 2.0), ibr_r, ibr_g, ibr_b, 255);

        // Content area (inside bevel, above status bar)
        let inset = 2.0;
        let content_y = y + inset;
        let content_h = h - inset * 2.0 - self.status_height;
        let content_rect = ItemRect::new(x + inset, content_y, w - inset * 2.0, content_h);
        renderer.push_clip(&content_rect);
        self.child.paint(renderer, content_rect);
        renderer.pop_clip();

        // Status bar (sunken 1px bevel)
        let sy = y + h - inset - self.status_height;
        let status_rect = ItemRect::new(x + inset, sy, w - inset * 2.0, self.status_height);
        let (sbr, sbg, sbb) = self.status_bg;
        renderer.fill_rect(&status_rect, sbr, sbg, sbb, 255);
        // Sunken 1-pixel bevel: dark on top-left, light on bottom-right
        renderer.fill_rect(&ItemRect::new(status_rect.x, status_rect.y, status_rect.width, 1.0), 128, 128, 128, 255);
        renderer.fill_rect(&ItemRect::new(status_rect.x, status_rect.y, 1.0, status_rect.height), 128, 128, 128, 255);
        renderer.fill_rect(&ItemRect::new(status_rect.x, status_rect.y + status_rect.height - 1.0, status_rect.width, 1.0), 255, 255, 255, 255);
        renderer.fill_rect(&ItemRect::new(status_rect.x + status_rect.width - 1.0, status_rect.y, 1.0, status_rect.height), 255, 255, 255, 255);

        // Status text (left-aligned)
        if !self.status.is_empty() {
            if let Some(ref engine) = self.engine {
                let mut eng = engine.borrow_mut();
                let (fr, fg, fb) = self.status_fg;
                renderer.draw_text(
                    &mut eng, &self.status,
                    status_rect.x + 6.0, status_rect.y + 2.0,
                    11.0, self.status_height - 4.0,
                    fr, fg, fb,
                    status_rect.width - 12.0,
                );
            }
        }
    }

    fn event(&mut self, event: &WidgetEvent) -> EventResponse {
        self.child.event(event)
    }

    fn dirty(&self) -> bool { self.dirty || self.child.dirty() }
    fn clear_dirty(&mut self) { self.dirty = false; self.child.clear_dirty(); }

    fn update(&mut self, dt: f32) { self.child.update(dt); }

    fn inject_engine(&mut self, engine: Rc<RefCell<TextEngine>>) {
        self.engine = Some(engine.clone());
        self.child.inject_engine(engine);
    }
}
