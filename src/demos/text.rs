use hayate_ui::widget::core::Widget;
use hayate_ui::widget::{LabelWidget, VStack};

use crate::demo::{Category, Demo, DemoCtx, DemoEntry, Lang};

struct TextDemo;

impl Demo for TextDemo {
    fn id(&self) -> &'static str { "text" }
    fn category(&self) -> Category { Category::Display }
    fn title(&self, lang: Lang) -> &'static str {
        match lang { Lang::Ja => "テキスト", Lang::En => "Text" }
    }
    fn build(&self, ctx: &DemoCtx) -> Box<dyn Widget> {
        let (default, larger) = match ctx.lang {
            Lang::Ja => ("LabelWidget のデフォルトスタイルです。", "大きめテキスト (18px)"),
            Lang::En => ("This is a LabelWidget with default styling.", "Larger text (18px)"),
        };
        Box::new(
            VStack::new(6.0)
                .add(Box::new(LabelWidget::new(default, 13.0)))
                .add(Box::new(LabelWidget::new(larger, 18.0))),
        )
    }
}

inventory::submit!(DemoEntry { demo: &TextDemo });
