//! Basic widgets page: ButtonWidget, CheckboxWidget, SwitchWidget, TextWidget.

use std::cell::RefCell;
use std::rc::Rc;

use hayate_ui::render::TextEngine;
use hayate_ui::style::theme::Color;
use hayate_ui::widget::core::Widget;
use hayate_ui::widget::{
    ButtonWidget, ButtonTheme, CheckboxWidget, CheckStyle, SwitchWidget, TextWidget,
    HStack, VStack, Padding,
};

use crate::i18n::L;

pub fn build(engine: Rc<RefCell<TextEngine>>, l: &L) -> Box<dyn Widget> {
    let section = |text: &str| -> Box<dyn Widget> {
        Box::new(TextWidget::new(text, 18.0).with_engine(engine.clone()))
    };
    let label = |text: &str| -> Box<dyn Widget> {
        Box::new(TextWidget::new(text, 12.0).with_engine(engine.clone()))
    };

    // Buttons: presets
    let row1 = HStack::new(6.0)
        .add(Box::new(ButtonWidget::new("Filled")))
        .add(Box::new(ButtonWidget::new("Outlined").theme(ButtonTheme::outlined())))
        .add(Box::new(ButtonWidget::new("Ghost").theme(ButtonTheme::ghost())))
        .add(Box::new(ButtonWidget::new("Pill").theme(ButtonTheme::pill())))
        .add(Box::new(ButtonWidget::new("Elevated").theme(ButtonTheme::elevated())));
    let row2 = HStack::new(6.0)
        .add(Box::new(ButtonWidget::new("Gradient").theme(ButtonTheme::gradient_accent())))
        .add(Box::new(ButtonWidget::new("Glass").theme(ButtonTheme::glass())))
        .add(Box::new(ButtonWidget::new("Neon").theme(ButtonTheme::neon())))
        .add(Box::new(ButtonWidget::new("Soft").theme(ButtonTheme::soft())));
    // Platform reference
    let row3 = HStack::new(6.0)
        .add(Box::new(ButtonWidget::new("Win11").theme(ButtonTheme::win11())))
        .add(Box::new(ButtonWidget::new("Win11 Sec").theme(ButtonTheme::win11_secondary())))
        .add(Box::new(ButtonWidget::new("macOS").theme(ButtonTheme::macos())))
        .add(Box::new(ButtonWidget::new("macOS Sec").theme(ButtonTheme::macos_secondary())));
    let row4 = HStack::new(6.0)
        .add(Box::new(ButtonWidget::new("Material 3").theme(ButtonTheme::material3())))
        .add(Box::new(ButtonWidget::new("Fluent").theme(ButtonTheme::fluent())))
        .add(Box::new(ButtonWidget::new("Adwaita").theme(ButtonTheme::adwaita_suggested())))
        .add(Box::new(ButtonWidget::new("Win95").theme(ButtonTheme::win95())))
        .add(Box::new(ButtonWidget::new("Mac OS 9").theme(ButtonTheme::mac_classic())));

    // Checkbox: 3 styles
    let cb_row = HStack::new(16.0)
        .add(Box::new(VStack::new(4.0)
            .add(label("Filled"))
            .add(Box::new(CheckboxWidget::new(l.option_a())))
            .add(Box::new(CheckboxWidget::new(l.option_b_checked()).checked(true)))))
        .add(Box::new(VStack::new(4.0)
            .add(label("Classic"))
            .add(Box::new(CheckboxWidget::new(l.option_a()).style(CheckStyle::Classic)))
            .add(Box::new(CheckboxWidget::new(l.option_b_checked()).style(CheckStyle::Classic).checked(true)))))
        .add(Box::new(VStack::new(4.0)
            .add(label("Minimal"))
            .add(Box::new(CheckboxWidget::new(l.option_a()).style(CheckStyle::Minimal)))
            .add(Box::new(CheckboxWidget::new(l.option_b_checked()).style(CheckStyle::Minimal).checked(true)))));

    // Switch
    let switch_row = HStack::new(16.0)
        .add(label(l.dark_mode()))
        .add(Box::new(SwitchWidget::new(false)))
        .add(label(l.notifications()))
        .add(Box::new(SwitchWidget::new(true)));

    // CJK text samples
    let cjk = VStack::new(4.0)
        .add(label(l.text_sample()))
        .add(Box::new(TextWidget::new("한국어 렌더링", 14.0).with_engine(engine.clone())))
        .add(Box::new(TextWidget::new("简体中文渲染", 14.0).with_engine(engine.clone())))
        .add(Box::new(TextWidget::new("繁體中文渲染", 14.0).with_engine(engine.clone())));

    let content = VStack::new(10.0)
        .add(section(l.button()))
        .add(Box::new(row1))
        .add(Box::new(row2))
        .add(Box::new(row3))
        .add(Box::new(row4))
        .add(section(l.checkbox()))
        .add(Box::new(cb_row))
        .add(section(l.switch()))
        .add(Box::new(switch_row))
        .add(section(l.text()))
        .add(Box::new(cjk));

    Box::new(Padding::all(14.0, Box::new(content)))
}
