//! New widgets page: TextArea, ListView, Breadcrumb, CanvasView, RadioGroup,
//! NavigationList, ThumbnailList.

use std::cell::RefCell;
use std::rc::Rc;

use hayate_ui::render::TextEngine;
use hayate_ui::scroll::delegate::ItemRect;
use hayate_ui::widget::core::Widget;
use hayate_ui::widget::{
    TextWidget, TextAreaWidget, ListViewWidget, BreadcrumbWidget,
    CanvasViewWidget, RadioGroupWidget,
    NavigationListWidget, NavItem, ThumbnailListWidget,
    HStack, VStack, Padding,
};

use crate::i18n::L;

pub fn build(engine: Rc<RefCell<TextEngine>>, l: &L) -> Box<dyn Widget> {
    let section = |text: &str| -> Box<dyn Widget> {
        Box::new(TextWidget::new(text, 18.0).with_engine(engine.clone()))
    };
    let label = |text: &str| -> Box<dyn Widget> {
        Box::new(TextWidget::new(text, 13.0).with_engine(engine.clone()))
    };

    let textarea = TextAreaWidget::new()
        .with_placeholder(l.textarea_placeholder())
        .with_size(300.0, 120.0);

    let listview = ListViewWidget::new(50, 24.0)
        .with_size(300.0, 150.0)
        .with_paint_item(|renderer, engine, rect, index, selected| {
            let lbl = format!("Item #{}", index + 1);
            let bg = if selected { 64 } else if index % 2 == 0 { 36 } else { 42 };
            renderer.fill_rect(&rect, bg, bg, bg + 4, 255);
            renderer.draw_text(engine, &lbl, rect.x + 8.0, rect.y + 2.0,
                13.0, 20.0, 220, 220, 220, rect.width);
        });

    let top_row = HStack::new(16.0)
        .add(Box::new(VStack::new(4.0).add(label(l.text_area())).add(Box::new(textarea))))
        .add(Box::new(VStack::new(4.0).add(label(l.list_view())).add(Box::new(listview))));

    let breadcrumb = BreadcrumbWidget::new()
        .with_segments(&["Home", "Documents", "Projects", "hayate-ui"])
        .with_engine(engine.clone());

    let radio = RadioGroupWidget::new(&["Small", "Medium", "Large"])
        .with_selected(1)
        .with_engine(engine.clone());

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
    }).with_min_size(250.0, 120.0);

    let mid_row = HStack::new(16.0)
        .add(Box::new(VStack::new(4.0).add(label(l.radio_group())).add(Box::new(radio))))
        .add(Box::new(VStack::new(4.0).add(label(l.canvas_view())).add(Box::new(canvas))));

    let nav = NavigationListWidget::new()
        .with_items(vec![
            NavItem::section("Favorites"),
            NavItem::entry_with_icon("🏠", "Home"),
            NavItem::entry_with_icon("📁", "Documents"),
            NavItem::entry_with_icon("📥", "Downloads"),
            NavItem::section("Devices"),
            NavItem::entry_with_icon("💾", "SSD 500GB"),
        ]);

    let thumbs = ThumbnailListWidget::new()
        .with_item_count(10)
        .with_thumb_size(60.0, 60.0)
        .with_paint_thumb(|renderer, rect, index, _selected| {
            let r = ((index * 25) % 256) as u8;
            let g = ((index * 40 + 80) % 256) as u8;
            let b = ((index * 60 + 160) % 256) as u8;
            renderer.fill_rect(&rect, r, g, b, 255);
        });

    let bottom_row = HStack::new(16.0)
        .add(Box::new(VStack::new(4.0).add(label(l.nav_list())).add(Box::new(nav))))
        .add(Box::new(VStack::new(4.0).add(label(l.thumb_list())).add(Box::new(thumbs))));

    let content = VStack::new(16.0)
        .add(section(l.new_widgets()))
        .add(label(l.breadcrumb()))
        .add(Box::new(breadcrumb))
        .add(Box::new(top_row))
        .add(Box::new(mid_row))
        .add(Box::new(bottom_row));

    Box::new(Padding::all(20.0, Box::new(content)))
}
