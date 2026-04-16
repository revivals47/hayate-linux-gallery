use hayate_ui::widget::DropdownWidget;
use hayate_ui::widget::core::Widget;

use crate::demo::{Category, Demo, DemoCtx, DemoEntry, Lang};

struct DropdownDemo;

impl Demo for DropdownDemo {
    fn id(&self) -> &'static str { "dropdown" }
    fn category(&self) -> Category { Category::Input }
    fn title(&self, lang: Lang) -> &'static str {
        match lang { Lang::Ja => "ドロップダウン", Lang::En => "Dropdown" }
    }
    fn build(&self, ctx: &DemoCtx) -> Box<dyn Widget> {
        let items = match ctx.lang {
            Lang::Ja => vec!["りんご".into(), "みかん".into(), "ぶどう".into(), "いちご".into()],
            Lang::En => vec!["Apple".into(), "Orange".into(), "Grape".into(), "Strawberry".into()],
        };
        Box::new(
            DropdownWidget::new(items)
                .with_selected(0)
                .on_select(|i, s| println!("dropdown: [{i}] {s}")),
        )
    }
}

inventory::submit!(DemoEntry { demo: &DropdownDemo });
