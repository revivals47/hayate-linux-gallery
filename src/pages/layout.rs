//! Layout widgets page: VStack, HStack, GridLayout, SplitView, Padding, Spacer.

use std::cell::RefCell;
use std::rc::Rc;

use hayate_ui::render::TextEngine;
use hayate_ui::widget::core::Widget;
use hayate_ui::widget::{
    AppTheme, TextWidget, ButtonWidget, GridLayout,
    HStack, VStack, Padding, Spacer, SplitViewWidget, SplitOrientation,
};

use crate::i18n::L;

pub fn build(engine: Rc<RefCell<TextEngine>>, l: &L, theme: Option<&AppTheme>) -> Box<dyn Widget> {
    let themed = theme.is_some();
    let (sec_size, lbl_size) = if themed { (14.0, 11.0) } else { (18.0, 13.0) };
    let section = |text: &str| -> Box<dyn Widget> {
        let mut w = TextWidget::new(text, sec_size).with_engine(engine.clone());
        if themed { w = w.with_color(0, 0, 0); }
        Box::new(w)
    };
    let label = |text: &str| -> Box<dyn Widget> {
        let mut w = TextWidget::new(text, lbl_size).with_engine(engine.clone());
        if themed { w = w.with_color(0, 0, 0); }
        Box::new(w)
    };
    let btn = |lbl: &str| -> Box<dyn Widget> {
        let mut b = ButtonWidget::new(lbl);
        if let Some(t) = theme { b = b.theme(t.button.clone()); }
        Box::new(b)
    };

    let hstack = HStack::new(8.0)
        .add(btn("L"))
        .add(btn("C"))
        .add(btn("R"));

    let vstack_demo = VStack::new(8.0)
        .add(btn("Top"))
        .add(Box::new(Spacer::with_min(20.0)))
        .add(btn("Bot"));

    let stacks_row = HStack::new(32.0)
        .add(Box::new(VStack::new(4.0).add(label("HStack")).add(Box::new(hstack))))
        .add(Box::new(VStack::new(4.0).add(label("VStack")).add(Box::new(vstack_demo))));

    let mut grid = GridLayout::new(3, 8.0);
    for i in 1..=6 {
        let mut b = ButtonWidget::new(format!("{i}"));
        if let Some(t) = theme { b = b.theme(t.button.clone()); }
        grid.push(Box::new(b));
    }

    let mk_pane_text = |text: &str| -> Box<dyn Widget> {
        let mut w = TextWidget::new(text, 14.0).with_engine(engine.clone());
        if themed { w = w.with_color(0, 0, 0); }
        Box::new(w)
    };
    let split = SplitViewWidget::new(
        Box::new(Padding::all(8.0, mk_pane_text(l.left_pane()))),
        Box::new(Padding::all(8.0, mk_pane_text(l.right_pane()))),
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
