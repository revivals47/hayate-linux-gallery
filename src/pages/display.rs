//! Display widgets page: ProgressBar, ImageWidget, StatusBar.

use std::cell::RefCell;
use std::rc::Rc;

use hayate_ui::render::TextEngine;
use hayate_ui::widget::core::Widget;
use hayate_ui::widget::{
    TextWidget, ProgressBarWidget, ImageWidget,
    StatusBar, StatusItem,
    HStack, VStack, Padding,
};

use crate::i18n::L;

fn make_checkerboard() -> (Vec<u8>, u32, u32) {
    let (w, h) = (128u32, 128u32);
    let mut buf = vec![0u8; (w * h * 4) as usize];
    for y in 0..h {
        for x in 0..w {
            let off = ((y * w + x) * 4) as usize;
            let dark = ((x / 16) + (y / 16)) % 2 == 0;
            let v = if dark { 60 } else { 200 };
            buf[off] = v;
            buf[off + 1] = v;
            buf[off + 2] = v;
            buf[off + 3] = 255;
        }
    }
    (buf, w, h)
}

pub fn build(engine: Rc<RefCell<TextEngine>>, l: &L) -> Box<dyn Widget> {
    let section = |text: &str| -> Box<dyn Widget> {
        Box::new(TextWidget::new(text, 18.0).with_engine(engine.clone()))
    };
    let label = |text: &str| -> Box<dyn Widget> {
        Box::new(TextWidget::new(text, 13.0).with_engine(engine.clone()))
    };

    let progress_section = VStack::new(8.0)
        .add(label("30%"))
        .add(Box::new(ProgressBarWidget::new(0.3)))
        .add(label("75%"))
        .add(Box::new(ProgressBarWidget::new(0.75)))
        .add(label("100%"))
        .add(Box::new(ProgressBarWidget::new(1.0)));

    let (pixels, iw, ih) = make_checkerboard();
    let mut img = ImageWidget::new();
    img.load_from_rgba(&pixels, iw, ih);

    let img_section = HStack::new(16.0)
        .add(Box::new(img))
        .add(label(l.checkerboard_desc()));

    let mut status = StatusBar::new(24.0);
    status.add_left(StatusItem::text("Ready"));
    status.add_center(StatusItem::text("hayate-gallery"));
    status.add_right(StatusItem::text("Ln 1, Col 1"));

    let content = VStack::new(16.0)
        .add(section(l.progress_bar()))
        .add(Box::new(progress_section))
        .add(section(l.image_widget()))
        .add(Box::new(img_section))
        .add(section(l.status_bar()))
        .add(Box::new(status));

    Box::new(Padding::all(20.0, Box::new(content)))
}
