use hayate_ui::widget::core::Widget;
use hayate_ui::widget::{ImageWidget, ScaleMode};

use crate::demo::{Category, Demo, DemoCtx, DemoEntry, Lang};

struct ImageDemo;

impl Demo for ImageDemo {
    fn id(&self) -> &'static str { "image" }
    fn category(&self) -> Category { Category::Display }
    fn title(&self, lang: Lang) -> &'static str {
        match lang { Lang::Ja => "画像", Lang::En => "Image" }
    }
    fn build(&self, _ctx: &DemoCtx) -> Box<dyn Widget> {
        let w: u32 = 240;
        let h: u32 = 120;
        let mut data = vec![0u8; (w * h * 4) as usize];
        for y in 0..h {
            for x in 0..w {
                let idx = ((y * w + x) * 4) as usize;
                data[idx]     = (x * 255 / w.max(1)) as u8;
                data[idx + 1] = (y * 255 / h.max(1)) as u8;
                data[idx + 2] = ((x + y) & 0xff) as u8;
                data[idx + 3] = 255;
            }
        }
        let mut img = ImageWidget::new();
        img.load_from_rgba(&data, w, h);
        img.set_scale_mode(ScaleMode::Contain);
        Box::new(img)
    }
}

inventory::submit!(DemoEntry { demo: &ImageDemo });
