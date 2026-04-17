//! App shell: assembles chrome + category tabs + demo cards from the registry.
//!
//! The shell doesn't know about any specific demo — it walks `demo::all()`,
//! groups by `Category`, and builds one tab per category with a vertical
//! stack of demo cards inside. Adding a new demo is a pure `demos/` operation.

use std::cell::RefCell;
use std::rc::Rc;

use hayate_ui::render::TextEngine;
use hayate_ui::widget::core::Widget;
use hayate_ui::widget::win95::STATUS_BAR_HEIGHT;
use hayate_ui::widget::{
    AppTheme, HStack, Padding, Spacer, StatusBar, StatusItem, TabEntry,
    TabViewWidget, TextWidget, VStack, Win95FrameWidget,
};

use crate::chrome::ThemeId;
use crate::demo::{self, Category, DemoCtx, Lang};

/// Wrap a demo's widget with a header row (title on the left, id on the right)
/// plus some padding so cards are visually distinct on the tab.
fn demo_card(
    engine: Rc<RefCell<TextEngine>>,
    title: &str,
    id: &str,
    themed: bool,
    body: Box<dyn Widget>,
) -> Box<dyn Widget> {
    let title_color = if themed { (0, 0, 0) } else { (235, 235, 235) };
    let id_color    = if themed { (90, 90, 90) } else { (140, 140, 160) };
    let (tr, tg, tb) = title_color;
    let (ir, ig, ib) = id_color;

    let header = HStack::new(8.0)
        .add(Box::new(
            TextWidget::new(title, 14.0)
                .with_engine(engine.clone())
                .with_color(tr, tg, tb),
        ))
        .add(Box::new(Spacer::new()))
        .add(Box::new(
            TextWidget::new(id, 10.0)
                .with_engine(engine.clone())
                .with_color(ir, ig, ib),
        ));

    let inner = VStack::new(8.0)
        .add(Box::new(header))
        .add(body);

    Box::new(Padding::symmetric(12.0, 10.0, Box::new(inner)))
}

/// Build a VStack of all demos for one category.
fn build_category_body(
    cat: Category,
    ctx: &DemoCtx,
    themed: bool,
) -> Box<dyn Widget> {
    let mut stack = VStack::new(8.0);
    for d in demo::all() {
        if d.category() != cat { continue; }
        let body = d.build(ctx);
        stack = stack.add(demo_card(
            ctx.engine.clone(),
            d.title(ctx.lang),
            d.id(),
            themed,
            body,
        ));
    }
    Box::new(Padding::symmetric(6.0, 6.0, Box::new(stack)))
}

/// Build the main content area: TabView with one tab per non-empty category.
pub fn build_content(ctx: &DemoCtx, themed: bool) -> Box<dyn Widget> {
    let mut tabs = TabViewWidget::new();
    for &cat in Category::all() {
        let has_any = demo::all().iter().any(|d| d.category() == cat);
        if !has_any { continue; }
        tabs = tabs.add_tab(TabEntry::new(
            cat.label(ctx.lang).to_string(),
            build_category_body(cat, ctx, themed),
        ));
    }
    Box::new(tabs)
}

pub struct Shell {
    pub engine: Rc<RefCell<TextEngine>>,
    pub theme_id: ThemeId,
    pub lang: Lang,
    pub app_theme: Option<AppTheme>,
}

impl Shell {
    pub fn new() -> Self {
        let lang = Lang::from_args();
        let theme_id = ThemeId::from_args();
        let engine = Rc::new(RefCell::new(TextEngine::new()));
        let app_theme = theme_id.app_theme();
        Self { engine, theme_id, lang, app_theme }
    }

    /// Assemble the full root widget: chrome bar + content + status bar.
    pub fn build_root(&self) -> Box<dyn Widget> {
        let themed = self.app_theme.is_some();
        let ctx = DemoCtx {
            lang: self.lang,
            theme: self.app_theme.clone(),
            engine: self.engine.clone(),
        };
        let chrome_bar = crate::chrome::build(
            self.engine.clone(), self.theme_id, self.lang, self.app_theme.as_ref(),
        );
        let content = build_content(&ctx, themed);
        let status_bar = build_status_bar(self.theme_id, self.lang, demo::all().len());
        let shell: Box<dyn Widget> = Box::new(
            VStack::new(0.0)
                .add(chrome_bar)
                .add_flex(content, 1.0)
                .add(Box::new(status_bar)),
        );
        // Win95FrameWidget paints a Win95-palette 2-stage raised bevel, so it
        // only fits under the Win95 skin. Other themes (macOS / XP / Win10)
        // ship their own window aesthetics and would clash with a hard grey
        // bevel, so they keep the naked VStack at the root.
        if self.theme_id == ThemeId::Win95 {
            Box::new(Win95FrameWidget::from_box(shell))
        } else {
            shell
        }
    }
}

/// Win95-style bottom status bar: theme on the left, demo count + lang on the
/// right. The bar paints as 22 px BUTTON_FACE grey with a 1 px raised top
/// edge when the app is running under a Win95-family skin (handled by
/// `StatusBar::inject_theme`); other skins keep the hard-coded grey so the
/// footer is always present. Labels render through
/// `AppTheme::bitmap_text_default` when a bitmap font is wired in.
fn build_status_bar(theme_id: ThemeId, lang: Lang, demo_count: usize) -> StatusBar {
    let mut bar = StatusBar::new(STATUS_BAR_HEIGHT).with_bg(192, 192, 192, 255);
    let theme_label = match lang {
        Lang::En => format!("Theme: {}", theme_id.label()),
        Lang::Ja => format!("テーマ: {}", theme_id.label()),
    };
    let demos_label = match lang {
        Lang::En => format!("{demo_count} demos"),
        Lang::Ja => format!("{demo_count} デモ"),
    };
    let lang_label = match lang {
        Lang::En => "EN",
        Lang::Ja => "日本語",
    };
    bar.add_left(StatusItem::new(theme_label).with_color(0, 0, 0));
    bar.add_right(StatusItem::new(lang_label).with_color(0, 0, 0));
    bar.add_right(StatusItem::new(demos_label).with_color(0, 0, 0));
    bar
}
