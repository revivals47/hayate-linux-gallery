use std::cell::{Cell, RefCell};
use std::rc::Rc;

use hayate_ui::render::Renderer;
use hayate_ui::scroll::delegate::ItemRect;
use hayate_ui::widget::{ScrollBar, ScrollBarOrientation};
use hayate_ui::widget::core::{Constraints, EventResponse, Size, Widget, WidgetEvent};
use hayate_ui::widget::focus::WidgetId;

use crate::demo::{Category, Demo, DemoCtx, DemoEntry, Lang};

/// Wrapper that owns a standalone ScrollBar and paints it with a
/// synthetic content_size so the thumb + arrow caps are visible. The
/// parent Demo system does not wire real scroll content — the goal is
/// to show the bar chrome under every theme.
struct ScrollBarShowcase {
    /// Stable widget identity (Track B Option I). Allocated once at
    /// construction; never changes for this instance's lifetime.
    id: WidgetId,
    inner: RefCell<ScrollBar>,
    offset: Rc<Cell<f32>>,
}

impl ScrollBarShowcase {
    fn new() -> Self {
        let offset = Rc::new(Cell::new(0.0));
        let mut sb = ScrollBar::new(ScrollBarOrientation::Vertical)
            .theme(hayate_ui::widget::ScrollBarTheme::default());
        // Content much larger than viewport so the thumb is smaller
        // than half the track — exercises the dither pattern on both
        // sides of the thumb.
        sb.set_content_size(2000.0);
        sb.set_viewport_size(400.0);
        Self {
            id: hayate_ui::widget::alloc_widget_id(),
            inner: RefCell::new(sb.with_shared_offset(Rc::clone(&offset))),
            offset,
        }
    }
}

impl Widget for ScrollBarShowcase {
    fn id(&self) -> Option<WidgetId> {
        Some(self.id)
    }

    fn layout(&mut self, c: &Constraints) -> Size {
        // Fix a tall viewport so the track has enough room to see the
        // full arrow caps + a decent thumb travel.
        let h = c.max_height.min(400.0).max(160.0);
        let w = 16.0_f32.min(c.max_width);
        let mut sb = self.inner.borrow_mut();
        sb.set_viewport_size(h);
        sb.layout(&Constraints::tight(w, h))
    }

    fn paint(&mut self, renderer: &mut Renderer, rect: ItemRect) {
        self.inner.borrow_mut().paint(renderer, rect);
    }

    fn event(&mut self, event: &WidgetEvent) -> EventResponse {
        self.inner.borrow_mut().event(event)
    }

    /// Forward overlay-priority events to the inner bar. ScrollBar
    /// overrides `event_overlay` for pointer capture while dragging
    /// (so the drag ends reliably even when the cursor exits the
    /// widget rect / window). The default Widget impl walks
    /// `children_mut()` which we don't expose because `inner` lives
    /// inside a RefCell — without this forward the capture path is
    /// silently dead.
    fn event_overlay(&mut self, event: &WidgetEvent) -> EventResponse {
        self.inner.borrow_mut().event_overlay(event)
    }

    fn dirty(&self) -> bool { self.inner.borrow().dirty() }
    fn clear_dirty(&mut self) { self.inner.borrow_mut().clear_dirty() }

    fn inject_theme(&mut self, theme: std::rc::Rc<hayate_ui::widget::AppTheme>) {
        self.inner.borrow_mut().inject_theme(theme);
    }
}

struct ScrollBarDemo;

impl Demo for ScrollBarDemo {
    fn id(&self) -> &'static str { "scroll_bar" }
    fn category(&self) -> Category { Category::Navigation }
    fn title(&self, lang: Lang) -> &'static str {
        match lang { Lang::Ja => "スクロールバー", Lang::En => "Scroll Bar" }
    }
    fn build(&self, _ctx: &DemoCtx) -> Box<dyn Widget> {
        Box::new(ScrollBarShowcase::new())
    }
}

inventory::submit!(DemoEntry { demo: &ScrollBarDemo });
