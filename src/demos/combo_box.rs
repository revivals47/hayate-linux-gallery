use hayate_kit::widget::combo_box::ComboBoxWidget;
use hayate_platform::widget::core::Widget;

use crate::demo::{Category, Demo, DemoCtx, DemoEntry, Lang};

struct ComboBoxDemo;

impl Demo for ComboBoxDemo {
    fn id(&self) -> &'static str { "combo_box" }
    fn category(&self) -> Category { Category::Input }
    fn title(&self, lang: Lang) -> &'static str {
        match lang { Lang::Ja => "コンボボックス", Lang::En => "Combo Box" }
    }
    fn build(&self, ctx: &DemoCtx) -> Box<dyn Widget> {
        let items = match ctx.lang {
            Lang::Ja => vec!["東京".into(), "大阪".into(), "名古屋".into()],
            Lang::En => vec!["Tokyo".into(), "Osaka".into(), "Nagoya".into()],
        };
        Box::new(ComboBoxWidget::new(items).on_select(|s| println!("combo: {s}")))
    }
}

inventory::submit!(DemoEntry { demo: &ComboBoxDemo });
