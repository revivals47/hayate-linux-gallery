// AlertDialog is an app-level modal — its layout consumes the full parent
// rect (semi-transparent overlay + centered dialog box), so it can't be
// embedded inside a gallery demo card without breaking the row. The demo
// exposes a trigger button and logs; real apps construct AlertDialog at
// the App root and call `.show()` from anywhere via a shared handle.

use hayate_ui::widget::core::Widget;
use hayate_ui::widget::{ButtonWidget, HStack, LabelWidget};

use crate::demo::{Category, Demo, DemoCtx, DemoEntry, Lang};

struct AlertDialogDemo;

impl Demo for AlertDialogDemo {
    fn id(&self) -> &'static str { "alert_dialog" }
    fn category(&self) -> Category { Category::Overlay }
    fn title(&self, lang: Lang) -> &'static str {
        match lang { Lang::Ja => "アラートダイアログ", Lang::En => "Alert Dialog" }
    }
    fn build(&self, ctx: &DemoCtx) -> Box<dyn Widget> {
        let (btn_label, note) = match ctx.lang {
            Lang::Ja => ("アラートを開く", "※ 実際のモーダルはアプリルートで表示"),
            Lang::En => ("Show Alert", "(app-root modal; demo logs to stdout)"),
        };
        let btn = ButtonWidget::new(btn_label).on_click(|| {
            println!("alert: user requested dialog (would overlay the entire window)");
        });
        let label = LabelWidget::new(note, 11.0).with_engine(ctx.engine.clone());
        Box::new(HStack::new(12.0).add(Box::new(btn)).add(Box::new(label)))
    }
}

inventory::submit!(DemoEntry { demo: &AlertDialogDemo });
