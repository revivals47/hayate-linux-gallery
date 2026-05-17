use hayate_platform::widget::core::Widget;
use hayate_kit::widget::ButtonWidget;
use hayate_kit::widget::grid_layout::GridLayout;

use crate::demo::{Category, Demo, DemoCtx, DemoEntry, Lang};

struct GridDemo;

impl Demo for GridDemo {
    fn id(&self) -> &'static str { "grid" }
    fn category(&self) -> Category { Category::Layout }
    fn title(&self, lang: Lang) -> &'static str {
        match lang { Lang::Ja => "グリッドレイアウト", Lang::En => "Grid Layout" }
    }
    fn build(&self, _ctx: &DemoCtx) -> Box<dyn Widget> {
        let mut g = GridLayout::new(3, 6.0);
        for i in 1..=9 {
            g = g.add(Box::new(ButtonWidget::new(format!("{i}"))));
        }
        Box::new(g)
    }
}

inventory::submit!(DemoEntry { demo: &GridDemo });
