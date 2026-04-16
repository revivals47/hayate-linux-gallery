mod chrome;
mod i18n;
mod live;
mod pages;
mod theme_bar;

use std::cell::RefCell;
use std::rc::Rc;

use hayate_ui::render::TextEngine;
use hayate_ui::widget::core::Widget;
use hayate_ui::widget::{AppTheme, DragZone, VStack};
use hayate_ui::widget::titlebar_theme::TitleBarTheme;
use hayate_ui::App;

use i18n::{L, Lang};
use theme_bar::ThemeId;

fn main() {
    let lang = Lang::from_args();
    let l = L::new(lang);
    let engine = Rc::new(RefCell::new(TextEngine::new()));
    let theme_id = ThemeId::from_args();

    let title = if lang == Lang::Ja { "Hayate ウィジェット一覧" } else { "Hayate Widget Gallery" };

    let app_theme = match theme_id {
        ThemeId::Win95 => Some(AppTheme::win95()),
        ThemeId::MacOs9 => Some(AppTheme::mac_os9()),
        ThemeId::XpLuna => Some(AppTheme::xp_luna()),
        ThemeId::Win10 => Some(AppTheme::win10()),
        ThemeId::MacOsBigSur => Some(AppTheme::macos_big_sur()),
        ThemeId::Default => None,
    };

    let tabs = pages::build_all_tabs(engine.clone(), &l, app_theme.as_ref());
    let theme_bar = theme_bar::build(engine.clone(), theme_id, app_theme.as_ref());

    let content = VStack::new(0.0)
        .add(theme_bar)
        .add(Box::new(tabs));

    // Use the App's CSD title bar path for themed modes — App renders
    // the TitleBar widget directly at y=0..csd_h, which is the tested
    // Wayland-ready path. Trying to DIY the title bar inside a root-level
    // WindowFrame widget runs into render-order subtleties that we'd rather
    // not wrestle with per-app.
    let mut app = App::new(title, 1000, 700).with_min_size(600, 400);
    // with_app_theme propagates the widget theme bundle to every widget in
    // the tree via inject_theme, so pages no longer need per-widget
    // .theme(...) calls — the theme just flows down the tree.
    match theme_id {
        ThemeId::Win95 => {
            app = app
                .with_theme(&hayate_ui::style::theme::WIN95_THEME)
                .with_app_theme(AppTheme::win95())
                .with_csd(title)
                .with_titlebar_theme(TitleBarTheme::win95());
        }
        ThemeId::MacOs9 => {
            app = app
                .with_theme(&hayate_ui::style::theme::MACOS9_THEME)
                .with_app_theme(AppTheme::mac_os9())
                .with_csd(title)
                .with_titlebar_theme(TitleBarTheme::mac_os9());
        }
        ThemeId::XpLuna => {
            app = app
                .with_theme(&hayate_ui::style::theme::XP_LUNA_THEME)
                .with_app_theme(AppTheme::xp_luna())
                .with_csd(title)
                .with_titlebar_theme(TitleBarTheme::xp_luna());
        }
        ThemeId::Win10 => {
            app = app
                .with_theme(&hayate_ui::style::theme::WIN10_THEME)
                .with_app_theme(AppTheme::win10())
                .with_csd(title)
                .with_titlebar_theme(TitleBarTheme::win10());
        }
        ThemeId::MacOsBigSur => {
            app = app
                .with_theme(&hayate_ui::style::theme::MACOS_BIG_SUR_THEME)
                .with_app_theme(AppTheme::macos_big_sur())
                .with_csd(title)
                .with_titlebar_theme(TitleBarTheme::macos_big_sur());
        }
        ThemeId::Default => {}
    }
    let move_req = app.move_request();

    let root: Box<dyn Widget> = match theme_id {
        ThemeId::Win95 => Box::new(chrome::BevelFrame::win95(Box::new(content), "Ready")),
        ThemeId::MacOs9
        | ThemeId::XpLuna
        | ThemeId::Win10
        | ThemeId::MacOsBigSur => Box::new(content),
        ThemeId::Default => Box::new(
            VStack::new(0.0)
                .add(Box::new(DragZone::new(move_req)))
                .add(Box::new(content))
        ),
    };

    if let Err(e) = app.run(root) {
        eprintln!("Error: {e}");
    }
}
