//! Overlay & menu widgets page.

use std::cell::RefCell;
use std::rc::Rc;

use hayate_ui::render::TextEngine;
use hayate_ui::widget::core::Widget;
use hayate_ui::widget::{
    AppTheme, TextWidget, ButtonWidget,
    ToastWidget, ToastLevel,
    HStack, VStack, Padding,
};

use crate::i18n::L;
use crate::live::LiveText;

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

    let mut toast = ToastWidget::new(engine.clone());
    toast.show(l.welcome(), ToastLevel::Info, 5.0);

    // Click counter for overlay-trigger buttons (toast/alert/context).
    // We cannot cheaply re-trigger the Toast from this scope (the widget
    // is moved into the tree), so we expose a click count instead; it
    // proves the buttons fire their callbacks.
    let clicks: Rc<RefCell<(u32, u32, u32)>> = Rc::new(RefCell::new((0, 0, 0)));
    let c1 = clicks.clone();
    let c2 = clicks.clone();
    let c3 = clicks.clone();

    let mk_btn = |lbl: &str, cb: Box<dyn FnMut() + 'static>| -> Box<dyn Widget> {
        let mut b = ButtonWidget::new(lbl).on_click(cb);
        if let Some(t) = theme { b = b.theme(t.button.clone()); }
        Box::new(b)
    };

    let btn_row = HStack::new(12.0)
        .add(mk_btn("Toast", Box::new(move || c1.borrow_mut().0 += 1)))
        .add(mk_btn("Alert", Box::new(move || c2.borrow_mut().1 += 1)))
        .add(mk_btn("Context", Box::new(move || c3.borrow_mut().2 += 1)));

    let live = Box::new(LiveText::new(clicks.clone(),
        |(a, b, c): &(u32, u32, u32)| format!("Toast:{a}  Alert:{b}  Context:{c}"),
        lbl_size)
        .with_engine(engine.clone())
        .with_color(if themed {0} else {220}, if themed {0} else {220}, if themed {0} else {220})
        .with_width(280.0));

    let content = VStack::new(16.0)
        .add(section(l.toast()))
        .add(Box::new(toast))
        .add(section(l.overlay_widgets()))
        .add(Box::new(btn_row))
        .add(live)
        .add(label("ContextMenu / Tooltip / AlertDialog"));

    Box::new(Padding::all(20.0, Box::new(content)))
}
