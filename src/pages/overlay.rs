//! Overlay & menu widgets page: Toast, ContextMenu, AlertDialog.

use std::cell::RefCell;
use std::rc::Rc;

use hayate_ui::render::TextEngine;
use hayate_ui::widget::core::Widget;
use hayate_ui::widget::{
    TextWidget, ButtonWidget,
    ToastWidget, ToastLevel,
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

    let mut toast = ToastWidget::new(engine.clone());
    toast.show(l.welcome(), ToastLevel::Info, 5.0);

    let btn_row = HStack::new(12.0)
        .add(Box::new(ButtonWidget::new("Toast")))
        .add(Box::new(ButtonWidget::new("Alert")))
        .add(Box::new(ButtonWidget::new("Context")));

    let content = VStack::new(16.0)
        .add(section(l.toast()))
        .add(Box::new(toast))
        .add(section(l.overlay_widgets()))
        .add(Box::new(btn_row))
        .add(label("ContextMenu / Tooltip / AlertDialog"));

    Box::new(Padding::all(20.0, Box::new(content)))
}
