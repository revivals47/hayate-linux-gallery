use hayate_ui::widget::core::Widget;
use hayate_ui::widget::{SliderWidget, VStack};

use crate::demo::{Category, Demo, DemoCtx, DemoEntry, Lang};

struct SliderDemo;

impl Demo for SliderDemo {
    fn id(&self) -> &'static str { "slider" }
    fn category(&self) -> Category { Category::Input }
    fn title(&self, lang: Lang) -> &'static str {
        match lang { Lang::Ja => "スライダー", Lang::En => "Slider" }
    }
    fn build(&self, _ctx: &DemoCtx) -> Box<dyn Widget> {
        let continuous = SliderWidget::new(0.0, 100.0, 30.0)
            .on_change(|v| println!("slider: {v:.1}"));
        let stepped    = SliderWidget::new(0.0, 10.0, 5.0).with_step(1.0);
        Box::new(
            VStack::new(8.0)
                .add(Box::new(continuous))
                .add(Box::new(stepped)),
        )
    }
}

inventory::submit!(DemoEntry { demo: &SliderDemo });
