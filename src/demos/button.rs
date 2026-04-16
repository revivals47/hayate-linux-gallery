use hayate_ui::widget::core::Widget;
use hayate_ui::widget::{ButtonWidget, HStack};

use crate::demo::{Category, Demo, DemoCtx, DemoEntry, Lang};

struct ButtonDemo;

impl Demo for ButtonDemo {
    fn id(&self) -> &'static str { "button" }
    fn category(&self) -> Category { Category::Basic }
    fn title(&self, lang: Lang) -> &'static str {
        match lang { Lang::Ja => "ボタン", Lang::En => "Button" }
    }
    fn build(&self, _ctx: &DemoCtx) -> Box<dyn Widget> {
        let primary = ButtonWidget::new("Primary").on_click(|| println!("button: primary"));
        let action  = ButtonWidget::new("Action") .on_click(|| println!("button: action"));
        let muted   = ButtonWidget::new("Muted");
        Box::new(
            HStack::new(8.0)
                .add(Box::new(primary))
                .add(Box::new(action))
                .add(Box::new(muted)),
        )
    }
}

inventory::submit!(DemoEntry { demo: &ButtonDemo });
