//! Layout widgets page: VStack, HStack, GridLayout, SplitView, Padding, Spacer.

use std::cell::RefCell;
use std::rc::Rc;

use hayate_ui::render::TextEngine;
use hayate_ui::widget::core::Widget;
use hayate_ui::widget::{
    TextWidget, ButtonWidget, GridLayout,
    HStack, VStack, Padding, Spacer, SplitViewWidget, SplitOrientation,
};

use crate::i18n::L;

pub fn build(engine: Rc<RefCell<TextEngine>>, l: &L) -> Box<dyn Widget> {
    let section = |text: &str| -> Box<dyn Widget> {
        Box::new(TextWidget::new(text, 18.0).with_engine(engine.clone()))
    };
    let label = |text: &str| -> Box<dyn Widget> {
        Box::new(TextWidget::new(text, 13.0).with_engine(engine.clone()))
    };

    let hstack = HStack::new(8.0)
        .add(Box::new(ButtonWidget::new("L")))
        .add(Box::new(ButtonWidget::new("C")))
        .add(Box::new(ButtonWidget::new("R")));

    let vstack_demo = VStack::new(8.0)
        .add(Box::new(ButtonWidget::new("Top")))
        .add(Box::new(Spacer::with_min(20.0)))
        .add(Box::new(ButtonWidget::new("Bot")));

    let stacks_row = HStack::new(32.0)
        .add(Box::new(VStack::new(4.0).add(label("HStack")).add(Box::new(hstack))))
        .add(Box::new(VStack::new(4.0).add(label("VStack")).add(Box::new(vstack_demo))));

    let mut grid = GridLayout::new(3, 8.0);
    for i in 1..=6 {
        grid.push(Box::new(ButtonWidget::new(format!("{i}"))));
    }

    let split = SplitViewWidget::new(
        Box::new(Padding::all(8.0, Box::new(TextWidget::new(l.left_pane(), 14.0).with_engine(engine.clone())))),
        Box::new(Padding::all(8.0, Box::new(TextWidget::new(l.right_pane(), 14.0).with_engine(engine.clone())))),
        SplitOrientation::Horizontal,
    );

    let content = VStack::new(16.0)
        .add(section(l.hstack_vstack()))
        .add(Box::new(stacks_row))
        .add(section(l.grid()))
        .add(Box::new(grid))
        .add(section(l.split_view()))
        .add(Box::new(split));

    Box::new(Padding::all(20.0, Box::new(content)))
}
