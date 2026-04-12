pub mod basic;
pub mod input;
pub mod layout;
pub mod navigation;
pub mod display;
pub mod overlay;

use std::cell::RefCell;
use std::rc::Rc;

use hayate_ui::render::TextEngine;
use hayate_ui::widget::core::Widget;
use hayate_ui::widget::{AppTheme, TabViewWidget, TabEntry};

use crate::i18n::L;

/// Build all tabs, optionally applying an AppTheme to each page's widgets.
pub fn build_all_tabs(
    engine: Rc<RefCell<TextEngine>>,
    l: &L,
    theme: Option<&AppTheme>,
) -> TabViewWidget {
    let mut tabs = TabViewWidget::new()
        .add_tab(TabEntry::new(l.tab_basic(), basic::build(engine.clone(), l, theme)))
        .add_tab(TabEntry::new(l.tab_input(), input::build(engine.clone(), l, theme)))
        .add_tab(TabEntry::new(l.tab_layout(), layout::build(engine.clone(), l)))
        .add_tab(TabEntry::new(l.tab_nav(), navigation::build(engine.clone(), l)))
        .add_tab(TabEntry::new(l.tab_display(), display::build(engine.clone(), l, theme)))
        .add_tab(TabEntry::new(l.tab_overlay(), overlay::build(engine.clone(), l)));

    if let Some(t) = theme {
        tabs = tabs.theme(t.tab.clone());
    }
    tabs
}
