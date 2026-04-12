mod i18n;
mod pages;

use std::cell::RefCell;
use std::rc::Rc;

use hayate_ui::render::TextEngine;
use hayate_ui::widget::core::Widget;
use hayate_ui::widget::{
    AppTheme, DragZone, TabViewWidget, TabEntry, VStack,
    WindowFrame, WindowFrameTheme,
};
use hayate_ui::App;

use i18n::{L, Lang};

fn main() {
    let lang = Lang::from_args();
    let l = L::new(lang);
    let engine = Rc::new(RefCell::new(TextEngine::new()));
    let args: Vec<String> = std::env::args().collect();

    let title = if lang == Lang::Ja { "Hayate ウィジェット一覧" } else { "Hayate Widget Gallery" };
    let use_win95 = args.iter().any(|a| a == "--win95");
    let use_macos9 = args.iter().any(|a| a == "--macos9");

    let app_theme = if use_win95 {
        Some(AppTheme::win95())
    } else {
        None
    };

    let tabs = if let Some(ref theme) = app_theme {
        // Pass theme to page builders so widgets use Win95 theme
        pages::build_all_tabs(engine.clone(), &l, Some(theme))
    } else {
        pages::build_all_tabs(engine.clone(), &l, None)
    };

    let mut app = App::new(title, 1000, 700)
        .with_min_size(600, 400);
    if use_win95 {
        app = app.with_theme(&hayate_ui::style::theme::WIN95_THEME);
    }
    let move_req = app.move_request();

    let root: Box<dyn Widget> = if use_win95 || use_macos9 {
        let frame_theme = if use_win95 {
            WindowFrameTheme::win95()
        } else {
            WindowFrameTheme::mac_os9()
        };
        let mut frame = WindowFrame::new(title, engine.clone(), move_req)
            .theme(frame_theme)
            .with_action(app.window_action())
            .content(Box::new(tabs));
        if use_win95 {
            frame = frame.status("Ready");
        }
        Box::new(frame)
    } else {
        Box::new(
            VStack::new(0.0)
                .add(Box::new(DragZone::new(move_req)))
                .add(Box::new(tabs))
        )
    };

    if let Err(e) = app.run(root) {
        eprintln!("Error: {e}");
    }
}
