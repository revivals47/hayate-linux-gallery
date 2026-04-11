mod i18n;
mod pages;

use std::cell::RefCell;
use std::rc::Rc;

use hayate_ui::render::TextEngine;
use hayate_ui::widget::core::Widget;
use hayate_ui::widget::{DragZone, TitleBarTheme, TabViewWidget, TabEntry, VStack};
use hayate_ui::App;

use i18n::{L, Lang};

fn main() {
    let lang = Lang::from_args();
    let l = L::new(lang);
    let engine = Rc::new(RefCell::new(TextEngine::new()));

    // Theme selection: --win95, --macos9, --win11, --macos
    let titlebar_theme = if std::env::args().any(|a| a == "--win95") {
        Some(TitleBarTheme::win95())
    } else if std::env::args().any(|a| a == "--macos9") {
        Some(TitleBarTheme::mac_os9())
    } else if std::env::args().any(|a| a == "--macos") {
        Some(TitleBarTheme::macos())
    } else if std::env::args().any(|a| a == "--win11") {
        Some(TitleBarTheme::win11())
    } else {
        None // default Hayate theme
    };

    let title = if lang == Lang::Ja { "Hayate ウィジェット一覧" } else { "Hayate Widget Gallery" };
    let mut app = App::new(title, 1000, 700)
        .with_min_size(600, 400);
    if let Some(theme) = titlebar_theme {
        app = app.with_titlebar_theme(theme);
    }
    let move_req = app.move_request();

    let tabs = TabViewWidget::new()
        .add_tab(TabEntry::new(l.tab_basic(), pages::basic::build(engine.clone(), &l)))
        .add_tab(TabEntry::new(l.tab_input(), pages::input::build(engine.clone(), &l)))
        .add_tab(TabEntry::new(l.tab_layout(), pages::layout::build(engine.clone(), &l)))
        .add_tab(TabEntry::new(l.tab_nav(), pages::navigation::build(engine.clone(), &l)))
        .add_tab(TabEntry::new(l.tab_display(), pages::display::build(engine.clone(), &l)))
        .add_tab(TabEntry::new(l.tab_overlay(), pages::overlay::build(engine.clone(), &l)));

    let root: Box<dyn Widget> = Box::new(
        VStack::new(0.0)
            .add(Box::new(DragZone::new(move_req)))
            .add(Box::new(tabs))
    );
    if let Err(e) = app.run(root) {
        eprintln!("Error: {e}");
    }
}
