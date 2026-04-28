// ToastWidget requires the full App area to position stacked toasts at the
// bottom-center; it's not embeddable in a demo card. The demo triggers a
// print; a real app mounts one ToastWidget at the root and calls
// `.show(msg, level, duration)` from anywhere via a shared handle.

use hayate_ui::widget::core::Widget;
use hayate_ui::widget::{ButtonWidget, HStack, TextWidget};

use crate::demo::{Category, Demo, DemoCtx, DemoEntry, Lang};

struct ToastDemo;

impl Demo for ToastDemo {
    fn id(&self) -> &'static str { "toast" }
    fn category(&self) -> Category { Category::Overlay }
    fn title(&self, lang: Lang) -> &'static str {
        match lang { Lang::Ja => "トースト", Lang::En => "Toast" }
    }
    fn build(&self, ctx: &DemoCtx) -> Box<dyn Widget> {
        let (info, warn, err, note) = match ctx.lang {
            Lang::Ja => ("情報", "警告", "エラー", "※ 本物はアプリルートに常駐"),
            Lang::En => ("Info", "Warn", "Error", "(app-root widget; demo logs only)"),
        };
        let info_btn  = ButtonWidget::new(info).on_click(|| println!("toast: info"));
        let warn_btn  = ButtonWidget::new(warn).on_click(|| println!("toast: warn"));
        let err_btn   = ButtonWidget::new(err).on_click(|| println!("toast: error"));
        let label = TextWidget::new(note, 11.0).with_engine(ctx.engine.clone());
        Box::new(
            HStack::new(8.0)
                .add(Box::new(info_btn))
                .add(Box::new(warn_btn))
                .add(Box::new(err_btn))
                .add(Box::new(label)),
        )
    }
}

inventory::submit!(DemoEntry { demo: &ToastDemo });
