//! Basic widgets page: ButtonWidget, CheckboxWidget, SwitchWidget, TextWidget.

use std::cell::RefCell;
use std::rc::Rc;

use hayate_ui::render::TextEngine;
use hayate_ui::style::theme::Color;
use hayate_ui::widget::core::Widget;
use hayate_ui::widget::{
    AppTheme, ButtonWidget, ButtonTheme, CheckboxWidget, CheckStyle, SwitchWidget, TextWidget,
    HStack, VStack, Padding,
};

use crate::i18n::L;

pub fn build(engine: Rc<RefCell<TextEngine>>, l: &L, theme: Option<&AppTheme>) -> Box<dyn Widget> {
    let themed = theme.is_some();
    let (sec_size, lbl_size) = if themed { (14.0, 11.0) } else { (18.0, 12.0) };
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

    // Buttons
    let (row1, row2, row3, row4) = if let Some(t) = theme {
        // Themed mode: all buttons use the theme
        let r1 = HStack::new(6.0)
            .add(Box::new(ButtonWidget::new("Button 1").theme(t.button.clone())))
            .add(Box::new(ButtonWidget::new("Button 2").theme(t.button.clone())))
            .add(Box::new(ButtonWidget::new("Button 3").theme(t.button.clone())));
        (r1, HStack::new(0.0), HStack::new(0.0), HStack::new(0.0))
    } else {
        let r1 = HStack::new(6.0)
            .add(Box::new(ButtonWidget::new("Filled")))
            .add(Box::new(ButtonWidget::new("Outlined").theme(ButtonTheme::outlined())))
            .add(Box::new(ButtonWidget::new("Ghost").theme(ButtonTheme::ghost())))
            .add(Box::new(ButtonWidget::new("Pill").theme(ButtonTheme::pill())))
            .add(Box::new(ButtonWidget::new("Elevated").theme(ButtonTheme::elevated())));
        let r2 = HStack::new(6.0)
            .add(Box::new(ButtonWidget::new("Gradient").theme(ButtonTheme::gradient_accent())))
            .add(Box::new(ButtonWidget::new("Glass").theme(ButtonTheme::glass())))
            .add(Box::new(ButtonWidget::new("Neon").theme(ButtonTheme::neon())))
            .add(Box::new(ButtonWidget::new("Soft").theme(ButtonTheme::soft())));
        let r3 = HStack::new(6.0)
            .add(Box::new(ButtonWidget::new("Win11").theme(ButtonTheme::win11())))
            .add(Box::new(ButtonWidget::new("Win11 Sec").theme(ButtonTheme::win11_secondary())))
            .add(Box::new(ButtonWidget::new("macOS").theme(ButtonTheme::macos())))
            .add(Box::new(ButtonWidget::new("macOS Sec").theme(ButtonTheme::macos_secondary())));
        let r4 = HStack::new(6.0)
            .add(Box::new(ButtonWidget::new("Material 3").theme(ButtonTheme::material3())))
            .add(Box::new(ButtonWidget::new("Fluent").theme(ButtonTheme::fluent())))
            .add(Box::new(ButtonWidget::new("Adwaita").theme(ButtonTheme::adwaita_suggested())))
            .add(Box::new(ButtonWidget::new("Win95").theme(ButtonTheme::win95())))
            .add(Box::new(ButtonWidget::new("Mac OS 9").theme(ButtonTheme::mac_classic())));
        (r1, r2, r3, r4)
    };

    // Checkbox
    let cb_row = if let Some(t) = theme {
        HStack::new(16.0)
            .add(Box::new(CheckboxWidget::new(l.option_a()).theme(t.checkbox.clone())))
            .add(Box::new(CheckboxWidget::new(l.option_b_checked()).theme(t.checkbox.clone()).checked(true)))
            .add(Box::new(CheckboxWidget::new(l.option_c()).theme(t.checkbox.clone())))
    } else {
        HStack::new(16.0)
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
                .add(Box::new(CheckboxWidget::new(l.option_b_checked()).style(CheckStyle::Minimal).checked(true)))))
    };

    // Switch
    let switch_row = if let Some(t) = theme {
        HStack::new(16.0)
            .add(label("OFF"))
            .add(Box::new(SwitchWidget::new(false).theme(t.switch.clone())))
            .add(label("ON"))
            .add(Box::new(SwitchWidget::new(true).theme(t.switch.clone())))
    } else {
        HStack::new(16.0)
            .add(label(l.dark_mode()))
            .add(Box::new(SwitchWidget::new(false)))
            .add(label(l.notifications()))
            .add(Box::new(SwitchWidget::new(true)))
    };

    // CJK
    let cjk = VStack::new(4.0)
        .add(label(l.text_sample()))
        .add(Box::new(TextWidget::new("한국어 렌더링", 14.0).with_engine(engine.clone())))
        .add(Box::new(TextWidget::new("简体中文渲染", 14.0).with_engine(engine.clone())))
        .add(Box::new(TextWidget::new("繁體中文渲染", 14.0).with_engine(engine.clone())));

    let mut content = VStack::new(10.0)
        .add(section(l.button()))
        .add(Box::new(row1));
    if theme.is_none() {
        content = content
            .add(Box::new(row2))
            .add(Box::new(row3))
            .add(Box::new(row4));
    }
    content = content
        .add(section(l.checkbox()))
        .add(Box::new(cb_row))
        .add(section(l.switch()))
        .add(Box::new(switch_row))
        .add(section(l.text()))
        .add(Box::new(cjk));

    Box::new(Padding::all(14.0, Box::new(content)))
}
