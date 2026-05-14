use hayate_ui::widget::core::Widget;
use hayate_ui::widget::{CheckboxWidget, GroupBoxWidget, LabelWidget, VStack};

use crate::demo::{Category, Demo, DemoCtx, DemoEntry, Lang};

struct GroupBoxDemo;

impl Demo for GroupBoxDemo {
    fn id(&self) -> &'static str { "group_box" }
    fn category(&self) -> Category { Category::Layout }
    fn title(&self, lang: Lang) -> &'static str {
        match lang { Lang::Ja => "グループボックス", Lang::En => "Group Box" }
    }
    fn build(&self, _ctx: &DemoCtx) -> Box<dyn Widget> {
        let inner: Box<dyn Widget> = Box::new(
            VStack::new(6.0)
                .add(Box::new(CheckboxWidget::new("Enable X")))
                .add(Box::new(CheckboxWidget::new("Enable Y")))
                .add(Box::new(LabelWidget::new("Grouped content", 14.0))),
        );
        Box::new(GroupBoxWidget::new("Options", inner))
    }
}

inventory::submit!(DemoEntry { demo: &GroupBoxDemo });
