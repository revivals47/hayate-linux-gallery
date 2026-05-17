use hayate_platform::widget::core::Widget;
use hayate_kit::widget::toolbar::ToolbarWidget;

use crate::demo::{Category, Demo, DemoCtx, DemoEntry, Lang};

struct ToolbarDemo;

impl Demo for ToolbarDemo {
    fn id(&self) -> &'static str { "toolbar" }
    fn category(&self) -> Category { Category::Navigation }
    fn title(&self, lang: Lang) -> &'static str {
        match lang { Lang::Ja => "ツールバー", Lang::En => "Toolbar" }
    }
    fn build(&self, ctx: &DemoCtx) -> Box<dyn Widget> {
        let (new, open, save) = match ctx.lang {
            Lang::Ja => ("新規", "開く", "保存"),
            Lang::En => ("New", "Open", "Save"),
        };
        Box::new(
            ToolbarWidget::new()
                .add_button(new,  || println!("toolbar: new"))
                .add_button(open, || println!("toolbar: open"))
                .add_separator()
                .add_button(save, || println!("toolbar: save")),
        )
    }
}

inventory::submit!(DemoEntry { demo: &ToolbarDemo });
