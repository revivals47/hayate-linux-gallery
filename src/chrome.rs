//! Top-of-window chrome: theme picker + language picker.
//!
//! Kept independent of `Demo` — this bar is part of the app shell, not the
//! gallery content. Selecting a theme / language respawns the process with
//! the appropriate CLI flags, which is simpler than hot-swapping the entire
//! widget tree and matches what classic theme pickers do.

use std::cell::RefCell;
use std::rc::Rc;

use hayate_platform::render::TextEngine;
use hayate_platform::widget::core::Widget;
use hayate_platform::widget_themes::app::AppTheme;
use hayate_platform::widget_themes::button::ButtonTheme;
use hayate_kit::widget::{ButtonWidget, HStack, LabelWidget, Padding};
// Phase L2 Stage 3 Option C: vendor preset factories live in L2
// (L0 v2.0 §17 + L0 v0.5 round 4: retro/vendor preset L2 配置確定).
use hayate_kit::style::widget_theme_presets::app::{
    app_theme_mac_os9, app_theme_macos_big_sur, app_theme_win10, app_theme_win95, app_theme_xp_luna,
};

use crate::demo::Lang;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum ThemeId {
    Default,
    Win95,
    MacOs9,
    XpLuna,
    Win10,
    MacOsBigSur,
}

impl ThemeId {
    pub fn from_args() -> Self {
        let args: Vec<String> = std::env::args().collect();
        if args.iter().any(|a| a == "--win95")  { ThemeId::Win95 }
        else if args.iter().any(|a| a == "--macos9") { ThemeId::MacOs9 }
        else if args.iter().any(|a| a == "--xp")     { ThemeId::XpLuna }
        else if args.iter().any(|a| a == "--win10")  { ThemeId::Win10 }
        else if args.iter().any(|a| a == "--macos")  { ThemeId::MacOsBigSur }
        else                                         { ThemeId::Default }
    }

    pub fn app_theme(self) -> Option<AppTheme> {
        match self {
            ThemeId::Default     => None,
            ThemeId::Win95       => Some(app_theme_win95()),
            ThemeId::MacOs9      => Some(app_theme_mac_os9()),
            ThemeId::XpLuna      => Some(app_theme_xp_luna()),
            ThemeId::Win10       => Some(app_theme_win10()),
            ThemeId::MacOsBigSur => Some(app_theme_macos_big_sur()),
        }
    }

    fn flag(self) -> Option<&'static str> {
        match self {
            ThemeId::Default     => None,
            ThemeId::Win95       => Some("--win95"),
            ThemeId::MacOs9      => Some("--macos9"),
            ThemeId::XpLuna      => Some("--xp"),
            ThemeId::Win10       => Some("--win10"),
            ThemeId::MacOsBigSur => Some("--macos"),
        }
    }

    pub fn label(self) -> &'static str {
        match self {
            ThemeId::Default     => "Default",
            ThemeId::Win95       => "Win95",
            ThemeId::MacOs9      => "Mac OS 9",
            ThemeId::XpLuna      => "XP Luna",
            ThemeId::Win10       => "Win10",
            ThemeId::MacOsBigSur => "macOS",
        }
    }

    fn all() -> &'static [ThemeId] {
        &[ThemeId::Default, ThemeId::Win95, ThemeId::MacOs9,
          ThemeId::XpLuna, ThemeId::Win10, ThemeId::MacOsBigSur]
    }
}

/// Respawn the gallery process with the given theme and language flags.
fn relaunch(theme: ThemeId, lang: Lang) {
    let exe = match std::env::current_exe() {
        Ok(p) => p,
        Err(e) => { eprintln!("chrome: current_exe failed: {e}"); return; }
    };
    let mut cmd = std::process::Command::new(exe);
    if let Some(f) = theme.flag() { cmd.arg(f); }
    if matches!(lang, Lang::Ja)  { cmd.arg("--ja"); }
    match cmd.spawn() {
        Ok(_)  => std::process::exit(0),
        Err(e) => eprintln!("chrome: spawn failed: {e}"),
    }
}

/// Build the top toolbar: theme picker on the left, language toggle on the right.
pub fn build(
    engine: Rc<RefCell<TextEngine>>,
    current_theme: ThemeId,
    current_lang: Lang,
    theme: Option<&AppTheme>,
) -> Box<dyn Widget> {
    let themed = theme.is_some();
    let label_text = if current_lang == Lang::Ja { "テーマ:" } else { "Theme:" };
    let mut theme_label = LabelWidget::new(label_text, if themed { 11.0 } else { 13.0 })
        .with_engine(engine.clone());
    if themed { theme_label = theme_label.with_color(0, 0, 0); }

    let theme_btn = |id: ThemeId, active: bool| -> Box<dyn Widget> {
        let base = theme.map(|t| t.button.clone()).unwrap_or_else(ButtonTheme::filled);
        let bt = if active {
            if themed {
                base.clone().bg(hayate_kit::style::theme::Color::rgb(160, 160, 160))
            } else {
                base.clone().bg(hayate_kit::style::theme::Color::rgb(40, 110, 200))
            }
        } else {
            base
        };
        let mut b = ButtonWidget::new(id.label()).theme(bt);
        if !active {
            b = b.on_click(move || relaunch(id, current_lang));
        }
        Box::new(b)
    };

    let lang_btn = |lang: Lang, label: &str, active: bool| -> Box<dyn Widget> {
        let base = theme.map(|t| t.button.clone()).unwrap_or_else(ButtonTheme::filled);
        let bt = if active {
            if themed {
                base.clone().bg(hayate_kit::style::theme::Color::rgb(160, 160, 160))
            } else {
                base.clone().bg(hayate_kit::style::theme::Color::rgb(40, 110, 200))
            }
        } else {
            base
        };
        let mut b = ButtonWidget::new(label).theme(bt);
        if !active {
            b = b.on_click(move || relaunch(current_theme, lang));
        }
        Box::new(b)
    };

    let mut row = HStack::new(6.0).add(Box::new(theme_label));
    for &id in ThemeId::all() {
        row = row.add(theme_btn(id, id == current_theme));
    }
    row = row
        .add(Box::new(LabelWidget::new("  ", 11.0).with_engine(engine.clone())))
        .add(lang_btn(Lang::En, "EN", current_lang == Lang::En))
        .add(lang_btn(Lang::Ja, "JA", current_lang == Lang::Ja));

    Box::new(Padding::symmetric(10.0, 6.0, Box::new(row)))
}
