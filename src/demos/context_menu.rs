// ContextMenu is positioned via `show(x, y)` in absolute screen coordinates
// and expects to be mounted at the App root so it can render above all
// other widgets. The gallery shows a trigger button; real apps wire the
// right-click handler to `menu.show(click_x, click_y)`.

use hayate_ui::widget::core::Widget;
use hayate_ui::widget::{ButtonWidget, HStack, LabelWidget};

use crate::demo::{Category, Demo, DemoCtx, DemoEntry, Lang};

struct ContextMenuDemo;

impl Demo for ContextMenuDemo {
    fn id(&self) -> &'static str { "context_menu" }
    fn category(&self) -> Category { Category::Overlay }
    fn title(&self, lang: Lang) -> &'static str {
        match lang { Lang::Ja => "コンテキストメニュー", Lang::En => "Context Menu" }
    }
    fn build(&self, ctx: &DemoCtx) -> Box<dyn Widget> {
        let (btn, note) = match ctx.lang {
            Lang::Ja => ("メニューを表示", "※ 本物は右クリック座標で show(x,y)"),
            Lang::En => ("Trigger Menu", "(real usage: show(x,y) on right-click)"),
        };
        let b = ButtonWidget::new(btn).on_click(|| {
            println!("context_menu: would show at cursor (copy / paste / delete)");
        });
        let label = LabelWidget::new(note, 11.0).with_engine(ctx.engine.clone());
        Box::new(HStack::new(12.0).add(Box::new(b)).add(Box::new(label)))
    }
}

inventory::submit!(DemoEntry { demo: &ContextMenuDemo });
