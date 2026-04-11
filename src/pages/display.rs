//! Display widgets: ProgressBar, ImageWidget, StatusBar, CanvasView,
//! ListView, ThumbnailList.

use std::cell::RefCell;
use std::rc::Rc;

use hayate_ui::render::TextEngine;
use hayate_ui::scroll::delegate::ItemRect;
use hayate_ui::widget::core::Widget;
use hayate_ui::widget::{
    TextWidget, ProgressBarWidget, ImageWidget,
    StatusBar, StatusItem,
    CanvasViewWidget, ListViewWidget, ThumbnailListWidget,
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

    // ProgressBar row
    let progress_row = HStack::new(24.0)
        .add(Box::new(VStack::new(4.0)
            .add(label("30%")).add(Box::new(ProgressBarWidget::new(0.3)))))
        .add(Box::new(VStack::new(4.0)
            .add(label("75%")).add(Box::new(ProgressBarWidget::new(0.75)))))
        .add(Box::new(VStack::new(4.0)
            .add(label("100%")).add(Box::new(ProgressBarWidget::new(1.0)))));

    // Image + CanvasView row
    let (pixels, iw, ih) = make_checkerboard();
    let mut img = ImageWidget::new();
    img.load_from_rgba(&pixels, iw, ih);

    let canvas = CanvasViewWidget::new(|renderer, rect| {
        let bar_w = rect.width / 5.0;
        let heights = [0.3f32, 0.7, 0.5, 0.9, 0.4];
        for (i, &h) in heights.iter().enumerate() {
            let x = rect.x + i as f32 * bar_w + 2.0;
            let bar_h = rect.height * h;
            let y = rect.y + rect.height - bar_h;
            let g = (100.0 + h * 155.0) as u8;
            renderer.fill_rect(&ItemRect::new(x, y, bar_w - 4.0, bar_h), 60, g, 180, 255);
        }
    }).with_min_size(200.0, 100.0);

    let visual_row = HStack::new(16.0)
        .add(Box::new(VStack::new(4.0)
            .add(label(l.image_widget())).add(Box::new(img))))
        .add(Box::new(VStack::new(4.0)
            .add(label(l.canvas_view())).add(Box::new(canvas))));

    // ListView + ThumbnailList row
    let listview = ListViewWidget::new(30, 20.0)
        .with_size(300.0, 120.0)
        .with_paint_item(|renderer, engine, rect, index, selected| {
            let lbl = format!("Item #{}", index + 1);
            let bg = if selected { 64 } else if index % 2 == 0 { 36 } else { 42 };
            renderer.fill_rect(&rect, bg, bg, bg + 4, 255);
            renderer.draw_text(engine, &lbl, rect.x + 6.0, rect.y + 1.0,
                12.0, 18.0, 220, 220, 220, rect.width);
        });

    let thumbs = ThumbnailListWidget::new()
        .with_item_count(10)
        .with_thumb_size(50.0, 50.0)
        .with_paint_thumb(|renderer, rect, index, _selected| {
            let r = ((index * 25) % 256) as u8;
            let g = ((index * 40 + 80) % 256) as u8;
            let b = ((index * 60 + 160) % 256) as u8;
            renderer.fill_rect(&rect, r, g, b, 255);
        });

    let list_row = HStack::new(16.0)
        .add(Box::new(VStack::new(4.0)
            .add(label(l.list_view())).add(Box::new(listview))))
        .add(Box::new(VStack::new(4.0)
            .add(label(l.thumb_list())).add(Box::new(thumbs))));

    // StatusBar
    let mut status = StatusBar::new(24.0);
    status.add_left(StatusItem::text("Ready"));
    status.add_center(StatusItem::text("hayate-gallery"));
    status.add_right(StatusItem::text("Ln 1, Col 1"));

    let content = VStack::new(12.0)
        .add(section(l.progress_bar()))
        .add(Box::new(progress_row))
        .add(Box::new(visual_row))
        .add(Box::new(list_row))
        .add(section(l.status_bar()))
        .add(Box::new(status));

    Box::new(Padding::all(16.0, Box::new(content)))
}
