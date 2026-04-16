use hayate_ui::widget::core::Widget;
use hayate_ui::widget::{HStack, SwitchWidget, TextWidget};

use crate::demo::{Category, Demo, DemoCtx, DemoEntry, Lang};

struct SwitchDemo;

impl Demo for SwitchDemo {
    fn id(&self) -> &'static str { "switch" }
    fn category(&self) -> Category { Category::Input }
    fn title(&self, lang: Lang) -> &'static str {
        match lang { Lang::Ja => "スイッチ", Lang::En => "Switch" }
    }
    fn build(&self, ctx: &DemoCtx) -> Box<dyn Widget> {
        let (dark, notif) = match ctx.lang {
            Lang::Ja => ("ダークモード", "通知"),
            Lang::En => ("Dark Mode", "Notifications"),
        };
        let label = |s: &str| TextWidget::new(s, 13.0).with_engine(ctx.engine.clone());
        Box::new(
            HStack::new(16.0)
                .add(Box::new(label(dark)))
                .add(Box::new(SwitchWidget::new(false).on_toggle(|v| println!("switch dark: {v}"))))
                .add(Box::new(label(notif)))
                .add(Box::new(SwitchWidget::new(true).on_toggle(|v| println!("switch notif: {v}")))),
        )
    }
}

inventory::submit!(DemoEntry { demo: &SwitchDemo });
