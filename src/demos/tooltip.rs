use hayate_kit::widget::tooltip::TooltipWidget;
use hayate_platform::widget::core::Widget;
use hayate_kit::widget::{ButtonWidget, HStack};

use crate::demo::{Category, Demo, DemoCtx, DemoEntry, Lang};

struct TooltipDemo;

impl Demo for TooltipDemo {
    fn id(&self) -> &'static str { "tooltip" }
    fn category(&self) -> Category { Category::Overlay }
    fn title(&self, lang: Lang) -> &'static str {
        match lang { Lang::Ja => "ツールチップ", Lang::En => "Tooltip" }
    }
    fn build(&self, ctx: &DemoCtx) -> Box<dyn Widget> {
        let (hover_msg, tip) = match ctx.lang {
            Lang::Ja => ("ホバーしてみてください", "これはツールチップです"),
            Lang::En => ("Hover over me", "This is a tooltip"),
        };
        let btn = Box::new(ButtonWidget::new(hover_msg));
        Box::new(HStack::new(0.0).add(Box::new(TooltipWidget::new(btn, tip))))
    }
}

inventory::submit!(DemoEntry { demo: &TooltipDemo });
