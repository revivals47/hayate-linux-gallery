//! App shell: assembles chrome + category tabs + demo cards from the registry.
//!
//! The shell doesn't know about any specific demo — it walks `demo::all()`,
//! groups by `Category`, and builds one tab per category with a vertical
//! stack of demo cards inside. Adding a new demo is a pure `demos/` operation.

use std::cell::RefCell;
use std::rc::Rc;

use hayate_ui::render::TextEngine;
use hayate_ui::widget::core::Widget;
use hayate_ui::widget::{
    AppTheme, HStack, Padding, Spacer, TabEntry, TabViewWidget, TextWidget, VStack,
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

    /// Assemble the full root widget: chrome bar + content.
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
        Box::new(VStack::new(0.0).add(chrome_bar).add(content))
    }
}
