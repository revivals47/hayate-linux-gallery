use hayate_platform::widget::core::Widget;
use hayate_kit::widget::VStack;
use hayate_kit::widget::checkbox::CheckboxWidget;

use crate::demo::{Category, Demo, DemoCtx, DemoEntry, Lang};

struct CheckboxDemo;

impl Demo for CheckboxDemo {
    fn id(&self) -> &'static str { "checkbox" }
    fn category(&self) -> Category { Category::Basic }
    fn title(&self, lang: Lang) -> &'static str {
        match lang { Lang::Ja => "チェックボックス", Lang::En => "Checkbox" }
    }
    fn build(&self, ctx: &DemoCtx) -> Box<dyn Widget> {
        let (a, b_label, c) = match ctx.lang {
            Lang::Ja => ("選択肢A", "選択肢B (オン)", "選択肢C"),
            Lang::En => ("Option A", "Option B (on)", "Option C"),
        };
        Box::new(
            VStack::new(6.0)
                .add(Box::new(CheckboxWidget::new(a)))
                .add(Box::new(CheckboxWidget::new(b_label).checked(true)))
                .add(Box::new(CheckboxWidget::new(c))),
        )
    }
}

inventory::submit!(DemoEntry { demo: &CheckboxDemo });
