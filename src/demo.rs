//! Demo trait + registry.
//!
//! 1 demo = 1 self-contained file under `demos/`. A demo file imports only
//! `Demo`, `DemoCtx`, `Category`, `Lang` from this module and the widgets it
//! wants to showcase from `hayate_ui`. It never touches `App`, other demos,
//! or global state.
//!
//! Registration is via `inventory::submit!` at the bottom of each demo file;
//! the shell discovers demos through `inventory::iter::<DemoEntry>`.

use std::cell::RefCell;
use std::rc::Rc;

use hayate_ui::render::TextEngine;
use hayate_ui::widget::AppTheme;
use hayate_ui::widget::core::Widget;

#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
pub enum Lang {
    En,
    Ja,
}

impl Lang {
    pub fn from_args() -> Self {
        if std::env::args().any(|a| a == "--ja" || a == "-j") { Lang::Ja } else { Lang::En }
    }
}

/// Top-level grouping for sidebar/tab organisation. `Ord` controls tab order.
#[derive(Clone, Copy, PartialEq, Eq, Debug, PartialOrd, Ord)]
pub enum Category {
    Basic,
    Input,
    Display,
    Layout,
    Navigation,
    Overlay,
    Misc,
}

impl Category {
    pub fn label(self, lang: Lang) -> &'static str {
        match (self, lang) {
            (Category::Basic,      Lang::En) => "Basic",      (Category::Basic,      Lang::Ja) => "基本",
            (Category::Input,      Lang::En) => "Input",      (Category::Input,      Lang::Ja) => "入力",
            (Category::Display,    Lang::En) => "Display",    (Category::Display,    Lang::Ja) => "表示",
            (Category::Layout,     Lang::En) => "Layout",     (Category::Layout,     Lang::Ja) => "配置",
            (Category::Navigation, Lang::En) => "Navigation", (Category::Navigation, Lang::Ja) => "案内",
            (Category::Overlay,    Lang::En) => "Overlay",    (Category::Overlay,    Lang::Ja) => "重ね",
            (Category::Misc,       Lang::En) => "Misc",       (Category::Misc,       Lang::Ja) => "他",
        }
    }

    pub fn all() -> &'static [Category] {
        &[Category::Basic, Category::Input, Category::Display,
          Category::Layout, Category::Navigation, Category::Overlay, Category::Misc]
    }
}

/// Everything a demo is allowed to read from the outside world.
///
/// Theme/engine flow through the widget tree via `inject_theme` / `inject_engine`
/// at run time, so demos rarely need to touch these fields. They're here as a
/// convenience for demos that must size or paint at build time (e.g. a custom
/// CanvasView that measures text before the first frame).
pub struct DemoCtx {
    pub lang: Lang,
    #[allow(dead_code)]
    pub theme: Option<AppTheme>,
    pub engine: Rc<RefCell<TextEngine>>,
}

pub trait Demo: Sync + 'static {
    fn id(&self) -> &'static str;
    fn category(&self) -> Category;
    fn title(&self, lang: Lang) -> &'static str;
    fn build(&self, ctx: &DemoCtx) -> Box<dyn Widget>;
}

/// Wrapper so `inventory` can collect `&dyn Demo` (trait objects can't be
/// `inventory::Collect` directly because they need a concrete type).
pub struct DemoEntry {
    pub demo: &'static dyn Demo,
}

inventory::collect!(DemoEntry);

/// All registered demos in a deterministic order (category first, then id).
pub fn all() -> Vec<&'static dyn Demo> {
    let mut v: Vec<&'static dyn Demo> = inventory::iter::<DemoEntry>
        .into_iter()
        .map(|e| e.demo)
        .collect();
    v.sort_by_key(|d| (d.category(), d.id()));
    v
}
