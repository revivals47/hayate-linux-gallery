use hayate_ui::widget::{ResizeGripWidget};
use hayate_ui::widget::core::Widget;

use crate::demo::{Category, Demo, DemoCtx, DemoEntry, Lang};

struct ResizeGripDemo;

impl Demo for ResizeGripDemo {
    fn id(&self) -> &'static str { "resize_grip" }
    fn category(&self) -> Category { Category::Misc }
    fn title(&self, lang: Lang) -> &'static str {
        match lang { Lang::Ja => "リサイズグリップ", Lang::En => "Resize Grip" }
    }
    fn build(&self, _ctx: &DemoCtx) -> Box<dyn Widget> {
        Box::new(ResizeGripWidget::new())
    }
}

inventory::submit!(DemoEntry { demo: &ResizeGripDemo });
