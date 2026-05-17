use hayate_kit::widget::radio_button::RadioGroupWidget;
use hayate_platform::widget::core::Widget;

use crate::demo::{Category, Demo, DemoCtx, DemoEntry, Lang};

struct RadioDemo;

impl Demo for RadioDemo {
    fn id(&self) -> &'static str { "radio" }
    fn category(&self) -> Category { Category::Input }
    fn title(&self, lang: Lang) -> &'static str {
        match lang { Lang::Ja => "ラジオボタン", Lang::En => "Radio Group" }
    }
    fn build(&self, ctx: &DemoCtx) -> Box<dyn Widget> {
        let opts = match ctx.lang {
            Lang::Ja => vec!["小", "中", "大"],
            Lang::En => vec!["Small", "Medium", "Large"],
        };
        Box::new(
            RadioGroupWidget::new(&opts)
                .with_engine(ctx.engine.clone())
                .with_selected(1)
                .on_change(|i| println!("radio: {i}")),
        )
    }
}

inventory::submit!(DemoEntry { demo: &RadioDemo });
