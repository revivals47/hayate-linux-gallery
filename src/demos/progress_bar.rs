use hayate_ui::widget::core::Widget;
use hayate_ui::widget::{ProgressBarWidget, VStack};

use crate::demo::{Category, Demo, DemoCtx, DemoEntry, Lang};

struct ProgressBarDemo;

impl Demo for ProgressBarDemo {
    fn id(&self) -> &'static str { "progress_bar" }
    fn category(&self) -> Category { Category::Display }
    fn title(&self, lang: Lang) -> &'static str {
        match lang { Lang::Ja => "プログレスバー", Lang::En => "Progress Bar" }
    }
    fn build(&self, _ctx: &DemoCtx) -> Box<dyn Widget> {
        Box::new(
            VStack::new(8.0)
                .add(Box::new(ProgressBarWidget::new(0.25)))
                .add(Box::new(ProgressBarWidget::new(0.60)))
                .add(Box::new(ProgressBarWidget::indeterminate())),
        )
    }
}

inventory::submit!(DemoEntry { demo: &ProgressBarDemo });
