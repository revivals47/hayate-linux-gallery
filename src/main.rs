//! Gallery launcher. All real work lives in `app::Shell` and `demos/`.

mod app;
mod chrome;
mod demo;
mod demos;

use hayate_ui::App;
use hayate_ui::widget::titlebar_theme::TitleBarTheme;

use crate::app::Shell;
use crate::chrome::ThemeId;
use crate::demo::Lang;

fn main() {
    let shell = Shell::new();
    let title = if shell.lang == Lang::Ja {
        "Hayate ウィジェット一覧"
    } else {
        "Hayate Widget Gallery"
    };

    let mut app = App::new(title, 1000, 700).with_min_size(600, 400);

    if let Some(ref app_theme) = shell.app_theme {
        let (base_theme, bar_theme) = match shell.theme_id {
            ThemeId::Win95       => (&hayate_ui::style::theme::WIN95_THEME,        TitleBarTheme::win95()),
            ThemeId::MacOs9      => (&hayate_ui::style::theme::MACOS9_THEME,       TitleBarTheme::mac_os9()),
            ThemeId::XpLuna      => (&hayate_ui::style::theme::XP_LUNA_THEME,      TitleBarTheme::xp_luna()),
            ThemeId::Win10       => (&hayate_ui::style::theme::WIN10_THEME,        TitleBarTheme::win10()),
            ThemeId::MacOsBigSur => (&hayate_ui::style::theme::MACOS_BIG_SUR_THEME, TitleBarTheme::macos_big_sur()),
            ThemeId::Default     => unreachable!(),
        };
        // Window decoration Phase 1: `App::new` defaults to
        // `Decorations::SystemLike`, so `.with_csd(title)` is dropped.
        // `.with_titlebar_theme(...)` is retained so the gallery can
        // continue to showcase the 5 TitleBarTheme presets per RFC §3.4.
        app = app
            .with_theme(base_theme)
            .with_app_theme(app_theme.clone())
            .with_titlebar_theme(bar_theme);
    }

    if let Err(e) = app.run(shell.build_root()) {
        eprintln!("gallery: {e}");
    }
}
