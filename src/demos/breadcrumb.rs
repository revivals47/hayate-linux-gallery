use hayate_ui::widget::BreadcrumbWidget;
use hayate_ui::widget::core::Widget;

use crate::demo::{Category, Demo, DemoCtx, DemoEntry, Lang};

struct BreadcrumbDemo;

impl Demo for BreadcrumbDemo {
    fn id(&self) -> &'static str { "breadcrumb" }
    fn category(&self) -> Category { Category::Navigation }
    fn title(&self, lang: Lang) -> &'static str {
        match lang { Lang::Ja => "ブレッドクラム", Lang::En => "Breadcrumb" }
    }
    fn build(&self, ctx: &DemoCtx) -> Box<dyn Widget> {
        let segs: Vec<&str> = match ctx.lang {
            Lang::Ja => vec!["ホーム", "プロジェクト", "hayate-ui", "widget"],
            Lang::En => vec!["Home", "Projects", "hayate-ui", "widget"],
        };
        Box::new(
            BreadcrumbWidget::new()
                .with_segments(&segs)
                .with_engine(ctx.engine.clone())
                .on_navigate(|i| println!("breadcrumb nav: {i}")),
        )
    }
}

inventory::submit!(DemoEntry { demo: &BreadcrumbDemo });
