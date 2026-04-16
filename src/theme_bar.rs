//! Theme switcher toolbar. Re-launches the gallery with the chosen flag.
//!
//! Hot-swapping widget themes would require walking the entire widget tree
//! and mutating per-widget theme fields; that's a significant API addition.
//! For a demo app, spawning a fresh process is simpler and matches what
//! users expect from a theme picker in classic applications.

use std::cell::RefCell;
use std::rc::Rc;

use hayate_ui::render::TextEngine;
use hayate_ui::widget::core::Widget;
use hayate_ui::widget::{
    AppTheme, ButtonWidget, ButtonTheme, HStack, Padding, TextWidget,
};

/// Name of the active theme at launch time.
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
        if args.iter().any(|a| a == "--win95") { ThemeId::Win95 }
        else if args.iter().any(|a| a == "--macos9") { ThemeId::MacOs9 }
        else if args.iter().any(|a| a == "--xp") { ThemeId::XpLuna }
        else if args.iter().any(|a| a == "--win10") { ThemeId::Win10 }
        else if args.iter().any(|a| a == "--macos") { ThemeId::MacOsBigSur }
        else { ThemeId::Default }
    }

    fn arg(self) -> Option<&'static str> {
        match self {
            ThemeId::Default => None,
            ThemeId::Win95 => Some("--win95"),
            ThemeId::MacOs9 => Some("--macos9"),
            ThemeId::XpLuna => Some("--xp"),
            ThemeId::Win10 => Some("--win10"),
            ThemeId::MacOsBigSur => Some("--macos"),
        }
    }
}

fn relaunch(theme: ThemeId) {
    let exe = match std::env::current_exe() {
        Ok(p) => p,
        Err(e) => {
            eprintln!("theme switcher: cannot find current exe: {e}");
            return;
        }
    };
    let mut cmd = std::process::Command::new(exe);
    if let Some(a) = theme.arg() { cmd.arg(a); }
    // Preserve language flag across relaunch.
    for a in std::env::args().skip(1) {
        if a == "--ja" || a == "-j" || a == "--en" {
            cmd.arg(a);
        }
    }
    match cmd.spawn() {
        Ok(_) => std::process::exit(0),
        Err(e) => eprintln!("theme switcher: spawn failed: {e}"),
    }
}

/// Build the theme-picker toolbar for the top of the window.
///
/// `current` is the theme this process was launched with; the matching
/// button is drawn in a pressed style so the user can tell which theme
/// is active. Other buttons respawn the gallery with the chosen flag.
pub fn build(
    engine: Rc<RefCell<TextEngine>>,
    current: ThemeId,
    theme: Option<&AppTheme>,
) -> Box<dyn Widget> {
    let themed = theme.is_some();
    let mut label = TextWidget::new("Theme:", if themed { 11.0 } else { 13.0 })
        .with_engine(engine.clone());
    if themed { label = label.with_color(0, 0, 0); }

    let btn = |name: &str, id: ThemeId, active: bool| -> Box<dyn Widget> {
        let base = if let Some(t) = theme {
            t.button.clone()
        } else {
            ButtonTheme::filled()
        };
        let bt = if active {
            // Visual hint: active theme's button uses a darker face.
            // For Win95 (no hover), swap bg to a darker grey.
            if themed {
                base.clone()
                    .bg(hayate_ui::style::theme::Color::rgb(160, 160, 160))
            } else {
                base.clone()
                    .bg(hayate_ui::style::theme::Color::rgb(40, 110, 200))
            }
        } else {
            base
        };
        let mut b = ButtonWidget::new(name).theme(bt);
        if !active {
            b = b.on_click(move || relaunch(id));
        }
        Box::new(b)
    };

    let row = HStack::new(6.0)
        .add(Box::new(label))
        .add(btn("Default", ThemeId::Default, current == ThemeId::Default))
        .add(btn("Win95", ThemeId::Win95, current == ThemeId::Win95))
        .add(btn("Mac OS 9", ThemeId::MacOs9, current == ThemeId::MacOs9))
        .add(btn("XP Luna", ThemeId::XpLuna, current == ThemeId::XpLuna))
        .add(btn("Win10", ThemeId::Win10, current == ThemeId::Win10))
        .add(btn("macOS", ThemeId::MacOsBigSur, current == ThemeId::MacOsBigSur));

    Box::new(Padding::symmetric(10.0, 6.0, Box::new(row)))
}
