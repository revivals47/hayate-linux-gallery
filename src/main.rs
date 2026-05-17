//! Gallery launcher. All real work lives in `app::Shell` and `demos/`.

mod app;
mod chrome;
mod demo;
mod demos;

use hayate_platform::App;
// Phase L2 Stage 3 Option C: vendor preset factories live in L2
// (L0 v2.0 §17 + L0 v0.5 round 4: retro/vendor preset L2 配置確定).
use hayate_kit::style::widget_theme_presets::titlebar::{
    titlebar_theme_mac_os9, titlebar_theme_macos_big_sur, titlebar_theme_win10,
    titlebar_theme_win95, titlebar_theme_xp_luna,
};

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
            ThemeId::Win95       => (&hayate_kit::style::theme::WIN95_THEME,        titlebar_theme_win95()),
            ThemeId::MacOs9      => (&hayate_kit::style::theme::MACOS9_THEME,       titlebar_theme_mac_os9()),
            ThemeId::XpLuna      => (&hayate_kit::style::theme::XP_LUNA_THEME,      titlebar_theme_xp_luna()),
            ThemeId::Win10       => (&hayate_kit::style::theme::WIN10_THEME,        titlebar_theme_win10()),
            ThemeId::MacOsBigSur => (&hayate_kit::style::theme::MACOS_BIG_SUR_THEME, titlebar_theme_macos_big_sur()),
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
