use hayate_ui::widget::TextInputWidget;
use hayate_ui::widget::core::Widget;

use crate::demo::{Category, Demo, DemoCtx, DemoEntry, Lang};

struct TextInputDemo;

impl Demo for TextInputDemo {
    fn id(&self) -> &'static str { "text_input" }
    fn category(&self) -> Category { Category::Input }
    fn title(&self, lang: Lang) -> &'static str {
        match lang { Lang::Ja => "テキスト入力", Lang::En => "Text Input" }
    }
    fn build(&self, ctx: &DemoCtx) -> Box<dyn Widget> {
        let placeholder = match ctx.lang {
            Lang::Ja => "ここに入力...",
            Lang::En => "Type here...",
        };
        Box::new(
            TextInputWidget::new(ctx.engine.clone())
                .with_placeholder(placeholder)
                .with_width(240.0),
        )
    }
}

inventory::submit!(DemoEntry { demo: &TextInputDemo });
