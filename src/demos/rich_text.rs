use hayate_ui::widget::RichTextWidget;
use hayate_ui::widget::core::Widget;
use hayate_ui::widget::VStack;

use crate::demo::{Category, Demo, DemoCtx, DemoEntry, Lang};

struct RichTextDemo;

impl Demo for RichTextDemo {
    fn id(&self) -> &'static str { "rich_text" }
    fn category(&self) -> Category { Category::Display }
    fn title(&self, lang: Lang) -> &'static str {
        match lang { Lang::Ja => "リッチテキスト", Lang::En => "Rich Text" }
    }
    fn build(&self, ctx: &DemoCtx) -> Box<dyn Widget> {
        let (primary, muted, secondary, accent) = match ctx.lang {
            Lang::Ja => ("通常テキスト", "控えめ (muted)", "二次 (secondary)", "アクセント"),
            Lang::En => ("Primary text", "Muted text", "Secondary text", "Accent text"),
        };
        let mk = |s: &str| RichTextWidget::new(s, 13.0).with_engine(ctx.engine.clone());
        Box::new(
            VStack::new(4.0)
                .add(Box::new(mk(primary)))
                .add(Box::new(mk(muted).with_muted()))
                .add(Box::new(mk(secondary).with_secondary()))
                .add(Box::new(mk(accent).with_accent())),
        )
    }
}

inventory::submit!(DemoEntry { demo: &RichTextDemo });
