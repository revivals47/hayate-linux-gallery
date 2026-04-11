//! Basic widgets page: TextWidget, ButtonWidget, CheckboxWidget, SwitchWidget.

use std::cell::RefCell;
use std::rc::Rc;

use hayate_ui::render::TextEngine;
use hayate_ui::widget::core::Widget;
use hayate_ui::widget::{
    ButtonWidget, CheckboxWidget, SwitchWidget, TextWidget,
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

    let buttons_row = HStack::new(12.0)
        .add(Box::new(ButtonWidget::new(l.default_button())))
        .add(Box::new(ButtonWidget::new(l.action())))
        .add(Box::new(ButtonWidget::new(l.disabled())));

    let checkbox_row = HStack::new(16.0)
        .add(Box::new(CheckboxWidget::new(l.option_a())))
        .add(Box::new(CheckboxWidget::new(l.option_b_checked()).checked(true)))
        .add(Box::new(CheckboxWidget::new(l.option_c())));

    let switch_row = HStack::new(16.0)
        .add(label(l.dark_mode()))
        .add(Box::new(SwitchWidget::new(false)))
        .add(label(l.notifications()))
        .add(Box::new(SwitchWidget::new(true)));

    let content = VStack::new(16.0)
        .add(section(l.button()))
        .add(Box::new(buttons_row))
        .add(section(l.checkbox()))
        .add(Box::new(checkbox_row))
        .add(section(l.switch()))
        .add(Box::new(switch_row))
        .add(section(l.text()))
        .add(label(l.text_sample()))
        .add(Box::new(TextWidget::new(l.larger_text(), 18.0).with_engine(engine.clone())))
        .add(Box::new(TextWidget::new("Monospace", 14.0).with_engine(engine)));

    Box::new(Padding::all(20.0, Box::new(content)))
}
