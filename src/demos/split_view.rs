use hayate_ui::widget::core::Widget;
use hayate_ui::widget::{SplitOrientation, SplitViewWidget, TextWidget};

use crate::demo::{Category, Demo, DemoCtx, DemoEntry, Lang};

struct SplitViewDemo;

impl Demo for SplitViewDemo {
    fn id(&self) -> &'static str { "split_view" }
    fn category(&self) -> Category { Category::Layout }
    fn title(&self, lang: Lang) -> &'static str {
        match lang { Lang::Ja => "スプリットビュー", Lang::En => "Split View" }
    }
    fn build(&self, ctx: &DemoCtx) -> Box<dyn Widget> {
        let (l, r) = match ctx.lang {
            Lang::Ja => ("左ペイン", "右ペイン"),
            Lang::En => ("Left pane", "Right pane"),
        };
        let left  = Box::new(TextWidget::new(l, 13.0).with_engine(ctx.engine.clone()));
        let right = Box::new(TextWidget::new(r, 13.0).with_engine(ctx.engine.clone()));
        Box::new(
            SplitViewWidget::new(left, right, SplitOrientation::Horizontal)
                .with_ratio(0.4)
                .with_min_sizes(80.0, 80.0),
        )
    }
}

inventory::submit!(DemoEntry { demo: &SplitViewDemo });
