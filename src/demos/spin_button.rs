use hayate_ui::widget::SpinButtonWidget;
use hayate_ui::widget::core::Widget;

use crate::demo::{Category, Demo, DemoCtx, DemoEntry, Lang};

struct SpinButtonDemo;

impl Demo for SpinButtonDemo {
    fn id(&self) -> &'static str { "spin_button" }
    fn category(&self) -> Category { Category::Input }
    fn title(&self, lang: Lang) -> &'static str {
        match lang { Lang::Ja => "スピンボタン", Lang::En => "Spin Button" }
    }
    fn build(&self, _ctx: &DemoCtx) -> Box<dyn Widget> {
        Box::new(
            SpinButtonWidget::new(0.0, 100.0, 10.0, 1.0)
                .on_change(|v| println!("spin: {v:.0}")),
        )
    }
}

inventory::submit!(DemoEntry { demo: &SpinButtonDemo });
