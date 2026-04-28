use hayate_ui::widget::CanvasViewWidget;
use hayate_ui::widget::core::Widget;

use crate::demo::{Category, Demo, DemoCtx, DemoEntry, Lang};

struct CanvasDemo;

impl Demo for CanvasDemo {
    fn id(&self) -> &'static str { "canvas" }
    fn category(&self) -> Category { Category::Display }
    fn title(&self, lang: Lang) -> &'static str {
        match lang { Lang::Ja => "キャンバス", Lang::En => "Canvas" }
    }
    fn build(&self, _ctx: &DemoCtx) -> Box<dyn Widget> {
        Box::new(
            CanvasViewWidget::new(|r, rect| {
                r.fill_rect(&rect, 40, 50, 80, 255);
                let cx = rect.x + rect.width  * 0.5;
                let cy = rect.y + rect.height * 0.5;
                for i in 0..6 {
                    let w = 20.0 + (i as f32) * 8.0;
                    let c = 60 + (i * 28) as u8;
                    let r_rect = hayate_ui::scroll::delegate::ItemRect::new(
                        cx - w * 0.5, cy - w * 0.5, w, w,
                    );
                    r.fill_rect(&r_rect, c, c / 2, 255 - c / 2, 255);
                }
            })
            .with_min_size(360.0, 140.0)
            .on_click(|x, y| println!("canvas click @ ({x:.0},{y:.0})")),
        )
    }
}

inventory::submit!(DemoEntry { demo: &CanvasDemo });
